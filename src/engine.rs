use crate::ast;
use num_bigint::BigInt;

pub fn walk_engine(ast: ast::Ast) -> Option<BigInt> {
    let mut values: Vec<BigInt> = Vec::new();
    for step in ast {
        let value = match step.rule {
            ast::Rule::int_lit => step.value,
            ast::Rule::op_plus => Some(&values[step.inputs[0]] + &values[step.inputs[1]]),
            ast::Rule::op_minus => Some(&values[step.inputs[0]] - &values[step.inputs[1]]),
            ast::Rule::op_mul => Some(&values[step.inputs[0]] * &values[step.inputs[1]]),
            ast::Rule::op_div => Some(&values[step.inputs[0]] / &values[step.inputs[1]]),
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
