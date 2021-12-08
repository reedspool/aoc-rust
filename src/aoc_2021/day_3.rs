#[derive(Clone, Copy, Debug)]
pub struct Count {
    zeros: i32,
    ones: i32,
}

pub fn count_it(lines: Vec<&str>) -> Vec<Count> {
    let mut num_bits = lines[0].len();
    let mut counts = vec![Count { zeros: 0, ones: 0 }; num_bits];

    for line in lines.clone() {
        for (index, bit) in line.bytes().enumerate() {
            match bit {
                b'0' => counts[index].zeros += 1,
                b'1' => counts[index].ones += 1,
                _ => panic!("Encountered unknown bit {}", bit),
            }
        }
    }

    counts
}

#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let mut lines = input.trim().split("\n");

    let first_line = lines.next().unwrap();

    let num_bits = first_line.len();
    let mut counts = vec![Count { zeros: 0, ones: 0 }; num_bits];

    for line in lines {
        for (index, bit) in line.bytes().enumerate() {
            match bit {
                b'0' => counts[index].zeros += 1,
                b'1' => counts[index].ones += 1,
                _ => panic!("Encountered unknown bit {}", bit),
            }
        }
    }

    let mut gamma: i32 = 0;
    for count in counts {
        gamma = match count.zeros > count.ones {
            true => gamma << 1,
            false => (gamma << 1) + 1,
        }
    }

    let all_ones = 2_i32.pow(num_bits as u32) - 1;
    let epsilon = gamma ^ all_ones;

    gamma * epsilon
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let mut lines = input.trim().split("\n");

    let mut lines_vec = lines.clone().collect::<Vec<&str>>();

    let num_bits = lines_vec.clone()[0].len();
    let mut counts = count_it(lines_vec);

    let mut oxygen_rating_candidates = lines.collect::<Vec<&str>>();
    let mut co2_rating_candidates = oxygen_rating_candidates.clone();

    let mut ox_counts = counts.clone();

    let mut index = num_bits - 1;

    while index >= 0 {
        let up_index = (num_bits - index) - 1;
        let count = ox_counts[up_index];
        println!(
            "Before iter candidates: {:?}, Counts: {:?}",
            oxygen_rating_candidates, count
        );
        oxygen_rating_candidates = oxygen_rating_candidates
            .into_iter()
            .filter(|line| {
                let what = match count.zeros > count.ones {
                    true => b'0',
                    false => b'1',
                };

                line[up_index..up_index + 1].as_bytes()[0] == what
            })
            .collect();

        ox_counts = count_it(oxygen_rating_candidates.clone());

        if oxygen_rating_candidates.len() == 1 {
            break;
        }

        index -= 1;
    }

    index = num_bits - 1;

    ox_counts = counts.clone();

    while index >= 0 {
        let up_index = (num_bits - index) - 1;
        let count = ox_counts[up_index];
        println!(
            "Before iter co2 candidates: {:?}, Counts: {:?}",
            co2_rating_candidates, count
        );
        co2_rating_candidates = co2_rating_candidates
            .into_iter()
            .filter(|line| {
                let what = match count.zeros <= count.ones {
                    true => b'0',
                    false => b'1',
                };

                line[up_index..up_index + 1].as_bytes()[0] == what
            })
            .collect();

        ox_counts = count_it(co2_rating_candidates.clone());

        if co2_rating_candidates.len() == 1 {
            break;
        }

        index -= 1;
    }

    // for (index, count) in counts.iter().enumerate() {
    //     co2_rating_candidates = co2_rating_candidates
    //         .into_iter()
    //         .filter(|line| {
    //             let what = match count.zeros <= count.ones {
    //                 true => b'0',
    //                 false => b'1',
    //             };

    //             line[index..index + 1].as_bytes()[0] == what
    //         })
    //         .collect();

    //     if co2_rating_candidates.len() == 1 {
    //         break;
    //     }
    // }

    println!(
        "co2 binary {} ox bin {}",
        co2_rating_candidates[0], oxygen_rating_candidates[0]
    );

    let mut oxygen = 0;
    for (index, byte) in oxygen_rating_candidates[0].as_bytes().iter().enumerate() {
        oxygen = match byte {
            b'0' => oxygen << 1,
            b'1' => (oxygen << 1) + 1,
            _ => panic!(),
        }
    }

    let mut co2 = 0;
    for (index, byte) in co2_rating_candidates[0].as_bytes().iter().enumerate() {
        co2 = match byte {
            b'0' => co2 << 1,
            b'1' => (co2 << 1) + 1,
            _ => panic!(),
        }
    }

    co2 * oxygen
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_3_1");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 738234);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_3_1");

        let result = part_2(input.clone());

        assert!(result > 738423);
        assert!(result < 4184397);
        println!("My answer is {}", result);
        assert_eq!(result, 3969126);
    }

    #[test]
    fn test_part_2_with_test_data() {
        let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let result = part_2(input.to_string());

        assert_eq!(result, 230);
    }
}
