mod request;
mod test_case_file;

use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Write, path::Path};
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

    for test_case_file_name in test_cases {
        let config = read_test_case_file(Path::new("test-cases").join(&test_case_file_name))?;

        run_test_case(config, test_case_file_name).await.unwrap();
    }

    return Ok(());
}

async fn run_test_case(
    config: TestCase,
    test_case_file_name: String,
) -> Result<(), reqwest::Error> {
    println!("Running {}", config.name);

    let result = request::make_test_case_request(&config).await?;

    let mut result_log = OpenOptions::new()
        .append(true)
        .create(true)
        .open(
            Path::new("test-cases")
                .join(test_case_file_name)
                .with_extension("log"),
        )
        .unwrap();

    result_log
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
    assertions: Vec<Assertion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Assertion {
    ExactMatch(String),
}
