#[derive(Debug, PartialEq)]
enum Token {
    ParenOpen,
    ParenClose,
    Name(String),
    Number(String),
}

fn tokenizer(input: &str) -> Vec<Token> {
    let mut current: usize = 0;
    let mut tokens: Vec<Token> = vec![];

    let chars = input.chars().collect::<Vec<char>>();

    while current < chars.len() {
        if let Some(&c) = chars.get(current) {
            match c {
                '(' => tokens.push(Token::ParenOpen),
                ')' => tokens.push(Token::ParenClose),
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

                    tokens.push(Token::Number(value));
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

                    tokens.push(Token::Name(value))
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

#[derive(Debug)]
struct CallExpressionNode {
    name: String,
    params: Vec<Node>,
}

#[derive(Debug)]
struct ProgramNode {
    body: Vec<Node>,
}

#[derive(Debug)]
struct NumberLiteralNode {
    value: String,
}

#[derive(Debug)]
enum Node {
    Program(ProgramNode),
    CallExpression(CallExpressionNode),
    NumberLiteral(NumberLiteralNode),
}

impl Node {
    fn new_program(body: Vec<Node>) -> Node {
        Node::Program(ProgramNode { body })
    }
    fn new_call_expression(name: String, params: Vec<Node>) -> Node {
        Node::CallExpression(CallExpressionNode { name, params })
    }
    fn new_number_literal(value: String) -> Node {
        Node::NumberLiteral(NumberLiteralNode { value })
    }
}

/*
 * For our parser we're going to take our array of tokens and turn it into an
 * AST.
 *
 *   [{ type: 'paren', value: '(' }, ...]   =>   { type: 'Program', body: [...] }
 */
fn parser(tokens: &Vec<Token>) -> Node {
    let mut current: usize = 0;

    fn walk(current: &mut usize, tokens: &Vec<Token>) -> Node {
        let token = &tokens[*current];

        return match token {
            Token::ParenOpen => {
                *current += 1;
                let token = &tokens[*current];

                if let Token::Name(name) = token {
                    let mut params: Vec<Node> = vec![];

                    *current += 1;
                    let mut token = &tokens[*current];

                    while *token != Token::ParenClose {
                        params.push(walk(current, &tokens));
                        token = &tokens[*current];
                    }

                    // encounter `Token::ParenClose`, skip it.
                    *current += 1;

                    return Node::new_call_expression(name.to_owned(), params);
                }

                panic!("[parser] unexpected token: {:?}", token)
            }
            Token::Number(number) => {
                *current += 1;
                Node::new_number_literal(number.to_owned())
            }
            _ => panic!("[parser]: unexpected token: {:?}", token),
        };
    }

    let mut program_body: Vec<Node> = vec![];

    while current < tokens.len() {
        program_body.push(walk(&mut current, &tokens));
    }

    Node::new_program(program_body)
}

// trait Visitor {
//     fn enter_program(&self, program: &Program);
//     fn exit_program(&self, program: &Program);
//
//     fn enter_number_literal(&self, node: &Node, parent: Option<&Node>);
//     fn exit_number_literal(&self, node: &Node, parent: Option<&Node>);
//
//     fn enter_call_expression(&self, node: &Node, parent: Option<&Node>);
//     fn exit_call_expression(&self, node: &Node, parent: Option<&Node>);
// }
//
// fn traverser<T>(ast: &Program, visitor: &T)
// where
//     T: Visitor,
// {
//     fn traverseNode<T>(node: &Node, parent: &Node, visitor: &T)
//     where
//         T: Visitor,
//     {
//         match node.kind {
//             NodeKind::Program => {}
//             NodeKind::NumberLiteral => {
//                 visitor.enter_number_literal(node, Some(parent));
//                 visitor.exit_number_literal(node, Some(parent));
//             }
//             NodeKind::CallExpression => {
//                 visitor.enter_call_expression(node, Some(parent));
//
//                 // traverseNodeArray();
//
//                 visitor.exit_call_expression(node, Some(parent));
//             }
//         }
//     }
//
//     fn traverseNodeArray<T>(nodes: &Vec<Node>, parent: Some(&Node), visitor: &T)
//     where
//         T: Visitor,
//     {
//         for node in nodes {
//             traverseNode(node, parent, visitor)
//         }
//     }
//
//     fn traverseProgram<T>(program: &Program, visitor: &T)
//     where
//         T: Visitor,
//     {
//         visitor.enter_program(program);
//         traverseNodeArray(*program.body, None, visitor);
//         visitor.exit_program(program);
//     }
//
//     traverseProgram(ast, visitor);
// }

fn main() {
    let lisp_input = "(add 2 (subtract 4 2))";

    let tokens = tokenizer(lisp_input);
    let ast = parser(&tokens);
    println!("{:#?}", ast)
}
