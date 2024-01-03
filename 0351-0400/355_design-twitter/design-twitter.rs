
use std::collections::{HashMap, HashSet};

struct Twitter {
    users: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<(i32, i32)>>,
    time: i32,
}

impl Twitter {
    fn new() -> Self {
        Twitter {
            users: HashMap::new(),
            tweets: HashMap::new(),
            time: 0,
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        self.tweets.entry(user_id).or_insert(Vec::new()).push((self.time, tweet_id));
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feed = Vec::new();
        let mut tweets = self.tweets.get(&user_id).unwrap_or(&Vec::new()).clone();
        for &followee_id in self.users.get(&user_id).unwrap_or(&HashSet::new()) {
            if let Some(followee_tweets) = self.tweets.get(&followee_id) {
                tweets.extend_from_slice(followee_tweets);
            }
        }
        tweets.sort_by(|a, b| b.0.cmp(&a.0));
        for tweet in tweets.iter().take(10) {
            feed.push(tweet.1);
        }
        feed

    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users.entry(follower_id).or_insert(HashSet::new()).insert(followee_id);
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.users.get_mut(&follower_id) {
            followees.remove(&followee_id);
        }
    }
}
