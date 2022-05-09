use std::{io::BufRead, io::Read};
use std::collections::HashMap;
use super::parser::{ minimize, parse, SyntaxError };
use super::complex::Complex;
use super::serializer::{Keyword, Empty};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReplCommand {
    None,
    Exit,
    SkipEvaluation,
}
pub struct Intepreter {
    variables: HashMap<String, Complex>,
}
impl Intepreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }


    pub fn repl_handler<F: Fn(&str) -> ReplCommand>(&self, handler: F) {
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

            match handler(&input) {
                ReplCommand::None => (),
                ReplCommand::Exit => break,
                ReplCommand::SkipEvaluation => continue,
            }

            match &input as &str {
                "exit" => break,
                _ => match self.evaluate(&input) {
                    Ok(value) => println!("= {}\n", value),
                    Err(error) => println!("\n{}\n", error.pretty(&input)),
                }
            }
        }
    }
    pub fn repl(&self) {
        self.repl_handler(|_| ReplCommand::None);
    }

    pub fn evaluate(&self, input: &str) -> Result<Complex, SyntaxError> {
        Ok(parse::<Empty, Empty>(input)?.evaluate())
    }
}
