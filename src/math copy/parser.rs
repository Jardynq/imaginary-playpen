use std::{io::BufRead, iter::Peekable, io::Read};
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Debug;
use std::fmt::Display;
use super::complex::Complex;

// TODO have a keyword type thats const
// and get rid of the keyword type and use strings instead
// only use the keyword type in the serializer
use super::serializer::Keyword;








#[derive(Debug, Clone, PartialEq)]
pub enum Token<V: Keyword, F: Keyword> {
    Real(f64),
    Imaginary(f64),
    Type(TokenType),
    Keyword(TokenKeyword<V, F>),
    Invalid(TokenError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,

    ParenthesisOpen,
    ParenthesisClosed,
    
    AbsoluteOpen,
    AbsoluteClosed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKeyword<V: Keyword, F: Keyword> {
    Variable(V),
    Function(F),
}
#[derive(Clone, PartialEq, Eq)]
pub enum TokenError {
    InvalidKeyword(usize),
    DuplicateKeyword(usize),
    InvalidCharacter(usize),
    InvalidNumber(usize),
}
impl TokenError {
    pub fn get_index(&self) -> usize {
        match self {
            Self::DuplicateKeyword(index) => *index,
            Self::InvalidCharacter(index) => *index,
            Self::InvalidNumber(index) => *index,
            Self::InvalidKeyword(index) => *index,
        }
    }
}
impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} at token index {}", self, self.get_index())
    }
}
impl Debug for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateKeyword(_) => {
                write!(f, "Duplicate definition of keyword")
            }
            Self::InvalidCharacter(_) => {
                write!(f, "Invalid character")
            }
            Self::InvalidNumber(_) => {
                write!(f, "Invalid number")
            }
            Self::InvalidKeyword(_) => {
                write!(f, "Invalid keyword")
            }
        }
    }
}


// TODO, have the parse function be implemented on Their corrosponding struts

#[derive(Clone, PartialEq, Eq)]
pub enum SyntaxError {
    UnexpectedEndOfInput(usize),
    UnexpectedToken(usize),
    MissingClosingDelimiter(usize),
    EmptyDelimiter(usize),
    TokenError(TokenError),

    // "_*2"
    // MissingLeftHandSide
    //"2*_"
    // MissingRightHandSide
}
impl SyntaxError {
    pub fn get_index(&self) -> usize {
        match self {
            Self::UnexpectedEndOfInput(index) => *index,
            Self::UnexpectedToken(index) => *index,
            Self::MissingClosingDelimiter(index) => *index,
            Self::EmptyDelimiter(index) => *index,
            Self::TokenError(error) => error.get_index(),
        }
    }


    pub fn pretty(&self, input: &str) -> String {
        let input = sanitize(input);

        // Get error message and index of token, that generated the message
        let message = format!("{:?}", self);
        let index = self.get_index();

        // Parse tokens and calculate index in input string
        // for the token, that generated the error.
        // The index will be used for padding and aligning the error message.
        let mut padding = 0;

        let mut iter = input.chars().peekable();
        let mut count: isize = 0;
        while let Some(&c) = iter.peek() {
            // Subtract size of the token,
            // so the pointer points to the token and not after the token.
            let mut size = 1;
            match c {
                // Account for whitespace.
                ' ' => {
                    count -= 1;
                    padding += 1;
                    iter.next();
                    while let Some(&c) = iter.peek() {
                        match c {
                            ' ' => padding += 1,
                            _ => break,
                        };
                        iter.next();
                    }
                }

                // Account for numbers.
                '0'..='9' | '.' => {
                    padding += 1;
                    iter.next();
                    while let Some(&c) = iter.peek() {
                        match c {
                            '0'..='9' | '.' => {
                                size += 1;
                                padding += 1;
                                iter.next();
                            },
                            'i' => {
                                size += 1;
                                padding += 1;
                                iter.next();
                                break;
                            },
                            _ => break,
                        };
                    }
                }

                // Account for keywords.
                'a'..='z' | 'A'..='Z' => {
                    padding += 1;
                    iter.next();
                    while let Some(&c) = iter.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '_' => {
                                size += 1;
                                padding += 1;
                            }
                            _ => break,
                        };
                        iter.next();
                    }
                }

                // Account for single characters.
                _ => {
                    padding += 1;
                    iter.next();
                }
            };

