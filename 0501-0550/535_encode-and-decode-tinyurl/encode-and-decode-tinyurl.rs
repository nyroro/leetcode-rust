
struct Codec {
    urls: Vec<String>,
}

impl Codec {
    fn new() -> Self {
        // 初始化Codec对象

        Codec {
            urls: Vec::new(),
        }
    }
    
    fn encode(&mut self, longURL: String) -> String {
        // 将长URL编码为短URL

        self.urls.push(longURL.clone());
        (self.urls.len() - 1).to_string()
    }
    
    fn decode(&self, shortURL: String) -> String {
        // 将短URL解码为原始的长URL

        let index = shortURL.parse::<usize>().unwrap();
        self.urls[index].clone()
    }
}
