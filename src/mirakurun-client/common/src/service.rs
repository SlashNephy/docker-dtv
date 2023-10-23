use crate::config::Service;
use reqwest::blocking::Client;
use std::io;

impl Service {
    pub fn execute(&self, client: &Client, path: &str, debug: bool, dry_run: bool) -> Result<u64, String> {
        let url = format!("{}://{}{}", self.scheme, self.base_uri, path);
        let request_builder = client.get(url.clone());

        let request = match self.basic.clone() {
            Some(basic) => {
                assert_ne!(
                    self.scheme, "http",
                    "Security concern! YOU CANNOT use basic auth on HTTP plain connection."
                );

                let (user, password) = Service::parse_basic_token(&basic)
                    .unwrap_or_else(|| panic!("Failed parse basic token: {}", basic));
                request_builder.basic_auth(user, password)
            }
            None => request_builder,
        }
        .build()
        .unwrap_or_else(|error| panic!("Failed to build request for {}: {}", url, error));

        if debug {
            eprintln!("Request = {:#?}", request);
        }

        let mut response = client.execute(request).map_err(|error| {
            format!(
                "Failed to send request for {} (maybe due to network issue): {}",
                url, error
            )
        })?;

        if debug {
            eprintln!("Response = {:#?}", response);
        }

        // Just only accepts 2xx. 3xx can be meant OAuth2Proxy redirect.
        if !response.status().is_success() {
            return Err(format!(
                "Remote server returned invalid response with {}: {}",
                response.status(),
                url
            ))
        }

        if dry_run {
            return Ok(0);
        }

        let mut stdout = io::stdout();
        response
            .copy_to(&mut stdout)
            .map_err(|error| format!("Failed to write response into stdout: {}", error))
    }

    fn parse_basic_token(token: &str) -> Option<(String, Option<String>)> {
        token
            .split_once(":")
            .map(|(user, password)| (user.to_string(), Some(password.to_string())))
    }
}
