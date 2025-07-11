use std::fmt::Debug;

use crate::{Header, ObjectKey};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    File(Header, Box<Value>),
    Object(Vec<(ObjectKey, Value)>),
    Array(Vec<Value>),
    Flag(String, Box<Value>),
    String(String),
    MultilineString(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Value {
    pub fn is_file(&self) -> bool {
        matches!(self, Value::File(_, _))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn is_flag(&self) -> bool {
        matches!(self, Value::Flag(_, _))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_multiline_string(&self) -> bool {
        matches!(self, Value::MultilineString(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}
