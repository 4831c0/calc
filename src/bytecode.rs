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
    Int(i32),
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
        Operand::Int(x) => Ok(InsnOperand::Int(x)),
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
    registers: &mut Vec<InsnOperand>,
    operand: Operand,
    left_reg: InsnOperand,
    right_reg: InsnOperand,
    node: Node<Token>,
) -> Result<Vec<Instruction>, &'static str> {
    let mut insns: Vec<Instruction> = Vec::new();
    let mut recursed = false;
    // the next register that is going to be used
    let mut next_reg = InsnOperand::Int(-1);
    // the last register used inside node_to_instructions
    let mut recursed_last_reg = InsnOperand::Int(-1);

    match &node.left {
        None => return Err("No left hand side value for instruction"),
        Some(left_node) => match &left_node.value {
            None => return Err("No value in left hand side of instruction"),
            Some(left) => match left.op {
                Opcode::Const => match operand_to_insn_operand(left.or) {
                    Ok(val) => {
                        if left_reg == InsnOperand::Int(-1) {
                            // use the stack
                            insns.push(Instruction {
                                opcode: InsnOpcode::Push,
                                operands: vec![Reg0],
                            })
                        } else {
                            // use the register
                            insns.push(Instruction {
                                opcode: InsnOpcode::Ldc,
                                operands: vec![left_reg, val],
                            });
                        }
                    }
                    Err(err) => return Err(err),
                },
                Opcode::Operand => match node_to_instructions(registers, (**left_node).clone()) {
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

    match handle_right(registers, operand, left_reg, right_reg, node) {
        Err(err) => return Err(err),
        Ok(insns2) => {
            for insn in insns2 {
                insns.push(insn);
            }
        }
    }

    if left_reg == InsnOperand::Int(-1) {
        insns.push(Instruction {
            opcode: InsnOpcode::Pop,
            operands: vec![Reg0],
        })
    }

    Ok(insns)
}

fn handle_right(
    registers: &mut Vec<InsnOperand>,
    operand: Operand,
    left_reg: InsnOperand,
    right_reg: InsnOperand,
    node: Node<Token>,
) -> Result<Vec<Instruction>, &'static str> {
    let mut insns: Vec<Instruction> = Vec::new();

    let mut exec_reg_left = left_reg;
    let mut exec_reg_right = right_reg;

    if exec_reg_left == InsnOperand::Int(-1) {
        exec_reg_left = Reg0;
    }
    if exec_reg_right == InsnOperand::Int(-1) {
        exec_reg_right = Reg1;
    }

    match &node.right {
        None => return Err("No right hand side value for instruction"),
        Some(right_node) => match &right_node.value {
            None => return Err("No value in right hand side of instruction"),
            Some(right) => match right.op {
                Opcode::Const => match operand_to_insn_operand(right.or) {
                    Ok(val) => {
                        if right_reg == InsnOperand::Int(-1) {
                            // use the stack
                            insns.push(Instruction {
                                opcode: InsnOpcode::Push,
                                operands: vec![Reg1],
                            });
                        } else {
                            // use the register
                            insns.push(Instruction {
                                opcode: InsnOpcode::Ldc,
                                operands: vec![right_reg, val],
                            });
                        }

                        println!("{:?}", &node);
                        match operand_to_insn_opcode(operand) {
                            Err(err) => return Err(err),
                            Ok(op) => insns.push(Instruction {
                                opcode: op,
                                operands: vec![exec_reg_left, exec_reg_right],
                            }),
                        }
                    }
                    Err(err) => return Err(err),
                },

                Opcode::Operand => {
                    let i = (**right_node).clone();
                    match node_to_instructions(registers, i) {
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
                                operands: vec![exec_reg_right, operand.clone()],
                            }),
                        },
                    }

                    match operand_to_insn_opcode(operand) {
                        Err(err) => return Err(err),
                        Ok(op) => {
                            insns.push(Instruction {
                                opcode: op,
                                operands: vec![exec_reg_left, exec_reg_right],
                            });
                        }
                    }
                }
            },
        },
    }

    if right_reg == InsnOperand::Int(-1) {
        insns.push(Instruction {
            opcode: InsnOpcode::Pop,
            operands: vec![Reg1],
        });
    }

    Ok(insns)
}

fn node_to_instructions(
    registers: &mut Vec<InsnOperand>,
    node: Node<Token>,
) -> Result<Vec<Instruction>, &'static str> {
    let mut insns: Vec<Instruction> = Vec::new();

    match &node.value {
        None => {
            return Ok(insns);
        }
        Some(token) => match token.or {
            Operand::Add | Operand::Sub | Operand::Mul | Operand::Div => {
                let mut reg1 = InsnOperand::Int(-1);
                let mut reg2 = InsnOperand::Int(-1);
                if registers.len() > 1 {
                    reg1 = registers.swap_remove(0);
                    reg2 = registers.swap_remove(0);
                } else if registers.len() > 0 {
                    reg1 = registers.swap_remove(0);
                }

                match handle_node(registers, token.or, reg1, reg2, node) {
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

pub fn tree_to_instructions(tree: Node<Token>) -> Result<Vec<Instruction>, &'static str> {
    let mut insns: Vec<Instruction> = Vec::new();
    let mut registers = vec![Reg0, Reg1, Reg2, Reg3, Reg4, Reg5, Reg6, Reg7];

    match node_to_instructions(&mut registers, tree) {
        Err(err) => return Err(err),
        Ok(instructions) => {
            for insn in instructions {
                insns.push(insn)
            }
        }
    }

    Ok(insns)
}
