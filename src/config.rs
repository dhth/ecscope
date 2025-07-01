use std::fmt;

use regex::Regex;
use serde::{
    Deserialize, Deserializer,
    de::{self, Visitor},
};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Config {
    pub clusters: Vec<ClusterConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct ClusterConfig {
    pub keys: Vec<String>,
    pub arn: String,
    pub services: Vec<String>,
    pub config_source: ConfigSource,
}

impl<'de> Deserialize<'de> for ConfigSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfigSourceVisitor;

        impl Visitor<'_> for ConfigSourceVisitor {
            type Value = ConfigSource;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter
                    .write_str(r#"either "env" or "profile:<profile_name>" or "assume:<role_arn>""#)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "env" {
                    Ok(ConfigSource::Env)
                } else if value.starts_with("profile:") {
                    #[allow(clippy::unwrap_used)]
                    let profile_name = value.strip_prefix("profile:").unwrap().to_string();
                    Ok(ConfigSource::Profile { name: profile_name })
                } else if value.starts_with("assume:") {
                    #[allow(clippy::unwrap_used)]
                    let role_arn = value.strip_prefix("assume:").unwrap().to_string();
                    Ok(ConfigSource::AssumeRole { role_arn })
                } else {
                    Err(de::Error::invalid_value(de::Unexpected::Str(value), &self))
                }
            }
        }

        deserializer.deserialize_str(ConfigSourceVisitor)
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
pub enum ConfigSource {
    AssumeRole { role_arn: String },
    Env,
    Profile { name: String },
}

impl ClusterConfig {
    pub fn filter_by_cluster_key(self, re: &Regex) -> Option<Self> {
        for key in &self.keys {
            if re.is_match(key) {
                return Some(self);
            }
        }

        None
    }

    pub fn filter_by_service_name(mut self, re: &Regex) -> Option<Self> {
        let filtered_services = self
            .services
            .clone()
            .into_iter()
            .filter(|s| re.is_match(s))
            .collect::<Vec<_>>();

        if filtered_services.is_empty() {
            return None;
        }

        self.services = filtered_services;

        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    //-------------//
    //  SUCCESSES  //
    //-------------//

    #[test]
    fn deserializing_cluster_config_works() {
        // GIVEN
        let config = r#"
[[clusters]]
keys = ["qa"]
arn = "arn:aws:ecs:eu-central-1:111111111111:cluster/urlpreview-2-cluster-qa"
services = [
  "service-a",
  "service-b",
]
config_source = "env"

# --- #

[[clusters]]
keys = ["qa"]
arn = "arn:aws:ecs:eu-central-1:111111111111:cluster/prlserver-cluster-qa"
services = [
  "service-c",
  "service-d",
]
config_source = "profile:qa"

# --- #

[[clusters]]
keys = ["qa"]
arn = "arn:aws:ecs:eu-central-1:111111111111:cluster/prlserver-cluster-qa"
services = [
  "service-c",
  "service-d",
]
config_source = "assume:arn:aws:iam::222222222222:role/role-name"
"#;

        // WHEN
        let config: Config = toml::from_str(config).expect("config should've been deserialized");

        // THEN
        assert_yaml_snapshot!(config, @r#"
        clusters:
          - keys:
              - qa
            arn: "arn:aws:ecs:eu-central-1:111111111111:cluster/urlpreview-2-cluster-qa"
            services:
              - service-a
              - service-b
            config_source: Env
          - keys:
              - qa
            arn: "arn:aws:ecs:eu-central-1:111111111111:cluster/prlserver-cluster-qa"
            services:
              - service-c
              - service-d
            config_source:
              Profile:
                name: qa
          - keys:
              - qa
            arn: "arn:aws:ecs:eu-central-1:111111111111:cluster/prlserver-cluster-qa"
            services:
              - service-c
              - service-d
            config_source:
              AssumeRole:
                role_arn: "arn:aws:iam::222222222222:role/role-name"
        "#);
    }

    //------------//
    //  FAILURES  //
    //------------//

    #[test]
    fn deserializing_incorrect_cluster_config_fails() {
        // GIVEN
        let bad_configs = vec![
            r#"
[[clusters]]
keys = ["qa"]
arn = "arn:aws:ecs:eu-central-1:111111111111:cluster/cluster-a"
services = "service-a"
config_source = "env"
"#,
            r#"
[[clusters]]
keys = ["qa"]
arn = "arn:aws:ecs:eu-central-1:111111111111:cluster/cluster-a"
services = [
  "service-a",
  "service-b"
]
config_source = "unknown"
"#,
        ];

        for config in bad_configs {
            // WHEN
            let result: Result<ClusterConfig, toml::de::Error> = toml::from_str(config);

            // THEN
            assert!(result.is_err());
        }
    }
}
