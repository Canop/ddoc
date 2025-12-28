use {
    crate::*,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Element {
    Composite(CompositeElement),
    Attributes(Attributes),
}

impl Element {
    pub fn is_composite(&self) -> bool {
        matches!(self, Element::Composite(_))
    }
    pub fn is_attributes(&self) -> bool {
        matches!(self, Element::Attributes(_))
    }
    pub fn as_composite(&self) -> Option<&CompositeElement> {
        match self {
            Element::Composite(comp) => Some(comp),
            _ => None,
        }
    }
    pub fn as_attributes(&self) -> Option<&IndexMap<AttributeKey, AttributeValue>> {
        match self {
            Element::Attributes(map) => Some(map),
            _ => None,
        }
    }
}
