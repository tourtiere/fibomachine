extern crate num_bigint;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod engine;

use num_bigint::BigInt;

pub fn execute(formula: &str) -> Option<BigInt> {
    let ast = ast::generate(formula);
    println!("{:?}", ast);
    engine::walk_engine(ast)
}

fn main() {
    println!("{:?} ", execute("5/2"));
}

#[test]
fn check_answer_validity() {
    assert_eq!(execute("1+2").unwrap().to_string(), "3");
}
