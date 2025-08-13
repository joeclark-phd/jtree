

#[derive(PartialEq, Debug)]
pub enum TreeError {
    /// Caller attempted to add a duplicate value to a tree that only accepts unique values.
    ValueAlreadyStored,
}

/// My implementation of a regular (unbalanced) **binary search tree**
/// for unique values (no duplicates).
/// 
/// Currently holds "u32" data.
/// 
/// TODO: make generic
pub struct BinTree {
    root: Option<Node>,
}

impl BinTree {

    /// Create a new tree with no data
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    /// Insert a value
    pub fn add(&mut self, value: u32) -> Result<(),TreeError> {

        match &mut self.root {
            None => self.root = Some(Node::new(value)),
            Some(branch) => branch.add(value)?,
        }

        Ok(())
    }

}

impl Default for BinTree {
    fn default() -> Self {
        Self::new()
    }
}

struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, value: u32) -> Result<(),TreeError> {
        if value == self.value {
            // no duplicates allowed in this kind of tree
            return Err(TreeError::ValueAlreadyStored)
        }
        if value < self.value {
            // add to the left branch
            match &mut self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(branch) => branch.add(value)?,
            }
            return Ok(())
        } else {
            // add it to the right branch
            match &mut self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(branch) => branch.add(value)?,
            }
            return Ok(())
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_unique_items() {
        let mut my_tree = BinTree::new();
        assert_eq!( Ok(()), my_tree.add(5) );
        assert_eq!( Ok(()), my_tree.add(3) );
        assert_eq!( Ok(()), my_tree.add(7) );
        assert_eq!(
            Err(TreeError::ValueAlreadyStored),
            my_tree.add(7) // can't add duplicates
        );
    }
}
