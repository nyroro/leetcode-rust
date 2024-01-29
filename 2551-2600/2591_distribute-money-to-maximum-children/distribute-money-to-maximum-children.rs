


impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        }

        let mut ans = 0;
        let mut remaining_money = money;

        for j in 1..=children {
            let leftmoney = remaining_money - 8;
            let leftchildren = children - j;

            if leftmoney >= leftchildren {
                remaining_money = leftmoney;
                ans += 1;
            } else {
                let mut leftchildren = children - j + 1;
                if leftchildren == 1 && remaining_money == 4 {
                    ans -= 1;
                }
                remaining_money = 0;
                break;
            }
        }

        if remaining_money > 0 {
            ans -= 1;
        }

        ans

    }
}
