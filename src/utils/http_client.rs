use chrono::format;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::utils::url::Url;

pub struct HttpClient {
    pub url: Url,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/**
 * @todo 無限に抽象化できる
 */
impl HttpClient {
    fn default_headers() -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "Rust".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers
    }

    pub fn new(url: &str, headers: HashMap<String, String>) -> Self {
        let url = Url::parse(url);
        let host = url.host();
        let port = url.port();
        let path = url.path();
        let headers = headers
            .into_iter()
            .chain(Self::default_headers().into_iter())
            .collect();
        let query = url.query_pairs();

        Self {
            url,
            host,
            port,
            path,
            query,
            headers,
        }
    }

    pub async fn get(&self) -> Result<HttpResponse, std::io::Error> {
        println!("url: {}", self.url.to_string());
        // let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).await?;
        let mut stream = match TcpStream::connect(format!("{}:{}", self.host, self.port)).await {
            Ok(stream) => stream,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to connect: {}", e),
                ))
            }
        };

        println!("Connected to the server");

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\n",
            self.path, self.host
        );

        let mut request_headers = String::new();
        for (key, value) in &self.headers {
            request_headers.push_str(&format!("{}: {}\r\n", key, value));
        }

        println!("request: {}", request);

        let request = format!("{}\r\n{}", request, request_headers);
        stream.write_all(request.as_bytes()).await?;

        println!("Sent the request");

        let mut response = String::new();
        stream.read_to_string(&mut response).await?;

        println!("Received the response");

        let mut response = response.split("\r\n\r\n");
        let headers = response.next().unwrap();
        let body = response.next().unwrap();

        println!("headers: {}", headers);
        println!("body: {}", body);

        let mut headers = headers.split("\r\n");
        let status_line = headers.next().unwrap();
        let status_code = status_line.split_whitespace().nth(1).unwrap().parse().unwrap();

        println!("status_code: {}", status_code);

        let headers = headers
            .map(|header| {
                let mut header = header.splitn(2, ": ");
                let key = header.next().unwrap().to_string();
                let value = header.next().unwrap().to_string();
                (key, value)
            })
            .collect::<HashMap<String, String>>();

        Ok(HttpResponse {
            status_code,
            headers,
            body: body.to_string(),
        })

    }
}

pub async fn get(url: &str) -> Result<HttpResponse, std::io::Error> {
    let http_client = HttpClient::new(url, Default::default());
    http_client.get().await
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_fetch_url() {
//         let response = fetch_url("http://example.com");
//         assert!(response.contains("Example Domain"));
//     }
// }
