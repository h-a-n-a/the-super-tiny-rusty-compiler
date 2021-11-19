use std::cell::RefCell;
use std::rc::Rc;

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
    fn enter_program(&mut self, program: &LispProgramNode);
    fn exit_program(&mut self, program: &LispProgramNode);

    fn enter_number_literal(&mut self, node: &LispNumberLiteralNode, parent: &LispNode);
    fn exit_number_literal(&mut self, node: &LispNumberLiteralNode, parent: &LispNode);

    fn enter_call_expression(&mut self, node: &LispCallExpressionNode, parent: &LispNode);
    fn exit_call_expression(&mut self, node: &LispCallExpressionNode, parent: &LispNode);
}

fn traverser<T: Visitor>(ast: &LispNode, visitor: &mut T) {
    fn traverse_node<T: Visitor>(node: &LispNode, parent: &LispNode, visitor: &mut T) {
        match node {
            LispNode::CallExpression(current) => {
                visitor.enter_call_expression(current, parent);
                traverse_nodes(&current.params, node, visitor);
                visitor.exit_call_expression(current, parent);
            }
            LispNode::NumberLiteral(current) => {
                visitor.enter_number_literal(current, parent);
                visitor.exit_number_literal(current, parent);
            }
            _ => panic!("[traverser]: unexpected node type: {:?}", node),
        }
    }

    fn traverse_nodes<T>(nodes: &Vec<LispNode>, parent: &LispNode, visitor: &mut T)
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

#[derive(Debug, Clone)]
enum Callee {
    Identifier(String),
}

#[derive(Debug, Clone)]
struct CCallExpressionNode {
    callee: Callee,
    arguments: Vec<Rc<RefCell<CNode>>>,
}

#[derive(Debug, Clone)]
struct CExpressionStatementNode {
    expression: Rc<RefCell<CNode>>,
}

#[derive(Debug, Clone)]
struct CNumberLiteralNode {
    value: String,
}

#[derive(Debug, Clone)]
struct CProgramNode {
    body: Vec<Rc<RefCell<CNode>>>,
}

#[derive(Debug, Clone)]
enum CNode {
    Program(CProgramNode),
    ExpressionStatement(CExpressionStatementNode),
    CallExpression(CCallExpressionNode),
    NumberLiteral(CNumberLiteralNode),
}

impl CNode {
    fn new_program(body: Vec<Rc<RefCell<CNode>>>) -> CNode {
        CNode::Program(CProgramNode { body })
    }
    fn new_expression_statement(expression: Rc<RefCell<CNode>>) -> CNode {
        CNode::ExpressionStatement(CExpressionStatementNode { expression })
    }
    fn new_call_expression(callee: Callee, arguments: Vec<Rc<RefCell<CNode>>>) -> CNode {
        CNode::CallExpression(CCallExpressionNode { callee, arguments })
    }
    fn new_number_literal(value: String) -> CNode {
        CNode::NumberLiteral(CNumberLiteralNode { value })
    }
}

fn transformer(ast: &LispNode) -> Rc<RefCell<CNode>> {
    struct LispVisitor {
        stack: Vec<Rc<RefCell<CNode>>>,
    }

    impl Visitor for LispVisitor {
        fn enter_program(&mut self, _program: &LispProgramNode) {
            let new_program = CNode::new_program(vec![]);
            self.stack.push(Rc::new(RefCell::new(new_program)));
        }
        fn exit_program(&mut self, _program: &LispProgramNode) {}
        fn enter_number_literal(&mut self, node: &LispNumberLiteralNode, parent: &LispNode) {
            let new_node = CNode::new_number_literal(node.value.clone());

            let last_of_stack = self.stack.last();

            if let Some(rc_node) = last_of_stack {
                match *rc_node.borrow_mut() {
                    CNode::CallExpression(ref mut call_expr) => {
                        call_expr.arguments.push(Rc::new(RefCell::new(new_node)));
                    }
                    ref node => panic!("[transformer] unexpected node type {:#?}", node),
                }
            } else {
                panic!("[transformer] unexpected error, last of stack does not exist!")
            }
        }
        fn exit_number_literal(&mut self, _node: &LispNumberLiteralNode, _parent: &LispNode) {}
        fn enter_call_expression(&mut self, node: &LispCallExpressionNode, _parent: &LispNode) {
            let callee = Callee::Identifier(node.name.clone());
            let call_expr = Rc::new(RefCell::new(CNode::new_call_expression(callee, vec![])));

            let last_of_stack = self.stack.last();

            if let Some(rc_node) = last_of_stack {
                match *rc_node.borrow_mut() {
                    CNode::Program(ref mut new_program) => {
                        let expr_stmt =
                            RefCell::new(CNode::new_expression_statement(Rc::clone(&call_expr)));
                        new_program.body.push(Rc::new(expr_stmt));
                    }
                    CNode::CallExpression(ref mut new_call_expr) => {
                        new_call_expr.arguments.push(Rc::clone(&call_expr));
                    }
                    ref node => panic!("[transformer] unexpected node type {:#?}", node),
                }
            } else {
                panic!("[transformer] unexpected error, last of stack does not exist!");
            }

            self.stack.push(Rc::clone(&call_expr));
        }
        fn exit_call_expression(&mut self, _node: &LispCallExpressionNode, _parent: &LispNode) {
            self.stack.pop();
        }
    }

    let mut v = LispVisitor { stack: vec![] };
    traverser(ast, &mut v);

    let new_ast_ref = v.stack.get(0);
    if let Some(new_ast) = new_ast_ref {
        // &Rc<RefCell> -> Rc<RefCell> -> RefCell
        (*new_ast).clone()
    } else {
        panic!("panicked!")
    }
}

fn codegen(new_ast: &Rc<RefCell<CNode>>) -> String {
    let mut code = String::from("");

    match *new_ast.borrow_mut() {
        CNode::Program(ref node_ref) => {
            for node in &node_ref.body {
                code = format!("{}{}", code, codegen(node));
            }
            code
        }
        CNode::ExpressionStatement(ref node_ref) => codegen(&node_ref.expression),
        CNode::CallExpression(ref call_expr) => {
            if let Callee::Identifier(ref identifier) = call_expr.callee {
                code = format!("{}{}(", code, identifier.to_owned());

                let arguments = call_expr.arguments.clone();

                for (index, arg) in arguments.iter().enumerate() {
                    code = format!("{}{}", code, codegen(arg));
                    if index != (arguments.len() - 1) {
                        code = format!("{}, ", code)
                    }
                }

                code = format!("{})", code);
            }

            code
        }
        CNode::NumberLiteral(ref num) => format!("{}{}", code, num.value.to_owned()),
    }
}

fn main() {
    let lisp_input = "(add 2 (subtract 4 2))";

    let tokens = tokenizer(lisp_input);
    let ast = parser(&tokens);
    let new_ast = transformer(&ast);
    let code = codegen(&new_ast);

    println!("original code: {:#?}", lisp_input);
    println!("c code: {:#?}", code);
}
