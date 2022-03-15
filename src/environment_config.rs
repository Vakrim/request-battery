use std::{collections::HashMap, env, error::Error, fs, path::Path};

use serde::{Deserialize, Serialize};

pub fn get_environment_variables() -> HashMap<String, String> {
    let environment = env::args().nth(1).expect("No environment specified");

    let config = load_config_file().expect("Failed to load config file");

    match config.environments.get(&environment) {
        Some(config) => return config.clone(),
        None => panic!("Environment {} not found", environment),
    }
}

fn load_config_file() -> Result<EnvironmentConfig, Box<dyn Error>> {
    let path = Path::new("environments.yaml");

    let file = fs::read_to_string(path)?;

    match serde_yaml::from_str(&file) {
        Ok(data) => {
            return Ok(data);
        }
        Err(error) => {
            return Err(error.into());
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    environments: HashMap<String, HashMap<String, String>>,
}
