use std::convert::TryInto;
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct IntcodeComputer {
    pub memory: IntcodeProgram,
    instruction_pointer: usize,
    input: Option<Receiver<i64>>,
    output: Option<Sender<i64>>,
}

impl IntcodeComputer {
    pub fn load(&mut self, program: &IntcodeProgram) {
        self.memory = program.clone();
        self.instruction_pointer = 0;
    }

    pub fn create_input(&mut self) -> Sender<i64> {
        let (input_tx, input_rx) = mpsc::channel();
        self.input = Some(input_rx);
        input_tx
    }

    pub fn create_output(&mut self) -> Receiver<i64> {
        let (output_tx, output_rx) = mpsc::channel();
        self.output = Some(output_tx);
        output_rx
    }

    pub fn run(mut self) -> Self {
        loop {
            let next_instruction = IntcodeInstruction::from(&self);
            let instruction_pointer_before_instruction = self.instruction_pointer;
            let instruction_length = next_instruction.length();

            match next_instruction {
                IntcodeInstruction::Add(one, two, output) => {
                    let one = one.get_value(&self.memory);
                    let two = two.get_value(&self.memory);
                    let output_address = output
                        .get_address()
                        .expect("Add 'output' parameter must be an address");

                    self.memory.replace(output_address, one + two)
                }

                IntcodeInstruction::Multiply(one, two, output) => {
                    let one = one.get_value(&self.memory);
                    let two = two.get_value(&self.memory);
                    let output_address = output
                        .get_address()
                        .expect("Multiply 'output' parameter must be an address");

                    self.memory.replace(output_address, one * two)
                }

                IntcodeInstruction::Input(to) => {
                    let input_value = self
                        .input
                        .as_ref()
                        .expect("Program requires input but no input was connected!")
                        .recv()
                        .expect("Failed to receive from input");

                    let to_address = to
                        .get_address()
                        .expect("Input 'to' parameter must be an address");

                    self.memory.replace(to_address, input_value);
                }

                IntcodeInstruction::Output(from) => {
                    let output_value = from.get_value(&self.memory);

                    self.output
                        .as_ref()
                        .expect("Program requires output but no output was connected!")
                        .send(output_value)
                        .expect("Failed to send to output");
                }

                IntcodeInstruction::JumpIfTrue(test, jump_to) => {
                    if test.get_value(&self.memory) != 0 {
                        self.instruction_pointer =
                            jump_to.get_value(&self.memory).try_into().unwrap();
                    }
                }

                IntcodeInstruction::JumpIfFalse(test, jump_to) => {
                    if test.get_value(&self.memory) == 0 {
                        self.instruction_pointer =
                            jump_to.get_value(&self.memory).try_into().unwrap();
                    }
                }

                IntcodeInstruction::LessThan(one, two, output) => {
                    let one = one.get_value(&self.memory);
                    let two = two.get_value(&self.memory);

                    let output_value = if one < two { 1 } else { 0 };

                    let output_address = output
                        .get_address()
                        .expect("LessThan 'output' parameter must be an address");

                    self.memory.replace(output_address, output_value)
                }

                IntcodeInstruction::Equals(one, two, output) => {
                    let one = one.get_value(&self.memory);
                    let two = two.get_value(&self.memory);

                    let output_value = if one == two { 1 } else { 0 };

                    let output_address = output
                        .get_address()
                        .expect("LessThan 'output' parameter must be an address");

                    self.memory.replace(output_address, output_value)
                }

                IntcodeInstruction::Halt => break,
            }

            if instruction_pointer_before_instruction == self.instruction_pointer {
                self.instruction_pointer += instruction_length;
            }
        }

        self
    }
}

impl From<&IntcodeProgram> for IntcodeComputer {
    fn from(program: &IntcodeProgram) -> Self {
        Self {
            memory: program.clone(),
            instruction_pointer: 0,
            input: None,
            output: None,
        }
    }
}

impl From<&str> for IntcodeComputer {
    fn from(string: &str) -> Self {
        Self {
            memory: IntcodeProgram::from(string),
            instruction_pointer: 0,
            input: None,
            output: None,
        }
    }
}

#[derive(Debug)]
enum IntcodeInstruction {
    /// Adds the values from the first two parameters, writes the result to the third parameter
    Add(IntcodeParameter, IntcodeParameter, IntcodeParameter),

    /// Multiplies the values from the first two parameters, writes the result to the third parameter
    Multiply(IntcodeParameter, IntcodeParameter, IntcodeParameter),

    /// Takes a single integer from input and writes it to the first parameter
    Input(IntcodeParameter),

    /// Sends a single integer to output from the first parameter
    Output(IntcodeParameter),

    /// If the first parameter is non-zero, sets the instruction pointer to the value of the second parameter.
    JumpIfTrue(IntcodeParameter, IntcodeParameter),

    /// If the first parameter is zero, sets the instruction pointer to the value of the second parameter.
    JumpIfFalse(IntcodeParameter, IntcodeParameter),

    /// If the first parameter is less than the second parameter, writes 1 to the third parameter.
    /// Otherwise, writes 0 to the third parameter.
    LessThan(IntcodeParameter, IntcodeParameter, IntcodeParameter),

    /// If the first parameter is equal to the second parameter, writes 1 to the third parameter.
    /// Otherwise, writes 0 to the third parameter.
    Equals(IntcodeParameter, IntcodeParameter, IntcodeParameter),

    /// Halts the IntcodeComputer
    Halt,
}

impl IntcodeInstruction {
    pub fn length(&self) -> usize {
        match self {
            Self::Add(..) => 4,
            Self::Multiply(..) => 4,
            Self::Input(..) => 2,
            Self::Output(..) => 2,
            Self::JumpIfTrue(..) => 3,
            Self::JumpIfFalse(..) => 3,
            Self::LessThan(..) => 4,
            Self::Equals(..) => 4,
            Self::Halt => 1,
        }
    }
}

