use super::parser::Expression;

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
            Expression::Literal(l) => write!(f, "{}", l)?,
            Expression::Variable(v) => write!(f, "{}", v)?,
            Expression::Unary { operator, right } => write!(f, "{operator}{}", self.sub(right))?,
            e => {
                if self.grouped {
                    f.write_str("( ")?;
                }

                match e {
                    Expression::FunctionCall { name, arguments } => todo!(),
                    Expression::Cast { left, cast_to } => {
                        write!(f, "{} as {cast_to}", self.sub(left))
                    }
                    Expression::Binary {
                        operator,
                        left,
                        right,
                    } => write!(f, "{} {operator} {}", self.sub(left), self.sub(right)),
                    Expression::Format {
                        format_string,
                        arguments,
                    } => todo!(),
                    Expression::Access {
                        owner,
                        accessed_value,
                    } => todo!(),
                    Expression::Index { owner, index } => todo!(),
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