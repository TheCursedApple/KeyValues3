use serde_json::{Map, Value as JsonValue, json};

use crate::{ObjectKey, Value};

pub fn to_json(value: &Value) -> JsonValue {
    match value {
        Value::File(_, v) => to_json(v),
        Value::Null => json!(null),
        Value::Bool(b) => json!(b),
        Value::Number(n) => json!(n),
        Value::String(s) => json!(s),
        Value::Array(a) => json!(a.iter().map(to_json).collect::<Vec<_>>()),
        Value::Object(o) => {
            let mut map = Map::new();
            for (k, v) in o {
                let key = match k {
                    ObjectKey::Identifier(i) => i,
                    ObjectKey::String(s) => s,
                };
                map.insert(key.to_owned(), to_json(v));
            }
            json!(map)
        }
        Value::Flag(f, v) => json!({
            "flag": json!(f),
            "value": to_json(v)
        }),
    }
}
