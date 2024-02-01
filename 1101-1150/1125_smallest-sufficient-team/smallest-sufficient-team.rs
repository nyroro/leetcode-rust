
use std::collections::HashMap;



impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        // Create a map to store the index of each skill

        let mut skill_map: HashMap<String, usize> = HashMap::new();
        for (i, skill) in req_skills.iter().enumerate() {
            skill_map.insert(skill.clone(), i);
        }

        // Convert people's skills to binary representation

        let people_skills: Vec<usize> = people

            .iter()
            .map(|person| {
                let mut skills = 0;
                for skill in person {
                    if let Some(&index) = skill_map.get(skill) {
                        skills |= 1 << index;
                    }
                }
                skills

            })
            .collect();

        // Create a map to store the minimum team size for each skill combination

        let mut dp: HashMap<usize, Vec<usize>> = HashMap::new();
        dp.insert(0, vec![]);

        for (i, &skills) in people_skills.iter().enumerate() {
            for (&prev_skills, prev_team) in dp.clone() {
                let new_skills = prev_skills | skills;
                if !dp.contains_key(&new_skills) || prev_team.len() + 1 < dp[&new_skills].len() {
                    let mut new_team = prev_team.clone();
                    new_team.push(i);
                    dp.insert(new_skills, new_team);
                }
            }
        }

        // Return the result

        dp[&(2usize.pow(req_skills.len() as u32) - 1)]
            .iter()
            .map(|&index| index as i32)
            .collect()
    }
}
