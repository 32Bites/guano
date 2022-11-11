/// Module responsible for creating a structure
/// that is representative of the source code.
pub mod parser;

/// Module that accepts the output from the above module,
/// and desugars syntactic sugar for making later analysis easier.
pub mod desugar;