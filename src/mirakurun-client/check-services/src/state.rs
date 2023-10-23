use mirakurun_client_common::config::Service;
use std::path::PathBuf;
use std::fs;
use std::env;

pub struct State;

impl State {
    fn get_path(service: &Service) -> PathBuf {
        let mut path = env::temp_dir();
        path.push(env!("CARGO_PKG_NAME"));
        path.push(service.key.clone());

        path
    }

    pub fn exist(service: &Service) -> bool {
        let path = State::get_path(service);
        path.exists()
    }

    pub fn create(service: &Service) -> Result<(), String> {
        let path = State::get_path(service);
        fs::create_dir_all(&path).map_err(|error| format!("Failed to create state: {}", error))?;
        fs::write(path, "1").map_err(|error| format!("Failed to write state: {}", error))
    }

    pub fn update(service: &Service) -> Result<i32, String> {
        let path = State::get_path(service);
        if !path.exists() {
            State::create(service)?;
            return Ok(1);
        }

        let count_raw = fs::read_to_string(&path).map_err(|error| format!("Failed to read state: {}", error))?;
        let count = count_raw.parse::<i32>().map_err(|error| format!("Failed to parse state: {}", error))? + 1;
        fs::write(path, count.to_string()).map_err(|error| format!("Failed to update state: {}", error))?;
        return Ok(count);
    }

    pub fn delete(service: &Service) -> Result<(), String> {
        let path = State::get_path(service);
        fs::remove_dir(path).map_err(|error| format!("Failed to delete state: {}", error))
    }
}
