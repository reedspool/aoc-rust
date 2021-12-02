use crate::intcode_interpreter::Program;

#[allow(dead_code)]
fn part_1(input : &str) -> Vec<i32> {
    let mut program = Program::parse(input);
    program.set_input(1);
    program.run().output().clone()
}

#[allow(dead_code)]
fn part_2(input : &str) -> Vec<i32> {
    let mut program = Program::parse(input);
    program.set_input(5);
    program.run().output().clone()
}

#[cfg(test)]
mod tests {
    use crate::advent_utilities::{assert_array_eq, read_input};

    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2019_5_1_input");
        let result = part_1(&input);
        println!("{:?}", result);
        assert_array_eq(
            &result, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 16574641]);
    }

    // #[test]
    // fn test_part_2() {
    //     let input = read_input("aoc_2019_5_1_input");
    //     let result = part_2(&input);
    //     println!("{:?}", result);
    //     assert_array_eq(
    //         &result, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 16574641]);
    // }
}
