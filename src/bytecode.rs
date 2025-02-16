use crate::bytecode::InsnOperand::{Reg0, Reg1, Reg2, Reg3, Reg4, Reg5, Reg6, Reg7};
use crate::token::{Opcode, Operand, Token};
use crate::tree::Node;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InsnOpcode {
    // data
    Ldc,
    Push,
    Pop,
    Copy,

    // math
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InsnOperand {
    Imm(i32),
    Stack(usize),
    Reg0,
    Reg1,
    Reg2,
    Reg3,
    Reg4,
    Reg5,
    Reg6,
    Reg7,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: InsnOpcode,
    pub operands: Vec<InsnOperand>,
}

fn operand_to_insn_operand(opcode: Operand) -> Result<InsnOperand, &'static str> {
    match opcode {
        Operand::Int(x) => Ok(InsnOperand::Imm(x)),
        _ => Err("Invalid operand"),
    }
}

fn operand_to_insn_opcode(opcode: Operand) -> Result<InsnOpcode, &'static str> {
    match opcode {
        Operand::Add => Ok(InsnOpcode::Add),
        Operand::Sub => Ok(InsnOpcode::Sub),
        Operand::Mul => Ok(InsnOpcode::Mul),
        Operand::Div => Ok(InsnOpcode::Div),
        _ => Err("Invalid operand"),
    }
}

fn handle_node(
    reg_alloc: &mut RegisterAllocation,
    operand: Operand,
    left_reg: InsnOperand,
    right_reg: InsnOperand,
    node: Node<Token>,
) -> Result<Vec<Instruction>, &'static str> {
    let mut insns: Vec<Instruction> = Vec::new();
    let mut recursed = false;
    // the next register that is going to be used
    let mut next_reg = InsnOperand::Imm(-1);
    // the last register used inside node_to_instructions
    let mut recursed_last_reg = InsnOperand::Imm(-1);

    match &node.left {
        None => return Err("No left hand side value for instruction"),
        Some(left_node) => match &left_node.value {
            None => return Err("No value in left hand side of instruction"),
            Some(left) => match left.op {
                Opcode::Const => match operand_to_insn_operand(left.or) {
                    Ok(val) => {
                        insns.push(Instruction {
                            opcode: InsnOpcode::Ldc,
                            operands: vec![left_reg, val],
                        });
                    }
                    Err(err) => return Err(err),
                },
                Opcode::Operand => match node_to_instructions(reg_alloc, (**left_node).clone()) {
                    Err(err) => return Err(err),
                    Ok(insns2) => {
                        println!("recursed insns: {:?}", insns2);
                        for insn in insns2 {
                            match insn.clone().operands.get(0) {
                                None => {}
                                Some(reg) => recursed_last_reg = reg.clone(),
                            }
                            insns.push(insn);
                        }
                        recursed = true;
                        next_reg = left_reg;
                    }
                },
            },
        },
    }

    if recursed {
        insns.push(Instruction {
            opcode: InsnOpcode::Copy,
            operands: vec![next_reg, recursed_last_reg],
        });
    }

    match handle_right(reg_alloc, operand, left_reg, right_reg, node) {
        Err(err) => return Err(err),
        Ok(insns2) => {
            for insn in insns2 {
                insns.push(insn);
            }
        }
    }

    Ok(insns)
}

