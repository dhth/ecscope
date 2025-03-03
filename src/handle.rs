use crate::args::{Args, EcscopeCommand, ProfilesCommand};
use crate::cmds::{add_profile, list_profiles, run_monitor};
use crate::debug::display_debug_info;
use crate::errors::AppError;
use crate::utils::get_config_dir;
use std::path::PathBuf;

const TOOL_DIR: &str = "ecscope";

pub async fn handle(args: Args) -> Result<(), AppError> {
    let config_dir = get_config_dir()?;
    let config_dir = config_dir.join(PathBuf::from(TOOL_DIR));

    if args.debug {
        display_debug_info(&args, &config_dir);
        return Ok(());
    }

    match args.command {
        EcscopeCommand::Profiles { profiles_command } => match profiles_command {
            ProfilesCommand::Add { name } => add_profile(&config_dir, name, false)?,
            ProfilesCommand::List => list_profiles(&config_dir)?,
        },
        EcscopeCommand::Monitor {
            profile_name,
            service_name_filter,
            key_filter,
        } => run_monitor(&config_dir, profile_name, service_name_filter, key_filter).await?,
    }

    Ok(())
}
