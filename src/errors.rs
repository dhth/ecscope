use crate::cmds::{AddProfileError, ListDeploymentsError, ListProfilesError};
use crate::utils::{ConfigDirError, GetClustersError};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    GetConfigDirectory(#[from] ConfigDirError),
    #[error(transparent)]
    GetClusters(#[from] GetClustersError),
    #[error(transparent)]
    AddProfile(#[from] AddProfileError),
    #[error(transparent)]
    ListProfiles(#[from] ListProfilesError),
    #[error(transparent)]
    RunMonitor(#[from] anyhow::Error),
    #[error(transparent)]
    ListDeployments(#[from] ListDeploymentsError),
}

impl AppError {
    pub fn code(&self) -> Option<u16> {
        match self {
            AppError::GetConfigDirectory(e) => match e {
                ConfigDirError::XDGConfigHomeNotAbsolute => None,
                ConfigDirError::CouldntGetConfigDir => Some(100),
            },
            AppError::GetClusters(e) => match e {
                GetClustersError::ProfileDoesntExist => None,
                GetClustersError::CouldntReadProfileFile(_) => Some(200),
                GetClustersError::ConfigFileInvalid(_) => None,
            },
            AppError::AddProfile(e) => match e {
                AddProfileError::ProfileNameInvalid(_) => None,
                AddProfileError::CouldntCreateProfileDir(..) => Some(300),
                AddProfileError::ProfileAlreadyExists => None,
                AddProfileError::CouldntOpenFile(_) => Some(301),
                AddProfileError::CouldntWriteToFile(_) => Some(302),
            },
            AppError::ListProfiles(e) => match e {
                ListProfilesError::ReadFilesInDataDir(_) => Some(400),
                ListProfilesError::GetFileFromDataDir(_) => Some(401),
                ListProfilesError::GetFileStem(_) => Some(402),
            },
            AppError::RunMonitor(_) => Some(500),
            AppError::ListDeployments(e) => match e {
                ListDeploymentsError::SerialiseToJson(_) => Some(600),
                ListDeploymentsError::SerializeToCSV(_) => Some(601),
                ListDeploymentsError::FlushResultsToCSVWriter(_) => Some(602),
                ListDeploymentsError::Unexpected(_) => Some(603),
            },
        }
    }
}
