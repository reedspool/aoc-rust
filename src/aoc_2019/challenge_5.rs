use crate::intcode_interpreter::Program;

fn part_1(input : &str) -> Vec<i32> {
    let mut program = Program::parse(input);
    program.set_input(1);
    program.run().output().clone()
}

#[cfg(test)]
mod tests {
    use crate::advent_utilities::read_input;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2019_5_1_input");
        let result = part_1(&input);
        assert_eq!(result[0], 0);
    }
}
