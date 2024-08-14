use jb::{calculator::{CalculatorParser, Tokenizer}, hash::Sha1, net::SocketAddress};

fn main() {
    let expr = format!("2 ^ 4 ^ 2 - 1 + 2 + 3");
    let mut parser = CalculatorParser::new(&expr).unwrap();
    let expression = parser.parse().unwrap();
    
    println!("describe: {}", expression.describe().unwrap());
    println!("evaluate: {}", expression.evaluate().unwrap());
}