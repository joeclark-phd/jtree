use crate::errors::TreeError;



/// My implementation of a regular (unbalanced) **binary search tree**
/// for unique values (no duplicates).
/// 
/// Currently holds "u32" data.
/// 
/// TODO: make generic
pub struct BinTree {
    root: Option<Node>,
    size: u32,
}

impl BinTree {

    /// Create a new tree with no data
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// Insert a value
    pub fn add(&mut self, value: u32) -> Result<(),TreeError> {
        match &mut self.root {
            None => self.root = Some(Node::new(value)),
            Some(branch) => branch.add(value)?,
        }
        self.size += 1;
        Ok(())
    }

    /// Get the number of values in the tree
    pub fn get_size(&self) -> u32 {
        self.size
    }

    /// Short for `as_vec_l_to_r`, this method returns all the values in the tree as an ordered Vec
    /// from least to greatest.
    pub fn as_vec(&self) -> Vec<u32> {
        self.as_vec_l_to_r()
    }

    /// Returns all the values in the tree as an ordered Vec from least to greatest (left to right).
    pub fn as_vec_l_to_r(&self) -> Vec<u32> {
        if self.size == 0 {
            return Vec::new();
        } else {
            let mut vals = Vec::new();
            self.root.as_ref().unwrap().collect_values_l_to_r(&mut vals);
            vals
        }
    }

    /// Returns all the values in the tree as an ordered Vec from greatest to least  (right to left).
    pub fn as_vec_r_to_l(&self) -> Vec<u32> {
        if self.size == 0 {
            return Vec::new();
        } else {
            let mut vals = Vec::new();
            self.root.as_ref().unwrap().collect_values_r_to_l(&mut vals);
            vals
        }
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

    /// Recursively add values to the borrowed vector, traversing the tree from left to right.
    pub fn collect_values_l_to_r(&self, value_vector: &mut Vec<u32>) {
        match &self.left {
            Some(node) => node.collect_values_l_to_r(value_vector),
            None => (),
        }
        value_vector.push(self.value.clone());
        match &self.right {
            Some(node) => node.collect_values_l_to_r(value_vector),
            None => (),
        }
    }

    /// Recursively add values to the borrowed vector, traversing the tree from right to left.
    pub fn collect_values_r_to_l(&self, value_vector: &mut Vec<u32>) {
        match &self.right {
            Some(node) => node.collect_values_r_to_l(value_vector),
            None => (),
        }
        value_vector.push(self.value.clone());
        match &self.left {
            Some(node) => node.collect_values_r_to_l(value_vector),
            None => (),
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_unique_items() {
        let mut my_tree = BinTree::new();
        assert_eq!( 0, my_tree.get_size() );
        assert_eq!( Ok(()), my_tree.add(5) );
        assert_eq!( Ok(()), my_tree.add(3) );
        assert_eq!( Ok(()), my_tree.add(7) );
        assert_eq!( 3, my_tree.get_size() );
        assert_eq!(
            Err(TreeError::ValueAlreadyStored),
            my_tree.add(7) // can't add duplicates
        );
    }

    #[test]
    fn collect_values_l_to_r() {
        let mut my_tree = BinTree::new();
        assert_eq!( Ok(()), my_tree.add(5) );
        assert_eq!( Ok(()), my_tree.add(3) );
        assert_eq!( Ok(()), my_tree.add(7) );
        let output = my_tree.as_vec();
        println!("{:?}", output);
        assert_eq!(vec!(3,5,7), output);
    }

    #[test]
    fn collect_values_r_to_l() {
        let mut my_tree = BinTree::new();
        assert_eq!( Ok(()), my_tree.add(5) );
        assert_eq!( Ok(()), my_tree.add(3) );
        assert_eq!( Ok(()), my_tree.add(7) );
        let output = my_tree.as_vec_r_to_l();
        println!("{:?}", output);
        assert_eq!(vec!(7,5,3), output);
    }

}
