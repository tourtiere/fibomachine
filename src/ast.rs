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
pub struct Step {
    pub rule: Rule,
    pub value: Option<BigInt>,
    pub inputs: Vec<usize>,
}
pub type Ast = Vec<Step>;

pub fn walk_ast(ast: &mut Ast, token: Pair<Rule>, depth: i32) -> usize {
    let rule = token.as_rule();
    match rule {
        Rule::expr_plus | Rule::expr_mul | Rule::expr_exp => extract_expr(ast, token, depth),
        Rule::int_lit => {
            ast.push(Step {
                rule: Rule::int_lit,
                value: BigInt::parse_bytes(token.as_str().as_bytes(), 10),
                inputs: vec![],
            });
        }
        _ => {
            walk_ast(ast, token.into_inner().next().unwrap(), depth + 1);
        }
    };
    ast.len() - 1
}

fn extract_expr(ast: &mut Ast, tokens: Pair<Rule>, depth: i32) {
    println!("{:?}", tokens.as_rule());
    let is_reversed = tokens.as_rule() == Rule::expr_exp;
    let ordered_tokens: Vec<Pair<Rule>> = if is_reversed {
        tokens.into_inner().rev().collect()
    } else {
        tokens.into_inner().collect()
    };
    ordered_tokens.into_iter().enumerate().fold(
        (0usize, Rule::expr),
        |(left_hand, operation), (idx, token)| match idx {
            0 => (walk_ast(ast, token, depth + 1), operation),
            _ if idx % 2 == 1 => (left_hand, token.as_rule()),
            _ => {
                let right_hand = walk_ast(ast, token, depth + 1);
                ast.push(Step {
                    rule: operation,
                    value: None,
                    inputs: vec![left_hand, right_hand],
                });
                (ast.len(), operation)
            }
        },
    );
}

pub fn generate(formula: &str) -> Ast {
    let tokens = parse(formula);
    let mut ast: Ast = Vec::new();
    walk_ast(&mut ast, tokens, 0);
    ast
}
