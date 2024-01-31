use super::*;

pub enum Projectile {
    Item(Item),
    Variable(VariableLiteral),
}

impl Literal for Projectile {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Projectile::Item(i) => i.json(),
            Projectile::Variable(v) => v.json(),
        }
    }
}