use crate::ast::lisp_ast;

/*
 * For our parser we're going to take our array of tokens and turn it into an
 * AST.
 *
 *   [{ type: 'paren', value: '(' }, ...]   =>   { type: 'Program', body: [...] }
 */
pub fn parser(tokens: &Vec<lisp_ast::Token>) -> lisp_ast::Node {
    use lisp_ast::{Node, Token};

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
