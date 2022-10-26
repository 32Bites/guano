use super::{parser::Expression, BinaryExpression, FunctionCall};
use itertools::Itertools;

#[derive(Debug)]
pub struct Display<'expression> {
    pub expression: &'expression Expression,
    pub grouped: bool,
}

impl<'expression> Display<'expression> {
    pub fn new(expression: &'expression Expression, grouped: bool) -> Self {
        Self {
            expression,
            grouped,
        }
    }

    fn sub(&self, expression: &'expression Expression) -> Display<'_> {
        Display {
            expression,
            grouped: self.grouped,
        }
    }
}

impl std::fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expression {
            Expression::Group(g) => write!(f, "( {} )", self.sub(g))?,
            Expression::Literal(l) => write!(f, "{l}")?,
            Expression::Variable(v) => write!(f, "{v}")?,
            Expression::FunctionCall(FunctionCall {
                identifier,
                arguments,
            }) => write!(
                f,
                "{identifier}({})",
                arguments.iter().map(|e| self.sub(e).to_string()).join(", ")
            )?,
            Expression::Index { value, index } => {
                write!(f, "{}[{}]", self.sub(value), self.sub(index))?
            }
            Expression::Property { value, property } => {
                write!(f, "{}.{property}", self.sub(value))?
            }
            Expression::MethodCall { value, method } => write!(
                f,
                "{}.{}({})",
                self.sub(value),
                method.identifier,
                method
                    .arguments
                    .iter()
                    .map(|e| self.sub(e).to_string())
                    .join(", ")
            )?,
            Expression::Unary { operator, right } => write!(f, "{operator}{}", self.sub(right))?,
            Expression::Tuple(values) => write!(f, "({})", {
                match values.len() {
                    0 => "".to_string(),
                    1 => format!("{},", self.sub(&values[0])),
                    _ => values.iter().map(|e| self.sub(e).to_string()).join(", "),
                }
            })?,
            Expression::List(values) => write!(
                f,
                "[{}]",
                values.iter().map(|e| self.sub(e).to_string()).join(", ")
            )?,
            e => {
                if self.grouped {
                    f.write_str("( ")?;
                }

                match e {
                    Expression::Cast {
                        value: left,
                        new_type: cast_to,
                    } => {
                        write!(f, "{} as {cast_to}", self.sub(left))
                    }
                    Expression::Factor(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(left), self.sub(right)),
                    Expression::Term(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(left), self.sub(right)),
                    Expression::Comparison(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(left), self.sub(right)),
                    Expression::Bitwise(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(left), self.sub(right)),
                    Expression::Logical(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(left), self.sub(right)),
                    Expression::Format { format, with } => {
                        write!(f, "{format:?}: {}", self.sub(with))
                    }
                    _ => unreachable!(),
                }?;

                if self.grouped {
                    f.write_str(" )")?;
                }
            }
        }

        Ok(())
    }
}
