pub const UNKNOWN: &str = "<unknown>";
use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    /// Delimited output
    Delimited,
    /// JSON output
    Json,
    /// Plain output
    Plain,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            OutputFormat::Plain => "plain",
            OutputFormat::Json => "json",
            OutputFormat::Delimited => "delimited",
        };

        write!(f, "{value}")?;

        Ok(())
    }
}

#[derive(Clone, Debug, ValueEnum, Copy)]
pub enum DeploymentState {
    /// Deployment has no pending tasks
    Finished,
    /// Deployment is in progress
    InProgress,
    /// Deployment is in progress and has failed tasks
    Failing,
}

impl AsRef<str> for DeploymentState {
    fn as_ref(&self) -> &str {
        match self {
            DeploymentState::Finished => "finished",
            DeploymentState::InProgress => "in-progress",
            DeploymentState::Failing => "failing",
        }
    }
}

impl std::fmt::Display for DeploymentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())?;

        Ok(())
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputMode {
    /// Default one time output to stdout
    Default,
    /// Web view
    Web,
}

impl std::fmt::Display for OutputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            OutputMode::Default => "default",
            OutputMode::Web => "web",
        };

        write!(f, "{value}")?;

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum Environment {
    Dev,
    Prod,
}

pub fn get_env() -> Environment {
    match std::env::var("ECSCOPE_DEV").unwrap_or_default().as_str() {
        "1" => Environment::Dev,
        _ => Environment::Prod,
    }
}
