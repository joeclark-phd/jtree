use std::{cmp::max, i32::MAX};

use crate::errors::TreeError;

/// # Joe's AVL Tree
/// 
/// My implementation of a self-balancing **AVL tree** for unique values.
/// It works like a binary search tree (such as `Jbst`) but keeps the height of all
/// subtrees balanced.  Regular binary search trees can become very unbalanced
/// (and reduce or eliminate their usefulness as data structured) depending on the
/// order in which values are inserted.  A self-balancing structure modifies its
/// structure when inserts or deletions would make it lopsided.  This guarantees
/// that lookups will remain O(log(n)) complexity.
pub struct Javlt<T: PartialEq + PartialOrd + Clone> {
    size: u32,
    root: Option<Box<Node<T>>>,
}

impl <T: PartialEq + PartialOrd + Clone> Javlt<T> {

    /// Create a new tree with no data
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// Insert a value
    pub fn add(&mut self, value: T) -> Result<(),TreeError> {
        match &mut self.root {
            None => self.root = Some(Box::new(Node::new(value))),
            Some(branch) => branch.add(value)?, // TODO: handle errors if any are possible
        }
        self.size += 1;
        Ok(())
    }

    /// Get the number of values in the tree
    pub fn get_size(&self) -> u32 {
        self.size
    }

    /// Returns the 'value' field of the root node; used for automated tests only
    #[cfg(test)]
    fn get_root_value(&self) -> Option<T> {
        return match &self.root {
            None => None,
            Some(node) => Some(node.value.clone()),
        }
    }

    /// Returns true if the value is currently a member of the tree
    pub fn contains(&self, value: &T) -> bool {
        return match &self.root {
            None => false,
            Some(branch) => branch.contains(value), 
        };
    }

    /// Short for `as_vec_l_to_r`, this method returns all the values in the tree as an ordered Vec
    /// from least to greatest.
    pub fn as_vec(&self) -> Vec<T> {
        self.as_vec_l_to_r()
    }

    /// Returns all the values in the tree as an ordered Vec from least to greatest (left to right).
    pub fn as_vec_l_to_r(&self) -> Vec<T> {
        return match &self.root {
            None => Vec::new(),
            Some(branch) => {
                let mut vals = Vec::new();
                branch.collect_values_l_to_r(&mut vals);
                vals 
            }
        };
    }

    /// Returns all the values in the tree as an ordered Vec from greatest to least  (right to left).
    pub fn as_vec_r_to_l(&self) -> Vec<T> {
        return match &self.root {
            None => Vec::new(),
            Some(branch) => {
                let mut vals = Vec::new();
                branch.collect_values_r_to_l(&mut vals);
                vals 
            }
        };
    }

    /// Returns the smallest/lowest value in the tree, if any.
    pub fn least_value(&self) -> Option<T> {
        return match &self.root {
            None => None,
            Some(subtree) => Some(subtree.least_value()),
        }
    }

    /// Returns the largest/highest value in the tree, if any.
    pub fn greatest_value(&self) -> Option<T> {
        return match &self.root {
            None => None,
            Some(subtree) => Some(subtree.greatest_value()),
        }
    }

}

