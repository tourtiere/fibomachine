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

pub type Range = (usize, usize);

#[derive(Clone, Debug)]
pub struct Step {
    pub value: Value,
    pub range: Range,
}
#[derive(Clone, Debug)]
pub enum Value {
    //Values
    Number { value: BigInt },
    String { value: String },
    Boolean { value: bool },
    Var { name: String },
    Function { name: String, inputs: Vec<usize> },
    Operation { value: Rule, inputs: Vec<usize> },
}

fn add_range(Step { range: a, .. }: &Step, Step { range: b, .. }: &Step) -> Range {
    (usize::min(a.0, b.0), usize::max(a.1, b.1))
}

pub type Ast = Vec<Step>;

pub fn walk_ast(ast: &mut Ast, token: Pair<Rule>) -> usize {
    let rule = token.as_rule();
    let span = token.as_span();
    let range: Range = (span.start(), span.end());

    match rule {
        Rule::expr_plus
        | Rule::expr_mul
        | Rule::expr_exp
        | Rule::expr_logical
        | Rule::expr_binary => extract_expr(ast, token),
        Rule::fun => {
            let total_match = token.as_str();
            let crop = total_match.find("(").unwrap();
            let name = String::from(&total_match[0..crop]);

            let mut pairs = token.into_inner();
            //let _ = pairs.next().unwrap();
            let second = pairs.next().unwrap();

            let inputs = extract_fun_arguments(ast, second);
            ast.push(Step {
                range,
                value: Value::Function { name, inputs },
            });
        }

        Rule::int_lit => {
            let value = BigInt::parse_bytes(token.as_str().as_bytes(), 10).unwrap();
            ast.push(Step {
                range,
                value: Value::Number { value },
            });
        }
        Rule::var => {
            let name = String::from(token.as_str());
            ast.push(Step {
                range,
                value: Value::Var { name },
            });
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

                ast.push(Step {
                    range: add_range(&ast[left_hand], &ast[right_hand]),
                    value: Value::Operation {
                        value: operation,
                        inputs,
                    },
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

pub fn generate(formula: &str) -> Ast {
    let tokens = parse(formula);
    let mut ast: Ast = Vec::new();
    walk_ast(&mut ast, tokens);
    ast
}
