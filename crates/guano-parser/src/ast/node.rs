use super::prelude::*;

pub trait Node: std::fmt::Display {
    fn span(&self) -> &NodeSpan;
    fn code(&self) -> &str {
        self.span().fragment()
    }

    fn formatted_code(&self) -> String {
        self.to_string()
    }
}
