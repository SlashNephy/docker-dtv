use clap::{crate_authors, crate_version, Parser};

#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct CommandLine {
    #[clap(
        help = "Service keys. These keys are defined in external.json. If more than one is specified, they will fall back in order from the top.",
        required = true,
        use_delimiter = true,
        require_delimiter = true
    )]
    pub service_keys: Vec<String>,

    #[clap(validator = CommandLine::validate_path, help = "Request path. It must start with slash.")]
    pub path: String,

    #[clap(
        long,
        default_value = "external.json",
        help = "Custom external.json path.",
        env = "EXTERNAL_JSON_PATH"
    )]
    pub config_path: String,

    #[clap(long, default_value = "5", help = "Connection timeout in seconds.")]
    pub connect_timeout: u64,

    #[clap(long, help = "Timeout in seconds.")]
    pub timeout: Option<u64>,

    #[clap(
        short,
        long,
        help = "If enabled, prints request and response information into stderr."
    )]
    pub debug: bool,
}

impl CommandLine {
    fn validate_path(value: &str) -> Result<(), String> {
        if value.starts_with("/") {
            Ok(())
        } else {
            Err("Path should start with slash (/).".to_string())
        }
    }
}
