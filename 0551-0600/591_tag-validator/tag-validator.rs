
impl Solution {
    pub fn is_valid(code: String) -> bool {
        let mut stack: Vec<String> = Vec::new();
        let mut i = 0;
        while i < code.len() {
            if i > 0 && stack.is_empty() {
                return false;
            }
            if code[i..].starts_with("<![CDATA[") {
                let j = i + 9;
                i = code[j..].find("]]>").map(|k| k + j + 3).unwrap_or(code.len());
            } else if code[i..].starts_with("</") {
                let j = i + 2;
                let k = code[j..].find('>').map(|k| k + j).unwrap_or(code.len());
                let tag = &code[j..k];
                if stack.is_empty() || stack.pop().unwrap() != tag {
                    return false;
                }
                i = k + 1;
            } else if code[i..].starts_with('<') {
                let j = i + 1;
                let k = code[j..].find('>').map(|k| k + j).unwrap_or(code.len());
                let tag = &code[j..k];
                if tag.is_empty() || tag.len() > 9 || !tag.chars().all(|c| c.is_ascii_uppercase()) {
                    return false;
                }
                stack.push(tag.to_string());
                i = k + 1;
            } else {
                i += 1;
            }
        }
        stack.is_empty()
    }
}
