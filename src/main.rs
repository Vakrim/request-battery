mod environment_config;
mod request;
mod test_case_file;

use environment_config::get_environment_variables;
use request::create_client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
use test_case_file::read_test_cases_file;

use crate::test_case_file::to_path_name;

#[tokio::main]
async fn main() {
    match run_test_cases_batch().await {
        Ok(_) => {
            println!("Finished");
        }
        Err(reason) => {
            println!("Error running test cases: {}", reason);
        }
    }
}

async fn run_test_cases_batch() -> Result<(), Box<dyn Error>> {
    let environemnt_variables = get_environment_variables();

    let test_cases = read_test_cases_file(environemnt_variables)?;

    let request_client = create_client();

    create_dir_all(Path::new("test-summaries"))?;

    for test_case in test_cases {
        run_test_case(test_case, &request_client).await?;
    }

    return Ok(());
}

async fn run_test_case(
    test_case: TestCase,
    client: &reqwest::Client,
) -> Result<(), Box<dyn Error>> {
    println!("Running {}", test_case.name);

    let result = request::make_test_case_request(&test_case, client).await?;

    save_response(&test_case, &result);

    save_summary(test_case, result);

    return Ok(());
}

fn save_summary(test_case: TestCase, result: request::TestCaseRunResult) {
    let mut summary_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(Path::new("test-summaries").join(to_path_name(&test_case.name) + ".log"))
        .unwrap();

    summary_file
        .write_all(
            format!(
                "{}, {}, {}, {}\n",
                result.status, result.time, result.name, result.assert_results
            )
            .as_bytes(),
        )
        .unwrap();
}

fn save_response(test_case: &TestCase, result: &request::TestCaseRunResult) {
    let test_runs_dir_path = PathBuf::new()
        .join("test-runs")
        .join(to_path_name(&test_case.name));

    create_dir_all(test_runs_dir_path.as_path()).expect("Could not create test runs directory");

    let mut last_result = OpenOptions::new()
        .write(true)
        .create(true)
        .open(test_runs_dir_path.join(result.name.clone() + ".json"))
        .unwrap();

    last_result.write_all(result.body.as_bytes()).unwrap();
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
