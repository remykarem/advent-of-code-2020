fn main() {
    // let start = 1003240;
    // let (bus, end) = get_earliest_bus_timing(start, vec![19, 41, 37, 787, 13, 23, 29, 571, 17]);
    // println!("{}", bus * (end - start));
    let s = "19,x,x,x,x,x,x,x,x,41,x,x,x,37,x,x,x,x,x,787,x,x,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,29,x,571,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17";
    let (a, b) = parse(s);
    println!("{:?}", b);
    // println!("{}", go(0, a, b));

    let mut n: Vec<i64> = vec![19];
    
    let mut a: Vec<i64> = vec![0];

    b.iter().for_each(|(u,v)| {
        n.push(*v);
        a.push(-*u);
    });


    println!("{}", chinese_remainder(&n, &a));
}

fn get_earliest_bus_timing(mut time: u32, buses: Vec<u32>) -> (u32, u32) {
    loop {
        for &bus in &buses {
            if time % bus == 0 {
                return (bus, time);
            }
        }
        time += 1;
    }
}

fn go(start: u64, first: u64, delays: Vec<(u64, u64)>) -> u64 {
    let mut time: u64 = ((start as f64) / (first as f64)).ceil() as u64;

    let mut count: u64 = 1;
    loop {
        let mut start = true;
        for (delay, bus) in &delays {
            start &= (time + delay) % bus == 0;
            if !start {
                break;
            }
        }

        if start {
            return time;
        } else {
            time += first;
        }

        count += 1;
        if count % 1_000_000 == 0 {
            println!("{}", time);
        }
    }
}

fn parse(s: &str) -> (i64, Vec<(i64, i64)>) {
    let mut m = s.split(',');
    let g = m.next().unwrap().parse::<i64>().unwrap();
    let h: Vec<(i64, i64)> = m
        .enumerate()
        .filter(|(_, ch)| ch != &"x")
        .map(|(delay, ch)| (delay as i64 + 1, ch.parse::<i64>().unwrap()))
        .collect();

    (g, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_earliest_bus_timing1() {
        assert_eq!(get_earliest_bus_timing(7, vec![7]), (7, 7));
        assert_eq!(get_earliest_bus_timing(13, vec![7, 8]), (7, 14));
    }

    #[test]
    fn test_get_earliest_bus_timing2() {
        assert_eq!(
            get_earliest_bus_timing(939, vec![7, 13, 19, 31, 59]),
            (59, 944)
        );
    }

    #[test]
    fn test_go() {
        assert_eq!(go(0, 67, vec![(1, 7), (2, 59), (3, 61)]), 754018);
        assert_eq!(go(0, 1789, vec![(1, 37), (2, 47), (3, 1889)]), 1202161486);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("1,2,3"), (1, vec![(1, 2), (2, 3)]));
        assert_eq!(parse("1,x,2,3"), (1, vec![(2, 2), (3, 3)]));
        assert_eq!(parse("1,2,x,3"), (1, vec![(1, 2), (3, 3)]));
    }

    #[test]
    fn test_case_1() {
        let (a, b) = parse("17,x,13,19");
        assert_eq!(3417, go(0, a, b));
    }

    #[test]
    fn test_case_2() {
        let (a, b) = parse("67,7,59,61");
        assert_eq!(754018, go(0, a, b));
    }

    #[test]
    fn test_case_3() {
        let (a, b) = parse("67,x,7,59,61");
        assert_eq!(779210, go(0, a, b));
    }

    #[test]
    fn test_case_4() {
        let (a, b) = parse("67,7,x,59,61");
        assert_eq!(1261476, go(0, a, b));
    }

    #[test]
    fn test_case_5() {
        let (a, b) = parse("1789,37,47,1889");
        assert_eq!(1202161486, go(0, a, b));
    }

    #[test]
    fn test_chinese_remainder() {

    }
}

/// https://fangya.medium.com/chinese-remainder-theorem-with-python-a483de81fbb8
fn chinese_remainder(n: &[i64], a: &[i64]) -> i64 {
    let mut sum = 0;
    let prod = n.iter().product(); 
    for (n_i, a_i) in n.iter().zip(a.iter()) {
        let y_i = multiplicative_inverse(prod, *n_i);
        sum += a_i * y_i * prod;
    }
    sum % prod
}

/// https://fangya.medium.com/chinese-remainder-theorem-with-python-a483de81fbb8
/// ax = 1 (mod m); solve for x
/// Extended Euclidean algorithm
fn multiplicative_inverse(mut a: i64, mut m: i64) -> i64 {
    if m == 1 {
        return 1;
    }

    let m0 = m;
    let (mut x0, mut x1) = (0, 1);

    while a > 1 {
        // euclidean
        let q = a;
        a = m;
        m = a % m;

        // back substitution
        x0 = x1 - q * x0;
        x1 = x0;
    }

    // Correction to make it positive
    if x1 < 0 {
        x1 += m0;
    }
    x1
} 
