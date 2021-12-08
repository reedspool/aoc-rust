fn count_shared_pieces(output: &str, guess: Option<&str>) -> usize {
    let mut count = 0;
    match guess {
        Some(known) => {
            for ch in output.chars() {
                if known.contains(ch) {
                    count += 1
                }
            }
        }
        None => panic!("output {}", output),
    }

    count
}

#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let parts: Vec<&str> = input
        .trim()
        .split("\n")
        .map(|line| {
            let split = line.trim().split("|").collect::<Vec<&str>>();

            split[1]
        })
        .collect();

    let mut count = 0;
    for part in parts {
        let things: Vec<&str> = part.split(" ").collect();

        for thing in things {
            let len = thing.len();
            if len == 2 || len == 4 || len == 7 || len == 3 {
                count += 1;
            }
        }
    }

    count
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let parts: Vec<(&str, Vec<&str>, Vec<&str>)> = input
        .trim()
        .split("\n")
        .map(|line| {
            let split = line.trim().split(" | ").collect::<Vec<&str>>();
            let all_digits = split[0]
                .split(" ")
                .chain(split[1].split(" "))
                .collect::<Vec<&str>>();

            (split[0], split[1].split(" ").collect::<Vec<&str>>(), all_digits)
        })
        .collect();

        let mut total = 0;
    // First pass to identify known numbers
    for (left_side, right_side, all_digits) in &parts {
        let mut known = vec![None::<&str>; 10];
        let mut sum = 0;

        // println!("part {:?}", number);
        for output in all_digits {
            let len = output.len();
            match len {
                2 => known[1] = Some(output),
                4 => known[4] = Some(output),
                7 => known[8] = Some(output),
                3 => known[7] = Some(output),
                5 | 6 => { /* We don't know much about this */ }
                unknown => panic!("Unexpected length {}", unknown),
            }
        }
        // println!("part {:?}", number);
        for output in right_side {

            sum = sum * 10;
            let len = output.len();
            match len {
                6 => {
                    /* It must be a 6 or a 9 or a 0 */
                    /* And we can tell that it must be a nine only iff it shares both things that a 1 has */
                    match count_shared_pieces(output, known[1]) {
                        1 => sum += 6,
                        2 => match count_shared_pieces(output, known[4]) {
                            4 => sum += 9,
                            3 => sum += 0,
                            unknown => panic!("Unknown of length 6 {} not 0 or 9", unknown),
                        },
                        unknown => panic!("Unknown of length 6 {} a", unknown),
                    }
                }
                5 => {
                    /* It must be a 2, a 3, or a 5  */
                    /* And we can tell that it must be a nine only iff it shares both things that a 1 has */
                    match count_shared_pieces(output, known[1]) {
                        1 => match count_shared_pieces(output, known[4]) {
                            3 => sum += 5,
                            2 => sum += 2,

                            unknown => panic!("Unknown of length 6 {} b", unknown),
                        },
                        2 => sum += 3,
                        unknown => panic!(
                            "Unknown of length 6 {} c {} {:?}",
                            unknown, output, known[1]
                        ),
                    }
                }
                2 => sum += 1,
                4 => sum += 4,
                7 => sum += 8,
                3 => sum += 7,
                unknown => panic!("Unexpected length {}", unknown),
            }
        }
        println!("What do I know: {:?}", known);
        println!("What was the output? {:?}", right_side);

            total += sum;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_8_1");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_8_1");

        let result = part_2(input.clone());

        assert!(result < 9407240);
        println!("My answer is {}", result);
        assert_eq!(result, 940724);
    }

    #[test]
    fn test_count_shared() {
        let result = count_shared_pieces("dafec", Some("gf"));

        assert_eq!(result, 1);
    }
}
