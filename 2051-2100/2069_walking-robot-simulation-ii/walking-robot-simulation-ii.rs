
// Define the Robot struct

struct Robot {
    pos: i32,
    tot: i32,
    l1: i32,
    l2: i32,
    l3: i32,
    max_y: i32,
    max_x: i32,
    flag: bool,
}

// Implement methods for the Robot struct

impl Robot {
    // Constructor method to initialize the Robot

    fn new(width: i32, height: i32) -> Self {
        let pos = 0;
        let max_y = height - 1;
        let max_x = width - 1;
        let l1 = width;
        let l2 = l1 + height - 1;
        let l3 = l2 + width - 1;
        let tot = l3 + height - 2;
        let flag = true;

        Robot {
            pos,
            tot,
            l1,
            l2,
            l3,
            max_y,
            max_x,
            flag,
        }
    }

    // Method to move the robot

    fn step(&mut self, num: i32) {
        self.flag = false;
        self.pos = (self.pos + num) % self.tot;
    }

    // Method to get the current position of the robot

    fn get_pos(&self) -> Vec<i32> {
        if self.pos < self.l1 {
            return vec![self.pos, 0];
        } else if self.pos < self.l2 {
            return vec![self.max_x, self.pos - self.l1 + 1];
        } else if self.pos < self.l3 {
            return vec![self.l3 - self.pos - 1, self.max_y];
        } else {
            return vec![0, self.tot - self.pos];
        }
    }

    // Method to get the current direction of the robot

    fn get_dir(&self) -> String {
        if self.flag {
            return String::from("East");
        }

        if self.pos == 0 {
            return String::from("South");
        } else if self.pos < self.l1 {
            return String::from("East");
        } else if self.pos < self.l2 {
            return String::from("North");
        } else if self.pos < self.l3 {
            return String::from("West");
        } else {
            return String::from("South");
        }
    }
}
