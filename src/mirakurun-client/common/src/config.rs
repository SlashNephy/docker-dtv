use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub user_agent: String,
    pub services: HashMap<String, Service>,
}

impl Config {
    pub fn load(path: &str) -> Config {
        let reader =
            File::open(path).unwrap_or_else(|error| panic!("Failed to open {}: {}", path, error));

        let mut config: Config = if path.ends_with(".json") {
            serde_json::from_reader(reader)
                .unwrap_or_else(|error| panic!("Failed to parse JSON {}: {}", path, error))
        } else if path.ends_with(".yml") || path.ends_with(".yaml") {
            serde_yaml::from_reader(reader)
                .unwrap_or_else(|error| panic!("Failed to parse YAML {}: {}", path, error))
        } else {
            panic!("Unsupported config file: {}", path);
        };

        config
            .services
            .insert("localhost".to_string(), Service::localhost());
        for (k, v) in config.services.iter_mut() {
            v.key = k.to_string();
        }

        return config;
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
    #[serde(default)]
    pub key: String,

    #[serde(default = "Service::default_scheme")]
    pub scheme: String,
    pub base_uri: String,
    #[serde(default)]
    pub basic: Option<String>,
    #[serde(default)]
    pub maintainer: Option<String>,
}

impl Service {
    fn default_scheme() -> String {
        "https".to_string()
    }

    fn localhost() -> Service {
        Service {
            key: "localhost".to_string(),
            scheme: "http".to_string(),
            base_uri: "localhost:40772/api".to_string(),
            basic: None,
            maintainer: None,
        }
    }
}
