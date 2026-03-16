use abaco_core::Value;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum EvalError {
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Unknown function: {0}")]
    UnknownFunction(String),
    #[error("Unknown variable: {0}")]
    UnknownVariable(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid expression")]
    InvalidExpression,
}

pub type Result<T> = std::result::Result<T, EvalError>;

/// Lexer token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    LParen,
    RParen,
    Ident(String),
    Comma,
}

/// Tokenize an expression string.
pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' | '\r' => i += 1,
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' if i + 1 < chars.len() && chars[i + 1] == '*' => {
                tokens.push(Token::Power);
                i += 2;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }
            '%' => {
                tokens.push(Token::Percent);
                i += 1;
            }
            '^' => {
                tokens.push(Token::Power);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let num = num_str
                    .parse::<f64>()
                    .map_err(|_| EvalError::ParseError(format!("Invalid number: {num_str}")))?;
                tokens.push(Token::Number(num));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let ident: String = chars[start..i].iter().collect();
                tokens.push(Token::Ident(ident));
            }
            c => return Err(EvalError::ParseError(format!("Unexpected character: {c}"))),
        }
    }

    Ok(tokens)
}

/// Expression evaluator with variable support.
pub struct Evaluator {
    variables: HashMap<String, f64>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

    /// Evaluate an expression string and return a Value.
    pub fn eval(&self, expr: &str) -> Result<Value> {
        let tokens = tokenize(expr)?;
        if tokens.is_empty() {
            return Err(EvalError::InvalidExpression);
        }
        let mut pos = 0;
        let result = self.parse_expr(&tokens, &mut pos)?;
        if pos < tokens.len() {
            return Err(EvalError::ParseError(format!(
                "Unexpected token at position {pos}"
            )));
        }
        // Return as integer if the result is a whole number
        if result.fract() == 0.0 && result.abs() < i64::MAX as f64 {
            Ok(Value::Integer(result as i64))
        } else {
            Ok(Value::Float(result))
        }
    }

    // parse_expr: handles + and -
    fn parse_expr(&self, tokens: &[Token], pos: &mut usize) -> Result<f64> {
        let mut left = self.parse_term(tokens, pos)?;
        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Plus => {
                    *pos += 1;
                    left += self.parse_term(tokens, pos)?;
                }
                Token::Minus => {
                    *pos += 1;
                    left -= self.parse_term(tokens, pos)?;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    // parse_term: handles * / %
    fn parse_term(&self, tokens: &[Token], pos: &mut usize) -> Result<f64> {
        let mut left = self.parse_power(tokens, pos)?;
        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Star => {
                    *pos += 1;
                    left *= self.parse_power(tokens, pos)?;
                }
                Token::Slash => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    if right == 0.0 {
                        return Err(EvalError::DivisionByZero);
                    }
                    left /= right;
                }
                Token::Percent => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    if right == 0.0 {
                        return Err(EvalError::DivisionByZero);
                    }
                    left %= right;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    // parse_power: handles ^
    fn parse_power(&self, tokens: &[Token], pos: &mut usize) -> Result<f64> {
        let base = self.parse_unary(tokens, pos)?;
        if *pos < tokens.len() && tokens[*pos] == Token::Power {
            *pos += 1;
            let exp = self.parse_power(tokens, pos)?; // right-associative
            Ok(base.powf(exp))
        } else {
            Ok(base)
        }
    }

    // parse_unary: handles unary + and -
    fn parse_unary(&self, tokens: &[Token], pos: &mut usize) -> Result<f64> {
        if *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Minus => {
                    *pos += 1;
                    let val = self.parse_unary(tokens, pos)?;
                    Ok(-val)
                }
                Token::Plus => {
                    *pos += 1;
                    self.parse_unary(tokens, pos)
                }
                _ => self.parse_primary(tokens, pos),
            }
        } else {
            Err(EvalError::InvalidExpression)
        }
    }

