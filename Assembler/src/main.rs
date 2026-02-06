use std::collections::HashMap;
use std::env;
use std::fs;

struct Instruction<'a> {
    pub opcode: u8,
    pub mnemonic: &'a str,
    pub argument: InstructionArgument,
}

#[derive(PartialEq)]
enum InstructionArgument {
    None,
    Value,
    Address,
    Dereference,
}

impl<'a> Instruction<'a> {
    pub const fn new(opcode: u8, mnemonic: &'a str, argument: InstructionArgument) -> Self {
        Self { opcode, mnemonic, argument }
    }
}

struct Operation<'a> {
    pub instruction: &'a Instruction<'a>,
    pub argument: OperationArgument,
}

#[derive(PartialEq)]
enum OperationArgument {
    None,
    Value(u8),
    Address(String),
    Dereference(String),
}

impl<'a> Operation<'a> {
    pub fn new(instruction: &'a Instruction<'a>, argument: OperationArgument) -> Self {
        Self { instruction, argument }
    }
}

struct Label {
    pub address: u8,
}

impl Label {
    pub fn new(address: u8) -> Self {
        Self { address }
    }
}

const INSTRUCTIONS: [Instruction; 32] = [
    Instruction::new(0x01, "HLT", InstructionArgument::None),
    Instruction::new(0x02, "JMP", InstructionArgument::Address),
    Instruction::new(0x03, "JEQ", InstructionArgument::Address),
    Instruction::new(0x04, "JNQ", InstructionArgument::Address),
    Instruction::new(0x05, "JGR", InstructionArgument::Address),
    Instruction::new(0x06, "JGQ", InstructionArgument::Address),
    Instruction::new(0x07, "JLE", InstructionArgument::Address),
    Instruction::new(0x08, "JLQ", InstructionArgument::Address),
    Instruction::new(0x09, "LDA", InstructionArgument::Value),
    Instruction::new(0x0a, "LDA", InstructionArgument::Address),
    Instruction::new(0x0b, "LDA", InstructionArgument::Dereference),
    Instruction::new(0x0c, "LDB", InstructionArgument::Value),
    Instruction::new(0x0d, "LDB", InstructionArgument::Address),
    Instruction::new(0x0e, "LDB", InstructionArgument::Dereference),
    Instruction::new(0x0f, "STA", InstructionArgument::Address),
    Instruction::new(0x10, "STA", InstructionArgument::Dereference),
    Instruction::new(0x11, "STB", InstructionArgument::Address),
    Instruction::new(0x12, "STB", InstructionArgument::Dereference),
    Instruction::new(0x13, "ADD", InstructionArgument::None),
    Instruction::new(0x14, "SUB", InstructionArgument::None),
    Instruction::new(0x15, "NEG", InstructionArgument::None),
    Instruction::new(0x16, "AND", InstructionArgument::None),
    Instruction::new(0x17, "OR", InstructionArgument::None),
    Instruction::new(0x18, "XOR", InstructionArgument::None),
    Instruction::new(0x19, "NOT", InstructionArgument::None),
    Instruction::new(0x1a, "SHL", InstructionArgument::None),
    Instruction::new(0x1b, "SHR", InstructionArgument::None),
    Instruction::new(0x1c, "CMP", InstructionArgument::None),
    Instruction::new(0x1d, "OUTA", InstructionArgument::None),
    Instruction::new(0x1e, "OUTB", InstructionArgument::None),
    Instruction::new(0x1f, "OUTC", InstructionArgument::None),
    Instruction::new(0x20, "OUTD", InstructionArgument::None),
];

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut iter = args.iter().peekable();
    let mut input = None;
    let mut output = None;

    while let Some(token) = iter.next() {
        match token.as_str() {
            "-in" => {
                if let Some(path) = iter.peek() {
                    input = Some(*path);
                } else {
                    panic!("Invalid input path")
                }
            }
            "-out" => {
                if let Some(path) = iter.peek() {
                    output = Some(*path);
                } else {
                    panic!("Invalid output path")
                }
            }
            _ => {}
        }
    }

    let input = match input {
        Some(input) => input,
        None => panic!("No input path"),
    };

    let output = match output {
        Some(output) => output,
        None => panic!("No output path"),
    };

    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(err) => panic!("{}", err),
    };

    let result = compile(&source);

    println!("Binary size: {} bytes ({} left)", result.len(), 127 - result.len());
    println!("Saving to file...");

    fs::write(output, result).unwrap();

    println!("Done")
}