            // Only count padding up until the token.
            if count >= index as isize {
                // Subtract size / 2, so that the pointer points to the center of the token.
                padding -= (size as f32 / 2.0).ceil() as usize;
                break;
            }
            count += 1;
        }


        // Generete padding for error message to be properly aligned.
        // Create the final string combining input string and aligned error message.
        match String::from_utf8(vec![' ' as u8; padding]) {
            Ok(padding) => format!("{}\n{}^ {}", input, padding, message),
            Err(_) => format!("{}\n{}", input, message)
        }
    }
}
impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} at token index {}", self, self.get_index())
    }
}
impl Debug for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfInput(_) => {
                write!(f, "Unexpected end of input")
            }
            Self::UnexpectedToken(_) => {
                write!(f, "Unexpected token")
            }
            Self::MissingClosingDelimiter(_) => {
                write!(f, "Missing closing delimter")
            }
            Self::EmptyDelimiter(_)=> {
                write!(f, "Empty delimiter")
            }
            Self::TokenError(error) => {
                write!(f, "{:?}", error)
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Syntax {
    Ignore,
    NamedVariable(String),
    
    Real(f64),
    Imaginary(f64),
    Variable(u32),
    Function(u32),

    Parenthesis,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
    Absolute,
}
impl Syntax {
    pub fn serialize_syntax(&self) -> u32 {
        match self {
            Self::Real(_)       => 0,
            Self::Imaginary(_)  => 1,
            Self::Variable(_)   => 2,
            Self::Function(_)   => 3,

            Self::Parenthesis       => 4,
            Self::Addition          => 5,
            Self::Subtraction       => 6, 
            Self::Multiplication    => 7,
            Self::Division          => 8,
            Self::Exponent          => 9,
            Self::Absolute          => 10,

            _ => 0,
        }
    }
    pub fn serialize_value(&self) -> (f32, f32) {
        match self {
            Self::Real(value) => (*value as f32, 0.0),
            Self::Imaginary(value) => (0.0, *value as f32),
            _ => (0.0, 0.0),
        }
    }
    pub fn serialize_keyword(&self) -> u32 {
        match self {
            Self::Variable(variable) => *variable,
            Self::Function(function) => *function,
            _ => 0,
        }
    }
}




#[derive(Debug, Clone, Copy, Default)]
#[repr(packed)]
pub struct NodeData {
    // The std140 layout is weird so i make sure that the struct is only 16 bytes in size.
    // Also, for some reason the lo-hi order is reversed.

    // Pack left child index into lo
    pub left_index: u16,                // offset 2, size 2
    // Pack right child index into hi
    pub right_index: u16,               // offset 0, size 2
    
    // Pack syntax identifier into lo
    pub syntax: u16,                    // offset 6, size 2
    // Pack keyword identifier into hi
    pub keyword: u16,                   // offset 4, size 2

    pub value: (f32, f32),              // offset 8, size 8
}
#[derive(Debug, Clone)]
pub struct Node {
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
    pub syntax: Syntax,
}
impl Node {
    pub fn new(left: Option<Node>, right: Option<Node>, syntax: Syntax) -> Self {
        Self {
            left: match left {
                Some(node) => Some(Box::new(node)),
                None => None,
            },
            right: match right {
                Some(node) => Some(Box::new(node)),
                None => None,
            },
            syntax: syntax,
        }
    }
    pub fn from(left: Node, right: Node, syntax: Syntax) -> Self {
        Self {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            syntax: syntax,
        }
    }
    pub fn from_syntax(syntax: Syntax) -> Self {
        Self {
            left: None,
            right: None,
            syntax: syntax,
        }
    }
    pub fn empty() -> Self {
        Self {
            left: None,
            right: None,
            syntax: Syntax::Ignore,
        }
    }


    pub fn child_syntax(&self) -> (Syntax, Syntax) {
        let left = match &self.left {
            Some(node) => node.syntax.clone(),
            None => Syntax::Ignore,
        };
        let right = match &self.right {
            Some(node) => node.syntax.clone(),
            None => Syntax::Ignore,
        };
        (left, right)
    }


    // Array is sorted in pre-order
    pub fn serialize(&self) -> Vec<NodeData> {
        let mut result = Vec::new();
        self.serialize_next(&mut result);
        result
    }
    pub fn serialize_next(&self, result: &mut Vec<NodeData>) {
        let index = result.len();
        result.push(NodeData{
            right_index: 0,
            left_index: 0,
            keyword: self.syntax.serialize_keyword() as u16,
            syntax: self.syntax.serialize_syntax() as u16,
            value: self.syntax.serialize_value(),
        });

        let mut left = 0;
        if let Some(node) = &self.left {
            left = result.len();
            &mut node.serialize_next(result);
        }
        let mut right = 0;
        if let Some(node) = &self.right {
            right = result.len();
            &mut node.serialize_next(result);
        }

        if let Some(parent) = result.get_mut(index) {
            parent.left_index = left as u16;
            parent.right_index = right as u16;
        }
    }


    pub fn evaluate_variables(&self, variables: HashMap<String, Complex>) {
        /*match self.syntax {
            Syntax::Variable()
        }*/
    }
    pub fn evaluate(&self) -> Complex {
        match self.syntax {
            Syntax::Real(value) => Complex::new(value, 0.0),
            Syntax::Imaginary(value) => Complex::new(0.0, value),

            Syntax::Parenthesis => {
                self.left.clone().unwrap().evaluate()
            },

            Syntax::Addition => {
                let lhs = self.left.clone().unwrap().evaluate();
                let rhs = self.right.clone().unwrap().evaluate();
                lhs + rhs
            },
            Syntax::Subtraction => {
                let lhs = self.left.clone().unwrap().evaluate();
                let rhs = self.right.clone().unwrap().evaluate();
                lhs - rhs
            },
            Syntax::Multiplication => {
                let lhs = self.left.clone().unwrap().evaluate();
                let rhs = self.right.clone().unwrap().evaluate();
                lhs * rhs
            },
            Syntax::Division => {
                let lhs = self.left.clone().unwrap().evaluate();
                let rhs = self.right.clone().unwrap().evaluate();
                lhs / rhs
            },
            /*
            Syntax::Exponent => {
                let lhs = self.left.clone().unwrap().debug_eval();
                let rhs = self.right.clone().unwrap().debug_eval();
                lhs.powf(rhs)
            },
            */
            Syntax::Absolute => {
                let value = self.left.clone().unwrap().evaluate();
                Complex::new(value.abs(), 0.0)
            },

            _ => Complex::new(0.0, 0.0),
        }
    }
}

// TODO, pretty printing cant handle custom keywords anymore.
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn fmt_pretty(node: &Node, prefix: &str) -> String {
            let next_prefix = format!("{}|   ", prefix);
            let result = match &node.syntax {
                Syntax::Ignore => format!(""),
                Syntax::NamedVariable(name)  => format!("|> {:?}\n", name),

                Syntax::Real(value) => format!("|> {}\n", value),
                Syntax::Imaginary(value) => format!("|> {}i\n", value),
                Syntax::Variable(variable)  => format!("|> {:?}\n", variable),
                Syntax::Function(function)  => format!("|> {:?}\n{}", function, fmt_pretty(&*node.left.clone().unwrap(), &next_prefix)),

                Syntax::Parenthesis     => format!("|> Parenthesis\n{}",        fmt_pretty(&*node.left.clone().unwrap(), &next_prefix)),
                Syntax::Addition        => format!("|> Addition\n{}{}",         fmt_pretty(&*node.left.clone().unwrap(), &next_prefix), fmt_pretty(&*node.right.clone().unwrap(), &next_prefix)),
                Syntax::Subtraction     => format!("|> Subtraction\n{}{}",      fmt_pretty(&*node.left.clone().unwrap(), &next_prefix), fmt_pretty(&*node.right.clone().unwrap(), &next_prefix)),
                Syntax::Multiplication  => format!("|> Multiplication\n{}{}",   fmt_pretty(&*node.left.clone().unwrap(), &next_prefix), fmt_pretty(&*node.right.clone().unwrap(), &next_prefix)),
                Syntax::Division        => format!("|> Division\n{}{}",         fmt_pretty(&*node.left.clone().unwrap(), &next_prefix), fmt_pretty(&*node.right.clone().unwrap(), &next_prefix)),
                Syntax::Exponent        => format!("|> Exponent\n{}{}",         fmt_pretty(&*node.left.clone().unwrap(), &next_prefix), fmt_pretty(&*node.right.clone().unwrap(), &next_prefix)),
                Syntax::Absolute        => format!("|> Absolute\n{}",           fmt_pretty(&*node.left.clone().unwrap(), &next_prefix)),
            };
            format!("{}{}", prefix, result)
        }
        write!(f, "{}", fmt_pretty(self, ""))
    }
}








