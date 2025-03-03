use crate::config::ConfigSource;
use aws_config::SdkConfig;
use aws_config::profile::ProfileFileCredentialsProvider;

pub async fn get_sdk_config(config_source: &ConfigSource) -> SdkConfig {
    match config_source {
        ConfigSource::Profile { name } => {
            aws_config::from_env()
                .credentials_provider(
                    ProfileFileCredentialsProvider::builder()
                        .profile_name(name)
                        .build(),
                )
                .load()
                .await
        }
        ConfigSource::Env => aws_config::load_from_env().await,
    }
}
