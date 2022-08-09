use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use aws_sdk_config::types::SdkError;
use aws_sdk_dynamodb::{model::AttributeValue, output::QueryOutput, error::QueryError};
use aws_sdk_s3::{output::DeleteObjectOutput, error::DeleteObjectError};
use chrono::{DateTime, TimeZone, Duration, Utc};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::{info, debug, error};

static TABLE_NAME: &str = "wave_file";
static GLOBAL_INDEX: &str = "date-time-index";
static BUCKET_NAME: &str = "cloud-wav-file-bucket";
static DELETE_AFTER: i64 = 2;

async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    let (payload, _) = event.into_parts();

    // initializing clients
    let config = aws_config::load_from_env().await;
    let db_client = aws_sdk_dynamodb::Client::new(&config);
    let s3_client = aws_sdk_s3::Client::new(&config);

    // delete all files that are marked as downloaded and where created at the day of the request
    let deleted_downloaded = delete_downloaded(&db_client, &s3_client, &payload).await?;

    // delete all files that are older than DELETE_AFTER days !! NEED TO CHECK IF NANOSECONDS ARE CORRECT !!
    let deleted_old = delete_old(&s3_client, &payload).await?;

    info!("Deleted {} files that where already downloaded!\nDeleted {} files that were old and still in bucket", 
          deleted_downloaded.len(), deleted_old.len());
    debug!("Deleleted ids: \n{:?}\n{:?}", deleted_downloaded, deleted_old);

    Ok(())
}

async fn delete_downloaded (
    db_client: &aws_sdk_dynamodb::Client, 
    s3_client: &aws_sdk_s3::Client,
    payload: &CloudWatchEvent) 
-> Result<Vec<String>, Error> {
    // get timestamp of request
    let (date, time) = string_from_date_time(payload.time);
    info!("Date: {}, time: {}", date, time);

    let query_results = query_for_date("date", &date, db_client).await?;
    debug!("Query results: {:?}", query_results);
    info!("Found {} items for querying date.", query_results.count);
    
    let file_ids = match query_results.items {
        Some(items) => items
                        .iter()
                        .filter(|item| *item["is_downloaded"].as_bool().unwrap())
                        .map(|item| item["id"].as_s().unwrap().to_owned())
                        .collect(),
        None => vec![],
    };

    debug!("Found ids: {:?}", file_ids);

    let mut deleted_files = vec![];

    // delete found files from bucket
    for id in file_ids {
        let file_name = id.clone() + ".wav";
        match delete_from_bucket(&file_name, &s3_client).await {
            Ok(_) => { 
                info!("Deleted Object!");
                deleted_files.push(id);
            },
            Err(e) => error!("Error while handling delete request: {}", e),
        }
    }
    Ok(deleted_files)
}

async fn delete_old (
    s3_client: &aws_sdk_s3::Client,
    payload: &CloudWatchEvent) 
-> Result<Vec<String>, Error> {
    let delete_date = payload.time.checked_sub_signed(Duration::days(DELETE_AFTER)).unwrap();
    info!("Deleting everything older than: {:?}", delete_date);

    let list_output = s3_client.list_objects().bucket(BUCKET_NAME).send().await?;
    let mut deleted_files = vec![];
    if let Some(files) = list_output.contents {
        for file in files {
            // skip files that are not old enough
            let cmp_result = compare_datetimes(file.last_modified.unwrap(), delete_date);
            if cmp_result > 0 {
                info!("file not old enough, skipping: {:?}", file.key); // todo change to debug
                continue;
            }

            // if file key is none for some reason
            if file.key.is_none() {
                error!("Found empty key, please inspect bucket");
                continue;
            }

            match delete_from_bucket(file.key.as_ref().unwrap(), &s3_client).await {
                Ok(_) => {
                    info!("Deleted Object!");
                    deleted_files.push(file.key().unwrap().to_owned());
                },
                Err(e) => error!("Error while handling delete request: {}", e),
            }
        }
    }
    Ok(deleted_files)
}

/// Queries the database for entries that match are certain date.
/// value needs to be in the form of "yyyy-mm-dd"
/// Column is the name of the partition key of the index
async fn query_for_date(partition_key: &str, value: &str, client: &aws_sdk_dynamodb::Client)
-> Result<QueryOutput, SdkError<QueryError>> {
    client
        .query()
        .table_name(TABLE_NAME)
        .index_name(GLOBAL_INDEX)
        .key_condition_expression("#dt = :ymd")
        .expression_attribute_names("#dt", partition_key)
        .expression_attribute_values(":ymd", AttributeValue::S(value.to_owned()))
        .send().await
}

/// Compares two dates with each other, 
/// returns 0 if they're equal
/// returns negative if lhs is smaller than rhs
/// returns positive if lhs is larger than rhs
fn compare_datetimes<Tz: TimeZone>(lhs: aws_smithy_types::DateTime, rhs: chrono::DateTime<Tz>) -> i128 {
    // todo: rewrite as generic fn
    let lhs_nanos = lhs.as_nanos();
    let rhs_nanos = rhs.timestamp_nanos() as i128;
    lhs_nanos - rhs_nanos
}

async fn delete_from_bucket(key: &str, client: &aws_sdk_s3::Client) 
-> Result<DeleteObjectOutput, SdkError<DeleteObjectError>> {
    client
        .delete_object()
        .bucket(BUCKET_NAME)
        .key(key)
        .send().await
}

fn string_from_date_time<T: TimeZone>(dt: DateTime<T>) -> (String, String) 
where T::Offset: std::fmt::Display, chrono::DateTime<T>: From<chrono::DateTime<Utc>>
{
    // we are interested in the items from the previous day
    let dt_prev = dt.checked_sub_signed(Duration::days(1)).or(Some(Utc::now().into())).unwrap();
    (dt_prev.format("%F").to_string(), dt_prev.format("%T").to_string())
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

    run(service_fn(function_handler)).await
}

#[test]
fn test_string_from_date_time() {
    let dt = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z").unwrap();
    assert_eq!(("1994-08-05".to_string(), "08:00:00".to_string()), string_from_date_time(dt));
}

#[test]
fn test_compare_datetimes() {
    use aws_smithy_types::date_time::Format;

    let lhs = aws_smithy_types::DateTime::from_str("1996-12-19T16:39:57-08:00", Format::DateTime).unwrap();
    let rhs = chrono::DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap();
    let rhs_sub = rhs.checked_sub_signed(Duration::days(2)).unwrap();

    assert!(compare_datetimes(lhs, rhs_sub) > 0);
}