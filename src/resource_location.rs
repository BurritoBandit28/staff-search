// from https://github.com/BurritoBandit28/Memory-Game/blob/master/src/resource_location.rs

use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
/// Reference a resource from a resource hashmap, whether that is a sound, texture or tile. Can be used for other things too, not exclusively for file stuff.
pub struct ResourceLocation {
    pub namespace : String,
    pub path : String,
}


impl ResourceLocation {
    /// Create a new instance of a resource location given a namespace and path
    pub fn new(namespace : &str, path : &str) -> Self {
        Self {
            namespace : namespace.to_string(),
            path : path.to_string(),
        }
    }

    /// Create empty
    pub fn empty() -> Self {
        Self {
            namespace : "none".to_string(),
            path : "none".to_string(),
        }
    }

    /// Set the namespace part of a resource location
    pub fn set_namespace(&mut self, namespace : String) {
        self.namespace = namespace;
    }

    /// Set the path part of a resource location
    pub fn set_path(&mut self, path : String) {
        self.path = path;
    }

    /// Get the resource location as a string, in the form `namespace:path`
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }

    /// Parse a resource location from a string
    pub fn parse(value : String) -> Self {
        let vals = value.split(":").into_iter().collect::<Vec<&str>>();
        Self::new(vals.get(0).unwrap(), vals.get(1).unwrap())
    }
}