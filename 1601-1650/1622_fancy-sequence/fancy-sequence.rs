
// Define the Fancy struct to hold the state of the sequence

struct Fancy {
    arr: Vec<i64>,  // Vector to store the sequence

    sum: i64,       // Variable to store the sum for addAll operation

    mul: i64,       // Variable to store the multiplication factor for multAll operation

    mod_val: i64,   // Constant to store the modulo value

}

impl Fancy {
    // Implement the new method to initialize the Fancy object

    fn new() -> Self {
        Fancy {
            arr: Vec::new(),
            sum: 0,
            mul: 1,
            mod_val: 1_000_000_007,  // Set the modulo value to 10^9 + 7

        }
    }

    // Implement the append method to append an integer to the sequence

    fn append(&mut self, val: i32) {
        let val = (val as i64 * self.binpow(self.mul, self.mod_val - 2)) % self.mod_val;
        self.arr.push((val - self.sum + self.mod_val) % self.mod_val);
    }

    // Implement the add_all method to increment all existing values in the sequence by an integer

    fn add_all(&mut self, inc: i32) {
        self.sum = (self.sum + (inc as i64 * self.binpow(self.mul, self.mod_val - 2)) % self.mod_val) % self.mod_val;
    }

    // Implement the mult_all method to multiply all existing values in the sequence by an integer

    fn mult_all(&mut self, m: i32) {
        self.mul = (self.mul * (m as i64)) % self.mod_val;
    }

    // Implement the get_index method to get the current value at a given index of the sequence

    fn get_index(&self, idx: i32) -> i32 {
        if (idx as usize) >= self.arr.len() {
            -1

        } else {
            ((self.arr[idx as usize] + self.sum) * self.mul % self.mod_val) as i32

        }
    }

    // Implement the binpow method to calculate the power of a number with modulo

    fn binpow(&self, mut a: i64, mut b: i64) -> i64 {
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = (res * a) % self.mod_val;
            }
            b >>= 1;
            a = (a * a) % self.mod_val;
        }
        res

    }
}
