use crate::AstToken;

include!(concat!(env!("OUT_DIR"), "/generated_tokens.rs"));

impl Iden {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self.text(),
            "float" | "uint" | "int" | "char" | "string" | "boolean"
        )
    }
}
