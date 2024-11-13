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
