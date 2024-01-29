use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum IfGame {
    SignHasTxt {},
    ContainerHasRoomForItem {
        container_location: Location,
        items_to_check_with: Option<Item>,
    },
    EventBlockEquals { blocks_to_check_for: Vec<Block> },
    CommandEquals { strings_to_check_for: Vec<Text> },
    EventItemEquals { items_to_check_for: Vec<Item> },
    SignContainsText { sign_location: Location, text_to_check_for: Vec<MiniMessage> },
    AttackIsCritical {},
    ContainerHasItem { container_location: Location, items_to_check_for: Vec<Item> },
    BlockEquals {
        check_location: Location,
        blocks_to_check_for: Vec<Block>,
        block_data: Vec<BlockTag>,
    },
    BlockIsPowered { check_locations: Vec<Location> },
    GameHasPlayer { name_or_uuid: Vec<Text> },
    ContainerHasAllItems { container_location: Location, items_to_check_for: Vec<Item> },
    CommandArgumentEquals { strings_to_check_for: Vec<Text>, argument_number: Number },
    EventIsCancelled {},
}
impl IfGame {
    pub fn compile(&self) -> Value {
        match self {
            IfGame::SignHasTxt {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SignHasTxt".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::ContainerHasRoomForItem {
                container_location,
                items_to_check_with,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), items_to_check_with.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasRoomForItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::EventBlockEquals { blocks_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![blocks_to_check_for.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EventBlockEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::CommandEquals { strings_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![strings_to_check_for.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CommandEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::EventItemEquals { items_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![items_to_check_for.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EventItemEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::SignContainsText { sign_location, text_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![sign_location.json(), text_to_check_for.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" SignHasTxt ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::AttackIsCritical {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AttackIsCrit".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::ContainerHasItem { container_location, items_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), items_to_check_for.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ContainerHas".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::BlockEquals { check_location, blocks_to_check_for, block_data } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        check_location.json(), blocks_to_check_for.json(), block_data
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BlockEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::BlockIsPowered { check_locations } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![check_locations.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BlockPowered".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::GameHasPlayer { name_or_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![name_or_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::ContainerHasAllItems { container_location, items_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), items_to_check_for.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ContainerHasAll".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::CommandArgumentEquals { strings_to_check_for, argument_number } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![strings_to_check_for.json(), argument_number.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CmdArgEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::EventIsCancelled {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EventCancelled".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
