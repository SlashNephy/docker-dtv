use crate::cli::CommandLine;
use crate::reporter::Reporter;
use clap::Parser;
use mirakurun_client_common::config::Config;
use reqwest::blocking::Client;
use reqwest::redirect;
use std::time::Duration;

mod cli;
mod discord;
mod reporter;
mod state;

#[tokio::main]
async fn main() {
    let opts = CommandLine::parse();
    let config = Config::load(&opts.config_path);

    let client = Client::builder()
        .user_agent(format!(
            "{} {}/{}",
            config.user_agent,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .connect_timeout(Duration::from_secs(15))
        .timeout(None)
        .redirect(redirect::Policy::none())
        .referer(false)
        .tcp_nodelay(true)
        .build()
        .unwrap_or_else(|error| panic!("Failed to create http client: {}", error));

    for service_key in opts.service_keys {
        if let Some(service) = config.services.get(&service_key) {
            println!("Checking service for {}...", service_key);

            match service.execute(&client, "/status", false, true) {
                Ok(_) => {
                    println!("OK!: {}", service_key);

                    match Reporter::send_recovery(&client, &opts.discord_webhook_url, service).await
                    {
                        Ok(_) => {}
                        Err(error) => {
                            eprintln!("ERR: {}", error)
                        }
                    };
                }
                Err(error) => {
                    eprintln!("ERR: {}", error);

                    match Reporter::send_failure(
                        &client,
                        &opts.discord_webhook_url,
                        service,
                        &error,
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(error) => {
                            eprintln!("ERR: {}", error)
                        }
                    };
                }
            }
        } else {
            eprintln!("ERR: Unknown service {}", service_key);
        }
    }
}
