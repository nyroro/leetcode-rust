
struct TopVotedCandidate {
    persons: Vec<i32>,
    times: Vec<i32>,
    leaders: Vec<i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut count = vec![0; persons.len()];
        let mut max_votes = 0;
        let mut leader = 0;
        let mut leaders = Vec::new();

        for i in 0..persons.len() {
            count[persons[i] as usize] += 1;
            if count[persons[i] as usize] >= max_votes {
                max_votes = count[persons[i] as usize];
                leader = persons[i];
            }
            leaders.push(leader);
        }

        Self {
            persons,
            times,
            leaders,
        }
    }

    fn q(&self, t: i32) -> i32 {
        let mut left = 0;
        let mut right = self.times.len() - 1;

        while left < right {
            let mid = (left + right + 1) / 2;
            if self.times[mid] <= t {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        self.leaders[left]
    }
}

// let persons = vec![0, 1, 1, 0, 0, 1, 0];
// let times = vec![0, 5, 10, 15, 20, 25, 30];
// let obj = TopVotedCandidate::new(persons, times);
// let ret_1: i32 = obj.q(3);
// let ret_2: i32 = obj.q(12);
// let ret_3: i32 = obj.q(25);
// let ret_4: i32 = obj.q(15);
// let ret_5: i32 = obj.q(24);
// let ret_6: i32 = obj.q(8);

// println!("{}, {}, {}, {}, {}, {}", ret_1, ret_2, ret_3, ret_4, ret_5, ret_6);
