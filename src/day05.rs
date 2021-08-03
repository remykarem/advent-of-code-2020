use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let file = File::open("./data/day05.txt").expect("cannot open file");
    let reader = BufReader::new(file);
    let seats: Vec<&str> = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|seat| !seat.starts_with("FFFFFFF"));

    // println!("{}", get_seat_id("FFFFFBFRLR"));
}
// pub fn main() {
//     let mut max_seat_id = 0;

//     let file = File::open("./data/day05.txt").expect("cannot open file");
//     let reader = BufReader::new(file);
//     reader.lines().for_each(|line| {
//         let seat = line.unwrap();
//         let seat_id = get_seat_id(&seat);
//         if seat_id > max_seat_id {
//             max_seat_id = seat_id;
//         };
//     });

//     println!("{}", max_seat_id);

//     assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
//     assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
//     assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
//     assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
// }

fn get_seat_id(seat: &str) -> usize {
    let (mut min, mut max) = (0, 127);

    let (columns, rows) = seat.split_at(7);

    columns.chars().for_each(|c| {
        match c {
            'F' => max -= (max - min + 1) / 2,
            'B' => min += (max - min + 1) / 2,
            _ => panic!("abort"),
        };
    });

    let seat_row = min;

    let (mut min, mut max) = (0, 7);

    rows.chars().for_each(|c| {
        match c {
            'L' => max -= (max - min + 1) / 2,
            'R' => min += (max - min + 1) / 2,
            _ => panic!("abort"),
        };
    });

    let seat_column = min;

    let seat_id = seat_row * 8 + seat_column;

    println!("Row {} Col {}", seat_row, seat_column);

    seat_id
}
