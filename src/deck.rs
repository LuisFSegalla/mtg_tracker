use std::{fmt, write};

# [derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Format {
    Pauper,
    Modern,
} 

# [derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Deck {
    pub name: String,
    pub format: Format,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Modern => write!(f, "Modern"),
            Format::Pauper => write!(f, "Pauper"),
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Deck: {}\nFormat: {}", self.name, self.format)
    }   
}