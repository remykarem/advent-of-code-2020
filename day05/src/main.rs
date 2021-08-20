use std::fs::File;
use std::io::{BufRead, BufReader};

type SeatID = i32;

pub fn main() {
    let path = "./src/input.txt";
    qn1(path);
    qn2(path);
}

fn qn1(path: &str) {
    let file = File::open(path).expect("cannot open file");
    let reader = BufReader::new(file);

    let mut max_seat_id = 0;

    reader
        .lines()
        .map(Result::unwrap)
        .map(get_seat_id)
        .for_each(|seat| {
            if seat > max_seat_id {
                max_seat_id = seat;
            };
        });
    println!("{}", max_seat_id);
}

fn qn2(path: &str) {
    let file = File::open(path).expect("cannot open file");
    let reader = BufReader::new(file);

    let mut seats: Vec<SeatID> = reader
        .lines()
        .map(Result::unwrap)
        .map(get_seat_id)
        .collect();

    let missing_seat = find_missing_seat(&mut seats);

    println!("Missing seat: {}", missing_seat);
}

fn get_seat_id(seat: String) -> SeatID {
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

// Brute force omg O(n)
fn find_missing_seat(seats: &mut [SeatID]) -> SeatID {
    seats.sort_unstable();
    let (first, last) = (seats[0], seats[seats.len() - 1]);

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
        assert_eq!(find_missing_seat(&mut [0, 1, 3]), 2);
    }

    #[test]
    fn check_seat_ids() {
        assert_eq!(get_seat_id("FBFBBFFRLR".into()), 357);
        assert_eq!(get_seat_id("BFFFBBFRRR".into()), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR".into()), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL".into()), 820);
    }
}
