pub struct Url {
    scheme: String,
    user_info: Option<String>,
    domain: String,
    port: Option<u16>,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

impl Url {
    fn parse(url: &str) -> Self {
        let mut url_parts = url.splitn(2, "://");
        let scheme = url_parts.next().unwrap().to_string();
        let rest = url_parts.next().unwrap_or("");

        // オーソリティとパス以降を分割
        let mut rest_parts = rest.splitn(2, '/');
        let authority = rest_parts.next().unwrap_or("");
        let path_and_beyond = format!("/{}", rest_parts.next().unwrap_or(""));

        // ユーザー情報とドメイン（ポート含む）を分割
        let mut authority_parts = authority.split('@');
        let (user_info, domain_and_port) = if authority_parts.clone().count() > 1 {
            (
                Some(authority_parts.next().unwrap().to_string()),
                authority_parts.next().unwrap(),
            )
        } else {
            (None, authority_parts.next().unwrap_or(""))
        };

        // ドメインとポートを分割
        let mut domain_and_port_parts = domain_and_port.splitn(2, ':');
        let domain = domain_and_port_parts.next().unwrap_or("").to_string();
        let port = domain_and_port_parts
            .next()
            .map(|p| p.parse::<u16>().unwrap_or_default());

        // パス、クエリ、フラグメントを正確に分割
        let mut path = path_and_beyond.clone();
        let mut query = None;
        let mut fragment = None;

        if let Some(frag_pos) = path_and_beyond.rfind('#') {
            fragment = Some(path_and_beyond[frag_pos + 1..].to_string());
            path = path_and_beyond[..frag_pos].to_string();
        }

        if let Some(query_pos) = path.rfind('?') {
            query = Some(path[query_pos + 1..].to_string());
            path = path[..query_pos].to_string();
        }

        Url {
            scheme,
            user_info,
            domain,
            port,
            path,
            query,
            fragment,
        }
    }

    pub fn to_string(&self) -> String {
        let mut url = format!("{}://", self.scheme);
        if let Some(user_info) = &self.user_info {
            url.push_str(&format!("{}@", user_info));
        }
        url.push_str(&self.domain);
        if let Some(port) = self.port {
            url.push_str(&format!(":{}", port));
        }
        url.push_str(&self.path);
        if let Some(query) = &self.query {
            url.push_str(&format!("?{}", query));
        }
        if let Some(fragment) = &self.fragment {
            url.push_str(&format!("#{}", fragment));
        }
        url
    }

    pub fn host(&self) -> String {
        self.domain.clone()
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(80)
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn query(&self) -> Option<String> {
        self.query.clone()
    }

    pub fn fragment(&self) -> Option<String> {
        self.fragment.clone()
    }

    pub fn query_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        if let Some(query) = &self.query {
            for pair in query.split('&') {
                let mut pair_parts = pair.splitn(2, '=');
                let key = pair_parts.next().unwrap().to_string();
                let value = pair_parts.next().unwrap().to_string();
                pairs.push((key, value));
            }
        }
        pairs
    }
}

// テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parse() {
        let url = Url::parse("https://example.com:8080/path/to/somewhere?query#fragment");
        assert_eq!(url.scheme, "https");
        assert_eq!(url.user_info, None);
        assert_eq!(url.domain, "example.com");
        assert_eq!(url.port, Some(8080));
        assert_eq!(url.path, "/path/to/somewhere");
        assert_eq!(url.query, Some("query".to_string()));
        assert_eq!(url.fragment, Some("fragment".to_string()));

        let url = Url::parse("https://example.com/path/to/somewhere?query#fragment");
        assert_eq!(url.scheme, "https");
        assert_eq!(url.user_info, None);
        assert_eq!(url.domain, "example.com");
        assert_eq!(url.port, None);
        assert_eq!(url.path, "/path/to/somewhere");
        assert_eq!(url.query, Some("query".to_string()));
        assert_eq!(url.fragment, Some("fragment".to_string()));

        let url = Url::parse("https://example.com/path/to/somewhere#fragment");
        assert_eq!(url.scheme, "https");
        assert_eq!(url.user_info, None);
        assert_eq!(url.domain, "example.com");
        assert_eq!(url.port, None);
        assert_eq!(url.path, "/path/to/somewhere");
        assert_eq!(url.query, None);
        assert_eq!(url.fragment, Some("fragment".to_string()));

        let url = Url::parse("https://example.com/path/to/somewhere?foo=bar");
        assert_eq!(url.scheme, "https");
        assert_eq!(url.user_info, None);
        assert_eq!(url.domain, "example.com");
        assert_eq!(url.port, None);
        assert_eq!(url.path, "/path/to/somewhere");
        assert_eq!(url.query, Some("foo=bar".to_string()));
        assert_eq!(url.fragment, None);

        let url = Url::parse("https://example.com/path/to/somewhere");
        assert_eq!(url.scheme, "https");
        assert_eq!(url.user_info, None);
        assert_eq!(url.domain, "example.com");
        assert_eq!(url.port, None);
        assert_eq!(url.path, "/path/to/somewhere");
        assert_eq!(url.query, None);
        assert_eq!(url.fragment, None);

        let url = Url::parse("https://example.com");
        assert_eq!(url.scheme, "https");
        assert_eq!(url.user_info, None);
        assert_eq!(url.domain, "example.com");
        assert_eq!(url.port, None);
        assert_eq!(url.path, "/");
        assert_eq!(url.query, None);
        assert_eq!(url.fragment, None);
    }

    #[test]
    fn test_query_pairs() {
        let url = Url::parse("https://example.com/path/to/somewhere?foo=bar&baz=qux");
        let pairs = url.query_pairs();
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0], ("foo".to_string(), "bar".to_string()));
        assert_eq!(pairs[1], ("baz".to_string(), "qux".to_string()));
    }
}
