use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    qn1()
}

pub fn qn2() {
    let file = File::open("./data/day05.txt").expect("cannot open file");
    let reader = BufReader::new(file);
    let mut seats: Vec<i32> = reader
        .lines()
        .map(Result::unwrap)
        .map(|seat| get_seat_id(&seat))
        .collect();

    let missing_seat = check_missing_seat(&mut seats);

    println!("Missing seat: {}", missing_seat);
}

pub fn qn1() {
    let mut max_seat_id = 0;

    let file = File::open("./data/day05.txt").expect("cannot open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(Result::unwrap)
        .map(|seat| get_seat_id(&seat))
        .for_each(|seat| {
            if seat > max_seat_id {
                max_seat_id = seat;
            };
        });
    println!("{}", max_seat_id);
}

fn get_seat_id(seat: &str) -> i32 {
    let (mut min, mut max) = (0, 127);
    let (columns, rows) = seat.split_at(7);

    columns.chars().for_each(|col| {
        match col {
            'F' => max -= (max - min + 1) / 2,
            'B' => min += (max - min + 1) / 2,
            _ => panic!("abort"),
        };
    });

    let seat_row = min;

    let (mut min, mut max) = (0, 7);

    rows.chars().for_each(|row| {
        match row {
            'L' => max -= (max - min + 1) / 2,
            'R' => min += (max - min + 1) / 2,
            _ => panic!("abort"),
        };
    });

    let seat_column = min;

    seat_row * 8 + seat_column
}

fn check_missing_seat(seats: &mut [i32]) -> i32 {
    seats.sort_unstable();
    let (first, last) = (seats[0], seats[seats.len() - 1]);
    // Brute force omg O(n)
    // We can do better
    // Let's do binary search

    for (seat_id, expected_seat_id) in seats.iter().zip(first..last) {
        if *seat_id != expected_seat_id {
            return expected_seat_id;
        }
    }
    panic!("Illegal state");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_missing_seat() {
        assert_eq!(check_missing_seat(&mut [0, 1, 3]), 2);
    }

    #[test]
    fn check_seat_ids() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }
}
