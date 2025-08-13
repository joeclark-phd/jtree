use std::{error::Error, fmt};


#[derive(PartialEq, Debug)]
pub enum TreeError {
    /// Caller attempted to add a duplicate value to a tree that only accepts unique values.
    ValueAlreadyStored,
}

impl fmt::Display for TreeError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            TreeError::ValueAlreadyStored => "Caller attempted to add a duplicate value to a tree that only accepts unique values.",
        }.to_string();        
        write!(f, "TreeError: {description}")
    }
}

impl Error for TreeError {}
