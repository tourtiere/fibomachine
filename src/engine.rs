use crate::ast::{self, Ast, Rule, Step};
use num_bigint::{BigInt, ToBigInt};

type Values = Vec<ast::Step>;

pub type Sequence = Vec<Step>;

pub fn execute<'a>(ast: &Ast, sequence: &'a mut Sequence) -> &'a mut Sequence {
    let first_n = sequence.len();
    println!("{:?}", sequence);

    for n in first_n..50 {
        let mut values: Values = Vec::new();
        for step in ast {
            match step {
                Step::Operation { value, inputs } => extract_operation(&mut values, value, inputs),
                Step::Function { name, inputs } => {
                    extract_function(&mut values, name, &inputs, &sequence)
                }
                Step::Var { name } => extract_var(&mut values, name, n as i32),
                x => values.push(x.clone()),
            };
        }
        let term = values.pop().unwrap();
        sequence.push(term);
    }
    sequence
}

fn extract_var(values: &mut Values, name: &String, n: i32) {
    match name.as_str() {
        "n" => {
            values.push(Step::Number {
                value: n.to_bigint().unwrap(),
            });
        }
        _ => (),
    }
}

fn extract_function(values: &mut Values, name: &String, inputs: &Vec<usize>, sequence: &Sequence) {
    match name.as_str() {
        "if" => {
            if inputs.len() != 3 {
                return;
            }
            match values[inputs[0]] {
                Step::Boolean { value } => {
                    if value {
                        values.push(values[inputs[1]].clone());
                    } else {
                        values.push(values[inputs[2]].clone());
                    }
                }
                _ => (),
            };
        }
        "a" => {
            if inputs.len() != 1 {
                return;
            }
            let argument = &values[inputs[0]];
            match argument {
                Step::Number { value } => {
                    let (_, list) = value.to_u32_digits();
                    let index = if list.len() == 0 { 0 } else { list[0] };
                    let term = sequence[index as usize].clone();
                    //println!("term: {:?}", term);
                    values.push(term);
                }
                _ => (),
            };
        }
        _ => (),
    };
}

fn op_exp(a: &BigInt, b: &BigInt) -> Option<BigInt> {
    let (_, digits) = b.to_u32_digits();
    if digits.len() > 1 {
        return None;
    };
    let exponent = digits[0];
    if exponent > 1000 {
        return None;
    }
    Some(a.pow(digits[0]))
}

fn extract_operation(values: &mut Values, operation: &ast::Rule, inputs: &Vec<usize>) {
    let a = &values[inputs[0]];
    let b = &values[inputs[1]];

    if let Some((a, b)) = as_numbers(a, b) {
        if let Some(value) = match operation {
            Rule::op_eq => Some(a == b),
            Rule::op_ne => Some(a != b),
            Rule::op_ge => Some(a >= b),
            Rule::op_le => Some(a <= b),
            Rule::op_gt => Some(a > b),
            Rule::op_lt => Some(a < b),
            _ => None,
        } {
            return values.push(Step::Boolean { value });
        }

        if let Some(value) = match operation {
            Rule::op_plus => Some(a + b),
            Rule::op_minus => Some(a - b),
            Rule::op_mul => Some(a * b),
            Rule::op_div => Some(a / b),
            Rule::op_mod => Some(a % b),
            Rule::op_exp => op_exp(a, b),
            _ => None,
        } {
            return values.push(Step::Number { value });
        }
    }
    if let Some((a, b)) = as_boolean(a, b) {
        if let Some(value) = match operation {
            Rule::op_or => Some(*a && *b),
            Rule::op_and => Some(*a || *b),
            _ => None,
        } {
            return values.push(Step::Boolean { value });
        };
    }
}

fn as_numbers<'a>(a: &'a Step, b: &'a Step) -> Option<(&'a BigInt, &'a BigInt)> {
    match a {
        Step::Number { value: a_number } => match b {
            Step::Number { value: b_number } => Some((a_number, b_number)),
            _ => None,
        },
        _ => None,
    }
}

fn as_boolean<'a>(a: &'a Step, b: &'a Step) -> Option<(&'a bool, &'a bool)> {
    match a {
        Step::Boolean { value: a_number } => match b {
            Step::Boolean { value: b_number } => Some((a_number, b_number)),
            _ => None,
        },
        _ => None,
    }
}
