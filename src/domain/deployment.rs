#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct DeploymentError {
    pub service_name: String,
    pub error: String,
    pub cluster_arn: String,
    pub keys: String,
}

impl std::fmt::Display for DeploymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
Service     : {}
Cluster ARN : {}
Keys        : {:?}
Error       : {}"
"#,
            self.service_name, self.cluster_arn, self.keys, self.error,
        )?;

        Ok(())
    }
}

impl Ord for DeploymentError {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.service_name
            .cmp(&other.service_name)
            .then_with(|| self.keys.cmp(&other.keys))
    }
}

impl PartialOrd for DeploymentError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type DeploymentResult = Result<DeploymentDetails, DeploymentError>;

#[derive(Debug, Eq, PartialEq, Hash, Clone, serde::Serialize)]
pub struct DeploymentDetails {
    pub service_name: String,
    pub keys: String,
    pub cluster_arn: String,
    pub deployment_id: String,
    pub status: String,
    pub running_count: i32,
    pub desired_count: i32,
    pub pending_count: i32,
    pub failed_count: i32,
}

impl Ord for DeploymentDetails {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.service_name
            .cmp(&other.service_name)
            .then_with(|| self.keys.cmp(&other.keys))
    }
}

impl PartialOrd for DeploymentDetails {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl DeploymentDetails {
    pub fn dummy_running(name: &str, keys: &str) -> Self {
        Self {
            service_name: name.to_string(),
            keys: keys.to_string(),
            cluster_arn: "".to_string(),
            deployment_id: "".to_string(),
            status: "PRIMARY".to_string(),
            running_count: 2,
            desired_count: 2,
            pending_count: 0,
            failed_count: 0,
        }
    }

    pub fn dummy_pending(name: &str, keys: &str) -> Self {
        Self {
            service_name: name.to_string(),
            keys: keys.to_string(),
            cluster_arn: "".to_string(),
            deployment_id: "".to_string(),
            status: "PRIMARY".to_string(),
            running_count: 0,
            desired_count: 2,
            pending_count: 2,
            failed_count: 0,
        }
    }

    pub fn dummy_active(name: &str, keys: &str) -> Self {
        Self {
            service_name: name.to_string(),
            keys: keys.to_string(),
            cluster_arn: "".to_string(),
            deployment_id: "".to_string(),
            status: "ACTIVE".to_string(),
            running_count: 2,
            desired_count: 0,
            pending_count: 0,
            failed_count: 0,
        }
    }

    pub fn dummy_failing(name: &str, keys: &str) -> Self {
        Self {
            service_name: name.to_string(),
            keys: keys.to_string(),
            cluster_arn: "".to_string(),
            deployment_id: "".to_string(),
            status: "ACTIVE".to_string(),
            running_count: 0,
            desired_count: 2,
            pending_count: 2,
            failed_count: 3,
        }
    }

    pub fn dummy_draining(name: &str, keys: &str) -> Self {
        Self {
            service_name: name.to_string(),
            keys: keys.to_string(),
            cluster_arn: "".to_string(),
            deployment_id: "".to_string(),
            status: "DRAINING".to_string(),
            running_count: 0,
            desired_count: 0,
            pending_count: 0,
            failed_count: 0,
        }
    }
}

impl std::fmt::Display for DeploymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
Service        : {}
Keys           : {}
Cluster arn    : {}
Deployment id  : {}
Status         : {}
Running count  : {}
Desired count  : {}
Pending count  : {}
Failed tasks   : {}
"#,
            self.service_name,
            self.keys,
            self.cluster_arn,
            self.deployment_id,
            self.status,
            self.running_count,
            self.desired_count,
            self.pending_count,
            self.failed_count,
        )?;

        Ok(())
    }
}
