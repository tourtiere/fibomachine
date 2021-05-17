use num_bigint::BigInt;
use pest::iterators::Pair;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GeneratedParser;

pub fn parse(formula: &str) -> Pair<Rule> {
    GeneratedParser::parse(Rule::formula, formula)
        .expect("unsuccessful parse")
        .next()
        .unwrap()
}

#[derive(Debug)]
pub enum Step {
    Number { value: BigInt },
    String { value: String },
    Function { value: String, inputs: Vec<usize> },
    Operation { value: Rule, inputs: Vec<usize> },
}

/*
#[derive(Debug)]

pub struct Step {
    pub value: StepValue,
    pub inputs: Vec<usize>,
}
 */
pub type Ast = Vec<Step>;

pub fn walk_ast(ast: &mut Ast, token: Pair<Rule>) -> usize {
    let rule = token.as_rule();
    match rule {
        Rule::expr_plus | Rule::expr_mul | Rule::expr_exp => extract_expr(ast, token),
        Rule::fun => {
            let value = String::from(token.as_str());
            let child = token.into_inner().next().unwrap();
            let inputs = extract_fun_arguments(ast, child);
            ast.push(Step::Function { value, inputs });
        }
        Rule::int_lit => {
            let n_str = token.as_str();
            let value = BigInt::parse_bytes(n_str.as_bytes(), 10).unwrap();
            ast.push(Step::Number { value });
        }
        _ => {
            walk_ast(ast, token.into_inner().next().unwrap()); // next inner token
        }
    };
    ast.len() - 1
}

fn extract_expr(ast: &mut Ast, parent_token: Pair<Rule>) {
    let is_reversed = parent_token.as_rule() == Rule::expr_exp;
    let ordered_tokens: Vec<Pair<Rule>> = if is_reversed {
        parent_token.into_inner().rev().collect()
    } else {
        parent_token.into_inner().collect()
    };
    ordered_tokens.into_iter().enumerate().fold(
        (0usize, Rule::expr), // default values, will change before being used
        |(left_hand, operation), (idx, token)| match idx {
            0 => (walk_ast(ast, token), operation),
            _ if idx % 2 == 1 => (left_hand, token.as_rule()),
            _ => {
                let right_hand = walk_ast(ast, token);
                let inputs = if is_reversed {
                    vec![right_hand, left_hand]
                } else {
                    vec![left_hand, right_hand]
                };
                ast.push(Step::Operation {
                    value: operation,
                    inputs,
                });
                (ast.len() - 1, operation)
            }
        },
    );
}

fn extract_fun_arguments(ast: &mut Ast, token: Pair<Rule>) -> Vec<usize> {
    token
        .into_inner()
        .map(|token| walk_ast(ast, token))
        .collect()
}

////fn alternate(ast: &mut Ast, token: Pair<Rule>, odd: fn(i32) -> i32, even: fn(i32) -> i32) {}

pub fn generate(formula: &str) -> Ast {
    let tokens = parse(formula);
    let mut ast: Ast = Vec::new();
    walk_ast(&mut ast, tokens);
    ast
}
