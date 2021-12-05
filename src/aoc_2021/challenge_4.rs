use std::io::BufRead;

use regex::Regex;

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<(i32, bool)>>,
    did_win: bool,
}

#[allow(dead_code)]
pub fn part_1(input: String) -> (i32, i32) {
    let mut unmarked_sum = 0;
    let mut final_number = 0;
    let mut winning: i32 = -1;

    let mut boards: Vec<Board> = Vec::new();
    let chunks: Vec<&str> = input.split("\n\n").collect();

    // Each index refers to a number in numbers. The value is tuple of indices:
    // (board, row, col)
    let mut lookups = vec![Vec::<(usize, usize, usize)>::new(); 100];

    let mut chunks_iter = chunks.iter();
    let numbers: Vec<i32> = chunks_iter
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();

    for (board_index, chunk) in chunks_iter.enumerate() {
        let mut lines_iter = chunk.split("\n").into_iter();

        let mut board = Board {
            rows: Vec::new(),
            did_win: false,
        };

        for row_index in 0..5 {
            let row = lines_iter.next().unwrap().trim();

            let stripped_row = row.replace("  ", " ");

            let row_nums: Vec<(i32, bool)> = stripped_row
                .split(" ")
                .map(|item| item.trim().parse().unwrap())
                .map(|item| (item, false))
                .collect();

            for (column_index, (num, _flipped)) in row_nums.clone().iter().cloned().enumerate() {
                lookups[num as usize].push((board_index, row_index as usize, column_index));
            }

            board.rows.push(row_nums);
        }

        boards.push(board);
    }

    for num in numbers {
        // for board in boards {
        //     for row in board.rows {
        //         for (value, _flipped) in row {
        //                 if num == value {
        //                     column[i].1 = true;
        //                 }
        //         }
        //     }
        // }
        let items = lookups[num as usize].clone();

        for (board, row, col) in items {
            boards[board].rows[row][col].1 = true;

            // Check columns
            for row in 0..5 {
                let mut did_win = true;
                for col in 0..5 {
                    if boards[board].rows[row][col].1 == false {
                        did_win = false;
                        break;
                    }
                }
                if did_win == true {
                    winning = board as i32;
                    break;
                }
            }

            if winning > -1 {
                break;
            }

            for col in 0..5 {
                let mut did_win = true;
                for row in 0..5 {
                    if boards[board].rows[row][col].1 == false {
                        did_win = false;
                        break;
                    }
                }
                if did_win == true {
                    winning = board as i32;
                    break;
                }
            }

            if winning > -1 {
                break;
            }
        }

        if winning > -1 {
            final_number = num;
            break;
        }
    }

    // Calculate umarked_sum
    for col in 0..5 {
        for row in 0..5 {
            let (num, flipped) = boards[winning as usize].rows[row][col];
            if flipped == false {
                unmarked_sum += num;
            }
        }
    }

    (unmarked_sum, final_number)
}

#[allow(dead_code)]
pub fn part_2(input: String) -> (i32, i32) {
    let mut unmarked_sum = 0;
    let mut final_number = 0;
    let mut winning: i32 = -1;

    let mut boards: Vec<Board> = Vec::new();
    let chunks: Vec<&str> = input.split("\n\n").collect();

    // Each index refers to a number in numbers. The value is tuple of indices:
    // (board, row, col)
    let mut lookups = vec![Vec::<(usize, usize, usize)>::new(); 100];

    let mut chunks_iter = chunks.iter();
    let numbers: Vec<i32> = chunks_iter
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();

    for (board_index, chunk) in chunks_iter.enumerate() {
        let mut lines_iter = chunk.split("\n").into_iter();

        let mut board = Board {
            rows: Vec::new(),
            did_win: false,
        };

        for row_index in 0..5 {
            let row = lines_iter.next().unwrap().trim();

            let stripped_row = row.replace("  ", " ");

            let row_nums: Vec<(i32, bool)> = stripped_row
                .split(" ")
                .map(|item| item.trim().parse().unwrap())
                .map(|item| (item, false))
                .collect();

            for (column_index, (num, _flipped)) in row_nums.clone().iter().cloned().enumerate() {
                lookups[num as usize].push((board_index, row_index as usize, column_index));
            }

            board.rows.push(row_nums);
        }

        boards.push(board);
    }

    let mut boards_left = boards.len();
    let mut last_win = -1;

    for num in numbers {
        final_number = num;
        // for board in boards {
        //     for row in board.rows {
        //         for (value, _flipped) in row {
        //                 if num == value {
        //                     column[i].1 = true;
        //                 }
        //         }
        //     }
        // }
        let items = lookups[num as usize].clone();

        for (board, row, col) in items {
            boards[board].rows[row][col].1 = true;

            // Check columns
            for row in 0..5 {
                let mut did_win = true;
                for col in 0..5 {
                    if boards[board].rows[row][col].1 == false {
                        did_win = false;
                        break;
                    }
                }
                if did_win == true {
                    winning = board as i32;
                    break;
                }
            }

            for col in 0..5 {
                let mut did_win = true;
                for row in 0..5 {
                    if boards[board].rows[row][col].1 == false {
                        did_win = false;
                        break;
                    }
                }
                if did_win == true {
                    winning = board as i32;
                    break;
                }
            }

            if winning > -1 {
                if !boards[winning as usize].did_win {
                    println!("Hey! Board {} won finally!", winning);
                    boards_left -= 1;
                last_win = winning;
                };

                boards[winning as usize].did_win = true;

                winning = -1;
            }
        }

        if boards_left == 0 {
            println!("Last won: {}", last_win);
            break;
        }

        winning = -1;
    }

    // Calculate umarked_sum
    for col in 0..5 {
        for row in 0..5 {
            let (num, flipped) = boards[last_win as usize].rows[row][col];
            if flipped == false {
                unmarked_sum += num;
            }
        }
    }

    (unmarked_sum, final_number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_4_1_input");

        let (unmarked_sum, final_number) = part_1(input.clone());
        let result = unmarked_sum * final_number;

        println!("{} * {} = {}", unmarked_sum, final_number, result);

        println!("My answer is {}", result);
        assert_eq!(result, 58838);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_4_1_input");

        let (unmarked_sum, final_number) = part_2(input.clone());
        let result = unmarked_sum * final_number;

        println!("{} * {} = {}", unmarked_sum, final_number, result);

        assert!(result < 58838);
        assert!(result > 2714);
        println!("My answer is {}", result);
        assert_eq!(result, 6256);
    }

    #[test]
    fn test_part_1_example() {
        let input = read_input("aoc_2021_4_1_example");

        let (unmarked_sum, final_number) = part_1(input.clone());
        let result = unmarked_sum * final_number;

        println!("{} * {} = {}", unmarked_sum, final_number, result);

        println!("My answer is {}", result);
        assert_eq!(final_number, 24);
        assert_eq!(unmarked_sum, 188);
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_input("aoc_2021_4_1_example");

        let (unmarked_sum, final_number) = part_2(input.clone());
        let result = unmarked_sum * final_number;

        println!("{} * {} = {}", unmarked_sum, final_number, result);

        println!("My answer is {}", result);
        assert_eq!(final_number, 13);
        assert_eq!(unmarked_sum, 148);
        assert_eq!(result, 1924);
    }
}
