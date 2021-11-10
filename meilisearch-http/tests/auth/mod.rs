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

    pub async fn get_api_key(&self, key: impl AsRef<str>) -> (Value, StatusCode) {
        let url = format!("/keys/{}", key.as_ref());
        self.service.get(url).await
    }

    pub async fn put_api_key(&self, key: impl AsRef<str>, content: Value) -> (Value, StatusCode) {
        let url = format!("/keys/{}", key.as_ref());
        self.service.put(url, content).await
    }

    pub async fn list_api_keys(&self) -> (Value, StatusCode) {
        let url = "/keys";
        self.service.get(url).await
    }
}
