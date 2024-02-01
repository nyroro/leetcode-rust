
impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        // 创建一个辅助函数来检查一个人的喜爱公司列表是否是另一个人的喜爱公司列表的子集

        fn is_subset(person1: &Vec<String>, person2: &Vec<String>) -> bool {
            person1.iter().all(|company| person2.contains(company))
        }

        let mut result = Vec::new(); // 创建一个空的结果数组来存储不是其他人喜爱公司列表的子集的人的索引


        for (i, person1) in favorite_companies.iter().enumerate() {
            let mut is_subset_of_others = false;
            for (j, person2) in favorite_companies.iter().enumerate() {
                if i != j && is_subset(person1, person2) {
                    is_subset_of_others = true;
                    break;
                }
            }
            if !is_subset_of_others {
                result.push(i as i32); // 将该人的索引加入结果数组中

            }
        }

        result // 返回结果数组

    }
}
