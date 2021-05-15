use crate::ast;
use num_bigint::BigInt;

pub fn excute(ast: ast::Ast) -> Option<BigInt> {
    let mut values: Vec<BigInt> = Vec::new();
    for step in ast {
        let inputs = &step.inputs;
        let value = match step.rule {
            ast::Rule::int_lit => step.value,
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
        };
        if let Some(x) = value {
            values.push(x);
        } else {
            return None;
        };
    }
    values.pop()
}
