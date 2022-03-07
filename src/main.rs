use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_dir, OpenOptions},
    path::{Path, PathBuf},
    time::Instant, io::Write,
};

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

    let result = make_test_case_request(config.url).await?;

    let mut resultLog = OpenOptions::new().append(true).create(true).open(
        Path::new("test-cases")
            .join(test_case_file_name)
            .with_extension("log"),
    ).unwrap();

    resultLog.write_all(format!("{}, {}\n", result.status, result.time).as_bytes()).unwrap();

    return Ok(());
}

async fn make_test_case_request(url: String) -> Result<TestCaseRunResult, reqwest::Error> {
    let now = Instant::now();
    let response = reqwest::get(url).await?;

    Ok(TestCaseRunResult {
        status: response.status().as_u16(),
        time: now.elapsed().as_millis() as f32 / 1000.0,
    })
}

#[derive(Debug)]
struct TestCaseRunResult {
    status: u16,
    time: f32,
}

fn get_test_cases() -> Result<Vec<String>, String> {
    let dir = read_dir("test-cases").or(Err("Couldn't find test case dir".to_string()))?;

    let mut names = Vec::new();

    for d in dir {
        let d = d.or(Err("".to_string()))?;

        let name = d
            .file_name()
            .to_str()
            .map(|x| x.to_string())
            .ok_or("Couldn't find file name".to_string())?;

        if name.ends_with(".yml") || name.ends_with(".yaml") {
            names.push(name);
        }
    }

    return Ok(names);
}

fn read_test_case_file(path: PathBuf) -> Result<TestCase, String> {
    let file = fs::read_to_string(path).or(Err("Couldn't open file".to_string()))?;

    match serde_yaml::from_str(&file) {
        Ok(data) => {
            return Ok(data);
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
    name: String,
    url: String,
    data: String,
}
