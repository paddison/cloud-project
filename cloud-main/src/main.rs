use std::{fmt::Display, error, collections::HashMap};

use aws_sdk_dynamodb::{model::AttributeValue, output::PutItemOutput, error::PutItemError, types::SdkError};
use chrono::{Utc, Datelike, Timelike};
use lambda_runtime::{service_fn, LambdaEvent, Error};
use aws_sdk_lambda::{types::Blob, model::InvocationType};
use serde_json::{json, Value};
use sine_generator::data_formats::{WavData, WavSpec, Verifiable};
use tracing::{info, debug};

const GENERATOR_LAMBDA: Option<&str> = option_env!("TF_VAR_GENERATOR_LAMBDA");
const GENERATOR_LAMBDA_FALLBACK: &str = "cloud-sine-generator";
const TABLE_NAME: Option<&str> = option_env!("TF_VAR_TABLE_NAME");
const TABLE_NAME_FALLBACK: &str = "cloud-wave-file";
const ID_SEPARATOR: &str = "_";

#[derive(Debug)]
struct InvalidRequestErr(&'static str);

impl Display for InvalidRequestErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for InvalidRequestErr {}

#[derive(Debug)]
struct DBItem {
    id: AttributeValue,
    is_downloaded: AttributeValue,
    request_id: AttributeValue,
    specs: AttributeValue,
    date: AttributeValue,
    time: AttributeValue,
}

impl DBItem {
    pub fn new(partition_key: &str, 
            context: &lambda_runtime::Context, 
            data: Value, 
            spec: Value, 
            (a_date, a_time): (String, String)) -> Self {

        let is_downloaded = AttributeValue::Bool(false);
        let request_id = AttributeValue::S(context.request_id.clone());
        let id = AttributeValue::S(partition_key.to_owned());
        let data = value_to_item(data);
        let spec = value_to_item(spec);
        let specs = AttributeValue::M(HashMap::from([("wav_spec".to_owned(), spec), ("wav_data".to_owned(), data)]));
        let date = AttributeValue::S(a_date);
        let time = AttributeValue::S(a_time);
        DBItem { id, is_downloaded, request_id, specs, date, time }
    }
}

impl From<DBItem> for HashMap<String, AttributeValue> {
    fn from(item: DBItem) -> Self { 
        HashMap::from([
            ("id".to_owned(), item.id),
            ("is_downloaded".to_owned(), item.is_downloaded),
            ("request_id".to_owned(), item.request_id),
            ("specs".to_owned(), item.specs),
            ("date".to_owned(), item.date),
            ("time".to_owned(), item.time),
        ])
     }
}

async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {

    info!("Invoked lamba, loading config and intializing clients...");
    let config = aws_config::load_from_env().await;
    let lambda_client = aws_sdk_lambda::Client::new(&config);
    let db_client = aws_sdk_dynamodb::Client::new(&config);

    let (body, context) = event.into_parts();
    debug!("Request Body: {:?}", body);

    info!("Verifying request data");
    let (spec, _) = verify_specs(&body)?;

    info!("Creating entry for dynamoDB");
    let partition_key = create_partition_key(&spec, &context.request_id);
    let item = DBItem::new(&partition_key, &context, body["wav_data"].clone(), body["wav_spec"].clone(), get_date_time());

    // store in dynamo db
    info!("Inserting into dynamoDB");
    let request = store_item_to_db(&db_client, item).await?;
    debug!("DB Itemoutput:\n{:?}", request);

    let lambda_payload = json!({ "wav_id": partition_key, "wav_data": body["wav_data"], "wav_spec": body["wav_spec"] });
    
    info!("Invoking lambda with:\n{:?}", lambda_payload);
    let lambda = lambda_client
        .invoke()
        .invocation_type(InvocationType::Event)
        .function_name(GENERATOR_LAMBDA.unwrap_or(GENERATOR_LAMBDA_FALLBACK))
        .payload(Blob::new(lambda_payload.to_string()))
        .send()
        .await?; 
    
    debug!("Lambda output {:?}", lambda);

    let response = json!({"id": partition_key, "request_id": context.request_id});

    info!("Response: {}", response);
    Ok(response)
}

fn create_partition_key(spec: &WavSpec, request_id: &str) -> String {
    let prefix = match request_id.split('-').next() {
        Some(prefix) => prefix.to_owned(),
        None => "0000000".to_owned(),
    };
    format!("{}{ID_SEPARATOR}{}{ID_SEPARATOR}{}{ID_SEPARATOR}{}", prefix, spec.number_of_channels, spec.sample_rate, spec.bits_per_sample)
}

async fn store_item_to_db(client: &aws_sdk_dynamodb::Client, item: DBItem) -> Result<PutItemOutput, SdkError<PutItemError>> {
    client
        .put_item()
        .table_name(TABLE_NAME.unwrap_or(TABLE_NAME_FALLBACK))
        .set_item(Some(item.into()))
        .send().await
} 

fn verify_specs(body: &Value) -> Result<(WavSpec, WavData), InvalidRequestErr> {
    let (data, spec): (WavData, WavSpec) = match (body.get("wav_data"), body.get("wav_spec")) {
        (Some(data), Some(spec)) => match (serde_json::from_value(data.clone()) , serde_json::from_value(spec.clone())) {
            (Ok(data), Ok(spec)) => (data, spec),
            (_, _) => return Err(InvalidRequestErr("data or spec invalid format")),
        },
        (_, _) => return Err(InvalidRequestErr("data or spec not found in request")),
    };

    if !(data.is_valid() && spec.is_valid()) {
        return Err(InvalidRequestErr("data or spec contain invalid data"));
    }

    Ok((spec, data))
}

// found on aws examples on https://github.com/awslabs/aws-sdk-rust/blob/main/examples/dynamodb/src/bin/movies.rs
// saved my life
fn value_to_item(value: Value) -> AttributeValue {
    match value {
        Value::Null => AttributeValue::Null(true),
        Value::Bool(b) => AttributeValue::Bool(b),
        Value::Number(n) => AttributeValue::N(n.to_string()),
        Value::String(s) => AttributeValue::S(s),
        Value::Array(a) => AttributeValue::L(a.into_iter().map(value_to_item).collect()),
        Value::Object(o) => {
            AttributeValue::M(o.into_iter().map(|(k, v)| (k, value_to_item(v))).collect())
        }
    }
}

// get current date and time, to store into the database
fn get_date_time() -> (String, String) {
    let now = Utc::now();

    let date = format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());
    let time = format!("{:02}:{:02}:{:02}", hour12_to_hour24(now.hour12()), now.minute(), now.second());

    (date, time)
}

