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

impl Node<Token> {
    pub fn convert_dot(&self) -> String {
        let mut buff = String::from("graph G {\n    n0 [shape=Mdiamond];\n    n0 [label=\"start\"];\n");
        let mut last_id = 0;

        self.convert_dot_inner(&mut buff, &mut last_id, 0);

        buff.push_str("}\n");
        buff
    }

    fn convert_dot_inner(&self, buff: &mut String, last_id: &mut i32, id_above: i32) {
        let my_id = *last_id + 1;
        *last_id += 1;

        buff.push_str(&format!("    n{} -- n{}; \n", id_above, my_id));
        buff.push_str(&format!("    n{} [label=\"{:?}\"];\n", my_id, self.value.as_ref().unwrap().or));

        if let Some(right) = &self.right {
            right.convert_dot_inner(buff, last_id, my_id);
        }
        if let Some(left) = &self.left {
            left.convert_dot_inner(buff, last_id, my_id);
        }
    }
}