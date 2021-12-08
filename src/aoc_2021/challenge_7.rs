#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let mut xs: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    xs.sort();

    let length = xs.len();

    let mut median;
    if length % 2 == 0 {
        let middle_top = length / 2;
        let middle_bottom = middle_top - 1;

        median = (xs[middle_top] + xs[middle_bottom]) / 2;
    } else {
        median = xs[(length / 2) + 1];
    }

    let max = xs.iter().max().unwrap();

    let mut min_cost = i32::MAX;
    let mut tmp_sum = 0;
    for x in &xs {
        let difference: i32 = x - median;
        tmp_sum += difference.abs();
    }

    if tmp_sum < min_cost {
        min_cost = tmp_sum;
    }

    min_cost
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let xs: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let max = xs.iter().max().unwrap();

    let mut min_cost = i32::MAX;
    for guess in 0..=*max {
        let mut tmp_sum = 0;
        for x in &xs {
            let distance: i32 = (x - guess).abs();

            // This is the closed form for sum of 1..=N
            let difference = (distance * (distance + 1)) / 2;

            tmp_sum += difference.abs();
        }

        if tmp_sum < min_cost {
            min_cost = tmp_sum;
        }
    }

    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_7_1_input");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 359648);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_7_1_input");

        let result = part_2(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 100727924);
    }

    #[test]
    fn test_part_1_example() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");

        let result = part_1(input.clone());

        assert_eq!(result, 37);
    }

    #[test]
    fn test_part_2_example() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");

        let result = part_2(input.clone());

        assert_eq!(result, 168);
    }
}
