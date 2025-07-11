use std::path::PathBuf;

use crate::common::{DeploymentState, OutputFormat, OutputMode};
use clap::{Parser, Subcommand};
use regex::Regex;

const NOT_PROVIDED: &str = "<not provided>";

/// ecscope lets you monitor AWS ECS resources from the terminal
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: EcscopeCommand,
    /// Config directory (to override ecscope's default config directory)
    #[arg(short = 'c', long = "config-dir", value_name = "PATH", global = true)]
    pub config_dir: Option<PathBuf>,
    /// Output debug information without doing anything
    #[arg(long = "debug", global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum EcscopeCommand {
    /// List ECS deployments
    #[command(name = "deps")]
    Deployments {
        /// Profile to use
        #[arg(value_name = "PROFILE")]
        profile_name: String,
        /// Filtration query for service names
        #[arg(short = 's', long = "service-filter", value_name = "REGEX", value_parser=validate_filter_query)]
        service_name_filter: Option<Regex>,
        /// Filtration query for cluster keys
        #[arg(short = 'k', long = "key-filter", value_name = "REGEX", value_parser=validate_filter_query)]
        key_filter: Option<Regex>,
        /// Deployment state to query for
        #[arg(short = 'S', long = "state", value_name = "STRING")]
        state: Option<DeploymentState>,
        /// Format to use
        #[arg(
            short = 'f',
            long = "format",
            value_name = "STRING",
            default_value = "json"
        )]
        format: OutputFormat,
        /// Output mode
        #[arg(
            short = 'm',
            long = "mode",
            value_name = "STRING",
            default_value = "default"
        )]
        mode: OutputMode,
        /// Whether to skip opening web results in browser (when --mode=web)
        #[arg(long = "web-skip-opening")]
        web_skip_opening: bool,
    },
    /// Manage ecscope's profiles
    Profiles {
        #[command(subcommand)]
        profiles_command: ProfilesCommand,
    },
    /// Open monitoring TUI
    Monitor {
        /// Profile to use
        #[arg(value_name = "PROFILE")]
        profile_name: String,
        /// Filtration query for service names
        #[arg(short = 's', long = "service-filter", value_name = "REGEX", value_parser=validate_filter_query)]
        service_name_filter: Option<Regex>,
        /// Filtration query for cluster keys
        #[arg(short = 'k', long = "key-filter", value_name = "REGEX", value_parser=validate_filter_query)]
        key_filter: Option<Regex>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProfilesCommand {
    /// Add a new profile
    Add {
        /// Profile name
        #[arg(value_name = "PROFILE_NAME")]
        name: String,
    },
    /// List profiles
    List,
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self.command {
            EcscopeCommand::Deployments {
                profile_name,
                service_name_filter,
                key_filter,
                state,
                format,
                mode,
                web_skip_opening,
            } => format!(
                r#"
command:                     List Deployments
profile:                     {}
service name filter:         {}
key filter:                  {}
state:                       {}
format:                      {}
mode:                        {}
skip opening web results:    {}
"#,
                profile_name,
                service_name_filter
                    .as_ref()
                    .map_or(NOT_PROVIDED, |s| s.as_str()),
                key_filter.as_ref().map_or(NOT_PROVIDED, |r| r.as_str()),
                state.as_ref().map_or(NOT_PROVIDED, |s| s.as_ref()),
                format,
                mode,
                web_skip_opening,
            ),
            EcscopeCommand::Profiles { profiles_command } => match profiles_command {
                ProfilesCommand::Add { name } => format!(
                    r#"
command:    Add Profile
name:       {name}
"#
                ),
                ProfilesCommand::List => r#"
command:    List Profiles
"#
                .to_string(),
            },
            EcscopeCommand::Monitor {
                profile_name,
                service_name_filter,
                key_filter,
            } => format!(
                r#"
command:                Monitor resources
profile:                {}
service name filter:    {}
key filter:             {}
"#,
                profile_name,
                service_name_filter
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(NOT_PROVIDED.to_string()),
                key_filter
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(NOT_PROVIDED.to_string()),
            ),
        };

        f.write_str(&output)
    }
}

fn validate_filter_query(value: &str) -> Result<Regex, String> {
    Regex::new(value).map_err(|e| format!("query \"{value}\" is not valid regex: {e}"))
}
