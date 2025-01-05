use super::{CalculatorError, CalculatorResult, TokenType};
use crate::calculator::CalculatorErrorType;

/*
    MARK: Baseline types
*/
pub type Number = f64;
pub trait Expression {
    fn evaluate(&self) -> CalculatorResult<Number>;
    fn describe(&self) -> CalculatorResult<String>;
}
pub type BoxedExpression = Box<dyn Expression>;

/*
    MARK:  Binary Expression
*/

pub struct BinaryExpression {
    op: TokenType,
    lhs: BoxedExpression,
    rhs: BoxedExpression,
}

impl BinaryExpression {
    pub fn new(op: TokenType, lhs: BoxedExpression, rhs: BoxedExpression) -> Box<Self> {
        Box::new(Self {
            op,
            lhs,
            rhs,
        })
    }
}

impl Expression for BinaryExpression {
    fn evaluate(&self) -> CalculatorResult<Number> {
        match self.op {
            TokenType::Plus => Ok(self.lhs.evaluate()? + self.rhs.evaluate()?),
            TokenType::Minus => Ok(self.lhs.evaluate()? - self.rhs.evaluate()?),
            TokenType::Star => Ok(self.lhs.evaluate()? * self.rhs.evaluate()?),
            TokenType::Slash => Ok(self.lhs.evaluate()? / self.rhs.evaluate()?),
            TokenType::Hat => Ok(self.lhs.evaluate()?.powf(self.rhs.evaluate()?)),
            _ => Err(CalculatorError::new(0, CalculatorErrorType::InvalidBinaryOp)),
        }
    }

    fn describe(&self) -> CalculatorResult<String> {
        Ok(format!("({} {} {})", self.op.describe()?, self.lhs.describe()?, self.rhs.describe()?))
    }
}

/*
    MARK: CallExpression
*/

pub struct CallExpression {
    op: TokenType,
    arguments: Vec<BoxedExpression>,
}

impl CallExpression {
    pub fn new(op: TokenType, arguments: Vec<BoxedExpression>) -> Box<Self> {
        Box::new(Self {
            op,
            arguments,
        })
    }
}

impl Expression for CallExpression {
    fn evaluate(&self) -> CalculatorResult<Number> {
        if self.op == TokenType::KwLog || self.op == TokenType::KwPow {
            if self.arguments.len() != 2 {
                return Err(CalculatorError::new(0, CalculatorErrorType::ArityError(self.arguments.len(), 2)));
            }
        } else if self.arguments.len() != 1 {
            return Err(CalculatorError::new(0, CalculatorErrorType::ArityError(self.arguments.len(), 1)));
        }

        match self.op {
            TokenType::KwPow => Ok(self.arguments[0].evaluate()?.powf(self.arguments[1].evaluate()?)),
            TokenType::KwLog => Ok(self.arguments[0].evaluate()?.log(self.arguments[1].evaluate()?)),

            TokenType::KwSqrt => Ok(self.arguments[0].evaluate()?.sqrt()),
            TokenType::KwSin => Ok(self.arguments[0].evaluate()?.sin()),
            TokenType::KwCos => Ok(self.arguments[0].evaluate()?.cos()),
            TokenType::KwTan => Ok(self.arguments[0].evaluate()?.tan()),
            TokenType::KwLn => Ok(self.arguments[0].evaluate()?.ln()),
            TokenType::KwDeg => Ok(self.arguments[0].evaluate()?.to_degrees()),
            TokenType::KwRad => Ok(self.arguments[0].evaluate()?.to_radians()),
            TokenType::KwExp => Ok(self.arguments[0].evaluate()?.exp()),

            _ => Err(CalculatorError::new(0, CalculatorErrorType::InvalidCallOp)),
        }
    }

    fn describe(&self) -> CalculatorResult<String> {
        match self.op {
            TokenType::KwPow | TokenType::KwLog => {
                Ok(format!("({} {}, {})", self.op.describe()?, self.arguments[0].describe()?, self.arguments[1].describe()?))
            }

            TokenType::KwSqrt
            | TokenType::KwSin
            | TokenType::KwCos
            | TokenType::KwTan
            | TokenType::KwLn
            | TokenType::KwDeg
            | TokenType::KwRad
            | TokenType::KwExp => Ok(format!("({} {})", self.op.describe()?, self.arguments[0].describe()?)),

            _ => Err(CalculatorError::new(0, CalculatorErrorType::InvalidCallOp)),
        }
    }
}

/*
    MARK: NegateExpression
*/

pub struct NegateExpression {
    expr: BoxedExpression,
}

impl NegateExpression {
    pub fn new(expr: BoxedExpression) -> Box<Self> {
        Box::new(Self {
            expr,
        })
    }
}

impl Expression for NegateExpression {
    fn evaluate(&self) -> CalculatorResult<Number> {
        Ok(-self.expr.evaluate()?)
    }

    fn describe(&self) -> CalculatorResult<String> {
        Ok(format!("(neg {})", self.expr.describe()?))
    }
}

/*
    MARK: ValueExpression
*/

pub struct ValueExpression {
    value: Number,
}

impl ValueExpression {
    pub fn new(value: Number) -> Box<Self> {
        Box::new(Self {
            value,
        })
    }

    pub fn parse(literal: &str) -> Box<Self> {
        // we can unwrap here because we verified the string already in the tokenizer
        Self::new(literal.parse::<f64>().unwrap())
    }
}

impl Expression for ValueExpression {
    fn evaluate(&self) -> CalculatorResult<Number> {
        Ok(self.value)
    }

    fn describe(&self) -> CalculatorResult<String> {
        Ok(format!("{}", self.value))
    }
}