fn handle_right(
    reg_alloc: &mut RegisterAllocation,
    operand: Operand,
    left_reg: InsnOperand,
    right_reg: InsnOperand,
    node: Node<Token>,
) -> Result<Vec<Instruction>, &'static str> {
    let mut insns: Vec<Instruction> = Vec::new();

    match &node.right {
        None => return Err("No right hand side value for instruction"),
        Some(right_node) => match &right_node.value {
            None => return Err("No value in right hand side of instruction"),
            Some(right) => match right.op {
                Opcode::Const => match operand_to_insn_operand(right.or) {
                    Ok(val) => {
                        insns.push(Instruction {
                            opcode: InsnOpcode::Ldc,
                            operands: vec![right_reg, val],
                        });

                        println!("{:?}", &node);
                        match operand_to_insn_opcode(operand) {
                            Err(err) => return Err(err),
                            Ok(op) => insns.push(Instruction {
                                opcode: op,
                                operands: vec![left_reg, right_reg],
                            }),
                        }
                    }
                    Err(err) => return Err(err),
                },

                Opcode::Operand => {
                    let i = (**right_node).clone();
                    match node_to_instructions(reg_alloc, i) {
                        Err(err) => return Err(err),
                        Ok(insns2) => {
                            for insn in insns2 {
                                insns.push(insn);
                            }
                        }
                    }

                    match insns.get(insns.len() - 1) {
                        None => return Err("node_to_instructions didn't emit any instructions!"),
                        Some(last_insn) => match last_insn.operands.get(0) {
                            None => return Err("last_insn doesn't have any operands"),
                            Some(operand) => insns.push(Instruction {
                                opcode: InsnOpcode::Copy,
                                operands: vec![right_reg, operand.clone()],
                            }),
                        },
                    }

                    match operand_to_insn_opcode(operand) {
                        Err(err) => return Err(err),
                        Ok(op) => {
                            insns.push(Instruction {
                                opcode: op,
                                operands: vec![left_reg, right_reg],
                            });
                        }
                    }
                }
            },
        },
    }

    Ok(insns)
}

fn node_to_instructions(
    reg_alloc: &mut RegisterAllocation,
    node: Node<Token>,
) -> Result<Vec<Instruction>, &'static str> {
    let mut insns: Vec<Instruction> = Vec::new();

    match &node.value {
        None => {
            return Ok(insns);
        }
        Some(token) => match token.or {
            Operand::Add | Operand::Sub | Operand::Mul | Operand::Div => {
                let reg1;
                let reg2;

                if reg_alloc.registers.len() > 1 {
                    reg1 = reg_alloc.registers.swap_remove(0);
                    reg2 = reg_alloc.registers.swap_remove(0);
                } else if reg_alloc.registers.len() > 0 {
                    reg1 = reg_alloc.registers.swap_remove(0);
                    reg2 = InsnOperand::Stack(reg_alloc.stack_index);
                    reg_alloc.stack_index += 1;
                } else {
                    reg1 = InsnOperand::Stack(reg_alloc.stack_index);
                    reg_alloc.stack_index += 1;
                    reg2 = InsnOperand::Stack(reg_alloc.stack_index);
                    reg_alloc.stack_index += 1;
                }

                match handle_node(reg_alloc, token.or, reg1, reg2, node) {
                    Ok(insns2) => {
                        for insn in insns2 {
                            insns.push(insn);
                        }
                    }
                    Err(err) => return Err(err),
                }
            }
            _ => {
                return Err("Unexpected token");
            }
        },
    }

    Ok(insns)
}

struct RegisterAllocation {
    registers: Vec<InsnOperand>,
    stack_index: usize,
}

pub fn tree_to_instructions(tree: Node<Token>) -> Result<Vec<Instruction>, &'static str> {
    let mut insns: Vec<Instruction> = Vec::new();
    let mut reg_alloc = RegisterAllocation {
        registers: vec![Reg0, Reg1, Reg2, Reg3, Reg4, Reg5, Reg6, Reg7],
        stack_index: 0,
    };

    match node_to_instructions(&mut reg_alloc, tree) {
        Err(err) => return Err(err),
        Ok(instructions) => {
            for insn in instructions {
                insns.push(insn)
            }
        }
    }

    for _ in 0..reg_alloc.stack_index {
        insns.insert(
            0,
            Instruction {
                opcode: InsnOpcode::Push,
                operands: vec![InsnOperand::Imm(0)],
            },
        );
    }

    Ok(insns)
}
