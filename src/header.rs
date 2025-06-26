use std::fmt::Display;

use crate::Metadata;

#[derive(Debug, Clone, PartialEq)]
pub struct Header(pub Vec<Metadata>);

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let metadata = self
            .0
            .iter()
            .map(|metadata| format!("{}", metadata))
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "<!-- kv3 {} -->", metadata)
    }
}
