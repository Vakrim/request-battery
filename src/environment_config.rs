use std::{collections::HashMap, env, error::Error, fs, path::Path, process::Command};

use serde::{Deserialize, Serialize};

pub fn get_environment_variables() -> HashMap<String, String> {
    let mut variables = HashMap::new();

    let environment = get_environment();

    for (key, value) in environment.static_values {
        variables.insert(key, value);
    }

    for (key, command) in environment.commands {
        let cmd = Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("failed to execute process");

        variables.insert(key, String::from_utf8_lossy(&cmd.stdout).to_string());
    }

    return variables;
}

fn get_environment() -> EnvironmentConfig {
    let environment = env::args().nth(1).expect("No environment specified");

    let config = load_config_file().expect("Failed to load config file");

    match config.environments.get(&environment) {
        Some(config) => config.clone(),
        None => panic!("Environment {} not found", environment),
    }
}

fn load_config_file() -> Result<EnvironmentsSetConfig, Box<dyn Error>> {
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
pub struct EnvironmentsSetConfig {
    pub environments: HashMap<String, EnvironmentConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentConfig {
    pub commands: HashMap<String, String>,
    pub static_values: HashMap<String, String>,
}