    // parse_primary: numbers, identifiers (constants, functions, variables), parentheses
    fn parse_primary(&self, tokens: &[Token], pos: &mut usize) -> Result<f64> {
        if *pos >= tokens.len() {
            return Err(EvalError::InvalidExpression);
        }

        match &tokens[*pos] {
            Token::Number(n) => {
                let val = *n;
                *pos += 1;
                Ok(val)
            }
            Token::LParen => {
                *pos += 1;
                let val = self.parse_expr(tokens, pos)?;
                if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                    return Err(EvalError::ParseError("Missing closing parenthesis".into()));
                }
                *pos += 1;
                Ok(val)
            }
            Token::Ident(name) => {
                let name = name.clone();
                *pos += 1;

                // Check for function call
                if *pos < tokens.len() && tokens[*pos] == Token::LParen {
                    *pos += 1;
                    let mut args = Vec::new();
                    if *pos < tokens.len() && tokens[*pos] != Token::RParen {
                        args.push(self.parse_expr(tokens, pos)?);
                        while *pos < tokens.len() && tokens[*pos] == Token::Comma {
                            *pos += 1;
                            args.push(self.parse_expr(tokens, pos)?);
                        }
                    }
                    if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                        return Err(EvalError::ParseError("Missing closing parenthesis".into()));
                    }
                    *pos += 1;
                    self.call_function(&name, &args)
                } else {
                    // Constant or variable
                    match name.as_str() {
                        "pi" | "PI" => Ok(std::f64::consts::PI),
                        "e" | "E" => Ok(std::f64::consts::E),
                        "tau" | "TAU" => Ok(std::f64::consts::TAU),
                        _ => self
                            .variables
                            .get(&name)
                            .copied()
                            .ok_or(EvalError::UnknownVariable(name)),
                    }
                }
            }
            _ => Err(EvalError::ParseError(format!(
                "Unexpected token: {:?}",
                tokens[*pos]
            ))),
        }
    }

    fn call_function(&self, name: &str, args: &[f64]) -> Result<f64> {
        match (name, args.len()) {
            // 1-arg functions
            ("sqrt", 1) => Ok(args[0].sqrt()),
            ("sin", 1) => Ok(args[0].sin()),
            ("cos", 1) => Ok(args[0].cos()),
            ("tan", 1) => Ok(args[0].tan()),
            ("log" | "log10", 1) => Ok(args[0].log10()),
            ("ln", 1) => Ok(args[0].ln()),
            ("log2", 1) => Ok(args[0].log2()),
            ("abs", 1) => Ok(args[0].abs()),
            ("ceil", 1) => Ok(args[0].ceil()),
            ("floor", 1) => Ok(args[0].floor()),
            ("round", 1) => Ok(args[0].round()),
            ("exp", 1) => Ok(args[0].exp()),
            ("asin", 1) => Ok(args[0].asin()),
            ("acos", 1) => Ok(args[0].acos()),
            ("atan", 1) => Ok(args[0].atan()),
            // 2-arg functions
            ("min", 2) => Ok(args[0].min(args[1])),
            ("max", 2) => Ok(args[0].max(args[1])),
            ("pow", 2) => Ok(args[0].powf(args[1])),
            ("atan2", 2) => Ok(args[0].atan2(args[1])),
            // Unknown function or wrong arity
            ("sqrt" | "sin" | "cos" | "tan" | "log" | "log10" | "ln" | "log2"
            | "abs" | "ceil" | "floor" | "round" | "exp" | "asin" | "acos" | "atan", n) => {
                Err(EvalError::ParseError(format!(
                    "Function {name} expects 1 argument, got {n}"
                )))
            }
            ("min" | "max" | "pow" | "atan2", n) => {
                Err(EvalError::ParseError(format!(
                    "Function {name} expects 2 arguments, got {n}"
                )))
            }
            _ => Err(EvalError::UnknownFunction(name.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(expr: &str) -> Value {
        Evaluator::new().eval(expr).unwrap()
    }

    fn eval_f64(expr: &str) -> f64 {
        match Evaluator::new().eval(expr).unwrap() {
            Value::Integer(n) => n as f64,
            Value::Float(n) => n,
            other => panic!("Expected numeric, got {other:?}"),
        }
    }

    #[test]
    fn test_basic_addition() {
        assert_eq!(eval("2 + 3"), Value::Integer(5));
    }

    #[test]
    fn test_basic_subtraction() {
        assert_eq!(eval("10 - 4"), Value::Integer(6));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(eval("6 * 7"), Value::Integer(42));
    }

    #[test]
    fn test_division() {
        assert_eq!(eval("10 / 4"), Value::Float(2.5));
    }

    #[test]
    fn test_order_of_operations() {
        assert_eq!(eval("2 + 3 * 4"), Value::Integer(14));
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(eval("(2 + 3) * 4"), Value::Integer(20));
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(eval("((1 + 2) * (3 + 4))"), Value::Integer(21));
    }

    #[test]
    fn test_power() {
        assert_eq!(eval("2 ^ 10"), Value::Integer(1024));
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(eval("-5 + 3"), Value::Integer(-2));
    }

    #[test]
    fn test_modulo() {
        assert_eq!(eval("10 % 3"), Value::Integer(1));
    }

    #[test]
    fn test_sqrt_function() {
        assert_eq!(eval_f64("sqrt(16)"), 4.0);
    }

    #[test]
    fn test_abs_function() {
        assert_eq!(eval_f64("abs(-42)"), 42.0);
    }

    #[test]
    fn test_pi_constant() {
        let result = eval_f64("pi");
        assert!((result - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_e_constant() {
        let result = eval_f64("e");
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_variables() {
        let mut ev = Evaluator::new();
        ev.set_variable("x", 5.0);
        assert_eq!(ev.eval("x + 3").unwrap(), Value::Integer(8));
    }

    #[test]
    fn test_division_by_zero() {
        let result = Evaluator::new().eval("1 / 0");
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_variable() {
        let result = Evaluator::new().eval("xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_function() {
        let result = Evaluator::new().eval("foo(3)");
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_expression() {
        // 2^3 + sqrt(9) * 2 = 8 + 3*2 = 14
        assert_eq!(eval_f64("2^3 + sqrt(9) * 2"), 14.0);
    }

    #[test]
    fn test_floor_ceil_round() {
        assert_eq!(eval_f64("floor(3.7)"), 3.0);
        assert_eq!(eval_f64("ceil(3.2)"), 4.0);
        assert_eq!(eval_f64("round(3.5)"), 4.0);
    }

    #[test]
    fn test_min_function() {
        assert_eq!(eval_f64("min(3, 5)"), 3.0);
        assert_eq!(eval_f64("min(7, 2)"), 2.0);
        assert_eq!(eval_f64("min(-1, -5)"), -5.0);
    }

    #[test]
    fn test_max_function() {
        assert_eq!(eval_f64("max(3, 5)"), 5.0);
        assert_eq!(eval_f64("max(7, 2)"), 7.0);
        assert_eq!(eval_f64("max(-1, -5)"), -1.0);
    }

    #[test]
    fn test_pow_function() {
        assert_eq!(eval_f64("pow(2, 10)"), 1024.0);
        assert_eq!(eval_f64("pow(3, 0)"), 1.0);
        assert_eq!(eval_f64("pow(9, 0.5)"), 3.0);
    }

    #[test]
    fn test_log2_function() {
        assert_eq!(eval_f64("log2(8)"), 3.0);
        assert_eq!(eval_f64("log2(1)"), 0.0);
        assert!((eval_f64("log2(2)") - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_atan2_function() {
        let result = eval_f64("atan2(1, 1)");
        assert!((result - std::f64::consts::FRAC_PI_4).abs() < 1e-10);
        let result = eval_f64("atan2(0, 1)");
        assert!(result.abs() < 1e-10);
    }

    #[test]
    fn test_multi_arg_wrong_arity() {
        let result = Evaluator::new().eval("min(1)");
        assert!(result.is_err());
        let result = Evaluator::new().eval("max(1, 2, 3)");
        assert!(result.is_err());
        let result = Evaluator::new().eval("sqrt(1, 2)");
        assert!(result.is_err());
    }

    #[test]
    fn test_tokenize_basic() {
        let tokens = tokenize("2 + 3").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Number(2.0));
        assert_eq!(tokens[1], Token::Plus);
        assert_eq!(tokens[2], Token::Number(3.0));
    }
}
