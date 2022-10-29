use super::{parser::ExpressionKind, BinaryExpression, Expression, FunctionCall};
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
        match &self.expression.kind {
            ExpressionKind::Group(g) => write!(f, "( {} )", self.sub(&g))?,
            ExpressionKind::Literal(l) => write!(f, "{l}")?,
            ExpressionKind::Variable(v) => write!(f, "{v}")?,
            ExpressionKind::FunctionCall(FunctionCall {
                identifier,
                arguments,
            }) => write!(
                f,
                "{identifier}({})",
                arguments.iter().map(|e| self.sub(e).to_string()).join(", ")
            )?,
            ExpressionKind::Index { value, index } => {
                write!(f, "{}[{}]", self.sub(&value), self.sub(&index))?
            }
            ExpressionKind::Property { value, property } => {
                write!(f, "{}.{property}", self.sub(&value))?
            }
            ExpressionKind::MethodCall { value, method } => write!(
                f,
                "{}.{}({})",
                self.sub(&value),
                method.identifier,
                method
                    .arguments
                    .iter()
                    .map(|e| self.sub(e).to_string())
                    .join(", ")
            )?,
            ExpressionKind::Unary { operator, right } => {
                write!(f, "{operator}{}", self.sub(&right))?
            }
            ExpressionKind::Tuple(values) => write!(f, "({})", {
                match values.len() {
                    0 => "".to_string(),
                    1 => format!("{},", self.sub(&values[0])),
                    _ => values.iter().map(|e| self.sub(e).to_string()).join(", "),
                }
            })?,
            ExpressionKind::List(values) => write!(
                f,
                "[{}]",
                values.iter().map(|e| self.sub(e).to_string()).join(", ")
            )?,
            e => {
                if self.grouped {
                    f.write_str("( ")?;
                }

                match e {
                    ExpressionKind::Cast {
                        value: left,
                        new_type: cast_to,
                    } => {
                        write!(f, "{} as {cast_to}", self.sub(&left))
                    }
                    ExpressionKind::Factor(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(&left), self.sub(&right)),
                    ExpressionKind::Term(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(&left), self.sub(&right)),
                    ExpressionKind::Comparison(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(&left), self.sub(&right)),
                    ExpressionKind::Bitwise(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(&left), self.sub(&right)),
                    ExpressionKind::Logical(BinaryExpression {
                        left,
                        operator,
                        right,
                    }) => write!(f, "{} {operator} {}", self.sub(&left), self.sub(&right)),
                    ExpressionKind::Format { format, with } => {
                        write!(f, "{format:?}: {}", self.sub(&with))
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
