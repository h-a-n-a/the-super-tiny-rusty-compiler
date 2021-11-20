use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{c_ast, lisp_ast};

pub fn traverser<T>(ast: &lisp_ast::Node, visitor: &mut T)
where
    T: lisp_ast::Visitor,
{
    use lisp_ast::Node;

    fn traverse_nodes<T>(nodes: &Vec<Node>, parent: &Node, visitor: &mut T)
    where
        T: lisp_ast::Visitor,
    {
        nodes
            .iter()
            .for_each(|node| traverse_node(node, parent, visitor));
    }

    fn traverse_node<T>(node: &Node, parent: &Node, visitor: &mut T)
    where
        T: lisp_ast::Visitor,
    {
        match node {
            Node::CallExpression(current) => {
                visitor.enter_call_expression(current, parent);
                traverse_nodes(&current.params, node, visitor);
                visitor.exit_call_expression(current, parent);
            }
            Node::NumberLiteral(current) => {
                visitor.enter_number_literal(current, parent);
                visitor.exit_number_literal(current, parent);
            }
            _ => panic!("[traverser]: unexpected node type: {:?}", node),
        }
    }

    if let Node::Program(node) = ast {
        visitor.enter_program(node);
        traverse_nodes(&node.body, ast, visitor);
        visitor.exit_program(node);
    } else {
        panic!("[traverser]: unexpected `program` type: {:?}", ast);
    }
}

pub fn transformer(ast: &lisp_ast::Node) -> Rc<RefCell<c_ast::Node>> {
    struct LispAstVisitor {
        stack: Vec<Rc<RefCell<c_ast::Node>>>,
        program: Option<Rc<RefCell<c_ast::Node>>>,
    }

    impl lisp_ast::Visitor for LispAstVisitor {
        fn enter_program(&mut self, _program: &lisp_ast::Program) {
            use c_ast::Node;

            let new_program = Node::new_program(vec![]);
            self.program = Some(Rc::new(RefCell::new(new_program)));
        }
        fn exit_program(&mut self, _program: &lisp_ast::Program) {}

        fn enter_number_literal(
            &mut self,
            node: &lisp_ast::NumberLiteral,
            _parent: &lisp_ast::Node,
        ) {
            use c_ast::Node;

            let new_node = Node::new_number_literal(node.value.clone());
            let last_of_stack = self.stack.last();

            if let Some(rc_node) = last_of_stack {
                match *rc_node.borrow_mut() {
                    Node::CallExpression(ref mut call_expr) => {
                        call_expr.arguments.push(Rc::new(RefCell::new(new_node)));
                    }
                    ref node => panic!("[transformer] unexpected node type {:#?}", node),
                }
            } else {
                panic!("[transformer] unexpected error, last of stack does not exist!")
            }
        }
        fn exit_number_literal(
            &mut self,
            _node: &lisp_ast::NumberLiteral,
            _parent: &lisp_ast::Node,
        ) {
        }

        fn enter_call_expression(
            &mut self,
            node: &lisp_ast::CallExpression,
            _parent: &lisp_ast::Node,
        ) {
            use c_ast::{Callee, Node};

            let callee = Callee::Identifier(node.name.clone());
            let call_expr = Rc::new(RefCell::new(Node::new_call_expression(callee, vec![])));
            let last_of_stack = self.stack.last();

            if let Some(rc_node) = last_of_stack {
                match *rc_node.borrow_mut() {
                    Node::CallExpression(ref mut new_call_expr) => {
                        new_call_expr.arguments.push(Rc::clone(&call_expr));
                    }
                    ref node => panic!("[transformer] unexpected node type {:#?}", node),
                }
            } else if let Some(ref mut new_program) = self.program {
                match *new_program.borrow_mut() {
                    c_ast::Node::Program(ref mut program) => {
                        let expr_stmt =
                            RefCell::new(Node::new_expression_statement(Rc::clone(&call_expr)));
                        program.body.push(Rc::new(expr_stmt))
                    }
                    ref node => panic!("[transformer] unexpected node type {:#?}", node),
                }
            } else {
                panic!("[transformer] unexpected error, last of stack does not exist!");
            }

            self.stack.push(Rc::clone(&call_expr));
        }
        fn exit_call_expression(
            &mut self,
            _node: &lisp_ast::CallExpression,
            _parent: &lisp_ast::Node,
        ) {
            self.stack.pop();
        }
    }

    let mut v = LispAstVisitor {
        stack: vec![],
        program: None,
    };
    traverser(ast, &mut v);

    v.program.unwrap()
}
