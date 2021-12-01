#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    0
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use crate::advent_utilities::read_input;
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_1_1_input");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_1_1_input");

        let result = part_2(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 0);
    }
}
