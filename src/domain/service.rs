use crate::config::ConfigSource;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
#[allow(dead_code)]
pub struct Service {
    pub name: String,
    pub cluster_arn: String,
}

pub type ServiceResult = Result<ServiceDetails, ServiceError>;

#[derive(Debug, Clone)]
pub struct ServiceError {
    pub service_name: String,
    pub error: String,
    pub cluster_keys: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ServiceDetails {
    pub name: String,
    pub status: String,
    pub desired_count: i32,
    pub running_count: i32,
    pub pending_count: i32,
    pub cluster_keys: Vec<String>,
    pub cluster_arn: String,
    pub config_source: ConfigSource,
}
