use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut stack: Vec<char> = Vec::new();

    let mut score = 0;
    for line in lines {
        for ch in line.chars() {
            match ch {
                '(' => stack.push(ch),
                '[' => stack.push(ch),
                '{' => stack.push(ch),
                '<' => stack.push(ch),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        score += 3;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        score += 57;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        score += 1197;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        score += 25137;
                        break;
                    }
                }
                unknown => panic!(),
            }
        }
    }

    score
}

#[allow(dead_code)]
pub fn part_2(input: String) -> usize {
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut scores: Vec<usize> = lines
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();

            // Try to build the stack
            for ch in line.chars() {
                match ch {
                    '(' | '[' | '{' | '<' => stack.push(ch),
                    // If we ever get an invalid closing val, return None
                    ')' => {
                        if stack.pop().unwrap() != '(' {
                            return None;
                        }
                    }
                    ']' => {
                        if stack.pop().unwrap() != '[' {
                            return None;
                        }
                    }
                    '}' => {
                        if stack.pop().unwrap() != '{' {
                            return None;
                        }
                    }
                    '>' => {
                        if stack.pop().unwrap() != '<' {
                            return None;
                        }
                    }
                    unknown => panic!(),
                }
            }

            // Otherwise return Some(stack);
            Some(stack)
        })
        // Filter out None
        .filter(|option| option.is_some())
        .map(|some_stack| {
            let mut stack = some_stack.unwrap();
            // The completing things are stack.reverse()
            stack.reverse();

            let mut score = 0;
            for ch in stack {
                score *= 5;
                score += match ch {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!(),
                }
            }

            score
        })
        .collect::<Vec<usize>>();

    scores.sort();

    // Take the middle
    println!(
        "Getting score #{} from len {}",
        scores.len() / 2,
        scores.len()
    );
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_10_1");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 392421);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_10_1");

        let result = part_2(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 2769449099);
    }

    #[test]
    fn test_part_1_example() {
        let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            .to_string();

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_part_2_example() {
        let input = "<{([{{}}[<[[[<>{}]]]>[]]".to_string();

        let result = part_2(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 294);
    }
}
