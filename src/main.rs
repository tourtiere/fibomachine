extern crate num_bigint;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate clap;

use clap::{App, Arg, ArgMatches};
use engine::Sequence;
use num_bigint::BigInt;
use std::fmt;

pub mod ast;
pub mod engine;

// Custom Error type

type Result<T> = std::result::Result<T, ParseError>;
#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn extract_commandline_args<'a>() -> ArgMatches<'a> {
    App::new("Arbitrary long integer calculator")
        .author("Tourtiere")
        .arg(Arg::with_name("EXPRESSION").required(true))
        .get_matches()
}

#[allow(dead_code)]
fn print_ast(ast: &ast::Ast) {
    for (idx, step) in ast.iter().enumerate() {
        println!("{} - {:?}", idx, step);
    }
}

fn parse_first_terms(stack: &str) -> engine::Sequence {
    let mut seq: engine::Sequence = Vec::new();
    for term in stack.split(",") {
        let number = BigInt::parse_bytes(term.as_bytes(), 10).unwrap();
        let value = ast::Value::Number { value: number };
        seq.push(value);
    }
    seq
}

pub fn first_parse(input: &str) -> (ast::Ast, engine::Sequence) {
    match input.find(";") {
        Some(limit) => (
            ast::generate(&input[0..limit]),
            parse_first_terms(&input[limit + 1..]),
        ),
        None => (ast::generate(input), vec![]),
    }
}

fn print_sequence(sequence: &Sequence) {
    for term in sequence {
        let str_representation = match term {
            ast::Value::Number { value } => value.to_string(),
            ast::Value::Boolean { value } => {
                if *value {
                    String::from("true")
                } else {
                    String::from("false")
                }
            }
            ast::Value::String { value } => value.clone(),
            _ => String::from(""),
        };
        println!("{}", str_representation);
    }
}

pub fn run(input: &str) -> String {
    let (ast, mut first_terms) = first_parse(input);
    print_ast(&ast);
    let sequence = engine::execute(&ast, &mut first_terms);
    print_sequence(sequence);
    String::from("")
}

fn main() {
    let matches = extract_commandline_args();
    let formula = matches.value_of("EXPRESSION").unwrap();
    println!("{}", run(formula));
}

#[test]
fn check_formula() {
    assert_eq!(run("17/3 + 5*3 - 11"), "9");
}
#[test]
fn check_big_int() {
    let num = "70012410520695638594948118043706280297620717421509";
    let den = "1007242982";
    let result = "69508958386265170914785403829903557766978";
    let formula = &format!("{}/{}", num, den);
    assert_eq!(run(formula), result);
}
