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
        println!("{:?}", result);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], 0);
        assert_eq!(result[3], 0);
        assert_eq!(result[4], 0);
        assert_eq!(result[5], 0);
        assert_eq!(result[6], 0);
        assert_eq!(result[7], 0);
        assert_eq!(result[8], 0);
        assert_eq!(result[9], 16574641);
    }
}
