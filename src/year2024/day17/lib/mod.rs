use crate::year2024::day17::lib::parser::{parse_input, ParsedInput};
use crate::year2024::day17::lib::parser::{Program, RegisterA, RegisterB, RegisterC};
pub use crate::year2024::day17::lib::part1::part1;
pub use crate::year2024::day17::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use itertools::{iterate, Itertools};
use std::ops::{BitXor, Rem};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_17_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 17),
    parser: parse_input,
    part1,
    expected_part1: "6,5,7,4,5,7,3,1,0",
    part2,
    expected_part2: "105875099912602",
};

aoc_solver!(YEAR_2024_DAY_17_SOLUTION);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProgramState {
    instruction_pointer: usize,
    pub register_a: RegisterA,
    pub register_b: RegisterB,
    pub register_c: RegisterC,
    pub program: Program,
    pub output: Vec<isize>,
}

pub type Opcode = isize;
pub type Operand = isize;
impl ProgramState {
    pub fn new(
        register_a: RegisterA,
        register_b: RegisterB,
        register_c: RegisterC,
        program: Program,
    ) -> Self {
        ProgramState {
            instruction_pointer: 0,
            register_a,
            register_b,
            register_c,
            program,
            output: Vec::new(),
        }
    }

    pub fn run_program(&self) -> Self {
        iterate(Some(self.clone()), |state_option| {
            state_option
                .clone()
                .map(|mut state| state.execute_instruction())
                .flatten()
        })
        .take_while(|state| state.is_some())
        .last()
        .unwrap()
        .unwrap()
    }

    pub fn run_until_first_output(&self) -> Option<isize> {
        iterate(Some(self.clone()), |state_option| {
            state_option
                .clone()
                .map(|mut state| state.execute_instruction())
                .flatten()
        })
        .take_while_inclusive(|state| state.clone().filter(|s| s.output.len() <= 0).is_some())
        .last()
        .unwrap()
        .map(|state| state.output[0])
    }

    pub fn execute_instruction(&mut self) -> Option<Self> {
        let (&opcode, &operand) = self.get_next_opcode_with_operand()?;

        const ADV: Opcode = 0;
        const BXL: Opcode = 1;
        const BST: Opcode = 2;
        const JNZ: Opcode = 3;
        const BXC: Opcode = 4;
        const OUT: Opcode = 5;
        const BDV: Opcode = 6;
        const CDV: Opcode = 7;

        match opcode {
            ADV => {
                let combo_operand = self.get_combo_operand(operand);
                self.register_a = self.register_a / 2_isize.pow(combo_operand as u32);
            }
            BXL => {
                self.register_b = self.register_b.bitxor(operand as isize);
            }
            BST => {
                let combo_operand = self.get_combo_operand(operand);
                self.register_b = combo_operand.rem(8) as RegisterB;
            }
            JNZ => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand as usize;
                }
            }
            BXC => {
                self.register_b = self.register_b.bitxor(self.register_c);
            }
            OUT => {
                let combo_operand = self.get_combo_operand(operand);
                self.output.push(combo_operand.rem(8) as isize);
            }
            BDV => {
                let combo_operand = self.get_combo_operand(operand);
                self.register_b = self.register_a / 2_isize.pow(combo_operand as u32);
            }
            CDV => {
                let combo_operand = self.get_combo_operand(operand);
                self.register_c = self.register_a / 2_isize.pow(combo_operand as u32);
            }
            _ => panic!("Unexpected opcode encountered!"),
        }
        Some(self.clone())
    }

    fn get_combo_operand(&self, operand: Operand) -> Operand {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.register_a as Operand,
            5 => self.register_b as Operand,
            6 => self.register_c as Operand,
            7 => panic!("Not allowed for combo operands"),
            _ => panic!("Unexpected operand encountered!"),
        }
    }

    fn get_next_opcode_with_operand(&mut self) -> Option<(&Opcode, &Operand)> {
        let opcode = self.program.get(self.instruction_pointer)?;
        self.instruction_pointer += 1;
        let operand = self.program.get(self.instruction_pointer)?;
        self.instruction_pointer += 1;
        Some((opcode, operand))
    }
}
