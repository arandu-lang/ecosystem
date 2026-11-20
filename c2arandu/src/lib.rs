#![allow(non_upper_case_globals)]

pub mod ast;
pub mod codegen;
pub mod lifting;
pub mod parser;
pub mod shim;
pub mod types;

pub use codegen::{CodegenOptions, TranspileMode, generate_arandu};
pub use lifting::apply_semantic_lifting;
pub use parser::{ParserOptions, parse_c_file};
