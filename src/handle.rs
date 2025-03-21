use crate::args::{Args, EcscopeCommand, ProfilesCommand};
use crate::cmds::{add_profile, list_deployments, list_profiles, run_monitor};
use crate::common::{OutputMode, get_env};
use crate::debug::display_debug_info;
use crate::errors::AppError;
use crate::server::serve_deployments;
use crate::utils::{get_clusters, get_config_dir};
use std::path::PathBuf;
use std::sync::Arc;

const TOOL_DIR: &str = "ecscope";

pub async fn handle(args: Args) -> Result<(), AppError> {
    let config_dir = get_config_dir()?;
    let config_dir = config_dir.join(PathBuf::from(TOOL_DIR));

    if args.debug {
        display_debug_info(&args, &config_dir);
        return Ok(());
    }

    match args.command {
        EcscopeCommand::Deployments {
            profile_name,
            service_name_filter,
            key_filter,
            state,
            format,
            mode,
            web_skip_opening,
        } => {
            if let Some((clients_map, clusters)) = get_clusters(
                &config_dir,
                profile_name.clone(),
                service_name_filter,
                key_filter,
            )
            .await?
            {
                match mode {
                    OutputMode::Default => {
                        list_deployments(clusters, Arc::new(clients_map), state, format).await?
                    }
                    OutputMode::Web => {
                        let env = get_env();
                        serve_deployments(
                            clusters,
                            Arc::new(clients_map),
                            state,
                            web_skip_opening,
                            env,
                        )
                        .await
                        .map_err(AppError::ServeDeployments)?
                    }
                }
            }
        }
        EcscopeCommand::Profiles { profiles_command } => match profiles_command {
            ProfilesCommand::Add { name } => add_profile(&config_dir, name, false)?,
            ProfilesCommand::List => list_profiles(&config_dir)?,
        },
        EcscopeCommand::Monitor {
            profile_name,
            service_name_filter,
            key_filter,
        } => {
            if let Some((clients_map, clusters)) = get_clusters(
                &config_dir,
                profile_name.clone(),
                service_name_filter,
                key_filter,
            )
            .await?
            {
                run_monitor(profile_name.clone(), clients_map, clusters)
                    .await
                    .map_err(AppError::RunMonitor)?;
            }
        }
    }

    Ok(())
}
