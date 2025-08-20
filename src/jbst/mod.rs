use std::fmt;

use crate::errors::TreeError;



/// # Joe's Binary Search Tree
/// 
/// My implementation of a regular (unbalanced) **binary search tree**
/// for unique values (no duplicates).
///
///     use jtree::Jbst;
///     use jtree::errors::TreeError;
/// 
///     let mut my_tree = Jbst::new();
///     let _ = my_tree.add(2);
///     let _ = my_tree.add(1);
///     let _ = my_tree.add(3);
///     assert_eq!( 3, my_tree.get_size() );
///     assert_eq!( vec!(1,2,3), my_tree.as_vec() );
///     assert_eq!( Err(TreeError::ValueAlreadyStored), my_tree.add(1) ); // unique values only!
/// 
///     let mut tree_b = Jbst::from_collection([1,1,2,3,5]); // duplicate values are ignored but no error is thrown
///     assert_eq!( vec!(1,2,3,5), tree_b.as_vec() ); // the array was effectively converted into a set
///     assert!( tree_b.contains(&5) ); // fast test for set membership
/// 
/// Currently holds "u32" data.
/// 
/// TODO: make generic
pub struct Jbst {
    root: Option<Box<Node>>,
    size: u32,
}

impl Jbst {

    /// Create a new tree with no data
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// Create a new tree from a collection (vector, array, or whatever), skipping duplicates, effectively 
    /// turning a list into an ordered set of unique values.
    pub fn from_collection<T: IntoIterator<Item = u32>>(collection: T) -> Self {
        let mut new_tree = Self::new();
        let _ = new_tree.add_all_skipping_duplicates(collection);
        new_tree
    }

    /// Insert a value
    pub fn add(&mut self, value: u32) -> Result<(),TreeError> {
        match &mut self.root {
            None => self.root = Some(Box::new(Node::new(value))),
            Some(branch) => branch.add(value)?, // TODO: handle errors if any are possible
        }
        self.size += 1;
        Ok(())
    }

    /// Adds all members of a collection (vector, array, or whatever) to the tree,
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

    /// Returns the 'value' field of the root node; used for automated tests only
    #[cfg(test)]
    fn get_root_value(&self) -> Option<u32> {
        return match &self.root {
            None => None,
            Some(node) => Some(node.value),
        }
    }

    /// Returns true if the value is currently a member of the tree
    pub fn contains(&self, value: &u32) -> bool {
        return match &self.root {
            None => false,
            Some(branch) => branch.contains(value), 
        };
    }

    /// Short for `as_vec_l_to_r`, this method returns all the values in the tree as an ordered Vec
    /// from least to greatest.
    pub fn as_vec(&self) -> Vec<u32> {
        self.as_vec_l_to_r()
    }

