use crate::config::ConfigSource;
use aws_config::SdkConfig;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::sts::AssumeRoleProvider;

pub async fn get_sdk_config(config_source: &ConfigSource) -> SdkConfig {
    match config_source {
        ConfigSource::AssumeRole { role_arn } => {
            let provider = AssumeRoleProvider::builder(role_arn)
                .session_name("escope-session")
                .build()
                .await;
            aws_config::from_env()
                .credentials_provider(provider)
                .load()
                .await
        }
        ConfigSource::Env => aws_config::load_from_env().await,
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
    }
}
