use std::hash::{Hash, Hasher};

/*
    AnyCase is a string container meant to be useable as a hashmap key that ignores case
*/
#[derive(Clone, Debug, Default)]
pub struct AnyCase(String);

impl AnyCase {

    pub fn from_string(string: String) -> Self {
        Self(string)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn ref_inner(&self) -> &String {
        &self.0
    }
}

impl Hash for AnyCase {
    fn hash<H>(&self, h: &mut H) where H: Hasher { 
        self.0.to_ascii_lowercase().hash(h)
    }
}

impl Into<AnyCase> for String {
    fn into(self) -> AnyCase {
        AnyCase::from_string(self)
    }
}

impl PartialEq for AnyCase {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_ascii_lowercase() == other.0.to_ascii_lowercase()
    }
}

impl Eq for AnyCase {}