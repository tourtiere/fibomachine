extern crate num_bigint;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate clap;
use clap::{App, Arg, ArgMatches};

pub mod ast;

pub mod engine;

fn extract_commandline_args<'a>() -> ArgMatches<'a> {
    App::new("Arbitrary long integer calculator")
        .author("Tourtiere")
        .arg(Arg::with_name("EXPRESSION").required(true))
        .get_matches()
}

use num_bigint::BigInt;

#[allow(dead_code)]
fn print_ast(ast: &ast::Ast) {
    for (idx, step) in ast.iter().enumerate() {
        println!("{} - {:?}", idx, step);
    }
}
pub fn run(formula: &str) -> Option<BigInt> {
    let ast = ast::generate(formula);
    engine::excute(ast)
}

fn main() {
    let matches = extract_commandline_args();
    let formula = matches.value_of("EXPRESSION").unwrap();
    println!("{}", run(formula).unwrap().to_string());
}

#[test]
fn check_formula() {
    assert_eq!(run("17/3 + 5*3 - 11").unwrap().to_string(), "9");
}
#[test]
fn check_big_int() {
    let num = "70012410520695638594948118043706280297620717421509";
    let den = "1007242982";
    let result = "69508958386265170914785403829903557766978";
    let formula = &format!("{}/{}", num, den);
    assert_eq!(run(formula).unwrap().to_string(), result);
}
