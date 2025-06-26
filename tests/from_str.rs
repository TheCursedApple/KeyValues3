use kv3::from_str;

#[test]
fn test_from_str() {
    use kv3::{ObjectKey, Value};

    let str = r#"
{
    key_without_quotes = "value"
    "key_with_quotes" = "value"
    integer = 123
    float = 123.45
    bool_true = true
    bool_false = false
    null = null
    array = [1, 2, 3]
    object =
    {
        key = "value"
    }
    flag_resource = resource:"path/to/resource"
    flag_subclass = subclass:
    {
        key = "value"
    }
}
"#;
    let value_1 = from_str(&str).unwrap();

    let value_2 = Value::Object(vec![
        (
            ObjectKey::Identifier("key_without_quotes".to_string()),
            Value::String("value".to_string()),
        ),
        (
            ObjectKey::String("key_with_quotes".to_string()),
            Value::String("value".to_string()),
        ),
        (
            ObjectKey::Identifier("integer".to_string()),
            Value::Number(123.),
        ),
        (
            ObjectKey::Identifier("float".to_string()),
            Value::Number(123.45),
        ),
        (
            ObjectKey::Identifier("bool_true".to_string()),
            Value::Bool(true),
        ),
        (
            ObjectKey::Identifier("bool_false".to_string()),
            Value::Bool(false),
        ),
        (ObjectKey::Identifier("null".to_string()), Value::Null),
        (
            ObjectKey::Identifier("array".to_string()),
            Value::Array(vec![
                Value::Number(1.),
                Value::Number(2.),
                Value::Number(3.),
            ]),
        ),
        (
            ObjectKey::Identifier("object".to_string()),
            Value::Object(vec![(
                ObjectKey::Identifier("key".to_string()),
                Value::String("value".to_string()),
            )]),
        ),
        (
            ObjectKey::Identifier("flag_resource".to_string()),
            Value::Flag(
                "resource".to_string(),
                Box::new(Value::String("path/to/resource".to_string())),
            ),
        ),
        (
            ObjectKey::Identifier("flag_subclass".to_string()),
            Value::Flag(
                "subclass".to_string(),
                Box::new(Value::Object(vec![(
                    ObjectKey::Identifier("key".to_string()),
                    Value::String("value".to_string()),
                )])),
            ),
        ),
    ]);

    assert_eq!(value_1, value_2);
}
