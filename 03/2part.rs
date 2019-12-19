use std::cmp;
use std::io;

#[derive(Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.x < other.x {
            return cmp::Ordering::Less;
        } else if self.x > other.x {
            return cmp::Ordering::Greater;
        } else if self.y < other.y {
            return cmp::Ordering::Less;
        } else if self.y > other.y {
            return cmp::Ordering::Greater;
        } else {
            return cmp::Ordering::Equal;
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Intersection {
    steps: i32,
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
    amount: i32,
}

fn get_lines() -> (Vec<Segment>, Vec<Segment>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut list = [&mut vec1, &mut vec2];
    for vec in &mut list {
        let mut line = String::new();
        let mut t_amount = 0;
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let moves = line.trim().split(',');

        let mut start = Point { x: 0, y: 0 };
        let mut end: Point;
        for mov in moves {
            let amount: i32 = match mov[1..].parse() {
                Ok(amount) => amount,
                Err(_) => break,
            };
            t_amount += amount;

            let direction = &mov[0..1];
            let segment: Segment;
            match &direction as &str {
                "U" => {
                    end = Point {
                        x: start.x,
                        y: start.y + amount,
                    };
                    segment = Segment {
                        start,
                        end,
                        amount: t_amount,
                    };
                }
                "D" => {
                    end = Point {
                        x: start.x,
                        y: start.y - amount,
                    };
                    segment = Segment {
                        start,
                        end,
                        amount: t_amount,
                    };
                }
                "L" => {
                    end = Point {
                        x: start.x - amount,
                        y: start.y,
                    };
                    segment = Segment {
                        start,
                        end,
                        amount: t_amount,
                    };
                }
                "R" => {
                    end = Point {
                        x: start.x + amount,
                        y: start.y,
                    };
                    segment = Segment {
                        start,
                        end,
                        amount: t_amount,
                    };
                }
                _ => {
                    panic!("Not valid direction");
                }
            }
            vec.push(segment);
            start = end;
        }
    }
    (vec1, vec2)
}

fn intersects(seg1: &Segment, seg2: &Segment) -> Option<Intersection> {
    let start1 = cmp::min(seg1.start, seg1.end);
    let end1 = cmp::max(seg1.start, seg1.end);
    let start2 = cmp::min(seg2.start, seg2.end);
    let end2 = cmp::max(seg2.start, seg2.end);
    if start1.x == end1.x {
        // seg1 is vertical
        if start2.y == end2.y {
            // seg2 is horizontal
            if start1.x > start2.x && start1.x < end2.x && start1.y < start2.y && end1.y > start2.y
            {
                let x = start1.x;
                let y = start2.y;
                let dist1 = (seg1.end.y - y).abs();
                let dist2 = (seg2.end.x - x).abs();
                let int = Intersection {
                    steps: (seg1.amount + seg2.amount - dist1 - dist2),
                };
                return Some(int);
            }
        }
    } else {
        // seg1 is horizontal
        if start2.x == end2.x {
            // seg2 is vertical
            if start2.x > start1.x && start2.x < end1.x && start2.y < start1.y && end2.y > start1.y
            {
                let x = start2.x;
                let y = start1.y;
                let dist2 = (y - seg2.end.y).abs();
                let dist1 = (x - seg1.end.x).abs();
                let int = Intersection {
                    steps: (seg1.amount + seg2.amount - dist1 - dist2),
                };
                return Some(int);
            }
        }
    }
    return None;
}

fn main() {
    let (v1, v2) = get_lines();
    let mut vec: Vec<Intersection> = Vec::new();
    for seg1 in &v1 {
        for seg2 in &v2 {
            match intersects(seg1, seg2) {
                Some(int) => {
                    vec.push(int);
                }
                None => (),
            }
        }
    }
    let mut min = vec[0].steps;
    for curr_i in &vec {
        let curr = curr_i.steps;
        if curr < min {
            min = curr;
        }
    }
    println!("{}", min);
}
