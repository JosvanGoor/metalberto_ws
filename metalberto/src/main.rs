use jb::{calculator::{CalculatorParser, Tokenizer}, hash::Sha1, net::SocketAddress};

fn main() {
    let expr = format!("sqrt(-1)");
    let mut parser = CalculatorParser::new(&expr).unwrap();
    println!("result: {}", parser.parse().unwrap().evaluate().unwrap());
}