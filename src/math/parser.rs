use std::iter::Peekable;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use super::complex::Complex;



/*
// Validate duplicate definition in wrapper struct araung Vec<KeywordType>
pub enum KeywordError {
    DuplicateVariable(String),
    DuplicateFunction(String),
}
pub struct Keywords {
    definitions: Result<Vec<KeywordType>, Vec<KeywordError>>
}
impl Keywords {
    pub fn is_valid() -> bool {
        false
    }

    pub fn new(variables: Vec<String>, functions: Vec<String>) -> Self {

    }
    pub fn vars(input: Vec<String>) -> Option<Self> {
        let vars = input.iter().map(|name| {
            Self::Variable(name)
        }).collect();

        
    }
    pub fn funcs(input: Vec<String>) -> Option<Self> {
        input.iter().map(|name| {
            Self::Function(name)
        }).collect()
    }


    pub fn 

    fn validate(&self) {

    }
}
impl std::ops::Add {
}*/

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Function(String),
    Variable(String),
}
impl Keyword {
    pub fn vars(input: &Vec<&str>) -> Vec<Self> {
        input.iter().map(|&name| {
            Self::Variable(format!("{}", name))
        }).collect()
    }
    pub fn funcs(input: &Vec<&str>) -> Vec<Self> {
        input.iter().map(|&name| {
            Self::Function(format!("{}", name))
        }).collect()
    }
}



