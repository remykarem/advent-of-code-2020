fn main() {
    let mut s = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .skip(198);
    dbg!(s.next().unwrap());
    dbg!(s.next().unwrap());
    dbg!(s.next().unwrap());
}
