use std::{cell::RefCell, rc::Rc};

use crate::errors::TreeError;



/// My implementation of a regular (unbalanced) **binary search tree**
/// for unique values (no duplicates).
/// 
/// Currently holds "u32" data.
/// 
/// TODO: make generic
pub struct BinTree {
    root: Option<Rc<RefCell<Node>>>,
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
            None => self.root = Some(Rc::new(RefCell::new(Node::new(value)))),
            Some(branch) => branch.as_ref().borrow_mut().add(value)?,
        }
        self.size += 1;
        Ok(())
    }

    /// Adds all members of a collection (vector, array, whatever) to the tree,
    /// skipping over any that would be duplicates, so no error will stop the batch.
    pub fn add_all_skipping_duplicates<T: IntoIterator<Item = u32>>(&mut self, collection: T) -> Result<(),TreeError> {
        for elem in collection.into_iter() {
            let _ = self.add(elem);
        }
        Ok(())
    }

    /// Get the number of values in the tree
    pub fn get_size(&self) -> u32 {
        self.size
    }

    /// Returns true if the value is currently a member of the tree
    pub fn contains(&self, value: &u32) -> bool {
        if self.size == 0 {
            return false;
        } else {
            return self.root.as_ref().unwrap().borrow().contains(value);
        }
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
            self.root.as_ref().unwrap().borrow().collect_values_l_to_r(&mut vals);
            vals
        }
    }

    /// Returns all the values in the tree as an ordered Vec from greatest to least  (right to left).
    pub fn as_vec_r_to_l(&self) -> Vec<u32> {
        if self.size == 0 {
            return Vec::new();
        } else {
            let mut vals = Vec::new();
            self.root.as_ref().unwrap().borrow().collect_values_r_to_l(&mut vals);
            vals
        }
    }

    /// If the value is in the tree, delete it.  Otherwise a TreeError::ValueNotFound will be returned.
    pub fn drop(&mut self, value: u32) -> Result<(),TreeError> {
        // if no root exists: return TreeError::ValueNotFound
        if self.root.is_none() {
            return Err(TreeError::ValueNotFound);
        }
        // if root has the value:
        if self.root.as_ref().unwrap().borrow().value == value {
            // - if it has no children, just replace it with None
            if self.root.as_ref().unwrap().borrow().is_leaf() {
                self.root = None;
                self.size = 0;
                return Ok(());
            }
            // - if it has no left branch, replace it with its right child (and subtree)
            if self.root.as_ref().unwrap().borrow().left.is_none() {
                let temp = self.root.as_ref().unwrap().borrow().right.clone();
                self.root = temp;
                self.size -= 1;
                return Ok(());
            }
            // - if it has no right branch, replace it with its left child (and subtree)
            if self.root.as_ref().unwrap().borrow().right.is_none() {
                let temp = self.root.as_ref().unwrap().borrow().left.clone();
                self.root = temp;
                self.size -= 1;
                return Ok(());
            }
            // - if the root's right child is a leaf, replace its value with its right leaf (and drop that leaf)
            if self.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().is_leaf() {
                let val = self.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().value;
                self.root.as_mut().unwrap().borrow_mut().value = val;
                self.root.as_ref().unwrap().borrow_mut().right = None;
                self.size -= 1;
                return Ok(());
            }
            // - otherwise, if the root's left child is a leaf, replace its value with its left leaf (and drop that leaf)
            if self.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().is_leaf() {
                let val = self.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().value;
                self.root.as_mut().unwrap().borrow_mut().value = val;
                self.root.as_ref().unwrap().borrow_mut().left = None;
                self.size -= 1;
                return Ok(());
            }
            // - if we get to this point, both children are branches. Replace the root's value with its immediate successor, 
            //   then recursively tell its right branch to remove that successor
        }
        // if root does NOT have the value:
        // - if the value is less, 
        //   - if the root has a left child, recursively call 'drop' on the left
        //   - otherwise throw ValueNotFound
        // - if the value is greater,
        //   - if the root has a right child, recursively call 'drop' on the right
        //   - otherwise throw ValueNotFound
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
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {

    pub fn new(value: u32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    /// Insert a value
    pub fn add(&mut self, value: u32) -> Result<(),TreeError> {
        if value == self.value {
            // no duplicates allowed in this kind of tree
            return Err(TreeError::ValueAlreadyStored)
        }
        if value < self.value {
            // add to the left branch
            match &mut self.left {
                None => self.left = Some(Rc::new(RefCell::new(Node::new(value)))),
                Some(branch) => branch.borrow_mut().add(value)?,
            }
            return Ok(())
        } else {
            // add it to the right branch
            match &mut self.right {
                None => self.right = Some(Rc::new(RefCell::new(Node::new(value)))),
                Some(branch) => branch.borrow_mut().add(value)?,
            }
            return Ok(())
        }
    }

    /// Returns true if the value is currently a member of the (sub)tree
    pub fn contains(&self, value: &u32) -> bool {
        if *value == self.value {
            return true;
        }
        if value < &self.value {
            match &self.left {
                Some(node) => node.borrow().contains(value),
                None => return false
            }
        } else {
            match &self.right {
                Some(node) => node.borrow().contains(value),
                None => return false
            }
        }
    }

    /// Returns true if the node is a leaf or terminal node, with no child nodes of its own.
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// Recursively add values to the borrowed vector, traversing the tree from left to right.
    pub fn collect_values_l_to_r(&self, value_vector: &mut Vec<u32>) {
        match &self.left {
            Some(node) => node.borrow().collect_values_l_to_r(value_vector),
            None => (),
        }
        value_vector.push(self.value.clone());
        match &self.right {
            Some(node) => node.borrow().collect_values_l_to_r(value_vector),
            None => (),
        }
    }

    /// Recursively add values to the borrowed vector, traversing the tree from right to left.
    pub fn collect_values_r_to_l(&self, value_vector: &mut Vec<u32>) {
        match &self.right {
            Some(node) => node.borrow().collect_values_r_to_l(value_vector),
            None => (),
        }
        value_vector.push(self.value.clone());
        match &self.left {
            Some(node) => node.borrow().collect_values_r_to_l(value_vector),
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
    fn add_collection() {
        let mut my_tree = BinTree::new();
        assert_eq!( Ok(()), my_tree.add_all_skipping_duplicates(vec!(1,2,3,4,5)));
        assert_eq!( Ok(()), my_tree.add_all_skipping_duplicates([6,7,8,9,10]));
        assert_eq!( 10, my_tree.get_size() );
        assert_eq!( Ok(()), my_tree.add_all_skipping_duplicates([5,10,15,20])); // duplicates should NOT cause a panic
        assert_eq!( 12, my_tree.get_size() ); // duplicates were skipped
    }

    #[test]
    fn test_contains() {
        let mut my_tree = BinTree::new();
        assert_eq!( Ok(()), my_tree.add_all_skipping_duplicates(vec!(8,6,7,5,3,0,9)));
        assert_eq!( 7, my_tree.get_size() );
        assert!( my_tree.contains(&7) );
        assert!( my_tree.contains(&8) );
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

    #[test]
    fn test_dropping_values() {

        // an empty tree
        let mut my_tree = BinTree::new();
        assert_eq!( 0, my_tree.get_size() );
        assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop(1) );

        // a tree with only a root node
        let mut my_tree = BinTree::new();
        let _ = my_tree.add(1);
        assert_eq!( 1, my_tree.get_size() );
        //assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop(4) );
        assert_eq!( Ok(()), my_tree.drop(1) );
        assert_eq!( 0, my_tree.get_size() );

        // an unbalanced tree with no left branch from the root
        let mut my_tree = BinTree::new();
        let _ = my_tree.add_all_skipping_duplicates([1,2,3]);
        assert_eq!( 1, my_tree.root.as_ref().unwrap().borrow().value ); // root is 1
        assert_eq!( 3, my_tree.get_size() );
        //assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop(4) );
        assert_eq!( Ok(()), my_tree.drop(1) );
        assert_eq!( vec!(2,3), my_tree.as_vec_l_to_r() );
        assert_eq!( 2, my_tree.get_size() );

        // an unbalanced tree with no right branch from the root
        let mut my_tree = BinTree::new();
        let _ = my_tree.add_all_skipping_duplicates([3,1,2]);
        assert_eq!( 3, my_tree.root.as_ref().unwrap().borrow().value ); // root is 3
        assert_eq!( 3, my_tree.get_size() );
        //assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop(4) );
        assert_eq!( Ok(()), my_tree.drop(3) );
        assert_eq!( vec!(1,2), my_tree.as_vec_l_to_r() );
        assert_eq!( 2, my_tree.get_size() );

        // a tree where the root has two leaves
        let mut my_tree = BinTree::new();
        let _ = my_tree.add_all_skipping_duplicates([2,1,3]);
        assert_eq!( 2, my_tree.root.as_ref().unwrap().borrow().value ); // root is 2
        assert_eq!( 3, my_tree.get_size() );
        //assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop(4) );
        assert_eq!( Ok(()), my_tree.drop(2) );
        assert_eq!( vec!(1,3), my_tree.as_vec_l_to_r() );
        assert_eq!( 2, my_tree.get_size() );

        // a tree where the root has a leaf on the left, branching node on the right
        let mut my_tree = BinTree::new();
        let _ = my_tree.add_all_skipping_duplicates([2,1,5,3,7]);
        assert_eq!( 2, my_tree.root.as_ref().unwrap().borrow().value ); // root is 2
        assert_eq!( 5, my_tree.get_size() );
        //assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop(4) );
        assert_eq!( Ok(()), my_tree.drop(2) );
        assert_eq!( vec!(1,3,5,7), my_tree.as_vec_l_to_r() );
        assert_eq!( 4, my_tree.get_size() );

    }

}
