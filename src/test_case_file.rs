use std::{
    fs::{self},
    path::Path,
};

use regex::Regex;

use super::TestCase;

pub fn read_test_cases_file() -> Result<Vec<TestCase>, String> {
    let path = Path::new("test-cases.yaml");

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

pub fn to_path_name(name: &str) -> String {
    Regex::new(r"[\W]")
        .unwrap()
        .replace_all(name, "-")
        .to_lowercase()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_normalizes_paths_to_letters_and_dashes() {
        assert_eq!(
            to_path_name("This is a test case name"),
            "this-is-a-test-case-name"
        );
    }
}
