
struct Twitter {
    users: HashMap<i32, HashSet<i32>>,
    tweets: Vec<(i32, i32)>,
}

impl Twitter {
    fn new() -> Self {
        Twitter {
            users: HashMap::new(),
            tweets: Vec::new(),
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id, tweet_id));
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feed = Vec::new();
        if let Some(followees) = self.users.get(&user_id) {
            let mut tweets = self.tweets.iter().rev();
            for (uid, tid) in tweets {
                if *uid == user_id || followees.contains(uid) {
                    feed.push(*tid);
                }
                if feed.len() == 10 {
                    break;
                }
            }
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

fn main() {
    let mut obj = Twitter::new();
    obj.post_tweet(1, 5);
    let ret_1: Vec<i32> = obj.get_news_feed(1);
    obj.follow(1, 2);
    obj.post_tweet(2, 6);
    let ret_2: Vec<i32> = obj.get_news_feed(1);
    obj.unfollow(1, 2);
    let ret_3: Vec<i32> = obj.get_news_feed(1);
}