    /// Returns all the values in the tree as an ordered Vec from least to greatest (left to right).
    pub fn as_vec_l_to_r(&self) -> Vec<u32> {
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
    pub fn as_vec_r_to_l(&self) -> Vec<u32> {
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
    pub fn least_value(&self) -> Option<u32> {
        return match &self.root {
            None => None,
            Some(subtree) => Some(subtree.least_value()),
        }
    }

    /// Returns the largest/highest value in the tree, if any.
    pub fn greatest_value(&self) -> Option<u32> {
        return match &self.root {
            None => None,
            Some(subtree) => Some(subtree.greatest_value()),
        }
    }

    /// If the value is in the tree, delete it.  Otherwise a TreeError::ValueNotFound will be returned.
    pub fn drop_value(&mut self, value: u32) -> Result<(),TreeError> {
        match self.root.take() {
            None => {
                self.root = None;
                return Err(TreeError::ValueNotFound);
            },
            Some(child) => {
                match child.drop_value(value) {
                    (Err(_), new_node) => {
                        self.root = new_node;
                        return Err(TreeError::ValueNotFound);
                    },
                    (Ok(_), new_node) => {
                        self.root = new_node;
                        self.size -= 1;
                        return Ok(());
                    }
                }
            },
        }
    }

}

impl Default for Jbst {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Jbst {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Jbst")
            .field("size", &self.get_size())
            .field("values", &self.as_vec())
            .finish()
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

    /// Insert a value
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

    /// Returns true if the value is currently a member of the (sub)tree
    pub fn contains(&self, value: &u32) -> bool {
        if *value == self.value {
            return true;
        }
        if value < &self.value {
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

    /// Returns true if the node is a leaf or terminal node, with no child nodes of its own.
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// Returns the smallest/lowest value in this (sub)tree.
    pub fn least_value(&self) -> u32 {
        return match &self.left {
            None => self.value,
            Some(left_child) => left_child.least_value(),
        }
    }

    /// Returns the largest/highest value in this (sub)tree.
    pub fn greatest_value(&self) -> u32 {
        return match &self.right {
            None => self.value,
            Some(right_child) => right_child.greatest_value(),
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

    /// If the value exists in this sub-tree, drop it, returning to the parent
    /// a pointer to the Node that replaces this one, or None if this node
    /// is removed by the change.  Called recursively.
    /// 
    /// Because 'self' is consumed, we need to return a node to replace it
    /// even in case of error, hence we're returning a tuple of Result (to be interpreted)
    /// and Option<Box<Node>> to replace the current node in the parent.
    /// 
    pub fn drop_value(mut self, value: u32) -> (Result<(),TreeError>, Option<Box<Node>>) {

        // if the value is less than this node's value, and we have a left child, call 'drop_value' on the left child
        if value < self.value {
            match self.left {
                None => return (Err(TreeError::ValueNotFound), Some(Box::new(self))),
                Some(left_child) => {
                    match left_child.drop_value(value) {
                        (Err(_), new_node) => {
                            self.left = new_node;
                            return (Err(TreeError::ValueNotFound), Some(Box::new(self)));
                        },
                        (Ok(_), new_node) => {
                            self.left = new_node;
                            return (Ok(()), Some(Box::new(self)));
                        } 
                    }
                }
            }
        }
        // if the value is greater than this node's value, and we have a right child, call 'drop_value' on the right child
        else if value > self.value {
            match self.right {
                None => return (Err(TreeError::ValueNotFound), Some(Box::new(self))),
                Some(right_child) => {
                    match right_child.drop_value(value) {
                        (Err(_), new_node) => {
                            self.right = new_node;
                            return (Err(TreeError::ValueNotFound), Some(Box::new(self)));
                        },
                        (Ok(_), new_node) => {
                            self.right = new_node;
                            return (Ok(()), Some(Box::new(self)));
                        } 
                    }
                }
            }
        }
        // if this node has the exact value:
        else {
            // - if it has no children, just replace it with None
            if self.is_leaf() {
                return (Ok(()), None);
            }
            // - if it has no left branch, replace it with its right child (and subtree)
            if self.left.is_none() {
                return (Ok(()), self.right);
            }
            // - if it has no right branch, replace it with its left child (and subtree)
            if self.right.is_none() {
                return (Ok(()), self.left);
            }
            // - if the root's right child is a leaf, replace its value with its right leaf (and drop that leaf)
            let right_child = self.right.as_ref().unwrap();
            if right_child.is_leaf() {
                self.value = right_child.value;
                self.right = None;
                return (Ok(()), Some(Box::new(self)));
            }
            // - otherwise, if the root's left child is a leaf, replace its value with its left leaf (and drop that leaf)
            let left_child = self.left.as_ref().unwrap();
            if left_child.is_leaf() {
                self.value = left_child.value;
                self.left = None;
                return (Ok(()), Some(Box::new(self)));
            }
            // - if we get to this point, both children are branches. Replace the root's value with its immediate successor, 
            //   then recursively tell its right branch to remove that successor
            self.value = right_child.least_value();
            self.right = self.right.unwrap().drop_value(self.value).1;
            return (Ok(()), Some(Box::new(self)));
        }

    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_unique_items() {
        let mut my_tree = Jbst::new();
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
        let mut my_tree = Jbst::new();
        assert_eq!( Ok(()), my_tree.add_all_skipping_duplicates(vec!(1,2,3,4,5)));
        assert_eq!( Ok(()), my_tree.add_all_skipping_duplicates([6,7,8,9,10]));
        assert_eq!( 10, my_tree.get_size() );
        assert_eq!( Ok(()), my_tree.add_all_skipping_duplicates([5,10,15,20])); // duplicates should NOT cause a panic
        assert_eq!( 12, my_tree.get_size() ); // duplicates were skipped
    }

    #[test]
    fn test_contains() {
        let mut my_tree = Jbst::new();
        assert_eq!( Ok(()), my_tree.add_all_skipping_duplicates(vec!(8,6,7,5,3,0,9)));
        assert_eq!( 7, my_tree.get_size() );
        assert!( my_tree.contains(&7) );
        assert!( my_tree.contains(&8) );
    }

    #[test]
    fn collect_values_l_to_r() {
        let mut my_tree = Jbst::new();
        assert_eq!( Ok(()), my_tree.add(5) );
        assert_eq!( Ok(()), my_tree.add(3) );
        assert_eq!( Ok(()), my_tree.add(7) );
        let output = my_tree.as_vec();
        println!("{:?}", output);
        assert_eq!(vec!(3,5,7), output);
    }

    #[test]
    fn collect_values_r_to_l() {
        let mut my_tree = Jbst::new();
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
        let mut my_tree = Jbst::new();
        assert_eq!( 0, my_tree.get_size() );
        assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop_value(1) );

        // a tree with only a root node
        let mut my_tree = Jbst::new();
        let _ = my_tree.add(1);
        assert_eq!( 1, my_tree.get_size() );
        assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop_value(4) );
        assert_eq!( Ok(()), my_tree.drop_value(1) );
        assert_eq!( 0, my_tree.get_size() );

        // an unbalanced tree with no left branch from the root
        let mut my_tree = Jbst::new();
        let _ = my_tree.add_all_skipping_duplicates([1,2,3]);
        assert_eq!( Some(1), my_tree.get_root_value() ); // root is 1
        assert_eq!( 3, my_tree.get_size() );
        assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop_value(4) );
        assert_eq!( Ok(()), my_tree.drop_value(1) );
        assert_eq!( vec!(2,3), my_tree.as_vec_l_to_r() );
        assert_eq!( 2, my_tree.get_size() );

        // an unbalanced tree with no right branch from the root
        let mut my_tree = Jbst::new();
        let _ = my_tree.add_all_skipping_duplicates([3,1,2]);
        assert_eq!( Some(3), my_tree.get_root_value() ); // root is 3
        assert_eq!( 3, my_tree.get_size() );
        assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop_value(4) );
        assert_eq!( Ok(()), my_tree.drop_value(3) );
        assert_eq!( vec!(1,2), my_tree.as_vec_l_to_r() );
        assert_eq!( 2, my_tree.get_size() );

        // a tree where the root has two leaves
        let mut my_tree = Jbst::new();
        let _ = my_tree.add_all_skipping_duplicates([2,1,3]);
        assert_eq!( Some(2), my_tree.get_root_value() ); // root is 2
        assert_eq!( 3, my_tree.get_size() );
        assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop_value(4) );
        assert_eq!( Ok(()), my_tree.drop_value(2) );
        assert_eq!( vec!(1,3), my_tree.as_vec_l_to_r() );
        assert_eq!( 2, my_tree.get_size() );

        // a tree where the root has a leaf on the left, branching node on the right
        let mut my_tree = Jbst::new();
        let _ = my_tree.add_all_skipping_duplicates([2,1,5,3,7]);
        assert_eq!( Some(2), my_tree.get_root_value() ); // root is 2
        assert_eq!( 5, my_tree.get_size() );
        assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop_value(4) );
        assert_eq!( Ok(()), my_tree.drop_value(2) );
        assert_eq!( vec!(1,3,5,7), my_tree.as_vec_l_to_r() );
        assert_eq!( 4, my_tree.get_size() );

        // a tree where the root has branching nodes on both sides
        let mut my_tree = Jbst::new();
        let _ = my_tree.add_all_skipping_duplicates([5,3,8,1,2,7,9]);
        assert_eq!( Some(5), my_tree.get_root_value() ); // root is 5
        assert_eq!( 7, my_tree.get_size() );
        assert_eq!( Err(TreeError::ValueNotFound), my_tree.drop_value(4) );
        assert_eq!( Ok(()), my_tree.drop_value(5) );
        assert_eq!( Some(7), my_tree.get_root_value() ); // root is now 7
        assert_eq!( vec!(1,2,3,7,8,9), my_tree.as_vec_l_to_r() );
        assert_eq!( 6, my_tree.get_size() );

    }

    #[test]
    fn test_greatest_and_least() {
        let mut my_tree = Jbst::new();
        assert_eq!( None, my_tree.least_value() );
        assert_eq!( None, my_tree.greatest_value() );
        let _ = my_tree.add_all_skipping_duplicates([5,3,8,1,2,7,9]);
        assert_eq!( Some(1), my_tree.least_value() );
        assert_eq!( Some(9), my_tree.greatest_value() );
    }

}
