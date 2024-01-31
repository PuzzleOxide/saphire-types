use super::*;

pub enum Block {
    Text(Text),
    Item(Item),
    Variable(VariableLiteral),
}

impl Literal for Block {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Block::Text(text) => text.json(),
            Block::Item(item) => item.json(),
            Block::Variable(variable) => variable.json(),
        }
    }
}