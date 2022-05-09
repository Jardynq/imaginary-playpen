use std::io::BufRead;
use std::collections::HashMap;
use super::parser::{ Expression, Keyword, Node, Syntax };
use super::complex::Complex;




pub fn sanitize_input(input: &str) -> String {
    input.replace(|c| {
        match c {
            '\x00'..='\x1f' | '\x7f' => true,
            _ => false,
        }
    }, "")
}




#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ReplCommand {
    None,
    Exit,
    Evaluate,
    SetVariable(String, Complex),
    RemoveVariable(String),
}
pub struct Intepreter {
    variables: HashMap<String, Complex>,
    //functions: HashMap<String, dyn Fn(Complex) -> Complex>,
}
impl Intepreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            //functions: HashMap::new(),
        }
    }


    pub fn repl_handler<F: Fn(Expression) -> Vec<ReplCommand>>(&mut self, handler: F) {
        println!("Entered repl mode. Currently capturing input.");
        println!("Type 'exit' to escape.");

        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        loop {
            let mut input = String::new();
            match handle.read_line(&mut input) {
                Err(error) => {
                    println!("Failed to read standard in: {}", error);
                }
                Ok(_) => (),
            };
            let input = sanitize_input(&input);
            match &input as &str {
                "exit" => break,
                _ => (),
            }

            let vars = Keyword::vars(&self.variables.keys().map(|s| s as &str).collect());
            let expr =  Expression::parse(&input, &vars);
            if !expr.is_valid() {
                println!("{}", expr.pretty_error());
                continue;
            }


            for command in handler(expr.clone()) {
                match command {
                    ReplCommand::None => (),
                    ReplCommand::Exit => return,
                    ReplCommand::Evaluate => println!("= {}\n", self.evaluate(&expr)),
                    ReplCommand::SetVariable(name, value) => self.set_variable(&name, value),
                    ReplCommand::RemoveVariable(name) => { self.variables.remove(&name); },
                }
            }
        }
    }
    pub fn repl(&mut self) {
        self.repl_handler(|_| vec![ReplCommand::Evaluate]);
    }


    pub fn set_variable(&mut self, name: &str, value: Complex) {
        self.variables.entry(format!("{}", name))
            .and_modify( |old| *old = value)
            .or_insert(value);
    }




    pub fn evaluate(&self, expr: &Expression) -> Complex {
        match &expr.tree {
            Some(root) => self.evaluate_node(&root),
            None => Complex::zero(),
        }
    }
    fn evaluate_node_left(&self, node: &Node) -> Complex {
        match &node.left {
            Some(left)=> self.evaluate_node(&left),
            None => Complex::zero(),
        }
    }
    fn evaluate_node_right(&self, node: &Node) -> Complex {
        match &node.right {
            Some(right)=> self.evaluate_node(&right),
            None => Complex::zero(),
        }
    }
    fn evaluate_node(&self, node: &Node) -> Complex {
        match &node.syntax {
            Syntax::Real(value) => Complex::real(*value),
            Syntax::Imaginary(value) => Complex::imaginary(*value),

            Syntax::Parenthesis => {
                self.evaluate_node_left(&node)
            },

            Syntax::Addition => {
                let lhs = self.evaluate_node_left(&node);
                let rhs = self.evaluate_node_right(&node);
                lhs + rhs
            },
            Syntax::Subtraction => {
                let lhs = self.evaluate_node_left(&node);
                let rhs = self.evaluate_node_right(&node);
                lhs - rhs
            },
            Syntax::Multiplication => {
                let lhs = self.evaluate_node_left(&node);
                let rhs = self.evaluate_node_right(&node);
                lhs * rhs
            },
            Syntax::Division => {
                let lhs = self.evaluate_node_left(&node);
                let rhs = self.evaluate_node_right(&node);
                lhs / rhs
            },
            Syntax::Exponent => {
                let lhs = self.evaluate_node_left(&node);
                let rhs = self.evaluate_node_right(&node);
                lhs.pow(rhs)
            },
            Syntax::Absolute => {
                let lhs = self.evaluate_node_left(&node);
                Complex::real(lhs.abs())
            },
            _ => Complex::zero(),


            Syntax::Variable(name) => {
                match self.variables.get(name) {
                    Some(value) => *value,
                    None => Complex::zero(),
                }
            }
            Syntax::Function(name) => {
                Complex::zero()
            }
        }
    }
}
