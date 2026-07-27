use std::{fmt, write};

# [derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Deck {
    pub name: String,
}


impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Deck: {}", self.name)
    }   
}