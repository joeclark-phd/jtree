use std::{error::Error, fmt};


#[derive(PartialEq, Debug)]
pub enum TreeError {
    /// Caller attempted to add a duplicate value to a tree that only accepts unique values.
    ValueAlreadyStored,
    ValueNotFound,
}

impl fmt::Display for TreeError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            TreeError::ValueAlreadyStored => "Caller attempted to add a duplicate value to a tree that only accepts unique values.",
            TreeError::ValueNotFound => "Specified value was not found in the tree.",
        }.to_string();        
        write!(f, "TreeError: {description}")
    }
}

impl Error for TreeError {}
