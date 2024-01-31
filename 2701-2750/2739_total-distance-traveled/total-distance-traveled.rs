
impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        let mut d = 0;
        let mut main_tank = main_tank;
        let mut additional_tank = additional_tank;

        while main_tank > 0 {
            if main_tank >= 5 {
                main_tank -= 5;
                d += 5;
                if additional_tank >= 1 {
                    additional_tank -= 1;
                    main_tank += 1;
                }
            }
            if main_tank < 5 {
                d += main_tank;
                main_tank = 0;
            }
        }
        d * 10

    }
}
