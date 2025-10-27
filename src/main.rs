mod lexer;
#[allow(dead_code)]
mod parser;

pub fn main() {
    let mut input = String::new();
    let res = std::io::stdin().read_line(&mut input);
    let buf = std::fs::read_to_string(input);
    let tok = lexer::Tokens::new(&*buf.unwrap());
}
