use jb::calculator::{evaluate_calculation, CalculatorParser};


fn main() {
    let expr = "1 + 1 + 1";
    let ast = CalculatorParser::new(expr).unwrap().parse().unwrap();
    println!("{}", ast.describe().unwrap());

    println!("{}", evaluate_calculation(expr).unwrap())
}
