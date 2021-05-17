extern crate num_bigint;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate clap;
use clap::{App, Arg, ArgMatches};
use num_bigint::BigInt;

pub mod ast;
pub mod engine;

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
        //println!("{}", term);
        let number = BigInt::parse_bytes(term.as_bytes(), 10).unwrap();
        seq.push(ast::Step::Number { value: number });
    }
    seq
}

fn first_parse(input: &str) -> (ast::Ast, engine::Sequence) {
    let limit = input.find(";").unwrap();
    return (
        ast::generate(&input[0..limit]),
        parse_first_terms(&input[limit + 1..]),
    );
}

fn step_to_str(step: ast::Step) -> String {
    match step {
        ast::Step::Number { value } => value.to_string(),
        ast::Step::Boolean { value } => {
            if value {
                String::from("true")
            } else {
                String::from("false")
            }
        }
        ast::Step::String { value } => value,
        _ => String::from(""),
    }
}

pub fn run(input: &str) -> String {
    let (mut ast, mut first_terms) = first_parse(input);
    print_ast(&ast);
    let sequence = engine::execute(&ast, &mut first_terms);
    for term in sequence {
        println!("{}", step_to_str(term.clone()))
    }
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
