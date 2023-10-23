use crate::discord::*;
use crate::state::State;
use chrono::Local;
use mirakurun_client_common::config::Service;
use reqwest::blocking::Client;
use std::env;

pub struct Reporter;

impl Reporter {
    pub async fn send_failure(
        client: &Client,
        webhook_url: &String,
        service: &Service,
        error: &str,
    ) -> Result<(), String> {
        // Skip if last state was failure.
        if State::exist(service) {
            return Ok(());
        }

        let webhook = Webhook {
            username: env!("CARGO_PKG_NAME").to_string(),
            content: if let Some(maintainer) = &service.maintainer {
                Some(format!("<@{}>", maintainer))
            } else {
                None
            },
            embeds: vec![Embed {
                title: format!(":warning: 障害発生: {}", service.key),
                description: format!("{} にアクセスできません。", service.key),
                url: "https://github.com/StarryBlueSky/docker-dtv".to_string(),
                color: 0xcd0000,
                footer: Footer {
                    text: format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
                },
                fields: vec![
                    Field {
                        name: "詳細".to_string(),
                        value: error.to_string(),
                        inline: false,
                    },
                    Field {
                        name: "発生時刻".to_string(),
                        value: Reporter::get_timestamp(),
                        inline: false,
                    },
                ],
            }],
        };

        Self::send_webhook(&client, webhook_url, webhook).and_then(|_| State::create(service))
    }

    pub async fn send_recovery(
        client: &Client,
        webhook_url: &String,
        service: &Service,
    ) -> Result<(), String> {
        if !State::exist(service) {
            return Ok(());
        }

        let webhook = Webhook {
            username: env!("CARGO_PKG_NAME").to_string(),
            content: if let Some(maintainer) = &service.maintainer {
                Some(format!("<@{}>", maintainer))
            } else {
                None
            },
            embeds: vec![Embed {
                title: format!(":white_check_mark: 障害復旧: {}", service.key),
                description: format!("{} が復旧しました。", service.key),
                url: "https://github.com/StarryBlueSky/docker-dtv".to_string(),
                color: 0x54ff9f,
                footer: Footer {
                    text: format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
                },
                fields: vec![Field {
                    name: "復旧時刻".to_string(),
                    value: Reporter::get_timestamp(),
                    inline: false,
                }],
            }],
        };

        Self::send_webhook(&client, webhook_url, webhook).and_then(|_| {
            // Reset failure state
            State::delete(service)
        })
    }

    fn send_webhook(client: &Client, webhook_url: &String, webhook: Webhook) -> Result<(), String> {
        let response = client
            .post(webhook_url)
            .json(&webhook)
            .send()
            .map_err(|error| format!("Failed to send webhook payload: {}", error))?;

        if !response.status().is_success() {
            eprintln!("Response = {:#?}", response);

            return Err(format!(
                "Remote server returned invalid response with {}: {:?} ({})",
                response.status(),
                response.text(),
                webhook_url
            ));
        }

        Ok(())
    }

    fn get_timestamp() -> String {
        let datetime = Local::now();
        datetime.format("%Y/%m/%d %H:%M:%S").to_string()
    }
}