pub fn parse<V: Keyword, F: Keyword>(input: &str) -> Result<Node, SyntaxError> {
    Ok(minimize(&parse_syntax::<V, F>(&parse_tokens::<V, F>(&sanitize(&input)))?))
}
pub fn sanitize(input: &str) -> String {
    input.replace(|c| {
        match c {
            '\x00'..='\x1f' | '\x7f' => true,
            _ => false,
        }
    }, "")
}
pub fn minimize(root: &Node) -> Node {
    let mut new = root.clone();

    new.left = match &root.left {
        Some(left_node) => Some(Box::new(minimize(&*left_node))),
        None => None,
    };
    new.right = match &root.right {
        Some(right_node) => Some(Box::new(minimize(&*right_node))),
        None => None,
    };


    match &root.syntax {
        Syntax::Addition => match new.child_syntax() {
            // x + y
            (Syntax::Real(lhs), Syntax::Real(rhs)) => {
                new.syntax = Syntax::Real(lhs + rhs);
            }
            // xi + yi
            (Syntax::Imaginary(lhs), Syntax::Imaginary(rhs)) => {
                new.syntax = Syntax::Imaginary(lhs + rhs);
            }


            // a + a = 2a
            (Syntax::Variable(id_lhs), Syntax::Variable(id_rhs)) => {
                if id_lhs == id_rhs {
                    new.syntax = Syntax::Multiplication;
                    new.left = Some(Box::new(Node::from_syntax(Syntax::Real(2.0))));
                }
            }
            // a + xa = (x + 1)a
            (Syntax::Variable(id_lhs), Syntax::Multiplication) => {
                if let Some(op_rhs) = &mut new.right {
                    match op_rhs.child_syntax() {
                        // ax
                        (Syntax::Variable(id_rhs), Syntax::Real(value)) => {
                            if id_lhs == id_rhs {
                                new.syntax = Syntax::Multiplication;
                                **op_rhs = Node::from_syntax(Syntax::Real(value + 1.0));
                            }
                        }
                        // xa
                        (Syntax::Real(value), Syntax::Variable(id_rhs)) => {
                            if id_lhs == id_rhs {
                                new.syntax = Syntax::Multiplication;
                                **op_rhs = Node::from_syntax(Syntax::Real(value + 1.0));
                            }
                        }
                        _ => (),
                    }
                }
            }
            // xa + a = (x + 1)a
            (Syntax::Multiplication, Syntax::Variable(id_rhs)) => {
                if let Some(op_lhs) = &mut new.left {
                    match op_lhs.child_syntax() {
                        // ax
                        (Syntax::Variable(id_lhs), Syntax::Real(value)) => {
                            if id_lhs == id_rhs {
                                new.syntax = Syntax::Multiplication;
                                **op_lhs = Node::from_syntax(Syntax::Real(value + 1.0));
                            }
                        }
                        // xa
                        (Syntax::Real(value), Syntax::Variable(id_lhs)) => {
                            if id_lhs == id_rhs {
                                new.syntax = Syntax::Multiplication;
                                **op_lhs = Node::from_syntax(Syntax::Real(value + 1.0));
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        Syntax::Subtraction => match new.child_syntax() {
            // x - y
            (Syntax::Real(lhs), Syntax::Real(rhs)) => {
                new.syntax = Syntax::Real(lhs - rhs);
            }
            // xi - yi
            (Syntax::Imaginary(lhs), Syntax::Imaginary(rhs)) => {
                new.syntax = Syntax::Imaginary(lhs - rhs);
            }
            _ => (),
        }
        Syntax::Multiplication => match new.child_syntax() {
            // x * y = xy
            (Syntax::Real(lhs), Syntax::Real(rhs)) => {
                new.syntax = Syntax::Real(lhs * rhs);
            }
            // xi * yi = xy
            (Syntax::Imaginary(lhs), Syntax::Imaginary(rhs)) => {
                let value = (Complex::new(0.0, lhs) * Complex::new(0.0, rhs)).re;
                new.syntax = Syntax::Real(value);
            }
            // xi * y = xiy
            (Syntax::Imaginary(lhs), Syntax::Real(rhs)) => {
                new.syntax = Syntax::Imaginary(lhs / rhs);
            }
            // x * yi = xyi
            (Syntax::Real(lhs), Syntax::Imaginary(rhs)) => {
                new.syntax = Syntax::Imaginary(lhs / rhs);
            }


            // a * a = a^2
            (Syntax::Variable(id_lhs), Syntax::Variable(id_rhs)) => {
                if id_lhs == id_rhs {
                    if let Some(node) = &mut new.right {
                        new.syntax = Syntax::Exponent;
                        node.syntax = Syntax::Real(2.0);
                    }
                }
            }
            // a * a^n = a^(n + 1)
            (Syntax::Variable(id_lhs), Syntax::Exponent) => {
                if let Some(rhs) = &mut new.right {
                    match rhs.child_syntax() {
                        (Syntax::Variable(id_rhs), Syntax::Real(value)) => {
                            if id_lhs == id_rhs {
                                new.syntax = Syntax::Exponent;
                                **rhs = Node::from_syntax(Syntax::Real(value + 1.0));
                            }
                        }
                        _ => (),
                    }
                }
            }
            // a^n * a = a^(n + 1)
            (Syntax::Exponent, Syntax::Variable(id_rhs)) => {
                if let Some(op_lhs) = &mut new.left {
                    match op_lhs.child_syntax() {
                        (Syntax::Variable(id_lhs), Syntax::Real(value)) => {
                            if id_lhs == id_rhs {
                                new.syntax = Syntax::Exponent;
                                new.right = Some(Box::new(Node::from_syntax(Syntax::Real(value + 1.0))));
                                new.left = Some(Box::new(Node::from_syntax(Syntax::Variable(id_lhs))));
                            }
                        }
                        _ => (),
                    }
                }
            }
            // a^n * a^k = a^(n + k)
            (Syntax::Exponent, Syntax::Exponent) => {
                if let (Some(op_lhs), Some(op_rhs)) = (&mut new.left, &mut new.right) {
                    if let (Syntax::Variable(id_lhs), Syntax::Real(value_lhs)) = op_lhs.child_syntax() {
                        if let (Syntax::Variable(id_rhs), Syntax::Real(value_rhs)) = op_rhs.child_syntax() {
                            if id_lhs == id_rhs {
                                new.syntax = Syntax::Exponent;
                                new.right = Some(Box::new(Node::from_syntax(Syntax::Real(value_lhs + value_rhs))));
                                new.left = Some(Box::new(Node::from_syntax(Syntax::Variable(id_lhs))));
                            }
                        }
                    }
                }
            }

            // a * xa^k
            _ => (),
        }
        Syntax::Division => match new.child_syntax() {
            (Syntax::Real(lhs), Syntax::Real(rhs)) => {
                new.syntax = Syntax::Real(lhs / rhs);
            }
            (Syntax::Imaginary(lhs), Syntax::Imaginary(rhs)) => {
                let value = (Complex::new(0.0, lhs) / Complex::new(0.0, rhs)).re;
                new.syntax = Syntax::Real(value);
            }
            (Syntax::Imaginary(lhs), Syntax::Real(rhs)) => {
                new.syntax = Syntax::Imaginary(lhs * rhs);
            }
            (Syntax::Real(lhs), Syntax::Imaginary(rhs)) => {
                new.syntax = Syntax::Imaginary(lhs * rhs);
            }
            _ => (),
        }
        Syntax::Exponent => match new.child_syntax() {
            (Syntax::Real(lhs), Syntax::Real(rhs)) => {
                new.syntax = Syntax::Real(lhs.powf(rhs));
            }
            _ => (),
            //(Syntax::Imaginary(lhs), Syntax::Imaginary(rhs)) => {
            //    let value = (Complex::new(0.0, lhs) / Complex::new(0.0, rhs)).re;
            //    new.syntax = Syntax::Real(value);
            //}
            //(Syntax::Imaginary(lhs), Syntax::Real(rhs)) => {
            //    new.syntax = Syntax::Imaginary(lhs * rhs);
            //}
            //(Syntax::Real(lhs), Syntax::Imaginary(rhs)) => {
            //    new.syntax = Syntax::Imaginary(lhs * rhs);
            //}
        }
        _ => (),
    };
    new
}



pub fn parse_tokens<V: Keyword, F: Keyword>(input: &str) -> Vec<Token<V, F>> {
    let input = sanitize(input);
    
    let mut result = Vec::new();
    let mut delimiters: HashMap<char, bool> = HashMap::new();

    let mut index: isize = 0;
    let mut iter = input.chars().peekable();
    while let Some(&c) = iter.peek() {
        match c {
            ' ' => { index -= 1; iter.next(); },
            
            'a'..='z' | 'A'..='Z'   => result.push(parse_keyword::<V, F, _>(c, &mut iter, index as usize)),
            '0'..='9' | '.'         => result.push(parse_complex(c, &mut iter, index as usize)),

            '(' => { result.push(Token::Type(TokenType::ParenthesisOpen));      iter.next(); },
            ')' => { result.push(Token::Type(TokenType::ParenthesisClosed));    iter.next(); },
            '+' => { result.push(Token::Type(TokenType::Addition));             iter.next(); },
            '-' => { result.push(Token::Type(TokenType::Subtraction));          iter.next(); },
            '*' => { result.push(Token::Type(TokenType::Multiplication));       iter.next(); },
            '/' => { result.push(Token::Type(TokenType::Division));             iter.next(); },
            '^' => { result.push(Token::Type(TokenType::Exponent));             iter.next(); },

            '|' => {
                delimiters.entry('|').and_modify(|state| {
                    match state {
                        true => result.push(Token::Type(TokenType::AbsoluteClosed)),
                        false => result.push(Token::Type(TokenType::AbsoluteOpen)),
                    }
                    *state = !*state;
                }).or_insert_with(|| {
                    result.push(Token::Type(TokenType::AbsoluteOpen));
                    true
                });
                iter.next(); 
            },

            _ => { 
                result.push(Token::Invalid(TokenError::InvalidCharacter(index as usize))); 
                iter.next(); 
            },
        };
        index += 1;
    
    }
    
    result
}
fn parse_complex<V, F, T>(first: char, iter: &mut Peekable<T>, index: usize) -> Token<V, F>
    where
        T: Iterator<Item = char>,
        V: Keyword,
        F: Keyword,
{
    let mut buffer = first.to_string();
    let mut is_imaginary = false;

    iter.next();
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' | '.' => {
                buffer.push(c);
                iter.next();
            },
            'i' => {
                is_imaginary = true;
                iter.next();
                break;
            },
            '_' => { iter.next(); },
            _ => break,
        };
    }
    match buffer.parse::<f64>() {
        Ok(value) => match is_imaginary {
            true => Token::Imaginary(value),
            false => Token::Real(value),
        },
        Err(_) => Token::Invalid(TokenError::InvalidNumber(index)),
    }
}
fn parse_keyword<V, F, T>(first: char, iter: &mut Peekable<T>, index: usize) -> Token<V, F> 
    where
        T: Iterator<Item = char>,
        V: Keyword,
        F: Keyword,
{
    let mut keyword = first.to_string();

    iter.next();
    while let Some(&c) = iter.peek() {

        
        match c {
            'a'..='z' | 'A'..='Z' => keyword.push(c),
            '_' => (),
            _ => break,
        };
        iter.next();
    };

    match V::from_str(&keyword) {
        Ok(variable) => match F::from_str(&keyword) {
            Ok(_) => return Token::Invalid(TokenError::DuplicateKeyword(index)),
            Err(_) => return Token::Keyword(TokenKeyword::Variable(variable)),
        }
        Err(_) => match F::from_str(&keyword) {
            Ok(function) => return Token::Keyword(TokenKeyword::Function(function)),
            Err(_) => match &keyword as &str {
                "i" => Token::Imaginary(1.0),
                _ => Token::Invalid(TokenError::InvalidKeyword(index)),
            },
        }
    }
}
/*

// This can be biuld into analyse_lex
// but instead do the minimization on the parse tree
// that just makes it easier to do intelligent minimizing
// remember to shorthand number as in: (5 + 9) = (14)
pub fn lexical_minimize<V: KeywordEnum + ToString + Copy, F: KeywordEnum + ToString + Copy>(tokens: Vec<Token<V, F>>) -> Vec<Token<V, F>> {
    let mut result = Vec::new();

    let mut iter = tokens.iter().enumerate();
    while let Some((index, token)) = iter.next() {
        let next = tokens.get(index + 1);
        match token {
            Token::ParenthesisOpen => match next {
                // This is not all inclusive
                Some(Token::ParenthesisClosed) => { iter.next(); },
                _ => result.push(token.clone()),
            },
            Token::Type(TokenType::Subtraction => match next {
                Some(Token::Type(TokenType::Subtraction) => match tokens.get(index + 2) {
                    Some(Token::Number(_)) | Some(Token::Imaginary(_)) | Some(Token::ParenthesisOpen) | Some(Token::Variable(_)) => {
                        result.push(Token::Type(TokenType::Addition);
                        iter.next();
                    },
                    Some(Token::Type(TokenType::Subtraction) => { iter.next(); },
                    _ => (),
                },
                _ => result.push(token.clone()),
            },
            Token::Type(TokenType::Addition => match next {
                Some(Token::Type(TokenType::Addition) => match tokens.get(index + 2) {
                    Some(Token::Number(_)) | Some(Token::Imaginary(_)) | Some(Token::ParenthesisOpen) | Some(Token::Variable(_)) => {
                        result.push(Token::Type(TokenType::Addition);
                        iter.next();
                    },
                    Some(Token::Type(TokenType::Addition) => { iter.next(); },
                    _ => (),
                },
                _ => result.push(token.clone()),
            }
            _ => result.push(token.clone()),
        }
    }

    // Remove prefix
    match result.get(0) {
        Some(Token::Type(TokenType::Addition) => { result.remove(0); },
        Some(Token::Type(TokenType::Subtraction) => {
            result.remove(0);
            match result.get_mut(1) {
                Some(token) => match token {
                    Token::Number(value) => *token = Token::Number(-*value),
                    Token::Imaginary(value) => *token = Token::Imaginary(-*value),
                    _ => (),
                }
                _ => (),
            }
        }
        _ => (),
    } 

    result
}

*/



pub fn parse_syntax<V: Keyword, F: Keyword>(tokens: &Vec<Token<V, F>>) -> Result<Node, SyntaxError> {
    match parse_expression(tokens, 0) {
        Ok((node, index)) => {
            if index == tokens.len() {
                Ok(node)
            } else {
                Err(SyntaxError::UnexpectedEndOfInput(index))
            }
        }
        Err(error) => Err(error),
    }
}
fn parse_expression<V, F>(tokens: &Vec<Token<V, F>>, index: usize) -> Result<(Node, usize), SyntaxError>
    where
        V: Keyword,
        F: Keyword,
{
    let (node, next_index) = parse_factor(tokens, index)?;
    let token = tokens.get(next_index);
    match token {
        Some(Token::Type(TokenType::Addition)) => {
            let (right, i) = parse_expression(tokens, next_index + 1)?;
            Ok((Node::from(node, right, Syntax::Addition), i))
        }
        Some(Token::Type(TokenType::Subtraction)) => {
            let (right, i) = parse_expression(tokens, next_index + 1)?;
            Ok((Node::from(node, right, Syntax::Subtraction), i))
        }
        _ => {
            Ok((node, next_index))
        }
    }
}
fn parse_factor<V, F>(tokens: &Vec<Token<V, F>>, index: usize) -> Result<(Node, usize), SyntaxError>
    where
        V: Keyword,
        F: Keyword,
{
    let (node, next_index) = parse_exponent(tokens, index)?;
    let token = tokens.get(next_index);
    match token {
        Some(Token::Type(TokenType::Multiplication)) => {
            let (right, i) = parse_factor(tokens, next_index + 1)?;
            Ok((Node::from(node, right, Syntax::Multiplication), i))
        }
        Some(Token::Type(TokenType::Division)) => {
            let (right, i) = parse_factor(tokens, next_index + 1)?;
            Ok((Node::from(node, right, Syntax::Division), i))
        },
        _ => {
            Ok((node, next_index))
        },
    }
}
// TODO: rename the "i"s to next_index.
fn parse_exponent<V, F>(tokens: &Vec<Token<V, F>>, index: usize) -> Result<(Node, usize), SyntaxError>
    where
        V: Keyword,
        F: Keyword,
{
    let (node, next_index) = parse_preterm(tokens, index)?;
    let token = tokens.get(next_index);
    match token {
        Some(Token::Type(TokenType::Exponent)) => {
            let (right, next_index) = parse_exponent(tokens, next_index + 1)?;
            Ok((Node::from(node, right, Syntax::Exponent), next_index))
        }
        _ => {
            Ok((node, next_index))
        }
    }
}
fn parse_preterm<V, F>(tokens: &Vec<Token<V, F>>, index: usize) -> Result<(Node, usize), SyntaxError>
    where
        V: Keyword,
        F: Keyword,
{
    let token = tokens.get(index).ok_or(SyntaxError::UnexpectedEndOfInput(index))?;
    match token {
        Token::Type(TokenType::Subtraction) => {
            // Allows - symbol in front of expression.
            let (right, i) = parse_preterm(tokens, index + 1)?;
            Ok((Node::from(Node::from_syntax(Syntax::Real(0.0)), right, Syntax::Subtraction), i))
        },
        Token::Type(TokenType::Addition) => {
            // Allows + symbol in front of expression.
            let (right, i) = parse_preterm(tokens, index + 1)?;
            Ok((Node::from(Node::from_syntax(Syntax::Real(0.0)), right, Syntax::Addition), i))
        },
        _ => parse_term(tokens, index),
    }
}
fn parse_term<V, F>(tokens: &Vec<Token<V, F>>, index: usize) -> Result<(Node, usize), SyntaxError>
    where
        V: Keyword,
        F: Keyword,
{
    let token = tokens.get(index).ok_or(SyntaxError::UnexpectedEndOfInput(index))?;
    match token {
        Token::Keyword(TokenKeyword::Variable(variable)) => {
            let node= Node::from_syntax(Syntax::Variable(variable.serialize()));
            parse_implicit(tokens, index + 1, node)
        }
        Token::Keyword(TokenKeyword::Function(function)) => {
            // TODO: handle case where no opening paren is present.
            // TODO: that should be done for every delimiter!
            parse_delimiter(tokens, index + 2, Syntax::Function(function.serialize()), TokenType::ParenthesisClosed)             
        },
        Token::Real(value) => {
            let node = Node::from_syntax(Syntax::Real(*value));
            parse_implicit(tokens, index + 1, node)
        },
        Token::Imaginary(value) => {
            let node = Node::from_syntax(Syntax::Imaginary(*value));
            parse_implicit(tokens, index + 1, node)
        },
        Token::Type(TokenType::ParenthesisOpen) => {
            parse_delimiter(tokens, index + 1, Syntax::Parenthesis, TokenType::ParenthesisClosed)             
        },
        /*Token::ParenthesisClosed => {
            // TODO: fix
            Err(SyntaxError::MissingClosingDelimiter(index))
        }*/
        Token::Type(TokenType::AbsoluteOpen) => {
            parse_delimiter(tokens, index + 1, Syntax::Absolute, TokenType::AbsoluteClosed)             
        }

        Token::Invalid(invalid) => Err(SyntaxError::TokenError(invalid.clone())),
        token => Err(SyntaxError::UnexpectedToken(index)),
    }
}
fn parse_delimiter<V, F>(tokens: &Vec<Token<V, F>>, index: usize, syntax: Syntax, closing_delimiter: TokenType) 
    -> Result<(Node, usize), SyntaxError>
    where
        V: Keyword,
        F: Keyword,
{
    match parse_expression(tokens, index) {
        Ok((node, next_index)) => {
            match tokens.get(next_index) {
                Some(Token::Type(token)) => {
                    if *token == closing_delimiter {
                       let node = Node::new(Some(node), None, syntax);
                        parse_implicit(tokens, next_index + 1, node)
                    } else {
                        Err(SyntaxError::MissingClosingDelimiter(index))
                    }
                }
                _ => Err(SyntaxError::MissingClosingDelimiter(index))
            }
        }
        Err(SyntaxError::UnexpectedToken(_)) => {
            Err(SyntaxError::EmptyDelimiter(index))
        }
        Err(SyntaxError::UnexpectedEndOfInput(_))=> {
            Err(SyntaxError::MissingClosingDelimiter(index))
        }
        Err(error) => Err(error),
    }
}
fn parse_implicit<V, F>(tokens: &Vec<Token<V, F>>, index: usize, node: Node) 
    -> Result<(Node, usize), SyntaxError>
    where
        V: Keyword,
        F: Keyword,
{
    match parse_term(tokens, index) {
        Ok((right, next_index)) => {
            // Allows implicit multiplication between terms.
            Ok((Node::from(node, right, Syntax::Multiplication), next_index))
        }
        Err(SyntaxError::UnexpectedEndOfInput(_)) | Err(SyntaxError::UnexpectedToken(_))=> {
            Ok((node, index))
        }
        Err(error) => Err(error),
    }
}
