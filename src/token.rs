use crate::ast::lisp_ast;

pub fn tokenizer(input: &str) -> Vec<lisp_ast::Token> {
    use lisp_ast::Token;

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
