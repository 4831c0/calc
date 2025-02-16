use crate::token::{Opcode, Token};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: Option<Box<T>>,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

pub fn tokens_to_tree(tokens: Vec<Token>) -> Result<Node<Token>, String> {
    let mut tree = Node {
        value: None,
        left: None,
        right: None,
    };

    for token in tokens {
        match token.op {
            Opcode::Const => {
                if tree.left.is_none() {
                    tree.left = Some(Box::new(Node {
                        value: Some(Box::new(token)),
                        left: None,
                        right: None,
                    }));
                } else if tree.right.is_none() {
                    tree.right = Some(Box::new(Node {
                        value: Some(Box::new(token)),
                        left: None,
                        right: None,
                    }));
                } else {
                    let mut right: &mut Node<Token> = &mut tree;
                    loop {
                        match right.right.as_ref() {
                            None => {
                                right.right = Some(Box::new(Node {
                                    value: Some(Box::new(token)),
                                    left: None,
                                    right: None,
                                }));

                                break;
                            }
                            Some(right_right) => match right_right.value.as_ref() {
                                None => {
                                    return Err(format!("No value for {:#?}", right_right.value))
                                }
                                Some(right_token) => {
                                    if right_token.op == Opcode::Operand {
                                        right = right.right.as_mut().unwrap();
                                    }
                                }
                            },
                        }
                    }
                }
            }
            Opcode::Operand => {
                if tree.value.is_none() {
                    tree.value = Some(Box::new(token));
                } else {
                    let old_right = tree.right;
                    tree.right = Some(Box::new(Node {
                        value: Some(Box::new(token)),
                        left: old_right,
                        right: None,
                    }));
                }
            }
        }
    }

    Ok(tree)
}
