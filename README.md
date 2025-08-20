# jtree

This package contains my own implementations of some of the classic "tree" data structures in Rust, such as:

- `Jbst`: a simple binary search tree storing unique values in order (i.e. an ordered set)
- `Jblst`: a simple binary (list-like) search tree allowing duplicate entries (i.e. an ordered list)

# operations

Each tree type should support these operations:

- Adding a unique value
- Adding a vector of unique values
- Deleting a value
- Checking if a value exists in the tree
- Returning the values as an ordered vector (in either direction)
- Pretty-printing the tree

Some types may support:

- Mutating values in place
- Adding non-unique values

# usage

See individual modules for rustdocs.  Run tests with:

    cargo test

Build the docs with:

    cargo doc
