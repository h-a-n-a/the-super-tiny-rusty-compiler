use std::cell::{Cell, RefCell};

#[derive(Debug, PartialEq)]
enum TokenKind {
    Paren,
    Name,
    Number,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    value: String,
}

fn tokenizer(input: &str) -> Vec<Token> {
    println!("{}", input);

    let mut current: usize = 0;
    let mut tokens: Vec<Token> = vec![];

    let chars = input.chars().collect::<Vec<char>>();

    while current < chars.len() {
        if let Some(c) = chars.get(current) {
            match c {
                '(' | ')' => {
                    tokens.push(Token {
                        kind: TokenKind::Paren,
                        value: c.to_string(),
                    });
                }
                '0'..='9' => {
                    tokens.push(Token {
                        kind: TokenKind::Number,
                        value: c.to_string(),
                    });
                }
                _ => {
                    if *c == ' ' {
                        current += 1;
                        continue;
                    }

                    let mut value = c.to_string();
                    while let Some(&next_char) = chars.get(current + 1) {
                        match next_char {
                            'a'..='z' | 'A'..='Z' => {
                                value = format!("{}{}", value, next_char);
                                current += 1;
                            }
                            _ => break,
                        }
                    }

                    tokens.push(Token {
                        kind: TokenKind::Name,
                        value,
                    })
                }
            }
        }

        current += 1;
    }

    tokens
}

#[derive(Debug, Clone)]
enum NodeKind {
    Program,
    CallExpression,
    NumberLiteral,
}

#[derive(Debug)]
struct Program {
    kind: NodeKind,
    body: RefCell<Vec<Node>>,
}

#[derive(Debug, Clone)]
struct Node {
    kind: NodeKind,
    name: String,
    params: Vec<Box<Node>>,
}

/*
 * For our parser we're going to take our array of tokens and turn it into an
 * AST.
 *
 *   [{ type: 'paren', value: '(' }, ...]   =>   { type: 'Program', body: [...] }
 */
fn parser(tokens: &Vec<Token>) -> Program {
    let ast = Program {
        kind: NodeKind::Program,
        body: RefCell::new(vec![]),
    };

    let ref mut working_node: Option<&Node> = None;

    // for (index, token) in tokens.iter().enumerate() {
    //     if token.kind == TokenKind::Paren && token.value == '('.to_string() {
    //         let node = Node {
    //             kind: NodeKind::CallExpression,
    //             name: token.value.to_owned(),
    //             params: &mut vec![],
    //         };
    //
    //         // *working_node = ;
    //     }
    // }

    // let ref mut ref_mut = *ast.body.borrow_mut();
    //
    // *ref_mut = vec![Node {
    //     kind: NodeKind::CallExpression,
    //     name: "a".to_string(),
    //     params: vec![],
    // }]
    // .to_owned();
    //
    // println!("{:?}", tokens);

    ast
}

fn main() {
    let lisp_input = "(add 2 (subtract 4 2))";

    let tokens = tokenizer(lisp_input);
    let ast = parser(&tokens);
    println!("{:?}", ast)
}
