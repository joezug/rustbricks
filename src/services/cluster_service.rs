use crate::errors::HttpError;
use crate::models::ClusterInfo;
use crate::services::send_databricks_request;
use reqwest::Method;

pub async fn get_cluster_info(
    host: &str,
    token: &str,
    cluster_id: &str,
) -> Result<ClusterInfo, HttpError> {
    send_databricks_request(
        host,
        token,
        Method::GET,
        &format!("api/2.0/clusters/get?cluster_id={}", cluster_id),
        None::<()>, // No body for GET request
    )
    .await
}
