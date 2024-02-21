use std::collections::HashMap;
use tokio::io::AsyncBufReadExt;
use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpStream;

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    ///
    /// # Example
    /// ```
    /// let body = match response.json::<ChatGPTResponse>().await {
    ///    Ok(body) => body,
    ///    Err(_) => {
    ///       return "コンテンツの取得に失敗しました。".to_string();
    ///   }
    /// };
    /// ```
    pub async fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        let full = &self.body.as_bytes();
        match serde_json::from_slice(full) {
            Ok(body) => Ok(body),
            Err(why) => {
                println!("json parse error: {:?}", why);
                Err(why)
            }
        }
    }

    async fn parse(
        stream: &mut tokio_native_tls::TlsStream<TcpStream>,
    ) -> Result<HttpResponse, std::io::Error> {
        let mut stream_reader = io::BufReader::new(stream);
        let mut headers = HashMap::new();
        let mut body = Vec::new();
        let mut status_code = 0;

        let mut status_line = String::new();
        stream_reader.read_line(&mut status_line).await?;
        if let Some(code) = status_line.split_whitespace().nth(1) {
            status_code = code.parse().unwrap_or(0);
        }

        // header を読み込む
        let mut chunked = false;
        loop {
            let mut line = String::new();
            stream_reader.read_line(&mut line).await?;
            if line == "\r\n" {
                break;
            }

            let parts: Vec<&str> = line.trim_end_matches("\r\n").splitn(2, ": ").collect();
            if parts.len() == 2 {
                headers.insert(parts[0].to_string(), parts[1].to_string());
                // Transfer-Encoding: chunked の場合は chunked で処理するため、フラグを立てる
                if parts[0] == "Transfer-Encoding" && parts[1] == "chunked" {
                    chunked = true;
                }
            }
        }

        if chunked {
            // Transfer-Encoding: chunked の場合は chunked で処理する
            loop {
                let mut size_str = String::new();
                stream_reader.read_line(&mut size_str).await?;
                let size = match usize::from_str_radix(size_str.trim(), 16) {
                    Ok(size) => size,
                    Err(_) => {
                        break;
                    }
                };

                if size == 0 {
                    // chunk のサイズが0の場合は終了
                    break;
                }

                let mut buffer = vec![0; size];
                stream_reader.read_exact(&mut buffer).await?;
                body.extend(buffer);

                // chunk の終わりの CRLF を読み飛ばす
                let mut end_of_chunk = vec![0; 2];
                stream_reader.read_exact(&mut end_of_chunk).await?;
            }
        } else {
            if let Some(content_length) = headers.get("Content-Length") {
                let content_length = content_length.parse::<usize>().unwrap_or(0);
                let mut buffer = vec![0; content_length];
                stream_reader.read_exact(&mut buffer).await?;
                body.extend(buffer);
            }
        }

        let body_string = String::from_utf8(body).expect("Failed to convert body to String");

        Ok(HttpResponse {
            status_code,
            headers,
            body: body_string,
        })
    }

    pub async fn from_stream(
        stream: &mut tokio_native_tls::TlsStream<TcpStream>,
    ) -> Result<HttpResponse, std::io::Error> {
        HttpResponse::parse(stream).await
    }
}
