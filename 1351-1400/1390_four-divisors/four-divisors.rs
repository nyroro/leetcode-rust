
fn count_divisors(num: i32) -> i32 {
    let mut count = 0;
    let mut i = 1;
    while i * i <= num {
        if num % i == 0 {
            count += 2;
            if i * i == num {
                count -= 1;
            }
        }
        i += 1;
    }
    count

}
