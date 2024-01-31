use super::*;

pub enum AnyType {
    Number(Number),
    Text(Text),
    MiniMessage(MiniMessage),
    Location(Location),
    Item(Item),
    Particle(Particle),
    Vector(Vector),
    Sound(Sound),
    Block(Block),
    BlockTag(BlockTag),
    Projectile(Projectile),
    Potion(Potion),
    SpawnEgg(SpawnEgg),
    EntityType(EntityType),
    Variable(VariableLiteral),
}

impl Literal for AnyType {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            AnyType::Number(arg) => arg.json(),
            AnyType::Text(arg) => arg.json(),
            AnyType::MiniMessage(arg) => arg.json(),
            AnyType::Location(arg) => arg.json(),
            AnyType::Item(arg) => arg.json(),
            AnyType::Particle(arg) => arg.json(),
            AnyType::Vector(arg) => arg.json(),
            AnyType::Sound(arg) => arg.json(),
            AnyType::Block(arg) => arg.json(),
            AnyType::BlockTag(arg) => arg.json(),
            AnyType::Projectile(arg) => arg.json(),
            AnyType::Potion(arg) => arg.json(),
            AnyType::SpawnEgg(arg) => arg.json(),
            AnyType::EntityType(arg) => arg.json(),
            AnyType::Variable(arg) => arg.json(),
        }
    }
}