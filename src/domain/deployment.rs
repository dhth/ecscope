#[derive(Debug, Clone)]
pub struct DeploymentError {
    pub service_name: String,
    pub error: String,
    pub cluster_arn: String,
    pub cluster_keys: Vec<String>,
}

impl std::fmt::Display for DeploymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
Service     : {}
Cluster ARN : {}
Cluster Keys: {:?}
Error       : {}"
"#,
            self.service_name, self.cluster_arn, self.cluster_keys, self.error,
        )?;

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, serde::Serialize)]
pub struct DeploymentDetails {
    pub service_name: String,
    #[serde(skip)]
    pub cluster_keys: Vec<String>,
    pub cluster_arn: String,
    pub deployment_id: String,
    pub status: String,
    pub running_count: i32,
    pub desired_count: i32,
    pub pending_count: i32,
    pub num_failed_tasks: i32,
}

impl std::fmt::Display for DeploymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
service        : {}
deployment id  : {}
status         : {}
running count  : {}
desired count  : {}
pending count  : {}
failed tasks   : {}
"#,
            self.service_name,
            self.deployment_id,
            self.status,
            self.running_count,
            self.desired_count,
            self.pending_count,
            self.num_failed_tasks,
        )?;

        Ok(())
    }
}
