use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    pub key: String,
    pub value: String,
    pub version: String,
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:version{{{}}}", self.key, self.value, self.version)
    }
}
