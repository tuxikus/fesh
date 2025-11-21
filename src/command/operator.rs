#[derive(Debug, PartialEq)]
pub enum Operator {
    RedirectOverwrite, // >
    RedirectAppend,    // >>
    Pipe,              // |
}
