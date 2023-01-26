use deku::{DekuContainerRead, DekuContainerWrite};

use crate::opcode::Opcode;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Chunk {
    bytes: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_opcode(&mut self, opcode: Opcode) {
        self.bytes.extend(opcode.to_bytes().unwrap());
    }

    pub fn data(&self) -> &[u8] {
        &self.bytes
    }

    pub fn disas(&self) -> Disassemble<'_> {
        Disassemble::new(&self.bytes)
    }

    pub fn disas_at(&self, position: usize) -> Disassemble<'_> {
        Disassemble::new_at(&self.bytes, position)
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.disas().fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Disassemble<'chunk> {
    chunk: &'chunk [u8],
    position: usize,
}

impl<'chunk> Disassemble<'chunk> {
    pub fn new(chunk: &'chunk [u8]) -> Self {
        Self { chunk, position: 0 }
    }

    pub fn new_at(chunk: &'chunk [u8], position: usize) -> Self {
        Self { chunk, position }
    }

    pub fn remaining(&self) -> &[u8] {
        &self.chunk[self.position..]
    }

    pub fn read_opcode(&mut self) -> Option<Opcode> {
        let remaining = &self.chunk[self.position..];
        let ((remainder, _), opcode) = Opcode::from_bytes((remaining, 0)).ok()?;
        self.position += remaining.len() - remainder.len();

        Some(opcode)
    }
}

impl<'chunk> Iterator for Disassemble<'chunk> {
    type Item = Opcode;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_opcode()
    }
}

impl<'chunk> std::fmt::Display for Disassemble<'chunk> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for opcode in self.into_iter() {
            writeln!(f, "{opcode};")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use crate::opcode::Opcode;

    use super::Chunk;

    #[test]
    fn test_chunk() {
        let mut chunk = Chunk::new();

        chunk.write_opcode(Opcode::Return);
        chunk.write_opcode(Opcode::Add);

        for i in 0..100 {
            chunk.write_opcode(Opcode::Constant(i));
        }

        println!("The instructions occupy {} bytes.", chunk.data().len());
        println!("Instructions:");
        println!("{chunk}");

        std::fs::File::create("test.b")
            .unwrap()
            .write_all(chunk.data())
            .unwrap();
    }
}
