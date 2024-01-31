use super::*;

pub enum Item {
    Item(ItemLiteral),
    Variable(VariableLiteral),
}

impl Literal for Item {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Item::Item(item) => item.json(),
            Item::Variable(variable) => variable.json(),
        }
    }
}

// TODO: make sure this is a valid item
// TODO: provide item builder
pub struct ItemLiteral {
    item: String,
}

impl ItemLiteral {
    pub fn new(item: String) -> Self {
        Self {
            item
        }
    }
}

impl Literal for ItemLiteral {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("item".to_string(), Value::String(self.item.clone()));

        map.insert("id".to_string(), Value::String("item".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}