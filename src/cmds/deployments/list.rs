use crate::common::{DeploymentState, OutputFormat};
use crate::config::{ClusterConfig, ConfigSource};
use crate::service::get_deployments;
use aws_sdk_ecs::Client as ECSClient;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum ListDeploymentsError {
    #[error("couldn't deserialize results to JSON: {0}")]
    SerialiseToJson(#[from] serde_json::Error),
    #[error("couldn't serialize response to CSV: {0}")]
    SerializeToCSV(#[from] csv::Error),
    #[error("couldn't flush contents to csv writer: {0}")]
    FlushResultsToCSVWriter(#[from] std::io::Error),
    #[error("something unexpected happended: {0}")]
    Unexpected(String),
}

pub async fn list_deployments(
    clusters: Vec<ClusterConfig>,
    clients_map: Arc<HashMap<ConfigSource, ECSClient>>,
    state: Option<DeploymentState>,
    format: OutputFormat,
) -> Result<(), ListDeploymentsError> {
    if clusters.is_empty() {
        return Ok(());
    }

    let (deployments, errors) = get_deployments(clusters, clients_map, state)
        .await
        .map_err(ListDeploymentsError::Unexpected)?;

    if !deployments.is_empty() {
        match format {
            OutputFormat::Delimited => {
                let mut wtr = csv::Writer::from_writer(std::io::stdout());
                for dep in deployments {
                    wtr.serialize(dep)?;
                }
                wtr.flush()?;
            }
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&deployments)?),
            OutputFormat::Plain => {
                for dep in deployments {
                    println!("{dep}");
                }
            }
        }
    }

    if !errors.is_empty() {
        eprintln!(
            r#"
===
errors
==="#
        );
    }

    for error in errors {
        eprintln!(
            "{error}
---"
        );
    }

    Ok(())
}
