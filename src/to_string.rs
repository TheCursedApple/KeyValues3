use crate::Value;

pub fn to_string(value: &Value) -> String {
    fn format_indent(indent_level: usize) -> String {
        "\t".repeat(indent_level)
    }

    fn to_string_with_indent(value: &Value, indent_level: usize) -> String {
        let prev_indent = format_indent(indent_level - 1);
        let curr_indent = format_indent(indent_level);

        match value {
            Value::File(header, root) => {
                format!("{}\n{}", header, to_string_with_indent(root, indent_level))
            }
            Value::Object(map) => {
                if map.is_empty() {
                    return format!("{{\n{}}}", prev_indent);
                }

                let pairs = map
                    .iter()
                    .map(|(key, value)| {
                        let separator = if value.is_object() || value.is_array() {
                            format!("\n{}", format_indent(indent_level))
                        } else {
                            " ".to_owned()
                        };
                        format!(
                            "{}{} ={}{}",
                            curr_indent,
                            key,
                            separator,
                            to_string_with_indent(value, indent_level + 1)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("{{\n{}\n{}}}", pairs, prev_indent)
            }

            Value::Array(values) => {
                if values.is_empty() {
                    return format!("[\n{}]", prev_indent);
                }

                let elements = values
                    .iter()
                    .map(|value| {
                        format!(
                            "{}{}",
                            curr_indent,
                            to_string_with_indent(value, indent_level + 1)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",\n");

                format!("[\n{},\n{}]", elements, prev_indent)
            }

            Value::Flag(key, value) => {
                let separator = if value.is_object() || value.is_array() {
                    format!("\n{}", prev_indent)
                } else {
                    "".to_owned()
                };

                format!(
                    "{}:{}{}",
                    key,
                    separator,
                    to_string_with_indent(value, indent_level)
                )
            }

            Value::String(value) => format!("\"{}\"", value),
            Value::MultilineString(value) => format!("\"\"\"\n{}\n\"\"\"\n", value),
            Value::Number(value) => value.to_string(),
            Value::Bool(value) => value.to_string(),
            Value::Null => "null".to_string(),
        }
    }

    to_string_with_indent(value, 1).trim_end().to_string() + "\n"
}
