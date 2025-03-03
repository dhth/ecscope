use crate::config::{ClusterConfig, ConfigSource};
use crate::domain::{ServiceDetails, ServiceError, ServiceResult};

use super::command::Command;
use super::message::Message;
use aws_sdk_ecs::Client as ECSClient;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub(super) async fn handle_command(
    client: Arc<HashMap<ConfigSource, ECSClient>>,
    command: Command,
    event_tx: Sender<Message>,
) {
    match command {
        Command::GetServices(cluster) => {
            let clients_map = Arc::clone(&client);
            tokio::spawn(async move {
                handle_get_services(clients_map, cluster, event_tx).await;
            });
        }
        Command::RefreshService((service_details, index)) => {
            let clients_map = Arc::clone(&client);
            tokio::spawn(async move {
                handle_refresh_service(clients_map, service_details, index, event_tx).await;
            });
        }
        Command::GetTasks((service_details, refresh)) => {
            let clients_map = Arc::clone(&client);
            tokio::spawn(async move {
                handle_get_tasks(clients_map, service_details, event_tx, refresh).await;
            });
        }
    }
}

async fn handle_get_services(
    client: Arc<HashMap<ConfigSource, ECSClient>>,
    cluster: ClusterConfig,
    event_tx: Sender<Message>,
) {
    let mut si = Vec::new();
    let client = client.get(&cluster.config_source);

    let client = match client {
        Some(c) => c,
        None => {
            for service in &cluster.services {
                si.push(ServiceResult::Err(ServiceError {
                    service_name: service.to_string(),
                    error: "unexpected error".to_string(),
                    cluster_keys: cluster.keys.clone(),
                }));
            }
            let _ = event_tx.try_send(Message::ServicesFetched(si));
            return;
        }
    };

    let servs_result = client
        .describe_services()
        .cluster(&cluster.arn)
        .set_services(Some(cluster.services.clone()))
        .send()
        .await;

    match servs_result {
        Ok(describe_servs_output) => {
            for service in describe_servs_output.services() {
                let service_name = match service.service_name() {
                    Some(n) => n,
                    None => {
                        si.push(ServiceResult::Err(ServiceError {
                            service_name: "unknown".to_string(),
                            error: "service name returned was empty".to_string(),
                            cluster_keys: cluster.keys.clone(),
                        }));
                        continue;
                    }
                };
                let status = service.status().unwrap_or("unknown");
                let desired_count = service.desired_count();
                let running_count = service.running_count();
                let pending_count = service.pending_count();

                let sr = ServiceResult::Ok(ServiceDetails {
                    name: service_name.into(),
                    status: status.into(),
                    desired_count,
                    running_count,
                    pending_count,
                    cluster_keys: cluster.keys.clone(),
                    cluster_arn: cluster.arn.to_string(),
                    config_source: cluster.config_source.clone(),
                });

                si.push(sr);
            }

            let _ = event_tx.try_send(Message::ServicesFetched(si));
        }
        Err(sdk_error) => {
            let error = anyhow::anyhow!(sdk_error);
            for service in &cluster.services {
                si.push(ServiceResult::Err(ServiceError {
                    service_name: service.to_string(),
                    error: format!("{:?}", error),
                    cluster_keys: cluster.keys.clone(),
                }));
            }
            let _ = event_tx.try_send(Message::ServicesFetched(si));
        }
    }
}

async fn handle_refresh_service(
    client: Arc<HashMap<ConfigSource, ECSClient>>,
    service_details: ServiceDetails,
    index: usize,
    event_tx: Sender<Message>,
) {
    let client = client.get(&service_details.config_source);

    let client = match client {
        Some(c) => c,
        None => {
            let service_name = &service_details.name;
            let _ = event_tx.try_send(Message::ServiceDetailsRefreshed((
                ServiceResult::Err(ServiceError {
                    service_name: service_name.clone(),
                    error: "unexpected error".to_string(),
                    cluster_keys: service_details.cluster_keys.clone(),
                }),
                service_details,
                index,
            )));
            return;
        }
    };

    let servs_result = client
        .describe_services()
        .cluster(&service_details.cluster_arn)
        .set_services(Some(vec![service_details.name.clone()]))
        .send()
        .await;

    match servs_result {
        Ok(describe_servs_output) => {
            if describe_servs_output.services().len() != 1 {
                let _ = event_tx.try_send(Message::ServiceDetailsRefreshed((
                    ServiceResult::Err(ServiceError {
                        service_name: "unknown".to_string(),
                        error: "service name returned was empty".to_string(),
                        cluster_keys: service_details.cluster_keys.clone(),
                    }),
                    service_details,
                    index,
                )));
                return;
            }

            let service = &describe_servs_output.services()[0];

            let service_name = match service.service_name() {
                Some(n) => n,
                None => {
                    let _ = event_tx.try_send(Message::ServiceDetailsRefreshed((
                        ServiceResult::Err(ServiceError {
                            service_name: "unknown".to_string(),
                            error: "service name returned was empty".to_string(),
                            cluster_keys: service_details.cluster_keys.clone(),
                        }),
                        service_details,
                        index,
                    )));
                    return;
                }
            };

            let status = service.status().unwrap_or("unknown");
            let desired_count = service.desired_count();
            let running_count = service.running_count();
            let pending_count = service.pending_count();

            let sr = ServiceResult::Ok(ServiceDetails {
                name: service_name.into(),
                status: status.into(),
                desired_count,
                running_count,
                pending_count,
                cluster_keys: service_details.cluster_keys.clone(),
                cluster_arn: service_details.cluster_arn.to_string(),
                config_source: service_details.config_source.clone(),
            });

            let _ = event_tx.try_send(Message::ServiceDetailsRefreshed((
                sr,
                service_details,
                index,
            )));
        }
        Err(sdk_error) => {
            let error = anyhow::anyhow!(sdk_error);
            let _ = event_tx.try_send(Message::ServiceDetailsRefreshed((
                ServiceResult::Err(ServiceError {
                    service_name: service_details.name.clone(),
                    error: format!("{:?}", error),
                    cluster_keys: service_details.cluster_keys.clone(),
                }),
                service_details,
                index,
            )));
        }
    }
}

async fn handle_get_tasks(
    client: Arc<HashMap<ConfigSource, ECSClient>>,
    service_details: ServiceDetails,
    event_tx: Sender<Message>,
    refresh: bool,
) {
    std::thread::sleep(std::time::Duration::from_millis(2000));

    let client = client.get(&service_details.config_source);
    let client = match client {
        Some(c) => c,
        None => return,
    };

    let tr = client
        .list_tasks()
        .cluster(&service_details.cluster_arn)
        .service_name(&service_details.name)
        .send()
        .await;

    if let Ok(tasks) = tr {
        let describe_tasks_resp = client
            .describe_tasks()
            .cluster(&service_details.cluster_arn)
            .set_tasks(tasks.task_arns.clone())
            .send()
            .await;

        if let Ok(tasks_desc) = describe_tasks_resp {
            let tasks = tasks_desc.tasks().to_vec();
            let _ = event_tx.try_send(Message::TasksFetched((service_details, tasks, refresh)));
        }
    }
}
