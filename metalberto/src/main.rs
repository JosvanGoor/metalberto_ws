use jb::calculator::CalculatorParser;

fn main() {
    let expr = format!("   @");
    let mut parser = CalculatorParser::new(&expr).unwrap();
    let expression = parser.parse().unwrap();
    
    println!("describe: {}", expression.describe().unwrap());
    println!("evaluate: {}", expression.evaluate().unwrap());
}