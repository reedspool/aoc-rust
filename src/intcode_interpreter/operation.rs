use super::{Program, ProgramMemory};

pub type ParamModes = [i32; 3];
#[derive(Debug)]
pub struct Operation {
    code: OpCode,
    // The parameter modes are stored backwards in the opcode,
    // but I will be storing them 1st, 2nd, 3rd
    param_modes: ParamModes,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpCode {
    Add = 1,
    Multiply = 2,
    ReadInput = 3,
    WriteOutput = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl OpCode {
    pub fn from_int(num: i32) -> OpCode {
        match num {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::ReadInput,
            4 => OpCode::WriteOutput,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            unknown => panic!("Unknown opcode: {}", unknown),
        }
    }
}

impl Operation {
    pub fn from_code(code: i32) -> Operation {
        let ones = code % 10;
        let tens = (code / 10) % 10;
        let hundreds = (code / 100) % 10;
        let thousands = (code / 1000) % 10;
        let ten_thousands = (code / 10000) % 10;
        let code = OpCode::from_int(ones + tens * 10);

        Operation {
            code,
            param_modes: [hundreds, thousands, ten_thousands],
        }
    }

    pub fn exec(&self, program: &mut Program) {
        match self.code {
            OpCode::Add => {
                let (output, left, right) = Operation::get_stuff_for_basic_binary_op(
                    &program.memory,
                    &program.instruction_pointer,
                    &self.param_modes,
                );

                program.memory[output] = left + right;
            }
            OpCode::Multiply => {
                let (output, left, right) = Operation::get_stuff_for_basic_binary_op(
                    &program.memory,
                    &program.instruction_pointer,
                    &self.param_modes,
                );

                program.memory[output] = left * right;
            }
            OpCode::ReadInput => {
                let (output_location) = Operation::get_stuff_for_basic_unary_op(
                    &program.memory,
                    program.instruction_pointer,
                    &self.param_modes,
                );

                let output_location = program.memory[program.instruction_pointer + 1 as usize];

                program.memory[output_location as usize] = program.input;
            }
            OpCode::WriteOutput => {
                let (output) = Operation::get_stuff_for_basic_unary_op(
                    &program.memory,
                    program.instruction_pointer,
                    &self.param_modes,
                );

                program.push_output(output);
            }
            /*
            Hmmm. My normal bifurcaction of concerns doesn't work here. Jump's need to change the instruction pointer in both cases, but the two cases are wildly different. I guess I'll have to do both here and do a noop in the move pointer fn below
            */
            OpCode::JumpIfTrue => {

                // program.instruction_pointer = ???
            },
            OpCode::JumpIfFalse => {},
            OpCode::LessThan => todo!(),
            OpCode::Equals => todo!(),
            OpCode::Halt => program.halt(),
        }
    }

    pub fn move_instruction_pointer(&self, program: &mut Program) {
        program.instruction_pointer = match self.code {
            OpCode::Add => program.instruction_pointer + 4,
            OpCode::Multiply => program.instruction_pointer + 4,
            OpCode::ReadInput => program.instruction_pointer + 2,
            OpCode::WriteOutput => program.instruction_pointer + 2,
            OpCode::Halt => program.instruction_pointer,
            // Jumps are handled during their execution
            OpCode::JumpIfTrue => 0,
            OpCode::JumpIfFalse => 0,
            OpCode::LessThan => todo!(),
            OpCode::Equals => todo!(),
        };
    }

    pub fn get_stuff_for_basic_binary_op(
        memory: &ProgramMemory,
        pointer: &usize,
        param_modes: &ParamModes,
    ) -> (usize, i32, i32) {
        let left_operand_value = memory[pointer + 1];
        let right_operand_value = memory[pointer + 2];
        let output_location = memory[pointer + 3] as usize;

        let left = if param_modes[0] == 0 {
            memory[left_operand_value as usize]
        } else {
            left_operand_value
        };
        let right = if param_modes[1] == 0 {
            memory[right_operand_value as usize]
        } else {
            right_operand_value
        };

        (output_location, left, right)
    }

    pub fn get_stuff_for_basic_unary_op(
        memory: &ProgramMemory,
        pointer: usize,
        param_modes: &ParamModes,
    ) -> (i32) {
            Operation::get_value_by_mode(memory, memory[pointer + 1usize], param_modes[0])
    }

    pub fn get_value_by_mode(
        memory: &ProgramMemory,
        value: i32,
        param_mode: i32) -> i32 {

        if param_mode == 0 {
            memory[value as usize]
        } else {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_add_op() {
        let op = Operation::from_code(1);
        assert_eq!(op.code, OpCode::Add);
    }
}