#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Real(f64),
    Imaginary(f64),
    Variable(String),
    Function(String),
    Invalid(TokenError),

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
impl Token {
    pub fn parse(input: &str, keywords: &Vec<Keyword>) -> Vec<Self> {
        let mut result = Vec::new();
        let mut delimiters: HashMap<char, bool> = HashMap::new();
    
        let mut index: isize = 0;
        let mut iter = input.chars().peekable();
        while let Some(&c) = iter.peek() {
            match c {
                ' ' => { index -= 1; iter.next(); },
                
                'a'..='z' | 'A'..='Z' => {
                    result.push(Token::tokenize_keyword(
                        c, 
                        &mut iter, 
                        index as usize,
                        &keywords,
                    ));
                }
                '0'..='9' | '.' => {
                    match result.last() {
                        Some(Token::Real(_)) | Some(Token::Imaginary(_)) => {
                            result.push(Token::Invalid(TokenError::InvalidNumber(index as usize)));
                            iter.next(); 
                        }
                        _ => {
                            result.push(Token::tokenize_number(
                                c, 
                                &mut iter, 
                                index as usize,
                            ));
                        }
                    }
                }

                '(' => { result.push(Token::ParenthesisOpen);      iter.next(); },
                ')' => { result.push(Token::ParenthesisClosed);    iter.next(); },
                '+' => { result.push(Token::Addition);             iter.next(); },
                '-' => { result.push(Token::Subtraction);          iter.next(); },
                '*' => { result.push(Token::Multiplication);       iter.next(); },
                '/' => { result.push(Token::Division);             iter.next(); },
                '^' => { result.push(Token::Exponent);             iter.next(); },
    
                '|' => {
                    result.push(Token::tokenize_delimiter( 
                        '|', 
                        &Token::AbsoluteOpen, &Token::AbsoluteClosed, 
                        &mut delimiters,
                    ));
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
    fn tokenize_number<T>(first: char, iter: &mut Peekable<T>, index: usize) -> Token
        where T: Iterator<Item = char>
    {
        let mut buffer = first.to_string();
    
        iter.next();
        while let Some(&c) = iter.peek() {
            match c {
                '0'..='9' | '.' => buffer.push(c),
                '_' => (),
                _ => break,
            };
            iter.next();
        }
        match buffer.parse::<f64>() {
            Ok(value) => Token::Real(value),
            Err(_) => Token::Invalid(TokenError::InvalidNumber(index)),
        }
    }
    fn tokenize_keyword<T>(first: char, iter: &mut Peekable<T>, index: usize, keywords: &Vec<Keyword>) -> Token
        where T: Iterator<Item = char>
    {
        let mut keyword = first.to_string();

        iter.next();
        while let Some(&c) = iter.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => keyword.push(c),
                _ => break,
            };
            iter.next();
        };
        if keyword == "i" {
            return Token::Imaginary(1.0);
        }
        
        let mut token = Token::Invalid(TokenError::InvalidKeyword(index));
        for definition in keywords {
            match &definition {
                Keyword::Function(name) => {
                    if name == &keyword {
                        match token {
                            Token::Function(_) | Token::Variable(_) =>{
                                return Token::Invalid(TokenError::DuplicateKeyword(index));
                            }
                            _ => {
                                token = Token::Function(keyword.clone());
                            }
                        }
                    }
                }
                Keyword::Variable(name) => {
                    if name == &keyword {
                        match token {
                            Token::Function(_) | Token::Variable(_) =>{
                                return Token::Invalid(TokenError::DuplicateKeyword(index));
                            }
                            _ => {
                                token = Token::Variable(keyword.clone());
                            }
                        }
                    }
                }
            }
        }
        token
    }
    fn tokenize_delimiter(delimiter: char, open: &Token, closed: &Token, delimiters: &mut HashMap<char, bool>) -> Token {
        let mut token = open.clone();
        delimiters.entry(delimiter).and_modify(|state| {
            match state {
                true => token = closed.clone(),
                false => token = open.clone(),
            }
            *state = !*state;
        }).or_insert_with(|| {
            token = open.clone();
            true
        });
        token
    }
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




#[derive(Debug, Clone, PartialEq)]
pub enum Syntax {
    Ignore,
    
    Real(f64),
    Imaginary(f64),
    Variable(String),
    Function(String),

    Parenthesis,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
    Absolute,
}

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
            syntax,
        }
    }
    pub fn from(left: Node, right: Node, syntax: Syntax) -> Self {
        Self {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            syntax,
        }
    }
    pub fn from_syntax(syntax: Syntax) -> Self {
        Self {
            left: None,
            right: None,
            syntax,
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
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn fmt_pretty(node: &Node, prefix: &str) -> String {
            let next_prefix = format!("{}|   ", prefix);
            let result = match &node.syntax {
                Syntax::Ignore => format!(""),

                Syntax::Real(value) => format!("|> {}\n", value),
                Syntax::Imaginary(value) => format!("|> {}i\n", value),
                Syntax::Variable(variable)  => format!("|> {}\n", variable),
                Syntax::Function(function)  => format!("|> {}\n{}", function, fmt_pretty(&*node.left.clone().unwrap(), &next_prefix)),

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



#[derive(Debug, Clone)]
pub struct Expression {
    pub input: String,
    pub tree: Option<Node>,
    pub error: Option<SyntaxError>,
}
impl Expression {
    pub fn parse(input: &str, keywords: &Vec<Keyword>) -> Self {
        let mut error = None;
        let mut tree = None;

        let tokens = Token::parse(&input, &keywords);
        match Self::parse_expression(&tokens, 0) {
            Ok((root, index)) => {
                if index == tokens.len() {
                    tree = Some(root);
                } else {
                    error = Some(SyntaxError::UnexpectedEndOfInput(index));
                }
            }
            Err(syntax_error) => {
                error = Some(syntax_error);
            } 
        }

        Self {
            input: format!("{}", input),
            tree: tree,
            error: error,
        }
    }
    pub fn pretty_error(&self) -> String {
        match &self.error {
            Some(error) => error.pretty(&self.input),
            None => match self.tree {
                Some(_) => format!(""),
                None => format!("Uninitialized expression."),
            }
        }
    }
    pub fn is_valid(&self) -> bool {
        self.tree.is_some() && !self.error.is_some()
    }

    fn parse_expression(tokens: &Vec<Token>, index: usize) -> Result<(Node, usize), SyntaxError> {
        let (node, next_index) = Self::parse_factor(tokens, index)?;
        let token = tokens.get(next_index);
        match token {
            Some(Token::Addition) => {
                let (right, i) = Self::parse_expression(tokens, next_index + 1)?;
                Ok((Node::from(node, right, Syntax::Addition), i))
            }
            Some(Token::Subtraction) => {
                let (right, i) = Self::parse_expression(tokens, next_index + 1)?;
                Ok((Node::from(node, right, Syntax::Subtraction), i))
            }
            _ => {
                Ok((node, next_index))
            }
        }
    }
    fn parse_factor(tokens: &Vec<Token>, index: usize) -> Result<(Node, usize), SyntaxError> {
        let (node, next_index) = Self::parse_exponent(tokens, index)?;
        let token = tokens.get(next_index);
        match token {
            Some(Token::Multiplication) => {
                let (right, i) = Self::parse_factor(tokens, next_index + 1)?;
                Ok((Node::from(node, right, Syntax::Multiplication), i))
            }
            Some(Token::Division) => {
                let (right, i) = Self::parse_factor(tokens, next_index + 1)?;
                Ok((Node::from(node, right, Syntax::Division), i))
            },
            _ => {
                Ok((node, next_index))
            },
        }
    }
    // TODO: rename the "i"s to next_index.
    fn parse_exponent(tokens: &Vec<Token>, index: usize) -> Result<(Node, usize), SyntaxError> {
        let (node, next_index) = Self::parse_preterm(tokens, index)?;
        let token = tokens.get(next_index);
        match token {
            Some(Token::Exponent) => {
                let (right, next_index) = Self::parse_exponent(tokens, next_index + 1)?;
                Ok((Node::from(node, right, Syntax::Exponent), next_index))
            }
            _ => {
                Ok((node, next_index))
            }
        }
    }
    fn parse_preterm(tokens: &Vec<Token>, index: usize) -> Result<(Node, usize), SyntaxError> {
        let token = tokens.get(index).ok_or(SyntaxError::UnexpectedEndOfInput(index))?;
        match token {
            Token::Subtraction => {
                // Allows - symbol in front of expression.
                let (right, i) = Self::parse_preterm(tokens, index + 1)?;
                Ok((Node::from(Node::from_syntax(Syntax::Real(0.0)), right, Syntax::Subtraction), i))
            },
            Token::Addition => {
                // Allows + symbol in front of expression.
                let (right, i) = Self::parse_preterm(tokens, index + 1)?;
                Ok((Node::from(Node::from_syntax(Syntax::Real(0.0)), right, Syntax::Addition), i))
            },
            _ => Self::parse_term(tokens, index),
        }
    }
    fn parse_term(tokens: &Vec<Token>, index: usize) -> Result<(Node, usize), SyntaxError> {
        let token = tokens.get(index).ok_or(SyntaxError::UnexpectedEndOfInput(index))?;
        match token {
            Token::Variable(name) => {
                let node= Node::from_syntax(Syntax::Variable(format!("{}", name)));
                Self::parse_implicit(tokens, index + 1, node)
            }
            Token::Function(name) => {
                // TODO: handle case where no opening paren is present.
                // TODO: that should be done for every delimiter!
                Self::parse_delimiter(tokens, index + 2, Syntax::Function(format!("{}", name)), Token::ParenthesisClosed)             
            },
            Token::Real(value) => {
                let node = Node::from_syntax(Syntax::Real(*value));
                Self::parse_implicit(tokens, index + 1, node)
            },
            Token::Imaginary(value) => {
                let node = Node::from_syntax(Syntax::Imaginary(*value));
                Self::parse_implicit(tokens, index + 1, node)
            },
            Token::ParenthesisOpen => {
                Self::parse_delimiter(tokens, index + 1, Syntax::Parenthesis, Token::ParenthesisClosed)             
            },
            /*Token::ParenthesisClosed => {
                // TODO: fix
                Err(SyntaxError::MissingClosingDelimiter(index))
            }*/
            Token::AbsoluteOpen => {
                Self::parse_delimiter(tokens, index + 1, Syntax::Absolute, Token::AbsoluteClosed)             
            }
    
            Token::Invalid(invalid) => Err(SyntaxError::TokenError(invalid.clone())),
            _ => Err(SyntaxError::UnexpectedToken(index)),
        }
    }
    fn parse_delimiter(tokens: &Vec<Token>, index: usize, syntax: Syntax, closing_delimiter: Token) -> Result<(Node, usize), SyntaxError> {
        match Self::parse_expression(tokens, index) {
            Ok((node, next_index)) => {
                match tokens.get(next_index) {
                    Some(token) => {
                        if *token == closing_delimiter {
                           let node = Node::new(Some(node), None, syntax);
                           Self::parse_implicit(tokens, next_index + 1, node)
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
    fn parse_implicit(tokens: &Vec<Token>, index: usize, node: Node) -> Result<(Node, usize), SyntaxError> {
        match Self::parse_term(tokens, index) {
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
    




    // TODO fix optimize since keywords are strings now.
    pub fn optimize(&mut self) {
        self.tree = self.tree.as_ref().and_then(|root| { Some(Self::optimize_child(&root)) });
    }
    fn optimize_child(root: &Node) -> Node {
        let mut new = root.clone();

        new.left = match &root.left {
            Some(left_node) => Some(Box::new(Self::optimize_child(&*left_node))),
            None => None,
        };
        new.right = match &root.right {
            Some(right_node) => Some(Box::new(Self::optimize_child(&*right_node))),
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
}
