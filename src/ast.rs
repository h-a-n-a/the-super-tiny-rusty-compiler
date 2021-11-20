pub mod lisp_ast {
    #[derive(Debug, PartialEq)]
    pub enum Token {
        ParenOpen,
        ParenClose,
        Name(String),
        Number(String),
    }

    #[derive(Debug)]
    pub struct CallExpression {
        pub name: String,
        pub params: Vec<Node>,
    }

    #[derive(Debug)]
    pub struct Program {
        pub body: Vec<Node>,
    }

    #[derive(Debug)]
    pub struct NumberLiteral {
        pub value: String,
    }

    #[derive(Debug)]
    pub enum Node {
        Program(Program),
        CallExpression(CallExpression),
        NumberLiteral(NumberLiteral),
    }

    impl Node {
        pub fn new_program(body: Vec<Node>) -> Node {
            Node::Program(Program { body })
        }
        pub fn new_call_expression(name: String, params: Vec<Node>) -> Node {
            Node::CallExpression(CallExpression { name, params })
        }
        pub fn new_number_literal(value: String) -> Node {
            Node::NumberLiteral(NumberLiteral { value })
        }
    }

    pub trait Visitor {
        fn enter_program(&mut self, program: &Program);
        fn exit_program(&mut self, program: &Program);

        fn enter_number_literal(&mut self, node: &NumberLiteral, parent: &Node);
        fn exit_number_literal(&mut self, node: &NumberLiteral, parent: &Node);

        fn enter_call_expression(&mut self, node: &CallExpression, parent: &Node);
        fn exit_call_expression(&mut self, node: &CallExpression, parent: &Node);
    }
}

pub mod c_ast {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    pub enum Callee {
        Identifier(String),
    }

    #[derive(Debug)]
    pub struct CallExpression {
        pub callee: Callee,
        pub arguments: Vec<Rc<RefCell<Node>>>,
    }

    #[derive(Debug)]
    pub struct ExpressionStatement {
        pub expression: Rc<RefCell<Node>>,
    }

    #[derive(Debug)]
    pub struct NumberLiteral {
        pub value: String,
    }

    #[derive(Debug)]
    pub struct Program {
        pub body: Vec<Rc<RefCell<Node>>>,
    }

    #[derive(Debug)]
    pub enum Node {
        Program(Program),
        ExpressionStatement(ExpressionStatement),
        CallExpression(CallExpression),
        NumberLiteral(NumberLiteral),
    }

    impl Node {
        pub fn new_program(body: Vec<Rc<RefCell<Node>>>) -> Node {
            Node::Program(Program { body })
        }
        pub fn new_expression_statement(expression: Rc<RefCell<Node>>) -> Node {
            Node::ExpressionStatement(ExpressionStatement { expression })
        }
        pub fn new_call_expression(callee: Callee, arguments: Vec<Rc<RefCell<Node>>>) -> Node {
            Node::CallExpression(CallExpression { callee, arguments })
        }
        pub fn new_number_literal(value: String) -> Node {
            Node::NumberLiteral(NumberLiteral { value })
        }
    }
}
