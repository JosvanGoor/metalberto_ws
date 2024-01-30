use std::hash::{Hash, Hasher};

/*
    AnyCase is a string container meant to be useable as a hashmap key that ignores case
*/
struct AnyCase(String);

impl Hash for AnyCase {
    fn hash<H>(&self, h: &mut H) where H: Hasher { 
        self.0.to_ascii_lowercase().hash(h)
    }
}

impl PartialEq for AnyCase {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_ascii_lowercase() == other.0.to_ascii_lowercase()
    }
}

impl Eq for AnyCase {}