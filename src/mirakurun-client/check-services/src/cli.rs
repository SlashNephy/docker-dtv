use clap::{crate_authors, crate_version, Parser};

#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct CommandLine {
    #[clap(
        required = true,
        use_delimiter = true,
        require_delimiter = true,
        env = "CHECK_SERVICE_TARGETS"
    )]
    pub service_keys: Vec<String>,

    #[clap(required = true, env = "DISCORD_WEBHOOK_URL")]
    pub discord_webhook_url: String,

    #[clap(long, default_value = "external.json", env = "EXTERNAL_JSON_PATH")]
    pub config_path: String,
}
