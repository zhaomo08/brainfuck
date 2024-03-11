mod opcode;

use opcode::Opcode;

use std::io::Read;
use std::io::Write;

#[derive(Debug, PartialEq)]

pub enum IR {
    SHR(u32), // >>>> === SHR(4)
    SHL(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32), // Jump if zero
    JNZ(u32), // Jump if not zero
}

pub struct Code {
    pub instrs: Vec<IR>,
}

impl Code {
    pub fn from(data: Vec<opcode::Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instrs: Vec<IR> = Vec::new();
        let mut jstack: Vec<u32> = Vec::new();

        for e in &data {
            match e {
                Opcode::SHR => match instrs.last_mut() {
                    Some(IR::SHR(x)) => {
                        *x += 1;
                    }
                    _ => instrs.push(IR::SHR(1)),
                },
                Opcode::SHL => match instrs.last_mut() {
                    Some(IR::SHL(x)) => {
                        *x += 1;
                    }
                    _ => instrs.push(IR::SHL(1)),
                },
                Opcode::ADD => match instrs.last_mut() {
                    Some(IR::ADD(x)) => {
                        let (b, _) = x.overflowing_add(1);
                        *x = b;
                    }
                    _ => instrs.push(IR::ADD(1)),
                },
                Opcode::SUB => match instrs.last_mut() {
                    Some(IR::SUB(x)) => {
                        let (b, _) = x.overflowing_add(1);
                        *x = b;
                    }
                    _ => instrs.push(IR::SUB(1)),
                },
                Opcode::PUTCHAR => {
                    instrs.push(IR::PUTCHAR);
                }
                Opcode::GETCHAR => {
                    instrs.push(IR::GETCHAR);
                }
                Opcode::LB => {
                    instrs.push(IR::JIZ(0));
                    jstack.push((instrs.len() - 1) as u32);
                }
                Opcode::RB => {
                    let j = jstack.pop().ok_or("pop from empty list")?;
                    instrs.push(IR::JNZ(j));
                    let instrs_len = instrs.len();

                    match &mut instrs[j as usize] {
                        IR::JIZ(x) => *x = (instrs_len - 1) as u32,
                        _ => unreachable!(),
                    }
                }
            }
        }
        Ok(Code { instrs })
    }
}


struct Interpreter {
    stack: Vec<u8>,
}


impl Interpreter {
    fn fun(&mut self,data: Vec<u8>) ->Result<(),Box<dyn std::error::Error>>{
        let opcode_code = opcode::Code::from(data)?;
        let code = Code::from(opcode_code.instrs)?;
        let code_len = code.instrs.len();
        let mut pc = 0;
        let mut sp = 0;

        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                IR::SHR(x) => {
                    sp += x as usize;
                    if sp >= self.stack.len(){
                        let expand = sp - self.stack.len() + 1;
                        for _ in 0..expand{
                            self.stack.push(0);
                        }
                    }
                }
                IR::SHL(x) => {
                   for x in 0..x{
                    if sp != 0{
                        sp -= x as usize;
                    }else {
                        break;
                    }
                   }
                } ,
                IR::ADD(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;

                },
                IR::SUB(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                },
                IR::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                },
                IR::GETCHAR => {
                    let mut buf: Vec<u8> =vec![0;1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] =buf[0];
                },
                IR::JIZ(x) => {
                    if self.stack[sp] == 0x00 {
                        pc = x as usize;
                    }
                },
                IR::JNZ(x) => {
                    if self.stack[sp] != 0x00 {
                        pc = x as usize;
                }
            }
        }
            pc += 1;
        }

        Ok(())
    }
}
fn main() {}