impl From<&IntcodeComputer> for IntcodeInstruction {
    fn from(state: &IntcodeComputer) -> Self {
        let instruction_header = state.memory.get(state.instruction_pointer);
        let opcode = Opcode::from(instruction_header);
        let mut parser = ParameterParser::from(instruction_header);

        match opcode {
            Opcode(1) => Self::Add(
                parser.parse_next(state.memory.get(state.instruction_pointer + 1)),
                parser.parse_next(state.memory.get(state.instruction_pointer + 2)),
                parser.parse_writeonly(state.memory.get(state.instruction_pointer + 3)),
            ),
            Opcode(2) => Self::Multiply(
                parser.parse_next(state.memory.get(state.instruction_pointer + 1)),
                parser.parse_next(state.memory.get(state.instruction_pointer + 2)),
                parser.parse_writeonly(state.memory.get(state.instruction_pointer + 3)),
            ),
            Opcode(3) => {
                Self::Input(parser.parse_writeonly(state.memory.get(state.instruction_pointer + 1)))
            }
            Opcode(4) => {
                Self::Output(parser.parse_next(state.memory.get(state.instruction_pointer + 1)))
            }
            Opcode(5) => Self::JumpIfTrue(
                parser.parse_next(state.memory.get(state.instruction_pointer + 1)),
                parser.parse_next(state.memory.get(state.instruction_pointer + 2)),
            ),
            Opcode(6) => Self::JumpIfFalse(
                parser.parse_next(state.memory.get(state.instruction_pointer + 1)),
                parser.parse_next(state.memory.get(state.instruction_pointer + 2)),
            ),
            Opcode(7) => Self::LessThan(
                parser.parse_next(state.memory.get(state.instruction_pointer + 1)),
                parser.parse_next(state.memory.get(state.instruction_pointer + 2)),
                parser.parse_writeonly(state.memory.get(state.instruction_pointer + 3)),
            ),
            Opcode(8) => Self::Equals(
                parser.parse_next(state.memory.get(state.instruction_pointer + 1)),
                parser.parse_next(state.memory.get(state.instruction_pointer + 2)),
                parser.parse_writeonly(state.memory.get(state.instruction_pointer + 3)),
            ),
            Opcode(99) => Self::Halt,
            Opcode(other) => panic!("Invalid Opcode encountered: {}", other),
        }
    }
}

#[derive(Debug)]
struct Opcode(i64);
impl From<i64> for Opcode {
    fn from(instruction_header: i64) -> Self {
        Self(get_digit(instruction_header, 1) * 10 + get_digit(instruction_header, 0))
    }
}

#[derive(Debug)]
enum IntcodeParameter {
    /// PositionMode
    Position(usize),

    /// ImmediateMode
    Value(i64),
}

impl IntcodeParameter {
    fn get_address(&self) -> Option<usize> {
        match self {
            Self::Position(address) => Some(*address),
            Self::Value(_) => None,
        }
    }

    fn get_value(&self, memory: &IntcodeProgram) -> i64 {
        match self {
            Self::Position(address) => memory.get(*address),
            Self::Value(value) => *value,
        }
    }
}

#[derive(Debug)]
struct ParameterParser {
    instruction_header: i64,
    parameters_read: u32,
}

impl From<i64> for ParameterParser {
    fn from(instruction_header: i64) -> Self {
        Self {
            instruction_header,
            parameters_read: 0,
        }
    }
}

impl ParameterParser {
    fn parse_next(&mut self, parameter: i64) -> IntcodeParameter {
        let mode = ParameterMode::from(&*self);
        let parameter = match mode {
            ParameterMode::PositionMode => {
                IntcodeParameter::Position(parameter.try_into().unwrap())
            }
            ParameterMode::ImmediateMode => IntcodeParameter::Value(parameter),
        };

        self.parameters_read += 1;

        parameter
    }

    fn parse_writeonly(&mut self, parameter: i64) -> IntcodeParameter {
        let parameter = IntcodeParameter::Position(parameter.try_into().unwrap());

        self.parameters_read += 1;

        parameter
    }
}

#[derive(Debug)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

impl From<&ParameterParser> for ParameterMode {
    fn from(state: &ParameterParser) -> Self {
        match get_digit(state.instruction_header, 2 + state.parameters_read) {
            0 => Self::PositionMode,
            1 => Self::ImmediateMode,
            other => panic!("Invalid ParameterMode: {}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntcodeProgram {
    data: Vec<i64>,
}

impl IntcodeProgram {
    pub fn get(&self, address: usize) -> i64 {
        *self
            .data
            .get(address)
            .unwrap_or_else(|| panic!("Failed to get data at address {}", address))
    }

    pub fn replace(&mut self, address: usize, replacement: i64) {
        let integer = self
            .data
            .get_mut(address)
            .unwrap_or_else(|| panic!("Failed to get_mut data at address {}", address));

        *integer = replacement;
    }

    pub fn data(&self) -> &Vec<i64> {
        &self.data
    }

    pub fn data_serialized(&self) -> String {
        self.data
            .iter()
            .map(|integer| integer.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl From<&str> for IntcodeProgram {
    fn from(string: &str) -> Self {
        Self {
            data: string
                .trim()
                .split(',')
                .map(|integer| integer.parse::<i64>())
                .map(|parse_result| parse_result.expect("Failed to parse Intcode integer as i64"))
                .collect(),
        }
    }
}

/// Gets the digit from number at a zero-indexed position from the right (in base 10)
fn get_digit(number: i64, position: u32) -> i64 {
    (number / (10_i64.pow(position))) % 10
}
