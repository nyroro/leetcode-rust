
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        // 定义一个结构体来存储名字和身高

        #[derive(PartialEq, Eq)]
        struct Person {
            name: String,
            height: i32,
        }

        // 实现 Ord 和 PartialOrd trait 来对身高进行排序

        impl Ord for Person {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.height.cmp(&other.height).reverse()
            }
        }

        impl PartialOrd for Person {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        // 创建 Person 结构体的实例并存储到数组中

        let mut people: Vec<Person> = names

            .into_iter()
            .zip(heights)
            .map(|(name, height)| Person { name, height })
            .collect();

        // 使用 sort 方法对结构体进行排序

        people.sort();

        // 提取排序后的名字

        people.into_iter().map(|person| person.name).collect()
    }
}
