use std::collections::HashMap;

pub struct Url {
    #[allow(dead_code)]
    scheme: String,
    #[allow(dead_code)]
    user_info: Option<String>,
    domain: String,
    port: Option<u16>,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

impl Url {
    pub fn parse(url: &str) -> Self {
        let mut url_parts = url.splitn(2, "://");
        let scheme = url_parts.next().unwrap().to_string();
        let rest = url_parts.next().unwrap_or("");

        let mut rest_parts = rest.splitn(2, '/');
        let authority = rest_parts.next().unwrap_or("");
        let path_and_beyond = format!("/{}", rest_parts.next().unwrap_or(""));

        let mut authority_parts = authority.split('@');
        let (user_info, domain_and_port) = if authority_parts.clone().count() > 1 {
            (
                Some(authority_parts.next().unwrap().to_string()),
                authority_parts.next().unwrap(),
            )
        } else {
            (None, authority_parts.next().unwrap_or(""))
        };

        let mut domain_and_port_parts = domain_and_port.splitn(2, ':');
        let domain = domain_and_port_parts.next().unwrap_or("").to_string();
        let port = domain_and_port_parts
            .next()
            .map(|p| p.parse::<u16>().unwrap_or_default());

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

    #[allow(dead_code)]
    pub fn is_https(&self) -> bool {
        self.scheme == "https"
    }

    pub fn host(&self) -> String {
        self.domain.clone()
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(443)
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    #[allow(dead_code)]
    pub fn query(&self) -> Option<String> {
        self.query.clone()
    }

    #[allow(dead_code)]
    pub fn fragment(&self) -> Option<String> {
        self.fragment.clone()
    }

    pub fn query_pairs(&self) -> HashMap<String, String> {
        let mut pairs = Vec::new();
        if let Some(query) = &self.query {
            for pair in query.split('&') {
                let mut pair = pair.splitn(2, '=');
                let key = pair.next().unwrap_or("").to_string();
                let value = pair.next().unwrap_or("").to_string();
                pairs.push((key, value));
            }
        }

        pairs.into_iter().collect()
    }
}

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
        assert_eq!(pairs.get("foo"), Some(&"bar".to_string()));
        assert_eq!(pairs.get("baz"), Some(&"qux".to_string()));
    }
}
