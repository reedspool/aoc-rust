use crate::intcode_interpreter::operation::{OpCode, Operation};

mod operation;

pub type ProgramMemory = Vec<i32>;

#[derive(Clone,Debug)]
pub struct Program {
    memory: ProgramMemory,
    input: i32,
    output: Vec<i32>,
    instruction_pointer: usize,
    halted: bool
}

impl Program {
    pub fn parse(input: &str) -> Program {
        let intcode = input
            .trim()
            .split(",")
            .map(|code| code.parse::<i32>().unwrap());
        let memory = Vec::from_iter(intcode);

        Program { memory, input: 0, instruction_pointer: 0, halted: false, output: Vec::new() }
    }

    pub fn first_position_for(input: &str) -> i32 {
        Program::parse(input).run().first_position()
    }

    pub fn run(self) -> Program {
        let mut program = self.clone();

        loop {
            let opcode = program.memory[program.instruction_pointer];

            if program.halted {
                break;
            }

            let operation : Operation = Operation::from_code(opcode);

            operation.exec(&mut program);

            operation.move_instruction_pointer(&mut program);
        }

        program
    }

    pub fn output(&self) -> &Vec<i32> {
        &self.output
    }
    pub fn first_position(&self) -> i32 {
        self.memory[0]
    }

    pub fn set_noun(&mut self, input: i32) {
        self.memory[1] = input;
    }

    pub fn set_verb(&mut self, input: i32) {
        self.memory[2] = input;
    }


    pub fn set_input(&mut self, input: i32) {
        self.input = input;
    }

    pub fn push_output(&mut self, output: i32) {
        self.output.push(output);
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate_halt() {
        assert_eq!(Program::first_position_for("99"), 99);
    }

    #[test]
    fn basic_add_1() {
        assert_eq!(Program::first_position_for("1,0,0,0,99"), 2);
    }

    #[test]
    fn basic_multiply_1() {
        assert_eq!(Program::first_position_for("2,0,0,0,99"), 4);
    }

    #[test]
    fn basic_input_1() {
        let mut program = Program::parse("3,0,99");
        program.set_input(5);
        assert_eq!(program.run().first_position(), 5);
    }

    #[test]
    fn basic_output_1() {
        let mut program = Program::parse("4,0,99");
        assert_eq!(program.run().output()[0], 4);
    }

    #[test]
    fn basic_immediate_mode_1() {
        let mut program = Program::parse("1102,4,3,0,99");
        assert_eq!(program.run().first_position(), 12);

        let mut program = Program::parse("1002,4,3,0,99");
        assert_eq!(program.run().first_position(), 99 * 3);

        let mut program = Program::parse("102,4,3,0,99");
        assert_eq!(program.run().first_position(), 0);
    }

    #[test]
    fn overwrite_opcode() {
        assert_eq!(Program::first_position_for("1,1,1,4,99,5,6,0,99"), 30);
    }
}
