pub mod ast;
pub mod codegen;
pub mod parser;
pub mod token;
pub mod transform;

pub use codegen::codegen;
pub use parser::parser;
pub use token::tokenizer;
pub use transform::transformer;

// "(add 2 (subtract 4 2))";
pub fn compile(lisp_string: &str) -> String {
    let tokens = tokenizer(lisp_string);
    let ast = parser(&tokens);
    let new_ast = transformer(&ast);
    let c_string = codegen(&new_ast);

    c_string
}
