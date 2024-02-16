use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TunnelInfo {
    #[serde(rename = "tunnels")]
    tunnels: Vec<Tunnel>,

    #[serde(rename = "uri")]
    uri: String,

    #[serde(rename = "next_page_uri")]
    next_page_uri: Option<serde_json::Value>,
}

impl TunnelInfo {
    pub fn get_tunnels(&self) -> Vec<Tunnel> {
        self.tunnels.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tunnel {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "public_url")]
    public_url: String,

    #[serde(rename = "started_at")]
    started_at: String,

    #[serde(rename = "proto")]
    proto: String,

    #[serde(rename = "region")]
    region: String,

    #[serde(rename = "tunnel_session")]
    tunnel_session: Endpoint,

    #[serde(rename = "endpoint")]
    endpoint: Endpoint,

    #[serde(rename = "forwards_to")]
    forwards_to: String,
}

impl Tunnel {
    pub fn get_public_url(&self) -> String {
        self.public_url.to_owned()
    }
    pub fn get_forwards_to(&self) -> String {
        self.forwards_to.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Endpoint {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "uri")]
    uri: String,
}
