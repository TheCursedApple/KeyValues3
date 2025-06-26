use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::{Header, Metadata, ObjectKey, Value};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct KV3Parser;

pub fn from_str(str: &str) -> Result<Value, Error<Rule>> {
    let file = KV3Parser::parse(Rule::file, str)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> Value {
        match pair.as_rule() {
            Rule::file => {
                let mut inner = pair.into_inner();

                let next = inner.next().unwrap();

                if next.as_rule() == Rule::header {
                    let metadata = next
                        .into_inner()
                        .map(|pair| {
                            let mut inner = pair.into_inner();

                            let key = inner.next().unwrap().as_str().to_owned();
                            let value = inner.next().unwrap().as_str().to_owned();
                            let version = inner.next().unwrap().as_str().to_owned();

                            Metadata {
                                key,
                                value,
                                version,
                            }
                        })
                        .collect::<Vec<_>>();
                    let header = Header(metadata);
                    let root = Box::new(parse_value(inner.next().unwrap()));
                    Value::File(header, root)
                } else {
                    parse_value(next)
                }
            }
            Rule::object => Value::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner = pair.into_inner();

                        let key = inner.next().unwrap();
                        let key = match key.as_rule() {
                            Rule::identifier => ObjectKey::Identifier(key.as_str().to_owned()),
                            Rule::string => ObjectKey::String(
                                key.into_inner().next().unwrap().as_str().to_owned(),
                            ),
                            _ => unreachable!(),
                        };

                        let value = parse_value(inner.next().unwrap());

                        (key, value)
                    })
                    .collect(),
            ),
            Rule::array => Value::Array(pair.into_inner().map(parse_value).collect()),
            Rule::flag => {
                let mut inner = pair.into_inner();
                let key = inner.next().unwrap().as_str().to_owned();
                let value = parse_value(inner.next().unwrap());
                Value::Flag(key, Box::new(value))
            }
            Rule::string => Value::String(pair.into_inner().next().unwrap().as_str().to_owned()),
            Rule::number => Value::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => Value::Bool(pair.as_str().parse().unwrap()),
            Rule::null => Value::Null,
            _ => unreachable!(),
        }
    }

    Ok(parse_value(file))
}
