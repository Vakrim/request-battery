use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
    path::Path,
};

use super::TestCase;
use regex::Regex;

pub fn read_test_cases_file(
    variables: HashMap<String, String>,
) -> Result<Vec<TestCase>, Box<dyn Error>> {
    let path = Path::new("test-cases.yaml");

    let mut file = fs::read_to_string(path)?;

    for (key, val) in variables.iter() {
        file = file.replace(&format!("${{{key}}}", key = key), val);
    }

    match serde_yaml::from_str(&file) {
        Ok(data) => {
            return Ok(data);
        }
        Err(error) => {
            return Err(error.into());
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
