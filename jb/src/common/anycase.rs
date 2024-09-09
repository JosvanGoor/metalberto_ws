use std::fmt::Display;
use std::hash::{Hash, Hasher};

/*
    AnyCase is a string container meant to be useable as a hashmap key that ignores case
*/
#[derive(Clone, Debug, Default)]
pub struct AnyCase(String);

impl AnyCase {
    pub fn new(string: String) -> Self {
        Self(string)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn ref_inner(&self) -> &String {
        &self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Display for AnyCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Eq for AnyCase {}

impl Hash for AnyCase {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.0.to_ascii_lowercase().hash(h)
    }
}

impl From<String> for AnyCase {
    fn from(value: String) -> Self {
        AnyCase::new(value)
    }
}

impl From<&str> for AnyCase {
    fn from(value: &str) -> Self {
        AnyCase::new(String::from(value))
    }
}

impl PartialEq for AnyCase {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_ascii_lowercase() == other.0.to_ascii_lowercase()
    }
}
