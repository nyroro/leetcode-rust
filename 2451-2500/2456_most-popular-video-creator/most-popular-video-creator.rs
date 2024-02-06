
use std::collections::HashMap;



impl Solution {
    pub fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>> {
        let mut memo: HashMap<String, (i64, String, i32)> = HashMap::new();
        let mut overall_max_popular_video_count = -1;

        for i in 0..creators.len() {
            if let Some(creator) = memo.get_mut(&creators[i]) {
                creator.0 += views[i] as i64;
                if creator.2 < views[i] {
                    creator.1 = ids[i].clone();
                    creator.2 = views[i];
                } else if creator.2 == views[i] {
                    creator.1 = std::cmp::min(&creator.1, &ids[i]).to_string();
                }
            } else {
                memo.insert(creators[i].clone(), (views[i] as i64, ids[i].clone(), views[i]));
            }
            overall_max_popular_video_count = overall_max_popular_video_count.max(memo[&creators[i]].0);
        }

        let mut result: Vec<Vec<String>> = Vec::new();
        for (creator, (total_views, most_viewed_id, _)) in memo.iter() {
            if *total_views == overall_max_popular_video_count {
                result.push(vec![creator.clone(), most_viewed_id.clone()]);
            }
        }

        result

    }
}
