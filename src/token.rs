use std::fs::File;
use std::io::Read;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
    Const,
    Operand
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
    Int(i32),
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
    pub op: Opcode,
    pub or: Operand,
}

pub fn tokenize(mut file: File) -> Result<Vec<Token>, &'static str> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: Vec<u8> = vec![0; 1];

    let mut res = file.read_exact(&mut buf);
    let mut num_buff = String::new();
    let mut reading_num = false;
    while res.is_ok() {
        let c = buf[0] as char;
        if (reading_num && (c < '0' || c > '9')) {
            reading_num = false;

            match num_buff.parse::<i32>() {
                Ok(num) => {
                    tokens.push(Token{
                        op: Opcode::Const,
                        or: Operand::Int(num),
                    });
                }
                Err(_) => {
                    return Err("Could not parse number");
                }
            }

            num_buff = String::from("");
        }
        match c {
            '+' => {
                tokens.push(Token{
                    op: Opcode::Operand,
                    or: Operand::Add,
                });
            },
            '-' => {
                tokens.push(Token{
                    op: Opcode::Operand,
                    or: Operand::Sub,
                });
            },
            '*' => {
                tokens.push(Token{
                    op: Opcode::Operand,
                    or: Operand::Mul,
                });
            },
            '/' => {
                tokens.push(Token{
                    op: Opcode::Operand,
                    or: Operand::Div,
                });
            },
            '0'..'9' => {
                num_buff.push(c);
                reading_num = true;
            },
            _ => {
                panic!("Unexpected character: {}", c)
            },
        }

        res = file.read_exact(&mut buf);
    }

    if reading_num {
        match num_buff.parse::<i32>() {
            Ok(num) => {
                tokens.push(Token{
                    op: Opcode::Const,
                    or: Operand::Int(num),
                });
            }
            Err(_) => {
                return Err("Could not parse number");
            }
        }
    }

    Ok(tokens)
}