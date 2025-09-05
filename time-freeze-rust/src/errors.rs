use std::fmt;

#[derive(Debug)]
pub struct TimeFreezeError {
    pub details: String,
}

impl TimeFreezeError {
    pub fn new(msg: &str) -> TimeFreezeError {
        TimeFreezeError{details: msg.to_string()}
    }
}

impl fmt::Display for TimeFreezeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TimeFreezeError: {}", self.details)
    }
}

impl std::error::Error for TimeFreezeError {}
