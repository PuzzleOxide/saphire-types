use super::*;

pub enum Number {
    Literal(f64),
    Expression(String),
    Variable(VariableLiteral),
}

impl Literal for Number {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Number::Literal(literal) => {
                let mut map = Map::new();
                let mut data = Map::new();
                data.insert("name".to_string(), Value::String(literal.to_string()));

                map.insert("id".to_string(), Value::String("num".to_string()));
                map.insert("data".to_string(), Value::Object(data));
                vec![map]
            },
            Number::Expression(expression) => {
                let mut map = Map::new();
                let mut data = Map::new();
                data.insert("name".to_string(), Value::String(expression.clone()));

                map.insert("id".to_string(), Value::String("num".to_string()));
                map.insert("data".to_string(), Value::Object(data));
                vec![map]
            },
            Number::Variable(variable) => {
                variable.json()
            }
        }
    }
}