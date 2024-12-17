/*
    Comments.
*/

use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

#[derive(Debug)]
struct Program {
    register_a: i64,
    register_b: i64,
    register_c: i64,

    program: Vec<u8>,

    output: Vec<u8>,
}

impl Program {
    fn write(&mut self, value: u8) {
        self.output.push(value);
    }

    fn execute(&mut self) {
        let mut instruction_pointer = 0;
        let mut cpt = 0;
        while instruction_pointer < self.program.len() {
            cpt += 1;
            if cpt > 100_000_000 {
                panic!("It's probably an infinite loop");
            }
            let opcode = self.program[instruction_pointer];
            let operand = self.program[instruction_pointer + 1];
            //println!("instruction_pointer: {}", instruction_pointer);
            //println!("Executing opcode {} with operand {}", opcode, operand);
            match opcode {
                0 => execute_adv(self, operand),
                1 => execute_bxl(self, operand),
                2 => execute_bst(self, operand),
                3 => {
                    match execute_jnz(self, operand) {
                        Some(jump) => instruction_pointer = jump,
                        None => instruction_pointer += 2,
                    }
                    continue;
                }
                4 => execute_bxc(self, operand),
                5 => execute_out(self, operand),
                6 => execute_bdv(self, operand),
                7 => execute_cdv(self, operand),
                _ => panic!("Invalid opcode"),
            };
            instruction_pointer += 2;
        }
    }
}

fn literal_operand(operand: u8) -> i64 {
    if operand > 7 {
        panic!("Invalid operand");
    }
    operand as i64
}

fn combo_operand(program: &Program, operand: u8) -> i64 {
    match operand {
        0..=3 => operand as i64,
        4 => program.register_a,
        5 => program.register_b,
        6 => program.register_c,
        _ => panic!("Invalid operand"),
    }
}

/**
 * The adv instruction (opcode 0) performs division. The numerator is the value
 * in the A register. The denominator is found by raising 2 to the power of the
 * instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2);
 * an operand of 5 would divide A by 2^B.) The result of the division operation
 * is truncated to an integer and then written to the A register.
 */
fn execute_adv(program: &mut Program, operand: u8) {
    let numerator = program.register_a;
    let denumerator = 2_i64.pow(combo_operand(program, operand) as u32);
    let result = numerator / denumerator;
    program.register_a = result;
}

/**
 * The bxl instruction (opcode 1) calculates the bitwise XOR of register B and
 * the instruction's literal operand, then stores the result in register B.
 */
fn execute_bxl(program: &mut Program, operand: u8) {
    let result = program.register_b ^ literal_operand(operand);
    program.register_b = result;
}

/**
 * The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
 * (thereby keeping only its lowest 3 bits), then writes that value to the B register.
 */
fn execute_bst(program: &mut Program, operand: u8) {
    let result = combo_operand(program, operand) & 0b111;
    program.register_b = result;
}

/**
 * The jnz instruction (opcode 3) does nothing if the A register is 0. However,
 * if the A register is not zero, it jumps by setting the instruction pointer to the
 * value of its literal operand; if this instruction jumps, the instruction pointer
 * is not increased by 2 after this instruction.
 */
fn execute_jnz(program: &mut Program, operand: u8) -> Option<usize> {
    if program.register_a != 0 {
        return Some(literal_operand(operand) as usize);
    }
    None
}

/**
 * The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
 * register C, then stores the result in register B. (For legacy reasons, this
 * instruction reads an operand but ignores it.)
 */
fn execute_bxc(program: &mut Program, _operand: u8) {
    let result = program.register_b ^ program.register_c;
    program.register_b = result;
}

/**
 * The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
 * then outputs that value. (If a program outputs multiple values, they are
 * separated by commas.)
 */
fn execute_out(program: &mut Program, operand: u8) {
    let result = (combo_operand(program, operand) & 0b111) as u8;
    program.write(result);
}

/**
 * The bdv instruction (opcode 6) works exactly like the adv instruction except that
 * the result is stored in the B register. (The numerator is still read from the A register.)
 */
fn execute_bdv(program: &mut Program, operand: u8) {
    let numerator = program.register_a;
    let denumerator = 2_i64.pow(combo_operand(program, operand) as u32);
    let result = numerator / denumerator;
    program.register_b = result;
}

/**
 * The cdv instruction (opcode 7) works exactly like the adv instruction except that
 * the result is stored in the C register. (The numerator is still read from the A register.)
 */
fn execute_cdv(program: &mut Program, operand: u8) {
    let numerator = program.register_a;
    let denumerator = 2_i64.pow(combo_operand(program, operand) as u32);
    let result = numerator / denumerator;
    program.register_c = result;
}

fn parse_input_data(data: &str) -> IResult<&str, Program> {
    map(
        tuple((
            tag("Register A: "),
            nom::character::complete::i64,
            line_ending,
            tag("Register B: "),
            nom::character::complete::i64,
            line_ending,
            tag("Register C: "),
            nom::character::complete::i64,
            line_ending,
            line_ending,
            tag("Program: "),
            separated_list1(tag(","), nom::character::complete::u8),
        )),
        |(_, register_a, _, _, register_b, _, _, register_c, _, _, _, program)| Program {
            register_a,
            register_b,
            register_c,
            program,
            output: Vec::new(),
        },
    )(data)
}

pub fn day_17_part_1(data: &str) -> i64 {
    let (_, program) = parse_input_data(data).expect("Failed to parse input data");

    let mut program = program;
    program.execute();

    // join the output with commas
    let output = program
        .output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Day 17, part 1 output: {}", output);
    42
}

pub fn day_17_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_day_17_program() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut program = Program {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            program: vec![2, 6],
            output: Vec::new(),
        };
        program.execute();
        assert_eq!(program.register_b, 1);

        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut program = Program {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            output: Vec::new(),
        };
        program.execute();
        assert_eq!(program.output, vec![0, 1, 2]);

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut program = Program {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            output: Vec::new(),
        };
        program.execute();
        assert_eq!(program.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(program.register_a, 0);

        // If register B contains 29, the program 1,7 would set register B to 26.
        let mut program = Program {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            program: vec![1, 7],
            output: Vec::new(),
        };
        program.execute();
        assert_eq!(program.register_b, 26);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        let mut program = Program {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            program: vec![4, 0],
            output: Vec::new(),
        };
        program.execute();
        assert_eq!(program.register_b, 44354);
    }

    #[test]
    fn test_day_17_part_1() {
        assert_eq!(day_17_part_1(EXAMPLE), 42);
    }

    #[test]
    fn test_day_17_part_2() {
        assert_eq!(day_17_part_2(EXAMPLE), 42);
    }
}
