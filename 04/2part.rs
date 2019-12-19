fn meet2(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> bool {
    let mut flag = false;
    if a == b && b != c {
        flag |= true;
    }
    if a != b && b == c && c != d {
        flag |= true;
    }
    if b != c && c == d && d != e {
        flag |= true;
    }
    if c != d && d == e && e != f {
        flag |= true;
    }
    if d != e && e == f {
        flag |= true;
    }
    flag
}

fn meet(num: i32) -> bool {
    if num < 100_000 || num > 999_999 {
        return false;
    }
    let a = (num / 100_000) % 10;
    let b = (num / 10_000) % 10;
    let c = (num / 1_000) % 10;
    let d = (num / 100) % 10;
    let e = (num / 10) % 10;
    let f = num % 10;
    if !(a == b || b == c || c == d || d == e || e == f) {
        return false;
    }
    if !(a <= b && b <= c && c <= d && d <= e && e <= f) {
        return false;
    }
    meet2(a, b, c, d, e, f)
}

fn main() {
    let mut count = 0;
    for poss in 367470..893698 {
        if meet(poss) {
            count += 1;
        }
    }
    println!("{}", count);
}
