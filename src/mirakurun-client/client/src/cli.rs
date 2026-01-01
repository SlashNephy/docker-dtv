use clap::{crate_authors, crate_version, Parser};

#[derive(Parser)]
#[command(version = crate_version!(), author = crate_authors!())]
pub struct CommandLine {
    #[arg(
        help = "Service keys. These keys are defined in external.json. If more than one is specified, they will fall back in order from the top.",
        required = true,
        use_value_delimiter = true
    )]
    pub service_keys: Vec<String>,

    #[arg(value_parser = CommandLine::validate_path, help = "Request path. It must start with slash.")]
    pub path: String,

    #[arg(
        long,
        default_value = "external.json",
        help = "Custom external.json path.",
        env = "EXTERNAL_JSON_PATH"
    )]
    pub config_path: String,

    #[arg(long, default_value = "5", help = "Connection timeout in seconds.")]
    pub connect_timeout: u64,

    #[arg(long, help = "Timeout in seconds.")]
    pub timeout: Option<u64>,

    #[arg(
        short,
        long,
        help = "If enabled, prints request and response information into stderr."
    )]
    pub debug: bool,
}

impl CommandLine {
    fn validate_path(value: &str) -> Result<String, String> {
        if value.starts_with("/") {
            Ok(value.to_string())
        } else {
            Err("Path should start with slash (/).".to_string())
        }
    }
}
