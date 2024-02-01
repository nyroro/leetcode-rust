
use std::collections::{HashMap, HashSet, VecDeque};



impl Solution {
    pub fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String> {
        let n = watched_videos.len();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut level_friends = HashSet::new();
        let mut result = HashMap::new();

        queue.push_back((id, 0));
        visited.insert(id);

        while let Some((person, cur_level)) = queue.pop_front() {
            if cur_level == level {
                level_friends.insert(person);
            } else {
                for &friend in &friends[person as usize] {
                    if !visited.contains(&friend) {
                        queue.push_back((friend, cur_level + 1));
                        visited.insert(friend);
                    }
                }
            }
        }

        for &friend in &level_friends {
            for video in &watched_videos[friend as usize] {
                *result.entry(video.clone()).or_insert(0) += 1;
            }
        }

        let mut result_vec: Vec<(String, i32)> = result.into_iter().collect();
        result_vec.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        result_vec.iter().map(|(video, _)| video.clone()).collect()
    }
}
