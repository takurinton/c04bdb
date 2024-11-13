pub fn encode(input: &str) -> String {
    let mut encoded = String::new();

    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            // 他の文字は %XX の形式でエンコード
            _ => encoded.push_str(&format!("%{:02X}", byte)),
        }
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("abc"), "abc");
        assert_eq!(encode("あいう"), "%E3%81%82%E3%81%84%E3%81%86");
        assert_eq!(encode("a b c"), "a%20b%20c");
        assert_eq!(encode("a+b+c"), "a%2Bb%2Bc");
        assert_eq!(encode("a=b=c"), "a%3Db%3Dc");
        assert_eq!(encode("a:b:c"), "a%3Ab%3Ac");
        assert_eq!(encode("a~b~c"), "a~b~c");
        assert_eq!(encode("a!b!c"), "a%21b%21c");
        assert_eq!(encode("a*b*c"), "a%2Ab%2Ac");
        assert_eq!(encode("a'b'c"), "a%27b%27c");
        assert_eq!(encode("a(c)c"), "a%28c%29c");
        assert_eq!(encode("a)c)c"), "a%29c%29c");
        assert_eq!(encode("a;c;c"), "a%3Bc%3Bc");
        assert_eq!(encode("a:c:c"), "a%3Ac%3Ac");
        assert_eq!(encode("a,d,c"), "a%2Cd%2Cc");
        assert_eq!(encode("a/d/c"), "a%2Fd%2Fc");
        assert_eq!(encode("a\\d\\c"), "a%5Cd%5Cc");
        assert_eq!(encode("a?d?c"), "a%3Fd%3Fc");
        assert_eq!(encode("a#d#c"), "a%23d%23c");
        assert_eq!(encode("a&d&c"), "a%26d%26c");
        assert_eq!(encode("a=d=c"), "a%3Dd%3Dc");
        assert_eq!(encode("a@d@c"), "a%40d%40c");
        assert_eq!(encode("a$d$c"), "a%24d%24c");
        assert_eq!(encode("a`d`c"), "a%60d%60c");
    }
}
