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

        write!(f, "{}", value)?;

        Ok(())
    }
}

#[derive(Clone, Debug, ValueEnum, Copy)]
pub enum DeploymentState {
    /// Deployment has no pending tasks
    Finished,
    /// Deployment is pending
    Pending,
    /// Deployment is pending and has failed tasks
    Failing,
}

impl AsRef<str> for DeploymentState {
    fn as_ref(&self) -> &str {
        match self {
            DeploymentState::Finished => "finished",
            DeploymentState::Pending => "pending",
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
