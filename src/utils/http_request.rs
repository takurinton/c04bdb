use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::utils::http_response::HttpResponse;
use crate::utils::tls::TlsConnectorBuilder;
use crate::utils::url::Url;

pub struct HttpRequest {
    pub url: Url,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    request: String,
}

impl HttpRequest {
    pub fn new(url: &str, headers: HashMap<String, String>) -> Self {
        let url = Url::parse(url);
        let host = url.host();
        let port = url.port();
        let path = url.path();
        let headers = headers.into_iter().collect();
        let query = url.query_pairs();

        Self {
            url,
            host,
            port,
            path,
            query,
            headers,
            request: String::new(),
        }
    }

    async fn init_stream(&self) -> tokio_native_tls::TlsStream<TcpStream> {
        let tcp_stream = match TcpStream::connect((self.host.as_str(), self.port)).await {
            Ok(tcp_stream) => tcp_stream,
            Err(why) => panic!("tcp stream error: {:?}", why),
        };
        let tls_stream = match TlsConnectorBuilder::new()
            .connector
            .connect(self.host.as_str(), tcp_stream)
            .await
        {
            Ok(tls_stream) => tls_stream,
            Err(why) => panic!("tls stream error: {:?}", why),
        };

        tls_stream
    }

    pub async fn get(&mut self) -> &mut HttpRequest {
        let request = format!(
            "GET {}?{} HTTP/1.1\r\nHost: {}\r\n{}\r\n\r\n",
            self.path,
            self.query
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&"),
            self.host,
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join("\r\n")
        );

        self.request = request.clone();

        self
    }

    pub async fn post(&mut self, body: &str) -> &mut HttpRequest {
        let content_length = body.len();
        let request = format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\n{}\r\nContent-Length: {}\r\n\r\n{}",
            self.path,
            self.host,
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join("\r\n"),
            content_length,
            body
        );

        self.request = request.clone();

        self
    }

    pub async fn send(&self) -> Result<HttpResponse, std::io::Error> {
        let mut stream = self.init_stream().await;

        let _ = match stream.write_all(self.request.as_bytes()).await {
            Ok(_) => (),
            Err(why) => panic!("write error: {:?}", why),
        };

        HttpResponse::from_stream(&mut stream).await
    }
}
