use super::ngrok_api_response::Tunnel;

#[derive(Debug)]
pub struct BotResponse {
    pub url: String,
    pub forwards_to: String,
    pub port: u32,
}

impl std::fmt::Display for BotResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = format!(
            "URL > {}\nPort > {}\nForwards To > {}",
            self.url, self.port, self.forwards_to
        );
        write!(f, "{}", message)
    }
}

impl From<Tunnel> for BotResponse {
    fn from(tunnel: Tunnel) -> Self {
        let public_url = tunnel.get_public_url().clone();
        let mut parts = public_url
            .split(":")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let port = parts.pop().unwrap_or_default();
        let port = port.parse::<u32>().unwrap_or_default();
        let url = parts.join(":");
        BotResponse {
            url,
            forwards_to: tunnel.get_forwards_to(),
            port,
        }
    }
}
