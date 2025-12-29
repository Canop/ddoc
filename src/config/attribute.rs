use {
    indexmap::IndexMap,
    serde::Deserialize,
};

pub type AttributeKey = String;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AttributeValue {
    String(String),
    Bool(bool),
}
impl AttributeValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttributeValue::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AttributeValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

pub type Attributes = IndexMap<AttributeKey, AttributeValue>;
