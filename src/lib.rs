//! # jtree: Joe's tree-like data structures
//! 
//! These are my personal implementations of some tree-based data structures, including such classics as:
//! 
//! - `Jbst` : "Joe's BST", a simple binary search tree storing unique values in order (i.e. an ordered set).
//! - `Jblst` : "Joe's B(list-like)ST", a simple binary (list-like) search tree allowing duplicate entries (i.e. an ordered list).

pub mod jbst;
pub mod jblst;
pub mod avltree;
pub mod errors;

pub use jbst::Jbst;
pub use jblst::Jblst;


