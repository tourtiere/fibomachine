// error1.rs
use crate::ast::Step;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    Count,
    Undefined,
    Type,
    OutOfBounds,
}
#[derive(Debug)]
pub struct Error {
    pub step: Step,
    pub kind: ErrorKind,
}

//pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_type = match self.kind {
            ErrorKind::Count => "Count",
            ErrorKind::Undefined => "Undefined",
            ErrorKind::Type => "Type",
            ErrorKind::OutOfBounds => "OutOfBounds",
        };
        write!(
            f,
            r#"{{ "success":false, "errorType":"{}", "range":[{},{}] }}"#,
            error_type,
            self.step.range.0.to_string(),
            self.step.range.1.to_string()
        )
    }
}
