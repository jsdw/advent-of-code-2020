use structopt::StructOpt;
use shared::FileContentOpts;
use shared::regex;
use std::collections::{ HashSet };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let mut instructions: Vec<_> = parse_instructions(&opts.file);

    // Run the machine until it hits the same instr twice:
    println!("Star 1: {}", run(&instructions).1);

    // Which nop/jmp to flip?
    let acc = (0..instructions.len()).filter_map(|i| {
        flip_instruction(&mut instructions[i]);
        let (pos, acc) = run(&instructions);
        flip_instruction(&mut instructions[i]);
        if pos == instructions.len() as i32 {
            Some(acc)
        } else {
            None
        }
    }).next().unwrap();
    println!("Star 2: {}", acc);


    Ok(())
}

fn run(instructions: &[Instruction]) -> (i32, i32) {
    let mut machine = Machine::with_instructions(&instructions);
    let mut seen_instrs = HashSet::new();
    while seen_instrs.insert(machine.pos()) && machine.step() {}
    (machine.pos(), machine.acc())
}

struct Machine<'a> {
    instrs: &'a [Instruction],
    acc: i32,
    pos: i32
}

impl <'a> Machine<'a> {
    fn with_instructions(instrs: &'a [Instruction]) -> Machine<'a> {
        Machine { instrs, acc: 0, pos: 0 }
    }
    fn pos(&self) -> i32 {
        self.pos
    }
    fn acc(&self) -> i32 {
        self.acc
    }
    fn step(&mut self) -> bool {
        let i = match self.instrs.get(self.pos as usize) {
            Some(i) => i,
            None => return false
        };
        match i {
            Instruction::Nop(..) => {
                self.pos += 1;
            },
            Instruction::Acc(n) => {
                self.acc += n;
                self.pos += 1;
            },
            Instruction::Jmp(n) => {
                self.pos += n;
            }
        }
        return true
    }
}

#[derive(Copy,Clone,Debug)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

impl Instruction {
    fn from_str(s: &str) -> Option<Instruction> {
        let caps = regex!(r"^\s*([a-z]{3}) ([+-][0-9]+)\s*$").captures(s)?;
        let name = caps.get(1)?.as_str();
        let val = caps.get(2)?.as_str().parse().ok()?;
        match name {
            "nop" => Some(Instruction::Nop(val)),
            "acc" => Some(Instruction::Acc(val)),
            "jmp" => Some(Instruction::Jmp(val)),
            _ => None
        }
    }
}

fn flip_instruction(i: &mut Instruction) {
    match i {
        Instruction::Jmp(n) => {
            *i = Instruction::Nop(*n)
        },
        Instruction::Nop(n) => {
            *i = Instruction::Jmp(*n)
        },
        _ => {}
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines().filter_map(Instruction::from_str).collect()
}