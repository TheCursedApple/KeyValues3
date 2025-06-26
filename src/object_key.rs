use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectKey {
    Identifier(String),
    String(String),
}

impl ObjectKey {
    pub fn is_identifier(&self) -> bool {
        matches!(self, ObjectKey::Identifier(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, ObjectKey::String(_))
    }
}

impl Display for ObjectKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectKey::Identifier(id) => write!(f, "{}", id),
            ObjectKey::String(str) => write!(f, "\"{}\"", str),
        }
    }
}
