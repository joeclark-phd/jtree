//! # jtree: Joe's tree-like data structures
//! 
//! These are my personal implementations of some tree-based data structures, including such classics as:
//! 
//! - `Jbst` : "Joe's BST", an unbalanced binary search tree for unique values, like an ordered set.

pub mod jbst;
pub mod avltree;
pub mod errors;

pub use jbst::Jbst;


