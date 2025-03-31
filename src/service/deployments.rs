use crate::common::{DeploymentState, UNKNOWN};
use crate::config::{ClusterConfig, ConfigSource};
use crate::domain::{DeploymentDetails, DeploymentError, DeploymentResult};
use aws_sdk_ecs::Client as ECSClient;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;

const MAX_CONCURRENT_FETCHES: usize = 10;

// https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_Deployment.html
const DEPLOYMENT_STATUS_PRIMARY: &str = "PRIMARY";

pub async fn get_deployments(
    clusters: Vec<ClusterConfig>,
    clients_map: Arc<HashMap<ConfigSource, ECSClient>>,
    state: Option<DeploymentState>,
) -> Result<(Vec<DeploymentDetails>, Vec<DeploymentError>), String> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_FETCHES));
    let mut tasks = Vec::new();

    for cluster in clusters {
        let client = Arc::new(
            clients_map
                .get(&cluster.config_source)
                .ok_or("clients_map did not have entry for cluster".to_string())?
                .clone(),
        );
        let semaphore = Arc::clone(&semaphore);
        tasks.push(tokio::task::spawn(async move {
            deployments_for_cluster(cluster, client, semaphore, state).await
        }));
    }

    let mut deployments = Vec::new();
    let mut errors = Vec::new();

    for task in tasks {
        match task
            .await
            .map_err(|e| format!("couldn't join task: {}", e))?
        {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(d) => deployments.push(d),
                        Err(e) => errors.push(e),
                    }
                }
            }
            Err(error) => return Err(error),
        }
    }

    deployments.sort();
    errors.sort();

    Ok((deployments, errors))
}

async fn deployments_for_cluster(
    cluster: ClusterConfig,
    client: Arc<ECSClient>,
    semaphore: Arc<Semaphore>,
    state: Option<DeploymentState>,
) -> Result<Vec<DeploymentResult>, String> {
    let _permit = semaphore
        .acquire()
        .await
        .map_err(|e| format!("couldn't acquire semaphore permit: {}", e))?;

    let servs_result = client
        .describe_services()
        .cluster(&cluster.arn)
        .set_services(Some(cluster.services.clone()))
        .send()
        .await;

    let mut results = Vec::new();
    match servs_result {
        Ok(output) => {
            for service in output.services() {
                for dep in service.deployments() {
                    let include = match state {
                        Some(DeploymentState::Finished) => {
                            dep.status().unwrap_or_default() == DEPLOYMENT_STATUS_PRIMARY
                                && dep.running_count == dep.desired_count
                        }
                        Some(DeploymentState::InProgress) => {
                            dep.status().unwrap_or_default() != DEPLOYMENT_STATUS_PRIMARY
                                || dep.running_count != dep.desired_count
                        }
                        Some(DeploymentState::Failing) => {
                            dep.running_count != dep.desired_count && dep.failed_tasks != 0
                        }
                        None => true,
                    };

                    if !include {
                        continue;
                    }

                    results.push(Ok(DeploymentDetails {
                        service_name: service.service_name().unwrap_or(UNKNOWN).to_string(),
                        keys: cluster.keys.join(","),
                        cluster_arn: cluster.arn.clone(),
                        deployment_id: dep.id().unwrap_or(UNKNOWN).to_string(),
                        status: dep.status().unwrap_or(UNKNOWN).to_string(),
                        running_count: dep.running_count(),
                        desired_count: dep.desired_count(),
                        pending_count: dep.pending_count(),
                        failed_count: dep.failed_tasks(),
                    }));
                }
            }
        }
        Err(error) => {
            let error = anyhow::anyhow!(error);
            for service in &cluster.services {
                results.push(Err(DeploymentError {
                    service_name: service.to_string(),
                    error: format!("{:?}", error),
                    cluster_arn: cluster.arn.clone(),
                    keys: cluster.keys.join(","),
                }));
            }
        }
    }

    Ok(results)
}
