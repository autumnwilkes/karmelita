use std::time;

mod lexer;
#[allow(dead_code)]
mod parser;

pub fn main() {
    let x: Vec<u8> = vec![1, 2, 3];
    // let mut input = String::new();
    // let res = std::io::stdin().read_line(input);
    let buf = std::fs::read_to_string("src/parser.rs").unwrap();
    let mut tok = lexer::Tokens::new(&*buf);
    loop {
        println!("{:?}", tok.next().unwrap());
        std::thread::sleep(time::Duration::from_millis(10));
    }
    println!("{:?}", tok.collect::<Vec<lexer::Token>>())
}
