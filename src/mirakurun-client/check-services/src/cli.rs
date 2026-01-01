use clap::{crate_authors, crate_version, Parser};

#[derive(Parser)]
#[command(version = crate_version!(), author = crate_authors!())]
pub struct CommandLine {
    #[arg(
        required = true,
        use_value_delimiter = true,
        env = "CHECK_SERVICE_TARGETS"
    )]
    pub service_keys: Vec<String>,

    #[arg(required = true, env = "DISCORD_WEBHOOK_URL")]
    pub discord_webhook_url: String,

    #[arg(long, default_value = "external.json", env = "EXTERNAL_JSON_PATH")]
    pub config_path: String,
}
