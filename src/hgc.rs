use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(input: &str) -> Result<OpCode, Self::Err> {
        match input {
            "nop" => Ok(OpCode::Nop),
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            _ => Err(()),
        }
    }
}

pub type Op = (OpCode, i64);
pub struct Program {
    acc: i64,
    ip: usize,
    code: Vec<Op>,
}

impl Program {
    pub fn new(code: Vec<Op>) -> Self {
        Program {
            acc: 0,
            ip: 0,
            code,
        }
    }
    pub fn step(&mut self) -> bool {
        match self.code[self.ip] {
            (OpCode::Nop, _) => {
                self.ip += 1;
            }
            (OpCode::Acc, n) => {
                self.acc += n;
                self.ip += 1;
            }
            (OpCode::Jmp, n) => {
                self.ip += n as usize;
            }
        }
        self.ip >= self.code.len()
    }

    pub fn acc(&self) -> i64 {
        self.acc
    }
    pub fn ip(&self) -> usize {
        self.ip
    }
}

pub fn parse_code<'t>(code: &'t str) -> impl Iterator<Item = Option<Op>> + 't {
    code.lines().map(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return None;
        }
        let (op, val) = (parts[0], parts[1]);
        Some((OpCode::from_str(op).ok()?, i64::from_str(val).ok()?))
    })
}
