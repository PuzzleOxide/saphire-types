use super::*;

pub enum MiniMessage {
    Literal(MiniMessageLiteral),
    Variable(VariableLiteral),
}

impl Literal for MiniMessage {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            MiniMessage::Literal(literal) => literal.json(),
            MiniMessage::Variable(variable) => variable.json(),
        }
    }
}

pub struct MiniMessageLiteral {
    minimessage: String,
}

impl MiniMessageLiteral {
    pub fn new(minimessage: String) -> Self {
        // TODO: validate minimessage
        Self {
            minimessage,
        }
    }
}

impl Literal for MiniMessageLiteral {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("message".to_string(), Value::String(self.minimessage.clone()));

        map.insert("id".to_string(), Value::String("minimsg".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}