fn compile(source: &str) -> Vec<u8> {
    let tokens = source.split_whitespace();
    let mut iter = tokens.peekable();
    let mut operations = Vec::default();
    let mut labels = HashMap::<String, Label>::default();
    let mut output = Vec::default();
    let mut program_pointer = 0;
    let mut memory_pointer = 128;
    let mut comment = false;

    while let Some(token) = iter.next() {
        let mut value = None;
        let mut address = None;
        let mut dereference = None;

        if token.starts_with(";") {
            comment = !comment;
            continue;
        } else if token.ends_with(":") {
            labels.insert(token.trim_matches(':').to_string(), Label::new(program_pointer));
            continue;
        }

        if comment {
            continue;
        }

        if token == "VAR" {
            if let Some(next_token) = iter.peek() {
                labels.insert(next_token.to_string(), Label::new(memory_pointer));
            } else {
                panic!("Invalid variable");
            }

            memory_pointer += 1;
            iter.next();

            continue;
        }

        if let Some(next_token) = iter.peek() {
            if next_token.starts_with("0x") {
                value = Some(next_token);
            } else if next_token.starts_with("&&") {
                dereference = Some(next_token);
            } else if next_token.starts_with("&") {
                address = Some(next_token);
            }
        }

        let instruction = if value.is_some() {
            INSTRUCTIONS.iter().find(|p| p.mnemonic == token && p.argument == InstructionArgument::Value)
        } else if address.is_some() {
            INSTRUCTIONS.iter().find(|p| p.mnemonic == token && p.argument == InstructionArgument::Address)
        } else if dereference.is_some() {
            INSTRUCTIONS.iter().find(|p| p.mnemonic == token && p.argument == InstructionArgument::Dereference)
        } else {
            INSTRUCTIONS.iter().find(|p| p.mnemonic == token)
        };

        if let Some(instruction) = instruction {
            if let Some(value) = value {
                let value = match u8::from_str_radix(&value[2..value.len()], 16) {
                    Ok(value) => value,
                    Err(err) => panic!("Invalid argument: {}", err),
                };

                operations.push(Operation::new(instruction, OperationArgument::Value(value)));
            } else if let Some(address) = address {
                operations.push(Operation::new(instruction, OperationArgument::Address(address.trim_matches(['&']).to_string())));
            } else if let Some(dereference) = dereference {
                operations.push(Operation::new(instruction, OperationArgument::Address(dereference.trim_matches(['&']).to_string())));
            } else {
                operations.push(Operation::new(instruction, OperationArgument::None));
            }

            if value.is_some() || address.is_some() || dereference.is_some() {
                iter.next();
                program_pointer += 1;
            }

            program_pointer += 1;
        } else {
            panic!("Invalid instruction: {}", token);
        }
    }

    for operation in operations {
        match operation.argument {
            OperationArgument::None => {
                output.push(operation.instruction.opcode);
            }
            OperationArgument::Value(value) => {
                output.push(operation.instruction.opcode);
                output.push(value);
            }
            OperationArgument::Address(address) | OperationArgument::Dereference(address) => {
                if let Some(label) = labels.get(&address) {
                    output.push(operation.instruction.opcode);
                    output.push(label.address);
                } else {
                    panic!("Invalid symbol: {}", address);
                }
            }
        }
    }

    output
}
