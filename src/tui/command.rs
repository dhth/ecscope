use crate::config::ClusterConfig;
use crate::domain::ServiceDetails;

#[derive(Clone, Debug)]
pub(super) enum Command {
    GetServices(ClusterConfig),
    RefreshService((ServiceDetails, usize)),
    GetTasks((ServiceDetails, bool)),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::GetServices(_) => write!(f, "get services"),
            Command::RefreshService((service_details, _)) => {
                write!(f, "refresh service: {}", service_details.name)
            }
            Command::GetTasks((service_details, _)) => {
                write!(f, "get tasks for service: {}", service_details.name)
            }
        }
    }
}
