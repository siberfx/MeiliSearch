mod api_keys;
mod payload;

use crate::common::Server;
use actix_web::http::StatusCode;
use serde_json::Value;

impl Server {
    pub async fn new_auth() -> Self {
        let mut server = Server::new().await;
        server.service.options.master_key = Some("MASTER_KEY".to_string());

        server
    }

    pub fn use_api_key(&mut self, api_key: impl AsRef<str>) {
        self.service.api_key = Some(api_key.as_ref().to_string());
    }

    pub async fn add_api_key(&self, content: Value) -> (Value, StatusCode) {
        let url = "/keys";
        self.service.post(url, content).await
    }
}
