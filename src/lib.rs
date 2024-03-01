pub mod config;

pub mod models {
    pub mod cluster_info;
    pub mod sql_statement;

    pub use cluster_info::ClusterInfo;
    pub use sql_statement::{
        ChunkMetadata, ResultData, SqlParameter, SqlStatementRequest, SqlStatementResponse,
    };
}

pub mod services {
    mod cluster_service;
    mod databricks_request;
    mod sql_service;

    pub use cluster_service::get_cluster_info;
    pub use databricks_request::send_databricks_request;
    pub use sql_service::{
        execute_sql_statement, get_sql_statement_result_chunk, get_sql_statement_status,
    };
}

pub mod errors {
    mod http;

    pub use http::{ErrorResponse, HttpError};
}
