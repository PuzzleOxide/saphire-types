use super::*;

pub enum Text {
    Literal(String),
    Variable(VariableLiteral),
}

impl Literal for Text {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Text::Literal(literal) => {
                let mut map = Map::new();
                let mut data = Map::new();
                data.insert("name".to_string(), Value::String(literal.clone()));

                map.insert("id".to_string(), Value::String("text".to_string()));
                map.insert("data".to_string(), Value::Object(data));
                vec![map]
            },
            Text::Variable(variable) => {
                variable.json()
            }
        }
    }
}