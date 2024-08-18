use core::str;

use jb::calculator::CalculatorParser;
use jb::http::{HttpContent, HttpMethod, HttpRequest};
use jb::net::Uri;
use jb::random::Lcg;

fn main() {
    let expr = format!("1 + 2 + 3");
    let mut parser = CalculatorParser::new(&expr).unwrap();
    let expression = parser.parse().unwrap();
    
    println!("describe: {}", expression.describe().unwrap());
    println!("evaluate: {}", expression.evaluate().unwrap());

    let mut request = HttpRequest::new();
    let content = HttpContent::from_content("plain/txt".into(), "123".into());

    let uri = Uri::from("http://127.0.0.1/test?arg=12#skadeeb").unwrap();
    let raw = request.generate(HttpMethod::Post, &uri, Some(&content));
    let generated = String::from(str::from_utf8(raw.as_slice()).unwrap());
    println!("{}", generated);

    let mut lcg = Lcg::default();
    for _ in 0..10 {
        println!("{}", lcg.next());
    }
}