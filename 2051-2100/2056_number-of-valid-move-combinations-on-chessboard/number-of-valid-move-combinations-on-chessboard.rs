
#[derive(Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_towards(&mut self, other: &Point) {
        if self.x != other.x {
            self.x += if other.x > self.x { 1 } else { -1 };
        }
        if self.y != other.y {
            self.y += if other.y > self.y { 1 } else { -1 };
        }
    }
}

const HORIZONTAL_STEPS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];

const DIAGONAL_STEPS: [Point; 4] = [
    Point { x: 1, y: 1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: 1 },
    Point { x: -1, y: -1 },
];

fn get_moves(piece: &str, p: &Point) -> Vec<Point> {
    let mut res = vec![Point { x: p.x, y: p.y }];

    if piece == "rook" || piece == "queen" {
        for step in HORIZONTAL_STEPS.iter() {
            let mut i = p.x + step.x;
            let mut j = p.y + step.y;
            while i >= 0 && i < 8 && j >= 0 && j < 8 {
                res.push(Point { x: i, y: j });
                i += step.x;
                j += step.y;
            }
        }
    }

    if piece == "bishop" || piece == "queen" {
        for step in DIAGONAL_STEPS.iter() {
            let mut i = p.x + step.x;
            let mut j = p.y + step.y;
            while i >= 0 && i < 8 && j >= 0 && j < 8 {
                res.push(Point { x: i, y: j });
                i += step.x;
                j += step.y;
            }
        }
    }

    res

}

fn will_intersect(a: &Point, a2: &Point, b: &Point, b2: &Point) -> bool {
    let mut a = Point { x: a.x, y: a.y };
    let mut b = Point { x: b.x, y: b.y };

    while a != *a2 || b != *b2 {
        a.move_towards(a2);
        b.move_towards(b2);
        if a == b {
            return true;
        }
    }

    false

}

fn is_valid_destination(dest: &Vec<Point>, positions: &Vec<Point>) -> bool {
    for i in 0..dest.len() {
        for j in 0..i {
            if will_intersect(&positions[i], &dest[i], &positions[j], &dest[j]) {
                return false;
            }
        }
    }
    true

}

fn dfs(
    i: usize,
    pieces: &Vec<String>,
    positions: &Vec<Point>,
    destination: &mut Vec<Point>,
    count: &mut i32,
) {
    if i == pieces.len() {
        if is_valid_destination(destination, positions) {
            *count += 1;
        }
        return;
    }

    for move_point in get_moves(&pieces[i], &positions[i]) {
        destination[i] = move_point;
        dfs(i + 1, pieces, positions, destination, count);
    }
}



impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        let mut positions_vec: Vec<Point> = positions

            .iter()
            .map(|pos| Point {
                x: pos[0] - 1,
                y: pos[1] - 1,
            })
            .collect();
        let mut destination: Vec<Point> = vec![Point { x: 0, y: 0 }; pieces.len()];
        let mut count = 0;
        dfs(0, &pieces, &positions_vec, &mut destination, &mut count);
        count

    }
}
