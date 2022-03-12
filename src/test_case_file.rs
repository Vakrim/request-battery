use std::fs::{self, read_dir};

use super::TestCase;

use std::path::PathBuf;

pub fn read_test_case_file(path: PathBuf) -> Result<TestCase, String> {
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

pub fn get_test_cases() -> Result<Vec<String>, String> {
    let dir = read_dir("test-cases").or(Err("Couldn't open test cases dir".to_string()))?;

    let mut names = Vec::new();

    for d in dir {
        let d = d.or(Err("".to_string()))?;

        let name = d.file_name().to_string_lossy().to_string();

        if name.ends_with(".yml") || name.ends_with(".yaml") {
            names.push(name);
        }
    }

    return Ok(names);
}
