mod request;
mod test_case_file;

use request::create_client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions, io::Write, path::Path};
use test_case_file::{get_test_cases, read_test_case_file};

#[tokio::main]
async fn main() {
    match run_test_cases_batch().await {
        Ok(_) => {
            println!("Finished");
        }
        Err(reason) => {
            println!("Error parsing file: {}", reason);
        }
    }
}

async fn run_test_cases_batch() -> Result<(), String> {
    let test_cases = get_test_cases()?;

    let request_client = create_client();

    for test_case_file_name in test_cases {
        let config = read_test_case_file(Path::new("test-cases").join(&test_case_file_name))?;

        run_test_case(config, test_case_file_name, &request_client)
            .await
            .unwrap();
    }

    return Ok(());
}

async fn run_test_case(
    config: TestCase,
    test_case_file_name: String,
    client: &reqwest::Client,
) -> Result<(), reqwest::Error> {
    println!("Running {}", config.name);

    let result = request::make_test_case_request(&config, client).await?;

    let mut last_result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(
            Path::new("test-cases")
                .join(test_case_file_name.clone())
                .with_extension("txt"),
        )
        .unwrap();

    last_result.write_all(result.body.as_bytes()).unwrap();

    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(
            Path::new("test-cases")
                .join(test_case_file_name)
                .with_extension("log"),
        )
        .unwrap();

    log_file
        .write_all(
            format!(
                "{}, {}, {}\n",
                result.status, result.time, result.assert_results
            )
            .as_bytes(),
        )
        .unwrap();

    return Ok(());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase {
    name: String,
    url: String,
    data: String,
    method: Method,
    headers: HashMap<String, String>,
    assertions: Vec<Assertion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Assertion {
    ExactMatch(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}
