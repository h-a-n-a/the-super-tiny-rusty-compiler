use std::cell::RefCell;

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
        if let Some(&c) = chars.get(current) {
            match c {
                '(' | ')' => {
                    tokens.push(Token {
                        kind: TokenKind::Paren,
                        value: c.to_string(),
                    });
                }
                '0'..='9' => {
                    let mut value = c.to_string();

                    while let Some(&next_char) = chars.get(current + 1) {
                        match next_char {
                            '0'..='9' => {
                                value = format!("{}{}", value, next_char);
                                current += 1;
                            }
                            _ => break,
                        }
                    }

                    tokens.push(Token {
                        kind: TokenKind::Number,
                        value,
                    });
                }
                ' ' => {
                    // skip whitespaces
                }
                'a'..='z' | 'A'..='Z' => {
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
                _ => {
                    panic!("[tokenizer]: unexpected token {}", c)
                }
            }
        }

        current += 1;
    }

    tokens
}

#[derive(Debug, Clone, PartialEq)]
enum NodeKind {
    Program,
    CallExpression,
    NumberLiteral,
}

#[derive(Debug)]
struct Program {
    kind: NodeKind,
    body: RefCell<Vec<RefCell<Node>>>,
}

#[derive(Debug, Clone)]
struct Node {
    kind: NodeKind,
    name: Option<String>,
    value: Option<String>,
    params: Option<RefCell<Vec<RefCell<Node>>>>,
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

    let mut current = 0usize;

    fn walk(current: &mut usize, tokens: &Vec<Token>) -> RefCell<Node> {
        let token = &tokens[*current];

        return match token.kind {
            TokenKind::Paren => {
                if token.value == "(".to_string() {
                    *current += 1;
                    let name_token = &tokens[*current];

                    let mut params: Vec<RefCell<Node>> = vec![];

                    *current += 1;
                    let mut current_token = &tokens[*current];

                    while (current_token.kind != TokenKind::Paren)
                        || (current_token.kind == TokenKind::Paren
                            && current_token.value != ")".to_string())
                    {
                        params.push(walk(current, &tokens));
                        current_token = &tokens[*current];
                    }

                    *current += 1;

                    return RefCell::new(Node {
                        kind: NodeKind::CallExpression,
                        name: Some(name_token.value.to_owned()),
                        value: None,
                        params: Some(RefCell::new(params)),
                    });
                }

                panic!("[parser]: unmatched token {}", token.value);
            }
            TokenKind::Name => {
                *current += 1;
                RefCell::new(Node {
                    kind: NodeKind::NumberLiteral,
                    name: None,
                    value: Some(token.value.to_owned()),
                    params: None,
                })
            }
            TokenKind::Number => {
                *current += 1;
                RefCell::new(Node {
                    kind: NodeKind::NumberLiteral,
                    name: None,
                    value: Some(token.value.to_owned()),
                    params: None,
                })
            }
        };
    }

    while current < tokens.len() {
        ast.body.borrow_mut().push(walk(&mut current, &tokens));
    }

    // let mut stack: Vec<Rc<RefCell<Node>>> = vec![];
    //
    // for Token { kind, value } in tokens.iter() {
    //     match *kind {
    //         TokenKind::Paren => {
    //             match value.as_str() {
    //                 "(" => {
    //                     let node = Rc::new(RefCell::new(Node {
    //                         kind: NodeKind::CallExpression,
    //                         name: Some(String::default()),
    //                         value: None,
    //                         params: Some(RefCell::new(vec![])),
    //                     }));
    //
    //                     if stack.len() == 0 {
    //                         ast.body.borrow_mut().push(Rc::clone(&node));
    //                     } else {
    //                         if let Some(last) = stack.last() {
    //                             if let Some(ref params) = last.borrow_mut().params {
    //                                 params.borrow_mut().push(Rc::clone(&node))
    //                             }
    //                         }
    //                     }
    //
    //                     stack.push(Rc::clone(&node));
    //                 }
    //                 ")" => {
    //                     if stack.len() == 0 {
    //                         panic!("[parser] grammar error");
    //                     }
    //                     stack.pop();
    //                 }
    //                 _ => (),
    //             };
    //         }
    //         TokenKind::Name => {
    //             if let Some(last) = stack.last() {
    //                 last.borrow_mut().name = Some(value.to_string());
    //             }
    //         }
    //         TokenKind::Number => {
    //             if let Some(last) = stack.last() {
    //                 let node = Rc::new(RefCell::new(Node {
    //                     kind: NodeKind::NumberLiteral,
    //                     value: Some(value.to_string()),
    //                     name: None,
    //                     params: None,
    //                 }));
    //                 if let Some(ref params) = last.borrow_mut().params {
    //                     params.borrow_mut().push(Rc::clone(&node));
    //                 }
    //             }
    //         }
    //     }
    // }

    ast
}

fn main() {
    let lisp_input = "(add 2 (subtract 4 2))";

    let tokens = tokenizer(lisp_input);
    let ast = parser(&tokens);
    println!("{:#?}", ast)
}
