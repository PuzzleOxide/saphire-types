use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum SelectObject {
    LastMob {},
    SelectRandomPlayer { selection_size: Option<Number> },
    SelectLastSpawnedEntity {},
    Shooter {},
    AllMobs {},
    SelectEntitiesbyName { uuid_to_check_for: Vec<Either<Text, MiniMessage>> },
    FilterSelectionRandomly { selection_size: Option<Number> },
    DefaultEntity {},
    SelectPlayersbyName { name_or_uuid: Vec<Text> },
    SelectAllEntities {},
    Damager {},
    FilterSelectionbyDistance { location_to: Location, selection_size: Option<Number> },
    FilterSelectionbyRaycast {
        gets_the_end_or: Option<Variable>,
        ray_origin: Location,
        ray_distance: Number,
        ray_width: Option<Number>,
        selection_size: Option<Number>,
    },
    ResetSelection {},
    SelectEventTarget {},
    Killer {},
    Victim {},
    SelectEntitiesbyCondition { subaction: SelectEntity },
    SelectAllPlayers {},
    InvertSelection {},
    RandomEntity {},
    FilterSelectionbyCondition { subaction: AllSubactions },
    MobsCond { subaction: SelectEntity },
    FilterSelectionbySort { value_to: AnyType, selection_size: Option<Number> },
    Projectile {},
    DefaultPlayer {},
    SelectPlayersbyCondition { subaction: SelectPlayer },
    MobName {},
}
impl SelectObject {
    pub fn compile(&self) -> Value {
        match self {
            SelectObject::LastMob {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LastMob".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectRandomPlayer { selection_size } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![selection_size.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RandomPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectLastSpawnedEntity {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LastEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::Shooter {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Shooter".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::AllMobs {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AllMobs".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectEntitiesbyName { uuid_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![uuid_to_check_for.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EntityName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::FilterSelectionRandomly { selection_size } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![selection_size.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FilterRandom".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::DefaultEntity {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DefaultEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectPlayersbyName { name_or_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![name_or_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlayerName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectAllEntities {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AllEntities".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::Damager {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Damager".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::FilterSelectionbyDistance { location_to, selection_size } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![location_to.json(), selection_size.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FilterDistance".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::FilterSelectionbyRaycast {
                gets_the_end_or,
                ray_origin,
                ray_distance,
                ray_width,
                selection_size,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        gets_the_end_or.json(), ray_origin.json(), ray_distance.json(),
                        ray_width.json(), selection_size.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FilterRay".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::ResetSelection {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Reset".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectEventTarget {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EventTarget".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::Killer {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Killer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::Victim {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Victim".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectEntitiesbyCondition { subaction } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EntitiesCond".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                let mut subaction = subaction.compile();
                let value = subaction.as_object_mut().unwrap();
                value.insert("subaction".to_string(), value["action"].clone());
                value
                    .insert(
                        "action".to_string(),
                        serde_json::Value::String("EntitiesCond".to_string()),
                    );
                drop(value);
                subaction
            }
            SelectObject::SelectAllPlayers {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AllPlayers".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::InvertSelection {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Invert".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::RandomEntity {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RandomEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::FilterSelectionbyCondition { subaction } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FilterCondition".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                let mut subaction = subaction.compile();
                let value = subaction.as_object_mut().unwrap();
                value.insert("subaction".to_string(), value["action"].clone());
                value
                    .insert(
                        "action".to_string(),
                        serde_json::Value::String("FilterCondition".to_string()),
                    );
                drop(value);
                subaction
            }
            SelectObject::MobsCond { subaction } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MobsCond".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                let mut subaction = subaction.compile();
                let value = subaction.as_object_mut().unwrap();
                value.insert("subaction".to_string(), value["action"].clone());
                value
                    .insert(
                        "action".to_string(),
                        serde_json::Value::String("MobsCond".to_string()),
                    );
                drop(value);
                subaction
            }
            SelectObject::FilterSelectionbySort { value_to, selection_size } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![value_to.json(), selection_size.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FilterSort".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::Projectile {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Projectile".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::DefaultPlayer {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DefaultPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectPlayersbyCondition { subaction } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlayersCond".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                let mut subaction = subaction.compile();
                let value = subaction.as_object_mut().unwrap();
                value.insert("subaction".to_string(), value["action"].clone());
                value
                    .insert(
                        "action".to_string(),
                        serde_json::Value::String("PlayersCond".to_string()),
                    );
                drop(value);
                subaction
            }
            SelectObject::MobName {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MobName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
