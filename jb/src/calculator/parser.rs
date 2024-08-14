use core::f64;
use super::{BinaryExpression, BoxedExpression, CalculatorError, CalculatorErrorType, CalculatorResult, CallExpression, NegateExpression, Token, TokenType, Tokenizer, ValueExpression};

/*
    Syntax:
        expression      -> addition EOL; // top lvl
        addition        -> multiplication ( ( '+' | '-' ) multiplication )* ;
        multiplication  -> power ( ( '*' | '/' ) power )* ;
        power           -> unary ( '^' power )* ; // this way it binds to the right
        unary           -> '-' unary | primary ;
        primary         -> NUMBER | "(" expression ")" | call ;
        call            -> KEYWORD "(" argument_list? ")" ;
        argument_list   -> expression ( "," expression )* ;

        NUMBER          -> [0-9]* ( '.' [0-9]+ )? ( ( 'e' | 'E' ) '-'? [0-9]+ )? ;
        IDENTIFIER      -> ( '_' | ALPHA ) ( ALPHANUMERIC | '_' )* ;
        ALPHA           -> [a-z] | [A-Z] ;
        ALPHANUMERIC    -> ALPHA | [0-9] ;
        KEYWORD         -> "sqrt" | "pow" | "sin" | "cos" | "tan" | "e" | "pi" |
                           "log" | "ln" | "deg" | "rad" | "exp"
*/


pub struct CalculatorParser {
    index: usize,
    tokens: Vec<Token>,
}

impl CalculatorParser {
    pub fn new(expr: &str) -> CalculatorResult<Self> {
        Ok(Self{ index: 0, tokens: Tokenizer::from(expr).tokenize()? })
    }

    pub fn parse(&mut self) -> CalculatorResult<BoxedExpression> {
        self.addition()
    }

    /*
        Helpers
    */

    fn eol(&self) -> bool {
        self.peek().token == TokenType::EndOfLine
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.index].clone();
        self.index += 1;
        token
    }

    fn accept(&mut self, token: TokenType) -> bool {
        if !self.eol() && self.peek().token == token {
            self.index += 1;
            return true;
        }

        false
    }

    /*
        Parsers
    */

    fn addition(&mut self) -> CalculatorResult<BoxedExpression> {
        let mut lhs = self.multiplication()?;
        while self.peek().token == TokenType::Plus || self.peek().token == TokenType::Minus {
            let op = self.advance().token;
            lhs = BinaryExpression::new(op, lhs, self.multiplication()?);
        }
        Ok(lhs)
    }

    fn multiplication(&mut self) -> CalculatorResult<BoxedExpression> {
        let mut lhs = self.power()?;
        while self.peek().token == TokenType::Star || self.peek().token == TokenType::Slash {
            let op = self.advance().token;
            lhs = BinaryExpression::new(op, lhs, self.power()?);
        }
        Ok(lhs)
    }

    fn power(&mut self) -> CalculatorResult<BoxedExpression> {
        let mut lhs = self.unary()?;
        while self.accept(TokenType::Hat) {
            lhs = BinaryExpression::new(TokenType::Hat, lhs, self.power()?);
        }
        Ok(lhs)
    }

    fn unary(&mut self) -> CalculatorResult<BoxedExpression> {
        if self.accept(TokenType::Minus) {
            return Ok(NegateExpression::new(self.unary()?));
        }
        Ok(self.primary()?)
    }

    fn primary(&mut self) -> CalculatorResult<BoxedExpression> {
        match self.peek().token {
            TokenType::ConstE => Ok(ValueExpression::new(f64::consts::E)),
            TokenType::ConstPi => Ok(ValueExpression::new(f64::consts::PI)),
            TokenType::Number => Ok(ValueExpression::parse(&self.advance().literal.unwrap())),
            
            TokenType::LeftParen => {
                self.advance();
                let expr = self.addition()?;
                if !self.accept(TokenType::RightParen) {
                    return Err(CalculatorError::new(0, CalculatorErrorType::UnclosedParenthesis));
                }
                Ok(expr)
            }

            TokenType::KwCos  |
            TokenType::KwDeg  |
            TokenType::KwExp  |
            TokenType::KwLn   |
            TokenType::KwLog  |
            TokenType::KwPow  |
            TokenType::KwRad  |
            TokenType::KwSin  |
            TokenType::KwSqrt |
            TokenType::KwTan => { Ok(self.call()?) }
            
            _ => Err(CalculatorError::new(0, CalculatorErrorType::ExpectedPrimary))
        }
    }

    fn call(&mut self) -> CalculatorResult<BoxedExpression> {
        let keyword = self.advance().token;

        if !self.accept(TokenType::LeftParen) {
            return Err(CalculatorError::new(0, CalculatorErrorType::NoParenthesisAfterIdentifier));
        }

        let mut args = Vec::new();
        if !self.accept(TokenType::RightParen) {
            args.push(self.addition()?);
        }

        while !self.accept(TokenType::RightParen) {
            if !self.accept(TokenType::Comma) {
                return Err(CalculatorError::new(0, CalculatorErrorType::ExpectedCommaOrClosingParenthesis));
            }
            args.push(self.addition()?);
        }

        Ok(CallExpression::new(keyword, args))
    }
}