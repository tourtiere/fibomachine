use crate::ast;
use num_bigint::BigInt;

pub fn excute(ast: ast::Ast) -> Option<BigInt> {
    let mut values: Vec<BigInt> = Vec::new();
    for step in ast {
        match step {
            ast::Step::Operation { value, inputs } => {
                let new_value = extract_operation(&values, value, inputs).unwrap();
                values.push(new_value)
            }
            ast::Step::Number { value } => values.push(value),
            _ => (),
        };
    }
    values.pop()
}
fn extract_operation(
    values: &Vec<BigInt>,
    operation: ast::Rule,
    inputs: Vec<usize>,
) -> Option<BigInt> {
    match operation {
        ast::Rule::op_plus => Some(&values[inputs[0]] + &values[inputs[1]]),
        ast::Rule::op_minus => Some(&values[inputs[0]] - &values[inputs[1]]),
        ast::Rule::op_mul => Some(&values[inputs[0]] * &values[inputs[1]]),
        ast::Rule::op_div => Some(&values[inputs[0]] / &values[inputs[1]]),
        ast::Rule::op_exp => {
            let (_, digits) = values[inputs[1]].to_u32_digits();
            let exponent = digits[0];
            if exponent > 1000 {
                return None;
            }
            Some(values[inputs[0]].pow(digits[0]))
        }
        _ => None,
    }
}
