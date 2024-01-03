
impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        if query_ip.contains('.') {
            if Self::is_valid_ipv4(&query_ip) {
                return "IPv4".to_string();
            }
        } else if query_ip.contains(':') {
            if Self::is_valid_ipv6(&query_ip) {
                return "IPv6".to_string();
            }
        }
        "Neither".to_string()
    }
    
    fn is_valid_ipv4(ip: &str) -> bool {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            return false;
        }
        for part in parts {
            if part.is_empty() || part.len() > 3 {
                return false;
            }
            if part.starts_with('0') && part.len() > 1 {
                return false;
            }
            if let Ok(num) = part.parse::<u8>() {
                if num > 255 {
                    return false;
                }
            } else {
                return false;
            }
        }
        true

    }
    
    fn is_valid_ipv6(ip: &str) -> bool {
        let parts: Vec<&str> = ip.split(':').collect();
        if parts.len() != 8 {
            return false;
        }
        for part in parts {
            if part.is_empty() || part.len() > 4 {
                return false;
            }
            for ch in part.chars() {
                if !ch.is_digit(16) {
                    return false;
                }
            }
        }
        true

    }
}
