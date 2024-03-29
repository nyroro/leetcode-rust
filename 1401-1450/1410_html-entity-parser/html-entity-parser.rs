
use std::collections::HashMap;



impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut entity_map = HashMap::new();
        entity_map.insert("&quot;", "\"");
        entity_map.insert("&apos;", "'");
        entity_map.insert("&amp;", "&");
        entity_map.insert("&gt;", ">");
        entity_map.insert("&lt;", "<");
        entity_map.insert("&frasl;", "/");

        let mut result = String::new();
        let mut i = 0;
        let n = text.len();

        while i < n {
            if &text[i..=i] == "&" {
                let mut found = false;
                for (entity, ch) in &entity_map {
                    if text[i..].starts_with(entity) {
                        result.push_str(ch);
                        i += entity.len();
                        found = true;
                        break;
                    }
                }
                if !found {
                    result.push('&');
                    i += 1;
                }
            } else {
                result.push_str(&text[i..=i]);
                i += 1;
            }
        }

        result

    }
}
