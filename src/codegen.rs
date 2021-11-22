use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::c_ast;

pub fn codegen(new_ast: &Rc<RefCell<c_ast::Node>>) -> String {
    use c_ast::{Callee, Node};

    let mut code = String::from("");

    match *new_ast.borrow_mut() {
        Node::Program(ref node_ref) => node_ref
            .body
            .iter()
            .map(|node| format!("{}{}", code, codegen(node)))
            .collect::<Vec<String>>()
            .join("\n"),
        Node::ExpressionStatement(ref node_ref) => codegen(&node_ref.expression),
        Node::CallExpression(ref call_expr) => {
            let Callee::Identifier(ref identifier) = call_expr.callee;

            code = format!(
                "{})",
                call_expr.arguments.iter().enumerate().fold(
                    format!("{}{}(", code, identifier),
                    |mut code, (index, arg)| {
                        code = format!("{}{}", code, codegen(arg));
                        if index != (&call_expr.arguments.len() - 1) {
                            code = format!("{}, ", code)
                        }
                        code
                    }
                )
            );

            code
        }
        Node::NumberLiteral(ref num) => format!("{}{}", code, num.value),
    }
}
