use super::*;

pub enum Vector {
    Vector(VectorLiteral),
    Variable(VariableLiteral),
}

impl Literal for Vector {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Vector::Vector(v) => v.json(),
            Vector::Variable(v) => v.json(),
        }
    }
}

pub struct VectorLiteral {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Literal for VectorLiteral {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("x".to_string(), Value::Number(serde_json::Number::from_f64(self.x).unwrap()));
        data.insert("y".to_string(), Value::Number(serde_json::Number::from_f64(self.y).unwrap()));
        data.insert("z".to_string(), Value::Number(serde_json::Number::from_f64(self.z).unwrap()));

        map.insert("id".to_string(), Value::String("vec".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}