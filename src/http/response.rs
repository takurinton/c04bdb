use std::collections::HashMap;
use tokio::io::AsyncBufReadExt;
use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpStream;

use super::client::StatusCode;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
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
            Err(why) => Err(why),
        }
    }

    async fn parse(
        stream: &mut tokio_native_tls::TlsStream<TcpStream>,
    ) -> Result<HttpResponse, std::io::Error> {
        let mut stream_reader = io::BufReader::new(stream);
        let mut headers = HashMap::new();
        let mut body = Vec::new();
        let mut status_code = StatusCode::Unsupported;

        let mut status_line = String::new();
        stream_reader.read_line(&mut status_line).await?;
        if let Some(code) = status_line.split_whitespace().nth(1) {
            status_code = match code.parse::<u16>() {
                Ok(code) => match code {
                    200 => StatusCode::OK,
                    201 => StatusCode::Created,
                    202 => StatusCode::Accepted,
                    204 => StatusCode::NoContent,
                    301 => StatusCode::MovedPermanently,
                    302 => StatusCode::Found,
                    304 => StatusCode::NotModified,
                    400 => StatusCode::BadRequest,
                    401 => StatusCode::Unauthorized,
                    403 => StatusCode::Forbidden,
                    404 => StatusCode::NotFound,
                    405 => StatusCode::MethodNotAllowed,
                    408 => StatusCode::RequestTimeout,
                    429 => StatusCode::TooManyRequests,
                    500 => StatusCode::InternalServerError,
                    _ => StatusCode::Unsupported,
                },
                Err(_) => StatusCode::Unsupported,
            };
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
            // chunked ではない場合はそのまま処理する
            stream_reader.read_to_end(&mut body).await?;
        }

        let body_string = match String::from_utf8(body) {
            Ok(body) => body,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "response body is not utf-8",
                ))
            }
        };

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
