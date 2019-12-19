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
    true
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
