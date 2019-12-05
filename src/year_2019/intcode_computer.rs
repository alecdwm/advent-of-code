#[derive(Debug)]
pub struct IntcodeComputer {
    pub memory: IntcodeProgram,
    instruction_pointer: usize,
}

impl IntcodeComputer {
    pub fn load(&mut self, program: &IntcodeProgram) {
        self.memory = program.clone();
        self.instruction_pointer = 0;
    }

    pub fn run(mut self) -> Self {
        loop {
            let next_instruction = IntcodeInstruction::from(&self);

            match next_instruction {
                IntcodeInstruction::Add(one_address, two_address, output_address) => {
                    let one = *self.memory.get(one_address);
                    let two = *self.memory.get(two_address);

                    self.memory.replace(output_address, one + two)
                }

                IntcodeInstruction::Multiply(one_address, two_address, output_address) => {
                    let one = *self.memory.get(one_address);
                    let two = *self.memory.get(two_address);

                    self.memory.replace(output_address, one * two)
                }

                IntcodeInstruction::Halt => break,
            }

            self.instruction_pointer += next_instruction.length()
        }

        self
    }
}

impl From<&IntcodeProgram> for IntcodeComputer {
    fn from(program: &IntcodeProgram) -> Self {
        Self {
            memory: program.clone(),
            instruction_pointer: 0,
        }
    }
}

impl From<&str> for IntcodeComputer {
    fn from(string: &str) -> Self {
        Self {
            memory: IntcodeProgram::from(string),
            instruction_pointer: 0,
        }
    }
}

#[derive(Debug)]
enum IntcodeInstruction {
    /// Adds the values from the first two addresses, writes the result to the third address
    Add(usize, usize, usize),

    /// Multiplies the values from the first two addresses, writes the result to the third address
    Multiply(usize, usize, usize),

    /// Halts the IntcodeComputer
    Halt,
}

impl IntcodeInstruction {
    pub fn length(&self) -> usize {
        match self {
            Self::Add(..) => 4,
            Self::Multiply(..) => 4,
            Self::Halt => 1,
        }
    }
}

impl From<&IntcodeComputer> for IntcodeInstruction {
    fn from(state: &IntcodeComputer) -> Self {
        let opcode = state.memory.get(state.instruction_pointer);

        match opcode {
            1 => Self::Add(
                *state.memory.get(state.instruction_pointer + 1),
                *state.memory.get(state.instruction_pointer + 2),
                *state.memory.get(state.instruction_pointer + 3),
            ),
            2 => Self::Multiply(
                *state.memory.get(state.instruction_pointer + 1),
                *state.memory.get(state.instruction_pointer + 2),
                *state.memory.get(state.instruction_pointer + 3),
            ),
            99 => Self::Halt,
            other => panic!("Invalid Opcode encountered: {}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntcodeProgram {
    data: Vec<usize>,
}

impl IntcodeProgram {
    pub fn get(&self, address: usize) -> &usize {
        self.data
            .get(address)
            .unwrap_or_else(|| panic!("Failed to get data at address {}", address))
    }

    pub fn replace(&mut self, address: usize, replacement: usize) {
        let integer = self
            .data
            .get_mut(address)
            .unwrap_or_else(|| panic!("Failed to get_mut data at address {}", address));

        *integer = replacement;
    }

    pub fn data(&self) -> &Vec<usize> {
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
                .split(",")
                .map(|integer| integer.parse::<usize>())
                .map(|parse_result| parse_result.expect("Failed to parse Intcode integer as usize"))
                .collect(),
        }
    }
}
