use crate::year2016::day12::lib::parser::Instruction::{Copy, Dec, Inc, Jnz};
use crate::year2016::day12::lib::parser::Register::A;
use crate::year2016::day12::lib::parser::{
    Instruction, ParsedInput, Register, RegisterTable, Value,
};
use itertools::Either;

pub fn part1(instructions: ParsedInput) -> String {
    let mut state = State::new(instructions);
    state.run();

    state.registers[A].to_string()
}

#[derive(Debug, Clone)]
pub struct State {
    instruction_counter: isize,
    instructions: Vec<Instruction>,
    pub registers: RegisterTable<isize>,
}

impl State {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        State {
            instruction_counter: 0,
            instructions,
            registers: RegisterTable::new(0, 0, 0, 0),
        }
    }

    pub fn step(&mut self) -> &mut Self {
        if let Some(instruction) = self.instructions.get(self.instruction_counter as usize) {
            self.instruction_counter += 1;
            match instruction {
                Copy(either_value_register, register_to) => {
                    self.registers[*register_to] = self.get_value(either_value_register)
                }
                Inc(register) => self.registers[*register] += 1,
                Dec(register) => self.registers[*register] -= 1,
                Jnz(either_value_register, counter_increment) => {
                    let value_to_check = self.get_value(either_value_register);
                    if value_to_check != 0 {
                        self.instruction_counter += *counter_increment - 1;
                    }
                }
            };
        };
        self
    }

    fn get_value(&self, either_value_register: &Either<Value, Register>) -> isize {
        match either_value_register {
            Either::Left(value) => *value,
            Either::Right(register_from) => self.registers[*register_from],
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.instruction_counter < 0 || self.instruction_counter as usize >= self.instructions.len()
    }

    pub fn run(&mut self) {
        while !self.is_terminated() {
            self.step();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::year2016::day12::lib::part1::part1;
    use crate::year2016::day12::lib::YEAR_2016_DAY_12_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2016_DAY_12_SOLUTION.get_parsed_test_inputs(1)),
            "42"
        );
    }
}
