use crate::intcode_interpreter::Program;

#[allow(dead_code)]
pub fn part_1(input: &str) -> i32 {
    let mut program = Program::parse(input);

    program.set_noun(12);
    program.set_verb(2);

    program.run().first_position()
}

#[allow(dead_code)]
pub fn part_2(input: &str, output: i32) -> i32 {
    let program = Program::parse(input);
    let mut noun = 0;
    let mut verb = 0;

    'outer: for i in 0..100 {
        for j in 0..100 {
            noun = i;
            verb = j;
            let mut program = program.clone();
            program.set_noun(noun);
            program.set_verb(verb);
            let result = program.run().first_position();

            if result == output {
                break 'outer;
            }
        }
    }

    return 100 * noun + verb;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2019_2_1");

        let result = part_1(&input);
        assert_eq!(result, 6327510);
        println!("The value left at position 0 is {}", result);
    }

    fn test_part_2() {
        let input = read_input("aoc_2019_2_1");

        assert_eq!(part_2(&input, 6327510), 1202);
        let result = part_2(&input, 19690720);
        assert_eq!(result, 4112);
        println!("The value of 100 * noun + verb is {}", result);
    }
}
