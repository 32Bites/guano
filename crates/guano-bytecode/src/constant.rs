use std::ops::Deref;

use deku::{
    bitvec::{BitVec, Msb0},
    ctx::Endian,
    prelude::*,
};

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite, Default)]
pub struct Constants {
    #[deku(update = "self.pool.len()", endian = "big")]
    count: u16,
    #[deku(count = "count")]
    pool: Vec<Constant>,
}

impl Constants {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, constant: impl Into<Constant>) {
        self.pool.push(constant.into());
        self.update().unwrap();
    }

    pub fn pool(&self) -> &[Constant] {
        &self.pool
    }
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum Constant {
    #[deku(id = "0")]
    Uint(#[deku(endian = "big")] u64),
    #[deku(id = "1")]
    Int(#[deku(endian = "big")] i64),
    #[deku(id = "2")]
    Float(#[deku(endian = "big")] f64),
    #[deku(id = "3")]
    Boolean(#[deku(endian = "big")] bool),
    #[deku(id = "4")]
    Char(
        #[deku(
            endian = "big",
            map = "|c: u32| -> Result<_, DekuError> { ::std::char::from_u32(c).ok_or_else(|| DekuError::Parse(\"Expected char\".into())).map(|c| c)}",
            writer = "char_writer(deku::output, self)"
        )]
        char,
    ),
    #[deku(id = "5")]
    Str(Str),
}

impl From<i64> for Constant {
    fn from(value: i64) -> Self {
        Constant::Int(value)
    }
}

impl From<u64> for Constant {
    fn from(value: u64) -> Self {
        Constant::Uint(value)
    }
}

impl From<f64> for Constant {
    fn from(value: f64) -> Self {
        Constant::Float(value)
    }
}

impl From<bool> for Constant {
    fn from(value: bool) -> Self {
        Constant::Boolean(value)
    }
}

impl From<char> for Constant {
    fn from(value: char) -> Self {
        Constant::Char(value)
    }
}

impl From<String> for Constant {
    fn from(value: String) -> Self {
        Constant::Str(value.into())
    }
}

fn char_writer(output: &mut BitVec<u8, Msb0>, c: &Constant) -> Result<(), DekuError> {
    if let Constant::Char(c) = c {
        (*c as u32).write(output, Endian::Big)
    } else {
        unreachable!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Str {
    #[deku(update = "self.data.len()")]
    count: u64,
    #[deku(
        count = "count",
        map = "|data: Vec<u8>| -> Result<Vec<u8>, DekuError> { ::std::str::from_utf8(&data).map_err(|e| DekuError::Parse(e.to_string()))?; Ok(data) }"
    )]
    data: Vec<u8>,
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.data) }
    }
}

impl From<String> for Str {
    fn from(value: String) -> Self {
        Self {
            count: value.len() as u64,
            data: value.into_bytes(),
        }
    }
}
