use crate::ast::{self, Ast, Rule, Step, Value};
use num_bigint::{BigInt, ToBigInt};

use crate::error::{Error, ErrorKind};

type Values = Vec<Value>;

pub type Sequence = Vec<Value>;

pub fn execute<'a>(ast: &Ast, sequence: &'a mut Sequence) -> Result<&'a mut Sequence, Error> {
    let first_n = sequence.len();
    //println!("{:?}", sequence);
    for n in first_n..50 {
        let mut values: Values = Vec::new();
        for step in ast {
            let Step { value, .. } = step;
            let maybe_value: Result<Value, ErrorKind> = match value {
                Value::Operation { value, inputs } => extract_operation(&values, value, inputs),
                Value::Function { name, inputs } => {
                    extract_function(&values, name, &inputs, &sequence)
                }
                Value::Var { name } => extract_var(name, n as i32),
                _ => Ok(value.clone()),
            };

            match maybe_value {
                Ok(value) => values.push(value),
                Err(kind) => {
                    return Err(Error {
                        kind,
                        step: step.clone(),
                    })
                }
            }
        }
        let term = values.pop().unwrap();
        sequence.push(term);
    }
    Ok(sequence)
}

fn extract_var(name: &String, n: i32) -> Result<Value, ErrorKind> {
    match name.as_str() {
        "n" => Ok(Value::Number {
            value: n.to_bigint().unwrap(),
        }),
        _ => Err(ErrorKind::Undefined),
    }
}

fn extract_function(
    values: &Values,
    name: &String,
    inputs: &Vec<usize>,
    sequence: &Sequence,
) -> Result<Value, ErrorKind> {
    match name.as_str() {
        "if" => {
            if inputs.len() != 3 {
                return Err(ErrorKind::Count);
            }
            match values[inputs[0]] {
                Value::Boolean { value } => {
                    if value {
                        Ok(values[inputs[1]].clone())
                    } else {
                        Ok(values[inputs[2]].clone())
                    }
                }
                _ => Err(ErrorKind::Type),
            }
        }
        "a" => {
            if inputs.len() != 1 {
                return Err(ErrorKind::Count);
            }
            let argument = &values[inputs[0]];
            match argument {
                Value::Number { value } => {
                    let (_, list) = value.to_u32_digits();
                    let index = if list.len() == 0 { 0 } else { list[0] };
                    Ok(sequence[index as usize].clone())
                }
                _ => Err(ErrorKind::Type),
            }
        }
        _ => Err(ErrorKind::Undefined),
    }
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

fn extract_operation(
    values: &Values,
    operation: &ast::Rule,
    inputs: &Vec<usize>,
) -> Result<Value, ErrorKind> {
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
            return Ok(Value::Boolean { value });
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
            return Ok(Value::Number { value });
        }
    }
    if let Some((a, b)) = as_boolean(a, b) {
        if let Some(value) = match operation {
            Rule::op_or => Some(*a && *b),
            Rule::op_and => Some(*a || *b),
            _ => None,
        } {
            return Ok(Value::Boolean { value });
        };
    }
    return Err(ErrorKind::Type);
}

fn as_numbers<'a>(a: &'a Value, b: &'a Value) -> Option<(&'a BigInt, &'a BigInt)> {
    match a {
        Value::Number {
            value: a_number, ..
        } => match b {
            Value::Number {
                value: b_number, ..
            } => Some((a_number, b_number)),
            _ => None,
        },
        _ => None,
    }
}

fn as_boolean<'a>(a: &'a Value, b: &'a Value) -> Option<(&'a bool, &'a bool)> {
    match a {
        Value::Boolean { value: a_number } => match b {
            Value::Boolean { value: b_number } => Some((a_number, b_number)),
            _ => None,
        },
        _ => None,
    }
}
