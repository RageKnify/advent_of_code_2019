use std::io;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan(self) -> u32 {
        return self.x.abs() as u32 + self.y.abs() as u32;
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

fn get_lines() -> (Vec<Segment>, Vec<Segment>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut list = [&mut vec1, &mut vec2];
    for vec in &mut list {
        let mut line = String::new();
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

            let direction = &mov[0..1];
            let segment: Segment;
            match &direction as &str {
                "U" => {
                    end = Point {
                        x: start.x,
                        y: start.y + amount,
                    };
                    segment = Segment { start, end };
                }
                "D" => {
                    end = Point {
                        x: start.x,
                        y: start.y - amount,
                    };
                    segment = Segment {
                        start: end,
                        end: start,
                    };
                }
                "L" => {
                    end = Point {
                        x: start.x - amount,
                        y: start.y,
                    };
                    segment = Segment {
                        start: end,
                        end: start,
                    };
                }
                "R" => {
                    end = Point {
                        x: start.x + amount,
                        y: start.y,
                    };
                    segment = Segment { start, end };
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

fn intersect(seg1: &Segment, seg2: &Segment) -> Option<Point> {
    let start1 = seg1.start;
    let end1 = seg1.end;
    let start2 = seg2.start;
    let end2 = seg2.end;
    if start1.x == end1.x {
        // seg1 is vertical
        if start2.y == end2.y {
            // seg2 is horizontal
            if start1.x > start2.x && start1.x < end2.x && start1.y < start2.y && end1.y > start2.y
            {
                let int = Point {
                    x: start1.x,
                    y: start2.y,
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
                let int = Point {
                    x: start2.x,
                    y: start1.y,
                };
                return Some(int);
            }
        }
    }
    return None;
}

fn main() {
    let (v1, v2) = get_lines();
    let mut vec: Vec<Point> = Vec::new();
    for seg1 in &v1 {
        for seg2 in &v2 {
            match intersect(seg1, seg2) {
                Some(int) => {
                    vec.push(int);
                }
                None => (),
            }
        }
    }
    let mut min_p = &vec[0];
    let mut min = min_p.manhattan();
    for curr_p in &vec {
        let curr = curr_p.manhattan();
        if curr < min {
            min = curr;
            min_p = curr_p;
        }
    }
    println!("{}, {:?}", min, min_p);
}
