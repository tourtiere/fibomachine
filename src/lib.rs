extern crate num_bigint;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate clap;
extern crate wasm_bindgen;

//use clap::{App, Arg, ArgMatches};
use engine::Sequence;
use num_bigint::BigInt;
use wasm_bindgen::prelude::*;

pub mod ast;
pub mod engine;
pub mod error;

pub fn parse_first_terms(stack: &str) -> engine::Sequence {
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

fn printable_sequence(sequence: &Sequence) -> String {
    sequence
        .iter()
        .map(|term| match term {
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
        })
        .collect::<Vec<String>>()
        .join(",")
}

#[wasm_bindgen]
pub fn run(input: &str) -> String {
    let (ast, mut first_terms) = first_parse(input);
    /*
    for (idx, step) in ast.iter().enumerate() {
        println!("{} - {:?}", idx, step);
    }
    */
    let sequence = engine::execute(&ast, &mut first_terms);
    match sequence {
        Ok(sequence) => printable_sequence(sequence),
        Err(err) => format!("{}", err),
    }
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