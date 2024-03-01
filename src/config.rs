use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub databricks_host: String,
    pub databricks_token: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let databricks_host = env::var("DATABRICKS_HOST")
            .map_err(|_| "DATABRICKS_HOST must be set in the environment")?;
        let databricks_token = env::var("DATABRICKS_TOKEN")
            .map_err(|_| "DATABRICKS_TOKEN must be set in the environment")?;

        Ok(Config {
            databricks_host,
            databricks_token,
        })
    }
}
