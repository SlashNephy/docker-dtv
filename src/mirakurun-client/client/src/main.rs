use cli::CommandLine;
use mirakurun_client_common::config::Config;
use clap::Parser;
use reqwest::{blocking::Client, redirect};
use std::env;
use std::time::Duration;

mod cli;

fn main() {
    let opts = CommandLine::parse();
    let config = Config::load(&opts.config_path);

    let client = Client::builder()
        .user_agent(format!(
            "{} {}/{}",
            config.user_agent,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .connect_timeout(Duration::from_secs(opts.connect_timeout))
        .timeout(if let Some(t) = opts.timeout {
            Duration::from_secs(t).into()
        } else {
            None
        })
        .redirect(redirect::Policy::none())
        .referer(false)
        .tcp_nodelay(true)
        .build()
        .unwrap_or_else(|error| panic!("Failed to create http client: {}", error));

    for service_key in opts.service_keys {
        if let Some(service) = config.services.get(&service_key) {
            match service.execute(&client, &opts.path, opts.debug, false) {
                Ok(length) => {
                    eprintln!("Received {} bytes.", length);
                    return;
                }
                Err(error) => eprintln!("{}", error),
            }
        } else {
            eprintln!("Unknown service: {}", service_key);
        }
    }
}
