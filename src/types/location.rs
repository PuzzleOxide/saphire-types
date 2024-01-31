use super::*;

pub enum Location {
    Literal(LocationLiteral),
    Variable(VariableLiteral),
}

impl Literal for Location {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Location::Literal(literal) => literal.json(),
            Location::Variable(variable) => variable.json(),
        }
    }
}

pub struct LocationLiteral {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Literal for LocationLiteral {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("x".to_string(), Value::Number(serde_json::Number::from_f64(self.x).unwrap()));
        data.insert("y".to_string(), Value::Number(serde_json::Number::from_f64(self.y).unwrap()));
        data.insert("z".to_string(), Value::Number(serde_json::Number::from_f64(self.z).unwrap()));
        data.insert("pitch".to_string(), Value::Number(serde_json::Number::from_f64(self.pitch).unwrap()));
        data.insert("yaw".to_string(), Value::Number(serde_json::Number::from_f64(self.yaw).unwrap()));

        map.insert("id".to_string(), Value::String("location".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}