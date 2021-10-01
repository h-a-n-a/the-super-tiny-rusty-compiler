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
struct LispCallExpressionNode {
    name: String,
    params: Vec<LispNode>,
}

#[derive(Debug)]
struct LispProgramNode {
    body: Vec<LispNode>,
}

#[derive(Debug)]
struct LispNumberLiteralNode {
    value: String,
}

#[derive(Debug)]
enum LispNode {
    Program(LispProgramNode),
    CallExpression(LispCallExpressionNode),
    NumberLiteral(LispNumberLiteralNode),
}

impl LispNode {
    fn new_program(body: Vec<LispNode>) -> LispNode {
        LispNode::Program(LispProgramNode { body })
    }
    fn new_call_expression(name: String, params: Vec<LispNode>) -> LispNode {
        LispNode::CallExpression(LispCallExpressionNode { name, params })
    }
    fn new_number_literal(value: String) -> LispNode {
        LispNode::NumberLiteral(LispNumberLiteralNode { value })
    }
}

/*
 * For our parser we're going to take our array of tokens and turn it into an
 * AST.
 *
 *   [{ type: 'paren', value: '(' }, ...]   =>   { type: 'Program', body: [...] }
 */
fn parser(tokens: &Vec<Token>) -> LispNode {
    let mut current: usize = 0;

    fn walk(current: &mut usize, tokens: &Vec<Token>) -> LispNode {
        let token = &tokens[*current];

        return match token {
            Token::ParenOpen => {
                *current += 1;
                let token = &tokens[*current];

                if let Token::Name(name) = token {
                    let mut params: Vec<LispNode> = vec![];

                    *current += 1;
                    let mut token = &tokens[*current];

                    while *token != Token::ParenClose {
                        params.push(walk(current, &tokens));
                        token = &tokens[*current];
                    }

                    // encounter `Token::ParenClose`, skip it.
                    *current += 1;

                    return LispNode::new_call_expression(name.to_owned(), params);
                }

                panic!("[parser] unexpected token: {:?}", token)
            }
            Token::Number(number) => {
                *current += 1;
                LispNode::new_number_literal(number.to_owned())
            }
            _ => panic!("[parser]: unexpected token: {:?}", token),
        };
    }

    let mut program_body: Vec<LispNode> = vec![];

    while current < tokens.len() {
        program_body.push(walk(&mut current, &tokens));
    }

    LispNode::new_program(program_body)
}

trait Visitor {
    fn enter_program(&self, program: &LispProgramNode);
    fn exit_program(&self, program: &LispProgramNode);

    fn enter_number_literal(&self, node: &LispNumberLiteralNode, parent: &LispNode);
    fn exit_number_literal(&self, node: &LispNumberLiteralNode, parent: &LispNode);

    fn enter_call_expression(&self, node: &LispCallExpressionNode, parent: &LispNode);
    fn exit_call_expression(&self, node: &LispCallExpressionNode, parent: &LispNode);
}

fn traverser<T>(ast: &LispNode, visitor: &T)
where
    T: Visitor,
{
    fn traverse_node<T>(node: &LispNode, parent: &LispNode, visitor: &T)
    where
        T: Visitor,
    {
        match node {
            LispNode::CallExpression(current) => {
                visitor.enter_call_expression(current, parent);
                traverse_nodes(&current.params, parent, visitor);
                visitor.exit_call_expression(current, parent);
            }
            LispNode::NumberLiteral(current) => {
                visitor.enter_number_literal(current, parent);
                visitor.exit_number_literal(current, parent);
            }
            _ => panic!("[traverser]: unexpected node type: {:?}", node),
        }
    }

    fn traverse_nodes<T>(nodes: &Vec<LispNode>, parent: &LispNode, visitor: &T)
    where
        T: Visitor,
    {
        for node in nodes {
            traverse_node(node, parent, visitor)
        }
    }

    if let LispNode::Program(node) = ast {
        visitor.enter_program(node);
        traverse_nodes(&node.body, ast, visitor);
        visitor.exit_program(node);
    } else {
        panic!("[traverser]: unexpected `program` type: {:?}", ast);
    }
}

fn transformer(ast: &LispNode) {}

fn main() {
    let lisp_input = "(add 2 (subtract 4 2))";

    let tokens = tokenizer(lisp_input);
    let ast = parser(&tokens);
    println!("{:#?}", ast)
}
