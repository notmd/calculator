mod functions;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Expression;
use parser::Parser;

fn main() {
    let tokens = Lexer::lex(String::from("sin(sin(90)+cos(180))"));
    dbg!(tokens.clone());
    let mut parser = Parser::new(tokens.into_iter().peekable());
    let res = parser.parse();
    match res {
        Ok(expr) => {
            dbg!(&expr, expr.eval());
        }
        Err(e) => {
            dbg!(e);
        }
    };
}
