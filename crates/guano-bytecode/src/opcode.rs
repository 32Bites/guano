use deku::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(type = "u8", endian = "big")]
pub enum Opcode {
    #[deku(id = "0")]
    #[doc = "Pushes the constant at the given index onto the stack."]
    Constant(u16),

    #[deku(id = "1")]
    #[doc = "Return the current call"]
    Return,

    #[deku(id = "2")]
    #[doc = "Adds the top values on the stack"]
    Add,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Opcode::*;
        match self {
            Constant(index) => write!(f, "const %{index}"),
            Return => write!(f, "ret"),
            Add => write!(f, "add"),
        }
    }
}
