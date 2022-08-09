mod constants;
mod functions;
mod lexer;
mod parser;

use std::env;

use lexer::Lexer;
use parser::Expression;
use parser::Parser;

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let tokens = Lexer::lex(input.clone());
    let mut parser = Parser::new(tokens.into_iter().peekable());
    let res = parser.parse();
    match res {
        Ok(expr) => {
            let res = expr.eval(); 
            dbg!(res);
            println!("{} = {}", input, res);
        }
        Err(e) => {
            dbg!(e);
        }
    };

}
