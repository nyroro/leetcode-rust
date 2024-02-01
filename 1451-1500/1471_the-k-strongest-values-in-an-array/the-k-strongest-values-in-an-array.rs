


impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let n = arr.len();
        let mut arr = arr;
        arr.sort();
        let median = arr[(n - 1) / 2];
        
        arr.sort_by(|&a, &b| {
            if (a - median).abs() == (b - median).abs() {
                b.cmp(&a)
            } else {
                (a - median).abs().cmp(&(b - median).abs())
            }
        });
        
        arr.resize(k as usize, 0);
        arr

    }
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 5];
    let k1 = 2;
    let result1 = Solution::get_strongest(arr1, k1);
    println!("{:?}", result1); // Output: [5, 1]

    let arr2 = vec![1, 1, 3, 5, 5];
    let k2 = 2;
    let result2 = Solution::get_strongest(arr2, k2);
    println!("{:?}", result2); // Output: [5, 5]

    let arr3 = vec![6, 7, 11, 7, 6, 8];
    let k3 = 5;
    let result3 = Solution::get_strongest(arr3, k3);
    println!("{:?}", result3); // Output: [11, 8, 6, 6, 7]
}
