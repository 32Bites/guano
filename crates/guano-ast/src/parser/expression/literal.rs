use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use num::ToPrimitive;

use super::parser::Expression;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Character(char),
    Integer(i64),
    UnsignedInteger(u64),
    FloatingPoint(f64),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{:?}", s),
            Literal::Character(c) => write!(f, "{:?}", c),
            Literal::Integer(i) => write!(f, "{:?}", i),
            Literal::UnsignedInteger(u) => write!(f, "{:?}", u),
            Literal::FloatingPoint(fp) => write!(f, "{:?}", fp),
            Literal::Boolean(b) => write!(f, "{:?}", b),
            Literal::Nil => f.write_str("nil"),
        }
    }
}

impl Literal {
    pub fn to_expression(self) -> Expression {
        Expression::Literal(self)
    }

    pub fn primitive(&self) -> Option<&dyn ToPrimitive> {
        Some(match self {
            Literal::Integer(i) => i as &dyn ToPrimitive,
            Literal::UnsignedInteger(u) => u as &dyn ToPrimitive,
            Literal::FloatingPoint(f) => f as &dyn ToPrimitive,
            _ => return None,
        })
    }

    pub fn primitive_or_float(&self) -> Option<Result<&dyn ToPrimitive, &f64>> {
        if let Literal::FloatingPoint(f) = self {
            Some(Err(f))
        } else {
            Some(Ok(self.primitive()?))
        }
    }
}

