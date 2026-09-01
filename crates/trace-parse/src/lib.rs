//! Parse preprocessed C source into trace IR.

mod deps;
mod discover;
mod index_cache;
mod lower;
mod merge;
mod parse;

pub use deps::*;
pub use index_cache::IndexSourceCache;

pub use discover::*;
pub use lower::*;
pub use parse::*;
