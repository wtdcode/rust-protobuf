//! Pure rust `.proto` file parser.

pub mod convert;
pub mod model;
pub mod parse_and_typecheck;
pub mod parse_dependencies;
mod parser;

pub mod anyhow {
    pub use anyhow::*;
}
pub use parse_and_typecheck::parse_and_typecheck_custom;
pub use parse_dependencies::*;
