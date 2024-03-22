use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct JobRunRequest {
    pub job_id: i64,
    pub idempotency_token: Option<String>,
    pub queue: Option<QueueSettings>,
    pub jar_params: Option<Vec<String>>,
    pub notebook_params: Option<HashMap<String, String>>,
    pub python_params: Option<Vec<String>>,
    pub spark_submit_params: Option<Vec<String>>,
    pub python_named_params: Option<HashMap<String, String>>,
    pub pipeline_params: Option<HashMap<String, bool>>,
    pub sql_params: Option<HashMap<String, String>>,
    pub dbt_commands: Option<Vec<String>>,
    pub job_parameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct QueueSettings {
    pub enabled: bool,
}

#[derive(Deserialize)]
pub struct JobRunResponse {
    pub run_id: i64,
    pub number_in_job: Option<i64>,
}
