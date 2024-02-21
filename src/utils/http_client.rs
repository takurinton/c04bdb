use std::collections::HashMap;

use crate::utils::http_request::HttpRequest;
use crate::utils::http_response::HttpResponse;

fn default_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "Rust".to_string());
    headers.insert("Accept".to_string(), "*/*".to_string());
    headers.insert("Connection".to_string(), "close".to_string());
    headers
}

pub struct HttpClient {
    pub url: String,
    pub headers: HashMap<String, String>,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            url: "".to_string(),
            headers: default_headers(),
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn header_authorization(&mut self, token: String) -> &mut Self {
        self.headers
            .insert("Authorization".to_string(), format!("Bearer {}", token));
        self
    }

    pub async fn get(&self, url: &str) -> Result<HttpResponse, std::io::Error> {
        let mut request = HttpRequest::new(url, self.headers.clone());
        request.get().await;
        self.send(request).await
    }

    pub async fn post(&self, url: &str, body: String) -> Result<HttpResponse, std::io::Error> {
        let mut request = HttpRequest::new(url, self.headers.clone());
        request.post(body.as_str()).await;
        self.send(request).await
    }

    pub async fn send(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        request.send().await
    }
}
