#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdsProtoType {
    StreamableStateless,
    StreamableStateful,
    Other(String),
}

impl From<&str> for IdsProtoType {
    fn from(s: &str) -> Self {
        match s {
            "streamable-stateless" => IdsProtoType::StreamableStateless,
            "streamable-statefule" => IdsProtoType::StreamableStateful,
            other => IdsProtoType::Other(other.to_string()),
        }
    }
}
