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

pub fn walk_ast(ast: &mut Ast, tokens: Pair<Rule>, depth: i32) -> usize {
    let parent_rule = tokens.as_rule();

    for token in tokens.into_inner().into_iter() {
        let rule = token.as_rule();
        println!(
            "{}:{:?}",
            (0..depth).map(|_| "|  ").collect::<String>(),
            rule
        );
        match rule {
            Rule::expr_plus | Rule::expr_mul | Rule::expr_exp => extract_expr(ast, token, depth),
            Rule::int_lit => {
                ast.push(Step {
                    rule: Rule::int_lit,
                    value: BigInt::parse_bytes(token.as_str().as_bytes(), 10),
                    inputs: vec![],
                });
                ast.len()
            }
            _ => walk_ast(ast, token, depth + 1),
        };
    }
    if ast.len() > 0 {
        ast.len() - 1
    } else {
        0
    }
}

// TODO : dont return walk_ast. If child token is operator, extract_expr directly
// otherwise, an operation level will skip.
fn extract_expr(ast: &mut Ast, tokens: Pair<Rule>, depth: i32) -> usize {
    println!("{:?}", tokens.as_rule());
    let is_reversed = tokens.as_rule() == Rule::expr_exp;
    let ordered_tokens: Vec<Pair<Rule>> = if is_reversed {
        tokens.into_inner().rev().collect()
    } else {
        tokens.into_inner().collect()
    };
    ordered_tokens
        .into_iter()
        .enumerate()
        .fold(
            (0usize, Rule::expr),
            |(left_hand, operation), (idx, token)| {
                if idx == 0 {
                    //println!("{:?}", operation);
                    return (walk_ast(ast, token, depth + 1), operation);
                }
                // is an operator
                if idx % 2 == 1 {
                    return (left_hand, token.as_rule());
                }
                // is a term
                let right_hand = walk_ast(ast, token, depth + 1);
                ast.push(Step {
                    rule: operation,
                    value: None,
                    inputs: vec![left_hand, right_hand],
                });
                return (ast.len(), operation);
            },
        )
        .0
}

pub fn generate(formula: &str) -> Ast {
    let tokens = parse(formula);
    let mut ast: Ast = Vec::new();
    walk_ast(&mut ast, tokens, 0);
    ast
}
