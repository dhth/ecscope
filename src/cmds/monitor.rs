use crate::config::{ClusterConfig, ConfigSource};
use crate::tui::run_tui;
use aws_sdk_ecs::Client as ECSClient;
use std::collections::HashMap;

pub async fn run_monitor(
    profile_name: String,
    clients_map: HashMap<ConfigSource, ECSClient>,
    clusters: Vec<ClusterConfig>,
) -> anyhow::Result<()> {
    if clusters.is_empty() {
        return Ok(());
    }

    run_tui(profile_name, clients_map, clusters).await?;

    Ok(())
}
