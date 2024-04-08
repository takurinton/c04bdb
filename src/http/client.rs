use std::collections::HashMap;

use super::request::HttpRequest;
use super::response::HttpResponse;

fn default_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "Rust".to_string());
    headers.insert("Accept".to_string(), "*/*".to_string());
    headers.insert("Connection".to_string(), "close".to_string());
    headers
}

#[derive(Debug)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    RequestTimeout = 408,
    TooManyRequests = 429,
    InternalServerError = 500,
    Unsupported = 0,
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

    /// Set header
    /// # Example
    /// ```
    /// let mut client = HttpClient::new();
    /// let response = client.set_header("Content-Type", "application/json").post("https://example.com", "{}").await;
    /// ```
    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Set header for Authorization
    /// # Example
    /// ```
    /// let token = "your_token".to_string();
    /// let mut client = HttpClient::new();
    /// let response = client.header_author(token).post("https://example.com", "{}").await;
    /// ```
    pub fn header_authorization(&mut self, token: String) -> &mut Self {
        self.headers
            .insert("Authorization".to_string(), format!("Bearer {}", token));
        self
    }

    /// Send GET request
    /// # Example
    /// ```
    /// let mut client = HttpClient::new();
    /// let response = client.get("https://example.com").await;
    /// ```
    pub async fn get(&self, url: &str) -> Result<HttpResponse, std::io::Error> {
        let mut request = HttpRequest::new(url, self.headers.clone());
        request.get().await;
        self.send(request).await
    }

    /// Send POST request
    /// # Example
    /// ```
    /// let mut client = HttpClient::new();
    /// let response = client.post("https://example.com", "{}").await;
    /// ```
    pub async fn post(&self, url: &str, body: String) -> Result<HttpResponse, std::io::Error> {
        let mut request = HttpRequest::new(url, self.headers.clone());
        request.post(body.as_str()).await;
        self.send(request).await
    }

    pub async fn send(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        request.send().await
    }
}
