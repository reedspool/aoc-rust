#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let mut numbers = input
        .trim()
        .split("\n")
        .map(|s| -> i32 { s.trim().parse().unwrap() });

    // Peel off the first value first, can't compare the first to nothing
    let mut last = numbers.next().unwrap();
    let mut count = 0;
    for num in numbers {
        if num > last {
            count += 1
        }
        last = num
    }

    count
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let mut numbers = input
        .trim()
        .split("\n")
        .map(|s| s.trim().parse::<i32>().unwrap());

    let mut last2 = numbers.next().unwrap();
    let mut last1 = numbers.next().unwrap();
    let mut last = numbers.next().unwrap();
    let mut last_total = last + last1 + last2;
    let mut current_total;
    let mut count = 0;
    for num in numbers {
        current_total = num + last + last1;
        if current_total > last_total {
            count += 1
        }
        last2 = last1;
        last1 = last;
        last = num;
        last_total = last + last1 + last2;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_1_1");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 1162);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_1_1");

        let result = part_2(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 1190);
    }
}
