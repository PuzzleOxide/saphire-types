use either::Either;
use serde_json::{Value, Map};

/// turns a vector of arguments into a vector of json objects
pub fn compile(args: Vec<Vec<Map<String, Value>>>, tags: Vec<Map<String, Value>>) -> Vec<Value> {
    let mut vec = Vec::new();
    for (i, arg)  in args.into_iter().flatten().enumerate() {
        let mut map = Map::new();
        map.insert("slot".to_string(), Value::Number(serde_json::Number::from(i)));
        map.insert("item".to_string(), Value::Object(arg));
        vec.push(Value::Object(map));
    };

    for (tag, i) in tags.iter().zip(26..0) {
        let mut map = Map::new();
        map.insert("slot".to_string(), Value::Number(serde_json::Number::from(i)));
        map.insert("item".to_string(), Value::Object(tag.clone()));
        vec.push(Value::Object(map));
    } 
    vec
}

impl<L, R> Literal for Either<L, R> where L: Literal, R: Literal {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Either::Left(l) => l.json(),
            Either::Right(r) => r.json(),
        }
    }
}

/// A trait for any literal value that can be used as an argument item.
pub(crate) trait Literal {
    /// Must return a json string representing the argument.
    fn json(&self) -> Vec<Map<String, Value>>;
}

impl<T: Literal> Literal for Vec<T> {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut vec = Vec::new();
        for item in self {
            vec.extend(item.json());
        }
        vec
    }
}

impl<T: Literal> Literal for Option<T> {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Some(item) => item.json(),
            None => Vec::new(),
        }
    }
}

pub(crate) struct Tag {
    pub(crate) option: String,
    pub(crate) tag: String,
    pub(crate) action: String,
    pub(crate) block: String,
}

pub struct Number {
    pub value: String,
}

impl Literal for Number {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("name".to_string(), Value::String(self.value.clone()));

        map.insert("type".to_string(), Value::String("number".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}

pub struct Text {
    pub value: String,
}

impl Literal for Text {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("name".to_string(), Value::String(self.value.clone()));

        map.insert("type".to_string(), Value::String("txt".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}

pub struct MiniMessage {
    pub value: String,
}

impl Literal for MiniMessage {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("name".to_string(), Value::String(self.value.clone()));

        map.insert("type".to_string(), Value::String("minimsg".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}

pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Literal for Location {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("x".to_string(), Value::Number(serde_json::Number::from_f64(self.x).unwrap()));
        data.insert("y".to_string(), Value::Number(serde_json::Number::from_f64(self.y).unwrap()));
        data.insert("z".to_string(), Value::Number(serde_json::Number::from_f64(self.z).unwrap()));
        data.insert("pitch".to_string(), Value::Number(serde_json::Number::from_f64(self.pitch).unwrap()));
        data.insert("yaw".to_string(), Value::Number(serde_json::Number::from_f64(self.yaw).unwrap()));

        map.insert("type".to_string(), Value::String("location".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}

pub struct Item {
    pub item: String,
}

impl Literal for Item {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile items");
    }
}

pub struct Particle {
    pub particle: String,
}

impl Literal for Particle {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile particles");
    }
}

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Literal for Vector {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut map = Map::new();
        let mut data = Map::new();
        data.insert("x".to_string(), Value::Number(serde_json::Number::from_f64(self.x).unwrap()));
        data.insert("y".to_string(), Value::Number(serde_json::Number::from_f64(self.y).unwrap()));
        data.insert("z".to_string(), Value::Number(serde_json::Number::from_f64(self.z).unwrap()));

        map.insert("type".to_string(), Value::String("vector".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        vec![map]
    }
}

pub struct Sound {
    pub sound: String,
}

impl Literal for Sound {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile sounds");
    }
}

pub enum Block {
    Text(Text),
    Item(Item),
}

impl Literal for Block {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Block::Text(text) => text.json(),
            Block::Item(item) => item.json(),
        }
    }
}

pub struct BlockTag {
    pub block_tag: String,
}

impl Literal for BlockTag {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile block tags");
    }
}

pub struct Projectile {
    pub projectile: Item,
}

impl Literal for Projectile {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile projectiles");
    }
}

pub struct Potion {
    pub potion: String,
}

impl Literal for Potion {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile potions");
    }
}

pub struct SpawnEgg {
    pub spawn_egg: Item,
}

impl Literal for SpawnEgg {
    fn json(&self) -> Vec<Map<String, Value>> {
        self.spawn_egg.json()
    }
}

pub struct EntityType {
    pub entity_type: String,
}

impl Literal for EntityType {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile entity types");
    }
}

pub struct Variable {
    pub variable: String,
}

impl Literal for Variable {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile variables");
    }
}

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
    Variable(Variable),
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

pub struct Dict {
    pub dict: String,
}

impl Literal for Dict {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile dicts");
    }
}

pub struct List {
    pub list: String,
}

impl Literal for List {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile lists");
    }
}

pub struct Vehicle {
    pub vehicle: String,
}

impl Literal for Vehicle {
    fn json(&self) -> Vec<Map<String, Value>> {
        todo!("compile vehicles");
    }    
}