use super::Literal;

use std::fmt::{Display, Formatter};
use serde_json::{Map, Value};

pub struct VariableLiteral {
    pub name: String,
    pub scope: VariableScope
}

impl Literal for VariableLiteral {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("name".to_string(), Value::String(self.name.clone()));
        data.insert("scope".to_string(), Value::String(self.scope.to_string()));

        map.insert("id".to_string(), Value::String("var".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}

pub enum VariableScope {
    Save,
    Global,
    Local,
    Line
}

impl Display for VariableScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableScope::Save => write!(f, "save"),
            VariableScope::Global => write!(f, "game"),
            VariableScope::Local => write!(f, "local"),
            VariableScope::Line => write!(f, "line"),
        }
    }
}