struct Node<T: PartialEq + PartialOrd + Clone> {
    value: T,
    height: u32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl <T: PartialEq + PartialOrd + Clone> Node<T> {

    pub fn new(value: T) -> Self {
        Self {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    /// Insert a value
    pub fn add(&mut self, value: T) -> Result<(),TreeError> {
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
            self.height = self.compute_height();
            self.rebalance();
            return Ok(())
        } else {
            // add it to the right branch
            match &mut self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(branch) => branch.add(value)?,
            }
            self.height = self.compute_height();
            self.rebalance();
            return Ok(())
        }
    }

    /// Height of a subtree is the height of its largest child subtree, plus 1.
    fn compute_height(&self) -> u32 {
        let left_height = if self.left.is_none() {0} else {self.left.as_ref().unwrap().height};
        let right_height = if self.right.is_none() {0} else {self.right.as_ref().unwrap().height};
        max(left_height, right_height) + 1
    }

    /// Balancing factor is the height of the right subtree minus the height of the left subtree.
    /// Although this will never be outside the range -2 to +2, we use i64 for safe type casting.
    fn compute_balancing_factor(&self) -> i64 {
        let left_height = if self.left.is_none() {0} else {self.left.as_ref().unwrap().height};
        let right_height = if self.right.is_none() {0} else {self.right.as_ref().unwrap().height};
        i64::from(right_height) - i64::from(left_height)
    }

    fn rebalance(&mut self) {
        let bf = self.compute_balancing_factor();
        if bf >= -1 && bf <= 1 {
            // tree is balanced, do nothing
            return;
        }
        if bf > 1 {
            // tree is right-heavy
            if self.right.as_ref().unwrap().compute_balancing_factor() > 0 {
                // right child is right-heavy, this is a Right Right rotation
                let mut new_left_node = Node::new(self.value.clone());
                new_left_node.left = self.left.take();
                new_left_node.right = self.right.as_mut().unwrap().left.take();
                self.left = Some(Box::new(new_left_node));
                self.value = self.right.as_ref().unwrap().value.clone();
                let new_right_node = self.right.as_mut().unwrap().right.take();
                self.right = new_right_node;
            } else {
                // right child is left-heavy, this is a Right Left situation
                // step 1: rotate the right child's subtree right
                let mut new_right_right = Node::new(self.right.as_ref().unwrap().value.clone());
                new_right_right.right = self.right.as_mut().unwrap().right.take();
                new_right_right.left = self.right.as_mut().unwrap().left.as_mut().unwrap().right.take();
                let mut new_right = Node::new(self.right.as_ref().unwrap().left.as_ref().unwrap().value.clone());
                new_right.right = Some(Box::new(new_right_right));
                new_right.left = self.right.as_mut().unwrap().left.as_mut().unwrap().left.take();
                self.right = Some(Box::new(new_right));
                // step 2: rotate our subtree left (as in the above Right Right case)
                let mut new_left_node = Node::new(self.value.clone());
                new_left_node.left = self.left.take();
                new_left_node.right = self.right.as_mut().unwrap().left.take();
                self.left = Some(Box::new(new_left_node));
                self.value = self.right.as_ref().unwrap().value.clone();
                let final_right_node = self.right.as_mut().unwrap().right.take();
                self.right = final_right_node;
            }
        } else {
            // tree is left-heavy
            if self.left.as_ref().unwrap().compute_balancing_factor() < 0 {
                // left child is left-heavy, this is a Left Left rotation
                let mut new_right_node = Node::new(self.value.clone());
                new_right_node.right = self.right.take();
                new_right_node.left = self.left.as_mut().unwrap().right.take();
                self.right = Some(Box::new(new_right_node));
                self.value = self.left.as_ref().unwrap().value.clone();
                let new_left_node = self.left.as_mut().unwrap().left.take();
                self.left = new_left_node;
            } else {
                // left child is right-heavy, this is a Right Left rotation
                // step 1: rotate the left child's subtree left
                let mut new_left_left = Node::new(self.left.as_ref().unwrap().value.clone());
                new_left_left.left = self.left.as_mut().unwrap().left.take();
                new_left_left.right = self.left.as_mut().unwrap().right.as_mut().unwrap().left.take();
                let mut new_left = Node::new(self.left.as_ref().unwrap().right.as_ref().unwrap().value.clone());
                new_left.left = Some(Box::new(new_left_left));
                new_left.right = self.left.as_mut().unwrap().right.as_mut().unwrap().right.take();
                self.left = Some(Box::new(new_left));
                // step 2: rotate our subtree right (as in the above Left Left case)
                let mut new_right_node = Node::new(self.value.clone());
                new_right_node.right = self.right.take();
                new_right_node.left = self.left.as_mut().unwrap().right.take();
                self.right = Some(Box::new(new_right_node));
                self.value = self.left.as_ref().unwrap().value.clone();
                let final_left_node = self.left.as_mut().unwrap().left.take();
                self.left = final_left_node;
            }
        }
    }

    /// Returns true if the value is currently a member of the (sub)tree
    pub fn contains(&self, value: &T) -> bool {
        if *value == self.value {
            return true;
        }
        if *value < self.value {
            match &self.left {
                Some(node) => node.contains(value),
                None => return false
            }
        } else {
            match &self.right {
                Some(node) => node.contains(value),
                None => return false
            }
        }
    }

    /// Returns the smallest/lowest value in this (sub)tree.
    pub fn least_value(&self) -> T {
        return match &self.left {
            None => self.value.clone(),
            Some(left_child) => left_child.least_value(),
        }
    }

    /// Returns the largest/highest value in this (sub)tree.
    pub fn greatest_value(&self) -> T {
        return match &self.right {
            None => self.value.clone(),
            Some(right_child) => right_child.greatest_value(),
        }
    }

    /// Recursively add values to the borrowed vector, traversing the tree from left to right.
    pub fn collect_values_l_to_r(&self, value_vector: &mut Vec<T>) {
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
    pub fn collect_values_r_to_l(&self, value_vector: &mut Vec<T>) {
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
    fn do_left_left_rebalance() {
        let mut my_tree = Javlt::<u32>::new();
        assert_eq!( 0, my_tree.get_size() );
        assert_eq!( Ok(()), my_tree.add(5) );
        assert_eq!( Ok(()), my_tree.add(3) );
        assert_eq!( Some(5), my_tree.get_root_value() );
        assert_eq!( Ok(()), my_tree.add(1) );
        // this results in a Left Left unbalanced tree; it should automatically be rebalanced so 3 instead of 5 is the root
        assert_eq!( Some(3), my_tree.get_root_value() );
        assert_eq!( 0, my_tree.root.as_ref().unwrap().compute_balancing_factor() );

        assert_eq!( 3, my_tree.get_size() );
        assert_eq!(
            Err(TreeError::ValueAlreadyStored),
            my_tree.add(5) // can't add duplicates
        );
    }

    #[test]
    fn do_right_right_rebalance() {
        let mut my_tree = Javlt::<u32>::new();
        assert_eq!( 0, my_tree.get_size() );
        assert_eq!( Ok(()), my_tree.add(2) );
        assert_eq!( Ok(()), my_tree.add(4) );
        assert_eq!( Some(2), my_tree.get_root_value() );
        assert_eq!( Ok(()), my_tree.add(6) );
        // this results in a Right Right unbalanced tree; it should automatically be rebalanced so 4 instead of 2 is the root
        assert_eq!( Some(4), my_tree.get_root_value() );
        assert_eq!( 0, my_tree.root.as_ref().unwrap().compute_balancing_factor() );

        assert_eq!( 3, my_tree.get_size() );
        assert_eq!(
            Err(TreeError::ValueAlreadyStored),
            my_tree.add(4) // can't add duplicates
        );
    }

    #[test]
    fn do_left_right_rebalance() {
        let mut my_tree = Javlt::<u32>::new();
        assert_eq!( 0, my_tree.get_size() );
        assert_eq!( Ok(()), my_tree.add(2) );
        assert_eq!( Ok(()), my_tree.add(1) );
        assert_eq!( Ok(()), my_tree.add(6) );
        assert_eq!( Ok(()), my_tree.add(4) );
        assert_eq!( Ok(()), my_tree.add(7) );
        assert_eq!( Some(2), my_tree.get_root_value() );
        assert_eq!( Ok(()), my_tree.add(3) );
        // this results in a Left Right unbalanced tree; it should automatically be rebalanced so 4 instead of 2 is the root
        assert_eq!( Some(4), my_tree.get_root_value() );
        assert_eq!( 0, my_tree.root.as_ref().unwrap().compute_balancing_factor() );

        assert_eq!( 6, my_tree.get_size() );
        assert_eq!(
            Err(TreeError::ValueAlreadyStored),
            my_tree.add(7) // can't add duplicates
        );
    }

    #[test]
    fn do_right_left_rebalance() {
        let mut my_tree = Javlt::<u32>::new();
        assert_eq!( 0, my_tree.get_size() );
        assert_eq!( Ok(()), my_tree.add(6) );
        assert_eq!( Ok(()), my_tree.add(3) );
        assert_eq!( Ok(()), my_tree.add(7) );
        assert_eq!( Ok(()), my_tree.add(2) );
        assert_eq!( Ok(()), my_tree.add(4) );
        assert_eq!( Some(6), my_tree.get_root_value() );
        assert_eq!( Ok(()), my_tree.add(5) );
        // this results in a Right Left unbalanced tree; it should automatically be rebalanced so 4 instead of 6 is the root
        assert_eq!( Some(4), my_tree.get_root_value() );
        assert_eq!( 0, my_tree.root.as_ref().unwrap().compute_balancing_factor() );

        assert_eq!( 6, my_tree.get_size() );
        assert_eq!(
            Err(TreeError::ValueAlreadyStored),
            my_tree.add(7) // can't add duplicates
        );
    }



}