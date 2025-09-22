#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdsProtoType {
    StreamableStateless,
    Other(String),
}

impl From<&str> for IdsProtoType {
    fn from(s: &str) -> Self {
        match s {
            "streamable-stateless" => IdsProtoType::StreamableStateless,
            other => IdsProtoType::Other(other.to_string()),
        }
    }
}
