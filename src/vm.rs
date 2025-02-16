use crate::bytecode::{InsnOpcode, InsnOperand, Instruction};
use std::fmt;

#[derive(Debug)]
pub struct State {
    i: usize,
    instructions: Vec<Instruction>,
    reg0: i32,
    reg1: i32,
    reg2: i32,
    reg3: i32,
    reg4: i32,
    reg5: i32,
    reg6: i32,
    reg7: i32,
    stack: Vec<i32>,
}

impl State {
    fn step(&mut self) -> Result<(), String> {
        let insn = self.instructions.get(self.i).unwrap();
        println!("running: {:?}", insn);
        match insn.opcode {
            InsnOpcode::Ldc => match insn.operands.get(0) {
                None => return Err(format!("Illegal ldc instruction: {:?}", insn)),
                Some(arg1) => match insn.operands.get(1) {
                    None => return Err(format!("Illegal ldc instruction: {:?}", insn)),
                    Some(arg2) => {
                        let src: i32 = match arg2 {
                            InsnOperand::Imm(n) => n.clone(),
                            InsnOperand::Stack(n) => match self.stack.get(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => v.clone(),
                            },
                            _ => return Err(format!("Illegal ldc instruction: {:?}", insn)),
                        };

                        match arg1 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal ldc instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get_mut(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => *v += src,
                            },
                            InsnOperand::Reg0 => self.reg0 += src,
                            InsnOperand::Reg1 => self.reg1 += src,
                            InsnOperand::Reg2 => self.reg2 += src,
                            InsnOperand::Reg3 => self.reg3 += src,
                            InsnOperand::Reg4 => self.reg4 += src,
                            InsnOperand::Reg5 => self.reg5 += src,
                            InsnOperand::Reg6 => self.reg6 += src,
                            InsnOperand::Reg7 => self.reg7 += src,
                        }
                    }
                },
            },
            InsnOpcode::Add => match insn.operands.get(0) {
                None => return Err(format!("Illegal add instruction: {:?}", insn)),
                Some(arg1) => match insn.operands.get(1) {
                    None => return Err(format!("Illegal add instruction: {:?}", insn)),
                    Some(arg2) => {
                        let src: i32 = match arg2 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal add instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => v.clone(),
                            },
                            InsnOperand::Reg0 => self.reg0,
                            InsnOperand::Reg1 => self.reg1,
                            InsnOperand::Reg2 => self.reg2,
                            InsnOperand::Reg3 => self.reg3,
                            InsnOperand::Reg4 => self.reg4,
                            InsnOperand::Reg5 => self.reg5,
                            InsnOperand::Reg6 => self.reg6,
                            InsnOperand::Reg7 => self.reg7,
                        };

                        match arg1 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal add instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get_mut(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => *v += src,
                            },
                            InsnOperand::Reg0 => self.reg0 += src,
                            InsnOperand::Reg1 => self.reg1 += src,
                            InsnOperand::Reg2 => self.reg2 += src,
                            InsnOperand::Reg3 => self.reg3 += src,
                            InsnOperand::Reg4 => self.reg4 += src,
                            InsnOperand::Reg5 => self.reg5 += src,
                            InsnOperand::Reg6 => self.reg6 += src,
                            InsnOperand::Reg7 => self.reg7 += src,
                        }
                    }
                },
            },
            InsnOpcode::Sub => match insn.operands.get(0) {
                None => return Err(format!("Illegal sub instruction: {:?}", insn)),
                Some(arg1) => match insn.operands.get(1) {
                    None => return Err(format!("Illegal sub instruction: {:?}", insn)),
                    Some(arg2) => {
                        let src: i32 = match arg2 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal sub instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => v.clone(),
                            },
                            InsnOperand::Reg0 => self.reg0,
                            InsnOperand::Reg1 => self.reg1,
                            InsnOperand::Reg2 => self.reg2,
                            InsnOperand::Reg3 => self.reg3,
                            InsnOperand::Reg4 => self.reg4,
                            InsnOperand::Reg5 => self.reg5,
                            InsnOperand::Reg6 => self.reg6,
                            InsnOperand::Reg7 => self.reg7,
                        };

                        match arg1 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal sub instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get_mut(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => *v += src,
                            },
                            InsnOperand::Reg0 => self.reg0 -= src,
                            InsnOperand::Reg1 => self.reg1 -= src,
                            InsnOperand::Reg2 => self.reg2 -= src,
                            InsnOperand::Reg3 => self.reg3 -= src,
                            InsnOperand::Reg4 => self.reg4 -= src,
                            InsnOperand::Reg5 => self.reg5 -= src,
                            InsnOperand::Reg6 => self.reg6 -= src,
                            InsnOperand::Reg7 => self.reg7 -= src,
                        }
                    }
                },
            },
            InsnOpcode::Mul => match insn.operands.get(0) {
                None => return Err(format!("Illegal mul instruction: {:?}", insn)),
                Some(arg1) => match insn.operands.get(1) {
                    None => return Err(format!("Illegal mul instruction: {:?}", insn)),
                    Some(arg2) => {
                        let src: i32 = match arg2 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal mul instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => v.clone(),
                            },
                            InsnOperand::Reg0 => self.reg0,
                            InsnOperand::Reg1 => self.reg1,
                            InsnOperand::Reg2 => self.reg2,
                            InsnOperand::Reg3 => self.reg3,
                            InsnOperand::Reg4 => self.reg4,
                            InsnOperand::Reg5 => self.reg5,
                            InsnOperand::Reg6 => self.reg6,
                            InsnOperand::Reg7 => self.reg7,
                        };

                        match arg1 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal mul instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get_mut(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => *v += src,
                            },
                            InsnOperand::Reg0 => self.reg0 *= src,
                            InsnOperand::Reg1 => self.reg1 *= src,
                            InsnOperand::Reg2 => self.reg2 *= src,
                            InsnOperand::Reg3 => self.reg3 *= src,
                            InsnOperand::Reg4 => self.reg4 *= src,
                            InsnOperand::Reg5 => self.reg5 *= src,
                            InsnOperand::Reg6 => self.reg6 *= src,
                            InsnOperand::Reg7 => self.reg7 *= src,
                        }
                    }
                },
            },
            InsnOpcode::Div => match insn.operands.get(0) {
                None => return Err(format!("Illegal div instruction: {:?}", insn)),
                Some(arg1) => match insn.operands.get(1) {
                    None => return Err(format!("Illegal div instruction: {:?}", insn)),
                    Some(arg2) => {
                        let src: i32 = match arg2 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal div instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => v.clone(),
                            },
                            InsnOperand::Reg0 => self.reg0,
                            InsnOperand::Reg1 => self.reg1,
                            InsnOperand::Reg2 => self.reg2,
                            InsnOperand::Reg3 => self.reg3,
                            InsnOperand::Reg4 => self.reg4,
                            InsnOperand::Reg5 => self.reg5,
                            InsnOperand::Reg6 => self.reg6,
                            InsnOperand::Reg7 => self.reg7,
                        };

                        match arg1 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal div instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get_mut(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => *v += src,
                            },
                            InsnOperand::Reg0 => self.reg0 /= src,
                            InsnOperand::Reg1 => self.reg1 /= src,
                            InsnOperand::Reg2 => self.reg2 /= src,
                            InsnOperand::Reg3 => self.reg3 /= src,
                            InsnOperand::Reg4 => self.reg4 /= src,
                            InsnOperand::Reg5 => self.reg5 /= src,
                            InsnOperand::Reg6 => self.reg6 /= src,
                            InsnOperand::Reg7 => self.reg7 /= src,
                        }
                    }
                },
            },
            InsnOpcode::Push => match insn.operands.get(0) {
                None => return Err(format!("Illegal push instruction: {:?}", insn)),
                Some(arg1) => match arg1 {
                    InsnOperand::Imm(n) => self.stack.push(n.clone()),
                    InsnOperand::Stack(_) => {
                        return Err(format!("Illegal push instruction: {:?}", insn))
                    }
                    InsnOperand::Reg0 => self.stack.push(self.reg0),
                    InsnOperand::Reg1 => self.stack.push(self.reg1),
                    InsnOperand::Reg2 => self.stack.push(self.reg2),
                    InsnOperand::Reg3 => self.stack.push(self.reg3),
                    InsnOperand::Reg4 => self.stack.push(self.reg4),
                    InsnOperand::Reg5 => self.stack.push(self.reg5),
                    InsnOperand::Reg6 => self.stack.push(self.reg6),
                    InsnOperand::Reg7 => self.stack.push(self.reg7),
                },
            },
            InsnOpcode::Pop => match insn.operands.get(0) {
                None => return Err(format!("Illegal pop instruction: {:?}", insn)),
                Some(arg1) => match self.stack.pop() {
                    None => return Err(format!("Illegal pop instruction: {:?}", insn)),
                    Some(val) => match arg1 {
                        InsnOperand::Imm(_) | InsnOperand::Stack(_) => {
                            return Err(format!("Illegal pop instruction: {:?}", insn))
                        }
                        InsnOperand::Reg0 => self.reg0 = val,
                        InsnOperand::Reg1 => self.reg1 = val,
                        InsnOperand::Reg2 => self.reg2 = val,
                        InsnOperand::Reg3 => self.reg3 = val,
                        InsnOperand::Reg4 => self.reg4 = val,
                        InsnOperand::Reg5 => self.reg5 = val,
                        InsnOperand::Reg6 => self.reg6 = val,
                        InsnOperand::Reg7 => self.reg7 = val,
                    },
                },
            },
            InsnOpcode::Copy => match insn.operands.get(0) {
                None => return Err(format!("Illegal copy instruction: {:?}", insn)),
                Some(arg1) => match insn.operands.get(1) {
                    None => return Err(format!("Illegal copy instruction: {:?}", insn)),
                    Some(arg2) => {
                        let src: i32 = match arg2 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal copy instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => v.clone(),
                            },
                            InsnOperand::Reg0 => self.reg0,
                            InsnOperand::Reg1 => self.reg1,
                            InsnOperand::Reg2 => self.reg2,
                            InsnOperand::Reg3 => self.reg3,
                            InsnOperand::Reg4 => self.reg4,
                            InsnOperand::Reg5 => self.reg5,
                            InsnOperand::Reg6 => self.reg6,
                            InsnOperand::Reg7 => self.reg7,
                        };

                        match arg1 {
                            InsnOperand::Imm(_) => {
                                return Err(format!("Illegal copy instruction: {:?}", insn))
                            }
                            InsnOperand::Stack(n) => match self.stack.get_mut(n.clone()) {
                                None => {
                                    return Err(format!(
                                        "Stack index out of bounds: {} {:?}",
                                        n, insn
                                    ));
                                }
                                Some(v) => *v += src,
                            },
                            InsnOperand::Reg0 => self.reg0 = src,
                            InsnOperand::Reg1 => self.reg1 = src,
                            InsnOperand::Reg2 => self.reg2 = src,
                            InsnOperand::Reg3 => self.reg3 = src,
                            InsnOperand::Reg4 => self.reg4 = src,
                            InsnOperand::Reg5 => self.reg5 = src,
                            InsnOperand::Reg6 => self.reg6 = src,
                            InsnOperand::Reg7 => self.reg7 = src,
                        }
                    }
                },
            },
        }
        self.i += 1;

        Ok(())
    }
    pub fn debug(&self) -> StateDebug<'_> {
        StateDebug(self)
    }
}

pub struct StateDebug<'a>(&'a State);

impl fmt::Debug for StateDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("i", &self.0.i)
            .field("reg0", &self.0.reg0)
            .field("reg1", &self.0.reg1)
            .field("reg2", &self.0.reg2)
            .field("reg3", &self.0.reg3)
            .field("reg4", &self.0.reg4)
            .field("reg5", &self.0.reg5)
            .field("reg6", &self.0.reg6)
            .field("reg7", &self.0.reg7)
            .field("stack", &self.0.stack)
            .finish()
    }
}

pub fn run(instructions: Vec<Instruction>) -> Result<State, String> {
    let mut state = State {
        i: 0,
        instructions,
        reg0: 0,
        reg1: 0,
        reg2: 0,
        reg3: 0,
        reg4: 0,
        reg5: 0,
        reg6: 0,
        reg7: 0,
        stack: vec![],
    };

    while state.i < state.instructions.len() {
        match state.step() {
            Ok(()) => {}
            Err(msg) => return Err(msg),
        }
        println!("step: {:#?}", state.debug());
    }

    Ok(state)
}