impl Mul for Literal {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Literal::String(string) => match rhs {
                Literal::UnsignedInteger(amount) => {
                    Some(Literal::String(string.repeat(amount as usize)))
                }
                Literal::Integer(amount) => {
                    let amount = amount.to_u64()? as usize;
                    Some(Literal::String(string.repeat(amount)))
                }
                _ => None,
            },
            Literal::Character(character) => match rhs {
                Literal::UnsignedInteger(amount) => Some(Literal::String(
                    character.to_string().repeat(amount as usize),
                )),
                Literal::Integer(amount) => {
                    let amount = amount.to_u64()? as usize;
                    Some(Literal::String(character.to_string().repeat(amount)))
                }
                _ => None,
            },
            Literal::Integer(lhs) => {
                if let Some(rhs) = rhs.primitive_or_float().map(|r| r.map(|t| t.to_i64())) {
                    match rhs {
                        Ok(Some(rhs)) => Some(Literal::Integer(lhs * rhs)),
                        Ok(None) => None,
                        Err(rhs) => {
                            if let Some(lhs) = lhs.to_f64() {
                                Some(Literal::FloatingPoint(lhs * *rhs))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Literal::UnsignedInteger(lhs) => {
                if let Some(rhs) = rhs.primitive_or_float().map(|r| r.map(|t| t.to_u64())) {
                    match rhs {
                        Ok(Some(rhs)) => Some(Literal::UnsignedInteger(lhs * rhs)),
                        Ok(None) => None,
                        Err(rhs) => {
                            if let Some(lhs) = lhs.to_f64() {
                                Some(Literal::FloatingPoint(lhs * *rhs))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Literal::FloatingPoint(lhs) => {
                if let Some(rhs) = rhs.primitive().and_then(|p| p.to_f64()) {
                    Some(Literal::FloatingPoint(lhs * rhs))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Div for Literal {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Integer(lhs) => {
                if let Some(rhs) = rhs.primitive_or_float().map(|r| r.map(|t| t.to_i64())) {
                    match rhs {
                        Ok(Some(rhs)) => Some(Literal::Integer(lhs / rhs)),
                        Ok(None) => None,
                        Err(rhs) => {
                            if let Some(lhs) = lhs.to_f64() {
                                Some(Literal::FloatingPoint(lhs / *rhs))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Literal::UnsignedInteger(lhs) => {
                if let Some(rhs) = rhs.primitive_or_float().map(|r| r.map(|t| t.to_u64())) {
                    match rhs {
                        Ok(Some(rhs)) => Some(Literal::UnsignedInteger(lhs / rhs)),
                        Ok(None) => None,
                        Err(rhs) => {
                            if let Some(lhs) = lhs.to_f64() {
                                Some(Literal::FloatingPoint(lhs / *rhs))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Literal::FloatingPoint(lhs) => {
                if let Some(rhs) = rhs.primitive().and_then(|p| p.to_f64()) {
                    Some(Literal::FloatingPoint(lhs / rhs))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Add for Literal {
    type Output = Option<Literal>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Literal::String(lhs) => match rhs {
                Literal::String(rhs) => Some(Literal::String(format!("{lhs}{rhs}"))),
                Literal::Character(rhs) => Some(Literal::String(format!("{lhs}{rhs}"))),
                _ => None,
            },
            Literal::Character(lhs) => match rhs {
                Literal::String(rhs) => Some(Literal::String(format!("{lhs}{rhs}"))),
                Literal::Character(rhs) => Some(Literal::String(format!("{lhs}{rhs}"))),
                _ => None,
            },
            Literal::Integer(lhs) => {
                if let Some(rhs) = rhs.primitive_or_float().map(|r| r.map(|t| t.to_i64())) {
                    match rhs {
                        Ok(Some(rhs)) => Some(Literal::Integer(lhs + rhs)),
                        Ok(None) => None,
                        Err(rhs) => {
                            if let Some(lhs) = lhs.to_f64() {
                                Some(Literal::FloatingPoint(lhs + *rhs))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Literal::UnsignedInteger(lhs) => {
                if let Some(rhs) = rhs.primitive_or_float().map(|r| r.map(|t| t.to_u64())) {
                    match rhs {
                        Ok(Some(rhs)) => Some(Literal::UnsignedInteger(lhs + rhs)),
                        Ok(None) => None,
                        Err(rhs) => {
                            if let Some(lhs) = lhs.to_f64() {
                                Some(Literal::FloatingPoint(lhs + *rhs))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Literal::FloatingPoint(lhs) => {
                if let Some(rhs) = rhs.primitive().and_then(|p| p.to_f64()) {
                    Some(Literal::FloatingPoint(lhs + rhs))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Sub for Literal {
    type Output = Option<Literal>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Integer(lhs) => {
                if let Some(rhs) = rhs.primitive_or_float().map(|r| r.map(|t| t.to_i64())) {
                    match rhs {
                        Ok(Some(rhs)) => Some(Literal::Integer(lhs - rhs)),
                        Ok(None) => None,
                        Err(rhs) => {
                            if let Some(lhs) = lhs.to_f64() {
                                Some(Literal::FloatingPoint(lhs - *rhs))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Literal::UnsignedInteger(lhs) => {
                if let Some(rhs) = rhs.primitive_or_float().map(|r| r.map(|t| t.to_u64())) {
                    match rhs {
                        Ok(Some(rhs)) => Some(Literal::UnsignedInteger(lhs - rhs)),
                        Ok(None) => None,
                        Err(rhs) => {
                            if let Some(lhs) = lhs.to_f64() {
                                Some(Literal::FloatingPoint(lhs - *rhs))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Literal::FloatingPoint(lhs) => {
                if let Some(rhs) = rhs.primitive().and_then(|p| p.to_f64()) {
                    Some(Literal::FloatingPoint(lhs - rhs))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l), Self::String(r)) => l == r,
            (Self::Character(l), Self::Character(r)) => l == r,
            (Self::Integer(l), Self::Integer(r)) => l == r,
            (Self::UnsignedInteger(l), Self::UnsignedInteger(r)) => l == r,
            (Self::FloatingPoint(l), Self::FloatingPoint(r)) => l == r,
            (Self::Boolean(l), Self::Boolean(r)) => l == r,
            (Self::Integer(l), Self::UnsignedInteger(r)) => match r.to_i64() {
                Some(r) => *l == r,
                None => false,
            },
            (Self::UnsignedInteger(l), Self::Integer(r)) => match r.to_u64() {
                Some(r) => *l == r,
                None => false,
            },
            (Self::Nil, Self::Nil) => true,
            _ => false,
            // _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Literal::Character(l), Literal::Character(r)) => l.partial_cmp(r),
            (Literal::Integer(l), Literal::Integer(r)) => l.partial_cmp(r),
            (Literal::Integer(l), Literal::UnsignedInteger(r)) => l.partial_cmp(&r.to_i64()?),
            (Literal::Integer(_), Literal::FloatingPoint(_)) => None,
            (Literal::UnsignedInteger(l), Literal::Integer(r)) => l.partial_cmp(&r.to_u64()?),
            (Literal::UnsignedInteger(l), Literal::UnsignedInteger(r)) => l.partial_cmp(r),
            (Literal::UnsignedInteger(_), Literal::FloatingPoint(_)) => None,
            (Literal::FloatingPoint(_), Literal::Integer(_)) => None,
            (Literal::FloatingPoint(_), Literal::UnsignedInteger(_)) => None,
            (Literal::FloatingPoint(l), Literal::FloatingPoint(r)) => l.partial_cmp(r),
            (l, r) if r == l => Some(Ordering::Equal),
            _ => None,
        }
    }
}
