use std::collections::HashSet;
use std::io;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString, Hash, PartialEq, Copy, Clone)]
#[strum(serialize_all = "snake_case")]
enum InstructionType {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Hash, Copy, Clone)]
struct Instruction {
    t: InstructionType,
    v: i32,
}

#[derive(Debug)]
struct ParseError {
    details: String,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let data: Vec<_> = s.trim().split(" ").collect();
        if data.len() != 2 {
            return Result::Err(ParseError {
                details: "Foo".to_string(),
            });
        }

        let t = InstructionType::from_str(data[0]).unwrap(); //TODO ? but need to convert strum::ParseError to ParseError
        let v = data[1].parse::<i32>().unwrap(); //TODO ? but need to convert ??? to ParseError

        return Result::Ok(Instruction { t: t, v: v });
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.v == other.v
    }
}

impl Eq for Instruction {}

#[derive(Debug)]
struct Program {
    current: i32,
    current_flip: usize,
    val: i32,
    instructions: Vec<Instruction>,
    instructions_runned: HashSet<i32>,
    flipable_instructions: Vec<i32>,
    learning: bool,
}

impl Program {
    fn run_one_step(&mut self) -> i32 {
        let current = *self.get_current_instruction();

        if self.learning && (current.t == InstructionType::Jmp || current.t == InstructionType::Nop)
        {
            self.flipable_instructions.push(self.current);
        }
        self.instructions_runned.insert(self.current);

        let mut action = current.t;
        if !self.learning && self.flipable_instructions[self.current_flip] == self.current {
            //println!("Flip !");
            if action == InstructionType::Jmp {
                action = InstructionType::Nop;
            } else {
                action = InstructionType::Jmp;
            }
        }

        //println!("Run {:?} at {} : Current value : {}", action, self.current, self.val);

        match action {
            InstructionType::Acc => {
                self.val += current.v;
                self.current += 1;
            }
            InstructionType::Jmp => {
                self.current += current.v;
            }
            InstructionType::Nop => {
                self.current += 1;
            }
        }

        self.val
    }

    fn get_current_instruction(&self) -> &Instruction {
        &self.instructions[self.current as usize]
    }

    fn looped(&self) -> bool {
        self.instructions_runned.contains(&self.current)
    }

    fn is_finished(&self) -> bool {
        self.current as usize >= self.instructions.len()
    }

    fn new(instructions: Vec<Instruction>) -> Program {
        Program {
            current: 0,
            current_flip: 0,
            val: 0,
            instructions: instructions,
            instructions_runned: HashSet::new(),
            flipable_instructions: Vec::new(),
            learning: true,
        }
    }

    fn reset(&mut self) {
        //println!("== RESET ==");
        self.current = 0;
        self.val = 0;
        self.instructions_runned.clear();
        if self.learning {
            self.learning = false;
        } else {
            self.current_flip += 1;
        }
    }
}

impl Iterator for Program {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.looped() {
            self.reset();
            //println!("Will try to flip: {:?}", self.flipable_instructions[self.current_flip]);
        }
        if self.is_finished() {
            None
        } else {
            Some(self.run_one_step())
        }
    }
}

fn main() {
    let mut instructions = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }

        instructions.push(Instruction::from_str(input.trim()).unwrap());
    }

    let prog = Program::new(instructions);

    let mut last = 0;
    for val in prog {
        last = val;
    }
    println!("Got: {}", last);
}
