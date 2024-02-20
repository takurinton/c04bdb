// http client のフルスクラッチ実装

use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use crate::utils::url::Url;

pub struct HttpClient {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

impl HttpClient {
    pub fn new(host: &str, port: u16, path: &str) -> HttpClient {
        HttpClient {
            host: host.to_string(),
            port,
            path: path.to_string(),
            query: HashMap::new(),
            headers: HashMap::new(),
        }
    }

    pub fn add_query(&mut self, key: &str, value: &str) {
        self.query.insert(key.to_string(), value.to_string());
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self) -> String {
        let mut query_string = String::new();
        for (key, value) in &self.query {
            query_string.push_str(&format!("{}={}&", key, value));
        }
        let query_string = query_string.trim_end_matches('&');

        let mut headers = String::new();
        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }

        let request = format!(
            "GET {}?{} HTTP/1.1\r\nHost: {}\r\n{}\r\n",
            self.path, query_string, self.host, headers
        );

        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap();
        stream.write(request.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        response
    }

    pub fn post(&self, body: &str) -> String {
        let mut query_string = String::new();
        for (key, value) in &self.query {
            query_string.push_str(&format!("{}={}&", key, value));
        }
        let query_string = query_string.trim_end_matches('&');

        let mut headers = String::new();
        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }

        let request = format!(
            "POST {}?{} HTTP/1.1\r\nHost: {}\r\n{}\r\n{}\r\n",
            self.path,
            query_string,
            self.host,
            headers,
            body
        );

        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap();
        stream.write(request.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        response
    }

    pub fn put(&self, body: &str) -> String {
        let mut query_string = String::new();
        for (key, value) in &self.query {
            query_string.push_str(&format!("{}={}&", key, value));
        }
        let query_string = query_string.trim_end_matches('&');

        let mut headers = String::new();
        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }

        let request = format!(
            "PUT {}?{} HTTP/1.1\r\nHost: {}\r\n{}\r\n{}\r\n",
            self.path,
            query_string,
            self.host,
            headers,
            body
        );

        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap();
        stream.write(request.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        response
    }

    pub fn delete(&self) -> String {
        let mut query_string = String::new();
        for (key, value) in &self.query {
            query_string.push_str(&format!("{}={}&", key, value));
        }
        let query_string = query_string.trim_end_matches('&');

        let mut headers = String::new();
        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }

        let request = format!(
            "DELETE {}?{} HTTP/1.1\r\nHost: {}\r\n{}\r\n",
            self.path, query_string, self.host, headers
        );

        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap();
        stream.write(request.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        response
    }
}

// pub fn fetch_url(url: &str) -> String {
//     let url = Url::parse(url);
//     let host = url.host();
//     let port = url.port();
//     let path = url.path();
//     let mut query = HashMap::new();
//     for (key, value) in url.query_pairs() {
//         query.insert(key.to_string(), value.to_string());
//     }

//     let mut headers = HashMap::new();
//     headers.insert("User-Agent".to_string(), "Rust".to_string());

//     let mut client = HttpClient {
//         host: host.to_string(),
//         port,
//         path: path.to_string(),
//         query,
//         headers,
//     };

//     client.get()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_fetch_url() {
//         let response = fetch_url("http://example.com");
//         assert!(response.contains("Example Domain"));
//     }
// }