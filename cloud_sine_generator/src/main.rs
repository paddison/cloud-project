// TODOS:

// creat meaningful file name
// don't hard code bucket names, but set via env
// integrate with db to store metadata
// try to write integration tests
// improve errormessages
// find better way to create file paths
// 

use std::{fmt::Display, path::{Path, PathBuf}};

use aws_sdk_s3::types::ByteStream;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use tracing::{info, error};
use serde_json::{json, Value};
use sine_generator::{data_formats::{WavSpec, WavData} , frequency_writer::{SineWavSpec, self}};

const BUCKET_NAME: &str = "cloud-wav-file-bucket";

#[derive(Debug)]
struct WavSpecErr(&'static str);

impl WavSpecErr {
    fn new_default() -> Self {
        WavSpecErr("Error creating WavSpec, got invalid values/fields") 
    }
}

impl Display for WavSpecErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.0)
    }
}

impl std::error::Error for WavSpecErr {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    // disabling time is handy because CloudWatch will add the ingestion time.
    .without_time()
    .init();

    let func = service_fn(handle_event);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handle_event(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (mut event, _) = event.into_parts();
    
    // TODO refactor into function maybe
    info!("Looking for WavSpec in event");
    let wav_spec: WavSpec = serde_json::from_value(
        event
            .get_mut("wav_spec")
            .ok_or(WavSpecErr("WavSpec field missing"))?
            .take())?;
    
    info!("Looking for WavData in event");
    let wav_data: WavData = serde_json::from_value(
        event
            .get_mut("wav_data")
            .ok_or(WavSpecErr("WavData field missing"))?
            .take())?;

    let id: String = serde_json::from_value(
        event
            .get_mut("wav_id")
            .ok_or(WavSpecErr("Id field missing"))?
            .take())?;
    
    info!("Creating SineSpec");
    let sine_spec = match SineWavSpec::new(&wav_spec, &wav_data) {
        Some(spec) => spec,
        None => { 
            error!("Supplied data is invalid, cannot create SineWavSpec."); 
            return Err(Box::new(WavSpecErr::new_default()));
        }
    }; 
    
    info!("Creating WavWriter");
    let file_name: PathBuf = [r"/tmp", &(id.clone() + ".wav")].iter().collect(); // lambda functions only have write access to tmp folder
    let writer = sine_generator::wav_writer::WavWriter::new_with_spec(wav_spec, file_name.to_str().unwrap())?;

    info!("Writing to file...");
    frequency_writer::write_wave(sine_spec, writer)?;

    store_in_bucket(file_name.as_path()).await?;

    Ok(json!({ "message": format!("Stored Wav File in Bucket"), "id": id }))
}

async fn store_in_bucket(file_path: &Path) -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    info!("Getting file {:?} from lambda.", file_path);
    let file = ByteStream::from_path(file_path).await?;

    info!("Putting file into bucket...");
    let _ = client
        .put_object()
        .bucket(BUCKET_NAME)
        .key(file_path.file_name().unwrap().to_str().unwrap())
        .body(file)
        .send().await?;

    info!("Successfully put file into bucket");

    Ok(())
}

#[test]
fn test_deserialize_wavdata() {
    let obj = json!({
        "frequencies": [1 as u16,2 as u16,3 as u16],
        "duration": 10 as u16,
        "volume": 1.5 as f64
    });
    let wav_data: Result<WavData, _> = serde_json::from_str(&obj.to_string());

    assert!(wav_data.is_ok());
}