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

        // TODO: modify the below to rotate the subtree in order to maintain balance

        if value < self.value {
            // add to the left branch
            match &mut self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(branch) => branch.add(value)?,
            }
            self.height = self.compute_height();
            let bf = self.compute_balancing_factor();
            if bf < -1 || bf > 1 {
                // rebalance!
                println!("bf is {:?}. we need to rebalance", bf);
            }
            return Ok(())
        } else {
            // add it to the right branch
            match &mut self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(branch) => branch.add(value)?,
            }
            self.height = self.compute_height();
            let bf = self.compute_balancing_factor();
            if bf < -1 || bf > 1 {
                // rebalance!
                println!("bf is {:?}. we need to rebalance", bf);
            }
            return Ok(())
        }
    }

    /// Height of a subtree is the height of its largest child subtree, plus 1.
    fn compute_height(&self) -> u32 {
        let left_height = if self.left.is_none() {0} else {self.left.as_ref().unwrap().height};
        let right_height = if self.right.is_none() {0} else {self.right.as_ref().unwrap().height};
        max(left_height, right_height) + 1
    }

    /// Balancing factor is the height of the left subtree minus the height of the right subtree.
    /// Although this will never be outside the range -2 to +2, we use i64 for safe type casting.
    fn compute_balancing_factor(&self) -> i64 {
        let left_height = if self.left.is_none() {0} else {self.left.as_ref().unwrap().height};
        let right_height = if self.right.is_none() {0} else {self.right.as_ref().unwrap().height};
        i64::from(left_height) - i64::from(right_height)
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