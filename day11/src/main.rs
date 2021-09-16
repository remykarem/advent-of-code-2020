use State::*;

fn main() {}

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Empty,
    Floor,
    Occupied,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![vec![Floor, Empty, Occupied], vec![Floor, Floor, Occupied]];

        assert_eq!(count_adjacent_seats(&arr, 0, 0), 0);
        assert_eq!(count_adjacent_seats(&arr, 0, 2), 1);
        assert_eq!(count_adjacent_seats(&arr, 1, 2), 1);
        assert_eq!(count_adjacent_seats(&arr, 1, 1), 2);
    }

    #[test]
    fn it_works2() {
        let arr = vec![vec![Floor, Empty, Occupied], vec![Floor, Floor, Occupied]];
        let exp = [[Floor, Empty, Occupied], [Floor, Floor, Occupied]];

        let (obs, changes) = get_next_state(&arr);

        obs[0].iter().zip(exp[0].iter()).for_each(|(a, b)| {
            assert_eq!(a, b);
        });
        obs[1].iter().zip(exp[1].iter()).for_each(|(a, b)| {
            assert_eq!(a, b);
        });
        assert_eq!(changes, 0);
    }
}

fn count_adjacent_seats(arr: &[Vec<State>], x: i32, y: i32) -> u32 {
    let mut sum = 0;

    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            if (i == x && j == y) || i < 0 || j < 0 || i >= 2 || j >= 3 {
                continue;
            } else {
                match arr[i as usize][j as usize] {
                    Occupied => sum += 1,
                    _ => continue,
                }
            }
        }
    }
    sum
}

fn get_next_state(arr: &[Vec<State>]) -> (Vec<Vec<State>>, u32) {
    let mut new_arr = arr.clone();
    let mut changes: u32 = 0;

    for i in 0..2 {
        for j in 0..3 {
            let sum = count_adjacent_seats(&arr, i as i32, j as i32);
            match arr[i][j] {
                Occupied => {
                    if sum >= 4 {
                        new_arr[i][j] = Empty;
                        changes += 1;
                    }
                }
                Empty => {
                    if sum == 0 {
                        new_arr[i][j] = Occupied;
                        changes += 1;
                    }
                }
                _ => continue,
            }
        }
    }

    (new_arr, changes)
}
