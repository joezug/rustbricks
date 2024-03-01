use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlStatementRequest {
    pub statement: String,
    pub warehouse_id: String,
    pub catalog: Option<String>,
    pub schema: Option<String>,
    pub parameters: Option<Vec<SqlParameter>>,
    pub row_limit: Option<i64>,
    pub byte_limit: Option<i64>,
    pub disposition: String, // "INLINE" or "EXTERNAL_LINKS"
    pub format: String,      // "JSON_ARRAY", "ARROW_STREAM", or "CSV"
    pub wait_timeout: Option<String>,
    pub on_wait_timeout: Option<String>, // "CONTINUE" or "CANCEL"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlParameter {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "type")]
    pub sql_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlStatementResponse {
    pub statement_id: Option<String>,
    pub status: Option<StatementStatus>,
    pub manifest: Option<Manifest>,
    pub result: Option<ResultData>,
    pub external_links: Option<Vec<ExternalLink>>, // For EXTERNAL_LINKS disposition
    pub error: Option<String>,                     // Optional field to capture error messages
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatementStatus {
    pub state: String,
    pub error: Option<ErrorResponse>, // Changed from Option<String> to Option<ErrorResponse>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub format: String,         // "JSON_ARRAY", "ARROW_STREAM", "CSV"
    pub schema: Option<Schema>, // Schema is already optional
    #[serde(default)] // This will default to an empty vector if `chunks` is not present
    pub chunks: Vec<ChunkMetadata>,
    pub total_chunk_count: i32,
    pub total_row_count: i64,
    pub total_byte_count: Option<i64>, // Not available for INLINE disposition
    pub truncated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(default)]
    pub columns: Vec<ColumnDescription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnDescription {
    pub name: String,
    #[serde(rename = "type_name")]
    data_type: String,
    position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub chunk_index: i32,
    pub row_offset: i64,
    pub row_count: i64,
    pub byte_count: Option<i64>, // Not available for INLINE disposition
    pub next_chunk_index: Option<i32>,
    pub next_chunk_internal_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultData {
    // Removed the fields that are not directly under `result` when `external_links` is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_array: Option<Vec<Vec<Option<String>>>>, // For INLINE, JSON_ARRAY format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<ExternalLink>>, // For EXTERNAL_LINKS disposition
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalLink {
    pub chunk_index: i32,
    pub row_offset: i64,
    pub row_count: i64,
    pub byte_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_chunk_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_chunk_internal_link: Option<String>,
    pub external_link: String,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub expiration: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: Option<String>,
    pub message: Option<String>,
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        s.parse::<DateTime<Utc>>()
            .map(Some)
            .map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}
