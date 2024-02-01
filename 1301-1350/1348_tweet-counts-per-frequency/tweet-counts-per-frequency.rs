
use std::collections::HashMap;

struct TweetCounts {
    tweets: HashMap<String, Vec<i32>>,
}

impl TweetCounts {
    fn new() -> Self {
        TweetCounts {
            tweets: HashMap::new(),
        }
    }
    
    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        let tweet_list = self.tweets.entry(tweet_name).or_insert(Vec::new());
        tweet_list.push(time);
    }
    
    fn get_tweet_counts_per_frequency(&self, freq: String, tweet_name: String, start_time: i32, end_time: i32) -> Vec<i32> {
        let chunk_size = match freq.as_str() {
            "minute" => 60,
            "hour" => 3600,
            "day" => 86400,
            _ => panic!("Invalid frequency"),
        };
        
        let tweet_list = self.tweets.get(&tweet_name).unwrap_or(&Vec::new()).clone();
        let mut counts = vec![0; ((end_time - start_time) / chunk_size + 1) as usize];
        
        for &time in &tweet_list {
            if time >= start_time && time <= end_time {
                let index = ((time - start_time) / chunk_size) as usize;
                counts[index] += 1;
            }
        }
        
        counts

    }
}
