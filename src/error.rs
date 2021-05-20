// error1.rs
use crate::ast::Range;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseError {
    range: Range,
}

impl ParseError {
    fn new(range: &Range) -> ParseError {
        ParseError {
            range: range.clone(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.range)
    }
}

// a test function that returns our error result
/*
fn raises_parse_error(yes: bool) -> Result<(), ParseError> {
    let a = (0_usize, 0_usize);
    if yes {
        Err(ParseError::new(&a))
    } else {
        Ok(())
    }
}

 */
