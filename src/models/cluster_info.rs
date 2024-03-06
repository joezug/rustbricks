use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterInfo {
    pub cluster_id: String,
    pub cluster_name: String,
    pub state: String,
    pub creator_user_name: String,
    pub spark_context_id: Option<i64>,
    pub driver_healthy: bool,
    pub spark_version: String,
    pub spark_conf: HashMap<String, String>,
    pub azure_attributes: AzureAttributes,
    pub node_type_id: String,
    pub driver_node_type_id: String,
    pub custom_tags: HashMap<String, String>,
    pub autotermination_minutes: i64,
    pub enable_elastic_disk: bool,
    #[serde(default)]
    pub disk_spec: HashMap<String, serde_json::Value>,
    pub cluster_source: String,
    pub single_user_name: Option<String>,
    pub enable_local_disk_encryption: bool,
    pub instance_source: InstanceSource,
    pub driver_instance_source: InstanceSource,
    pub data_security_mode: String,
    pub runtime_engine: String,
    pub effective_spark_version: String,
    pub state_message: String,
    pub start_time: Option<i64>,
    pub terminated_time: Option<i64>,
    pub last_state_loss_time: Option<i64>,
    pub last_activity_time: Option<i64>,
    pub last_restarted_time: Option<i64>,
    pub num_workers: i32,
    pub default_tags: HashMap<String, String>,
    pub termination_reason: TerminationReason,
    pub pinned_by_user_name: Option<String>,
    pub init_scripts_safe_mode: bool,
    pub spec: ClusterSpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureAttributes {
    pub first_on_demand: i32,
    pub availability: String,
    pub spot_bid_max_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceSource {
    pub node_type_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminationReason {
    pub code: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterSpec {
    pub cluster_name: String,
    pub spark_version: String,
    pub spark_conf: HashMap<String, String>,
    pub azure_attributes: AzureAttributes,
    pub node_type_id: String,
    pub driver_node_type_id: String,
    pub custom_tags: HashMap<String, String>,
    pub autotermination_minutes: i64,
    pub enable_elastic_disk: bool,
    pub single_user_name: Option<String>,
    pub enable_local_disk_encryption: bool,
    pub data_security_mode: String,
    pub runtime_engine: String,
    pub num_workers: i32,
}

impl fmt::Display for ClusterInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cluster Information:")?;
        writeln!(f, "  ID: {}", self.cluster_id)?;
        writeln!(f, "  Name: {}", self.cluster_name)?;
        writeln!(f, "  State: {}", self.state)?;
        writeln!(f, "  Created by: {}", self.creator_user_name)?;
        writeln!(f, "  Spark Version: {}", self.spark_version)?;
        writeln!(f, "  Spark Configuration:")?;
        for (key, value) in &self.spark_conf {
            writeln!(f, "    {}: {}", key, value)?;
        }
        writeln!(f, "  Azure Attributes:")?;
        writeln!(
            f,
            "    First On Demand: {}",
            self.azure_attributes.first_on_demand
        )?;
        writeln!(
            f,
            "    Availability: {}",
            self.azure_attributes.availability
        )?;
        writeln!(
            f,
            "    Spot Bid Max Price: {}",
            self.azure_attributes.spot_bid_max_price
        )?;
        writeln!(f, "  Node Type ID: {}", self.node_type_id)?;
        writeln!(f, "  Driver Node Type ID: {}", self.driver_node_type_id)?;
        writeln!(f, "  Custom Tags:")?;
        for (key, value) in &self.custom_tags {
            writeln!(f, "    {}: {}", key, value)?;
        }
        writeln!(
            f,
            "  Autotermination Minutes: {}",
            self.autotermination_minutes
        )?;
        writeln!(f, "  Enable Elastic Disk: {}", self.enable_elastic_disk)?;
        writeln!(f, "  Disk Specification:")?;
        for (key, value) in &self.disk_spec {
            writeln!(f, "    {}: {}", key, value)?;
        }
        writeln!(f, "  Cluster Source: {}", self.cluster_source)?;
        if let Some(single_user_name) = &self.single_user_name {
            writeln!(f, "  Single User Name: {}", single_user_name)?;
        }
        writeln!(
            f,
            "  Enable Local Disk Encryption: {}",
            self.enable_local_disk_encryption
        )?;
        writeln!(
            f,
            "  Instance Source Node Type ID: {}",
            self.instance_source.node_type_id
        )?;
        writeln!(
            f,
            "  Driver Instance Source Node Type ID: {}",
            self.driver_instance_source.node_type_id
        )?;
        writeln!(f, "  Data Security Mode: {}", self.data_security_mode)?;
        writeln!(f, "  Runtime Engine: {}", self.runtime_engine)?;
        writeln!(
            f,
            "  Effective Spark Version: {}",
            self.effective_spark_version
        )?;
        writeln!(f, "  State Message: {}", self.state_message)?;
        if let Some(start_time) = self.start_time {
            writeln!(f, "  Start Time: {}", start_time)?;
        }
        if let Some(terminated_time) = self.terminated_time {
            writeln!(f, "  Terminated Time: {}", terminated_time)?;
        }
        if let Some(last_state_loss_time) = self.last_state_loss_time {
            writeln!(f, "  Last State Loss Time: {}", last_state_loss_time)?;
        }
        if let Some(last_activity_time) = self.last_activity_time {
            writeln!(f, "  Last Activity Time: {}", last_activity_time)?;
        }
        if let Some(last_restarted_time) = self.last_restarted_time {
            writeln!(f, "  Last Restarted Time: {}", last_restarted_time)?;
        }
        writeln!(f, "  Number of Workers: {}", self.num_workers)?;
        writeln!(f, "  Default Tags:")?;
        for (key, value) in &self.default_tags {
            writeln!(f, "    {}: {}", key, value)?;
        }
        writeln!(f, "  Termination Reason:")?;
        writeln!(f, "    Code: {}", self.termination_reason.code)?;
        writeln!(f, "    Type: {}", self.termination_reason.type_field)?;
        writeln!(f, "    Parameters:")?;
        for (key, value) in &self.termination_reason.parameters {
            writeln!(f, "      {}: {}", key, value)?;
        }
        if let Some(pinned_by_user_name) = &self.pinned_by_user_name {
            writeln!(f, "  Pinned By User Name: {}", pinned_by_user_name)?;
        }
        writeln!(
            f,
            "  Init Scripts Safe Mode: {}",
            self.init_scripts_safe_mode
        )?;
        writeln!(f, "  Cluster Specification:")?;
        writeln!(f, "    Cluster Name: {}", self.spec.cluster_name)?;
        writeln!(f, "    Spark Version: {}", self.spec.spark_version)?;
        writeln!(f, "    Node Type ID: {}", self.spec.node_type_id)?;
        writeln!(
            f,
            "    Driver Node Type ID: {}",
            self.spec.driver_node_type_id
        )?;
        writeln!(
            f,
            "    Autotermination Minutes: {}",
            self.spec.autotermination_minutes
        )?;
        writeln!(
            f,
            "    Enable Elastic Disk: {}",
            self.spec.enable_elastic_disk
        )?;
        writeln!(f, "    Number of Workers: {}", self.spec.num_workers)?;
        if let Some(single_user_name) = &self.spec.single_user_name {
            writeln!(f, "    Single User Name: {}", single_user_name)?;
        }
        writeln!(
            f,
            "    Enable Local Disk Encryption: {}",
            self.spec.enable_local_disk_encryption
        )?;
        writeln!(
            f,
            "    Data Security Mode: {}",
            self.spec.data_security_mode
        )?;
        writeln!(f, "    Runtime Engine: {}", self.spec.runtime_engine)?;
        Ok(())
    }
}
