use crate::models::{
    bot_response::BotResponse, config::NgrokConfig, ngrok_api_response::TunnelInfo,
};

#[derive(Debug, Clone)]
pub struct NgrokApi {
    config: NgrokConfig,
    client: reqwest::Client,
}

impl NgrokApi {
    pub fn new(config: NgrokConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub async fn get_tunnel(&self) -> Result<Vec<BotResponse>, String> {
        let url = self.config.url.clone().unwrap_or_default();
        let res = self
            .client
            .get(url)
            .header("authorization", format!("Bearer {}", self.config.api_key))
            .header("ngrok-version", "2")
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<TunnelInfo>()
            .await
            .map_err(|e| e.to_string())?;
        let tunnels = res.get_tunnels();
        let bot_responses = tunnels
            .into_iter()
            .map(|tunnel| tunnel.into())
            .collect::<Vec<BotResponse>>();
        Ok(bot_responses)
    }
}