// converts hours from 1-12 to 0-23
fn hour12_to_hour24((is_pm, hr): (bool, u32)) -> u32 {
    if is_pm { hr + 11 } else { hr - 1 }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // run(service_fn(function_handler)).await
    lambda_runtime::run(service_fn(function_handler)).await?;
    Ok(())
}

#[test]
fn test_create_db_item() {

    let request = json!({
        "wav_data": {
            "duration": 30 as u16,
            "frequency": [440 as u16, 660 as u16],
            "volume": 0.9 as f64,
        },
        "wav_spec": {
            "bits_per_sample": 8 as u16,
            "number_of_channels": 2 as u16,
            "sample_rate": 8000 as u16,
        }
    });

    let data = request["wav_data"].clone();
    let spec = request["wav_spec"].clone();
    let context = lambda_runtime::Context::default();

    let item = DBItem::new("123", &context, data, spec, ("2022-02-04".to_owned(), "12:12:12".to_owned()));
    println!("{:?}", item);
}


#[test]
fn test_create_partition_key() {
    // let data = WavData{ frequencies: vec![1, 2, 3], duration: 2, volume: 0.7};
    let spec = WavSpec{ number_of_channels: 2, bits_per_sample: 16, sample_rate: 23000};
    let request_id = "567fab82-770a-44ef-8aab-d434a0b07a33";

    let partition_key = create_partition_key(&spec, request_id);
    let expected = format!("567fab82{ID_SEPARATOR}2{ID_SEPARATOR}23000{ID_SEPARATOR}16");
    assert_eq!(partition_key, expected);
}