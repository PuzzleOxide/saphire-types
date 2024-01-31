use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum IfGame {
    SignHasTxt {
        sign_line_tag: SignLineSignHasTxt,
        check_mode_tag: CheckModeSignHasTxt,
    },
    ContainerHasRoomForItem {
        container_location: Location,
        items_to_check_with: Option<Item>,
        check_mode_tag: CheckModeContainerHasRoomForItem,
    },
    EventBlockEquals { blocks_to_check_for: Vec<Block> },
    CommandEquals {
        strings_to_check_for: Vec<Text>,
        check_mode_tag: CheckModeCommandEquals,
        ignore_case_tag: IgnoreCaseCommandEquals,
    },
    EventItemEquals {
        items_to_check_for: Vec<Item>,
        comparison_mode_tag: ComparisonModeEventItemEquals,
    },
    SignContainsText {
        sign_location: Location,
        text_to_check_for: Vec<MiniMessage>,
        sign_line_tag: SignLineSignContainsText,
        sign_side_tag: SignSideSignContainsText,
        check_mode_tag: CheckModeSignContainsText,
    },
    AttackIsCritical {},
    ContainerHasItem { container_location: Location, items_to_check_for: Vec<Item> },
    BlockEquals {
        check_location: Location,
        blocks_to_check_for: Vec<Block>,
        block_data: Vec<BlockTag>,
    },
    BlockIsPowered {
        check_locations: Vec<Location>,
        redstone_power_mode_tag: RedstonePowerModeBlockIsPowered,
    },
    GameHasPlayer { name_or_uuid: Vec<Text> },
    ContainerHasAllItems { container_location: Location, items_to_check_for: Vec<Item> },
    CommandArgumentEquals {
        strings_to_check_for: Vec<Text>,
        argument_number: Number,
        ignore_case_tag: IgnoreCaseCommandArgumentEquals,
    },
    EventIsCancelled {},
}
impl IfGame {
    pub fn compile(&self) -> Value {
        match self {
            IfGame::SignHasTxt { sign_line_tag, check_mode_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![sign_line_tag.json(), check_mode_tag.json()],
                );
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
                check_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), items_to_check_with.json()],
                    vec![check_mode_tag.json()],
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
                let item_args = compile(vec![blocks_to_check_for.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EventBlockEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::CommandEquals {
                strings_to_check_for,
                check_mode_tag,
                ignore_case_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![strings_to_check_for.json()],
                    vec![check_mode_tag.json(), ignore_case_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CommandEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::EventItemEquals { items_to_check_for, comparison_mode_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![items_to_check_for.json()],
                    vec![comparison_mode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EventItemEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfGame::SignContainsText {
                sign_location,
                text_to_check_for,
                sign_line_tag,
                sign_side_tag,
                check_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![sign_location.json(), text_to_check_for.json()],
                    vec![
                        sign_line_tag.json(), sign_side_tag.json(), check_mode_tag.json()
                    ],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                    vec![],
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
            IfGame::BlockIsPowered { check_locations, redstone_power_mode_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![check_locations.json()],
                    vec![redstone_power_mode_tag.json()],
                );
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
                let item_args = compile(vec![name_or_uuid.json()], vec![]);
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
                    vec![],
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
            IfGame::CommandArgumentEquals {
                strings_to_check_for,
                argument_number,
                ignore_case_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![strings_to_check_for.json(), argument_number.json()],
                    vec![ignore_case_tag.json()],
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
                let item_args = compile(vec![], vec![]);
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
#[derive(Debug, Clone)]
pub enum SignLineSignHasTxt {
    One,
    Two,
    Three,
    Four,
    Alllines,
}
impl SignLineSignHasTxt {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignLineSignHasTxt::One => Value::String("1".to_string()),
                SignLineSignHasTxt::Two => Value::String("2".to_string()),
                SignLineSignHasTxt::Three => Value::String("3".to_string()),
                SignLineSignHasTxt::Four => Value::String("4".to_string()),
                SignLineSignHasTxt::Alllines => Value::String("All lines".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Line".to_string()));
        data.insert("action".to_string(), Value::String("SignHasTxt".to_string()));
        data.insert("block".to_string(), Value::String("SignHasTxt".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignLineSignHasTxt {
    fn default() -> Self {
        Self::Alllines
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeSignHasTxt {
    Contains,
    Equals,
}
impl CheckModeSignHasTxt {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeSignHasTxt::Contains => Value::String("Contains".to_string()),
                CheckModeSignHasTxt::Equals => Value::String("Equals".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("SignHasTxt".to_string()));
        data.insert("block".to_string(), Value::String("SignHasTxt".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeSignHasTxt {
    fn default() -> Self {
        Self::Contains
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeContainerHasRoomForItem {
    HasRoomforAnyItem,
    HasRoomforAllItems,
}
impl CheckModeContainerHasRoomForItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeContainerHasRoomForItem::HasRoomforAnyItem => {
                    Value::String("Has Room for Any Item".to_string())
                }
                CheckModeContainerHasRoomForItem::HasRoomforAllItems => {
                    Value::String("Has Room for All Items".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("HasRoomForItem".to_string()));
        data.insert("block".to_string(), Value::String("HasRoomForItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeContainerHasRoomForItem {
    fn default() -> Self {
        Self::HasRoomforAnyItem
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeCommandEquals {
    Checkentirecommand,
    Checkbeginning,
}
impl CheckModeCommandEquals {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeCommandEquals::Checkentirecommand => {
                    Value::String("Check entire command".to_string())
                }
                CheckModeCommandEquals::Checkbeginning => {
                    Value::String("Check beginning".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("CommandEquals".to_string()));
        data.insert("block".to_string(), Value::String("CommandEquals".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeCommandEquals {
    fn default() -> Self {
        Self::Checkentirecommand
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseCommandEquals {
    True,
    False,
}
impl IgnoreCaseCommandEquals {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseCommandEquals::True => Value::String("True".to_string()),
                IgnoreCaseCommandEquals::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("CommandEquals".to_string()));
        data.insert("block".to_string(), Value::String("CommandEquals".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCaseCommandEquals {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum ComparisonModeEventItemEquals {
    Exactlyequals,
    Ignorestacksizedurability,
    Materialonly,
}
impl ComparisonModeEventItemEquals {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ComparisonModeEventItemEquals::Exactlyequals => {
                    Value::String("Exactly equals".to_string())
                }
                ComparisonModeEventItemEquals::Ignorestacksizedurability => {
                    Value::String("Ignore stack size/durability".to_string())
                }
                ComparisonModeEventItemEquals::Materialonly => {
                    Value::String("Material only".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Comparison Mode".to_string()));
        data.insert("action".to_string(), Value::String("EventItemEquals".to_string()));
        data.insert("block".to_string(), Value::String("EventItemEquals".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ComparisonModeEventItemEquals {
    fn default() -> Self {
        Self::Ignorestacksizedurability
    }
}
#[derive(Debug, Clone)]
pub enum SignLineSignContainsText {
    One,
    Two,
    Three,
    Four,
    Alllines,
}
impl SignLineSignContainsText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignLineSignContainsText::One => Value::String("1".to_string()),
                SignLineSignContainsText::Two => Value::String("2".to_string()),
                SignLineSignContainsText::Three => Value::String("3".to_string()),
                SignLineSignContainsText::Four => Value::String("4".to_string()),
                SignLineSignContainsText::Alllines => {
                    Value::String("All lines".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Line".to_string()));
        data.insert("action".to_string(), Value::String(" SignHasTxt ".to_string()));
        data.insert("block".to_string(), Value::String(" SignHasTxt ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignLineSignContainsText {
    fn default() -> Self {
        Self::Alllines
    }
}
#[derive(Debug, Clone)]
pub enum SignSideSignContainsText {
    Front,
    Back,
}
impl SignSideSignContainsText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignSideSignContainsText::Front => Value::String("Front".to_string()),
                SignSideSignContainsText::Back => Value::String("Back".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Side".to_string()));
        data.insert("action".to_string(), Value::String(" SignHasTxt ".to_string()));
        data.insert("block".to_string(), Value::String(" SignHasTxt ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignSideSignContainsText {
    fn default() -> Self {
        Self::Front
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeSignContainsText {
    Contains,
    Equals,
}
impl CheckModeSignContainsText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeSignContainsText::Contains => {
                    Value::String("Contains".to_string())
                }
                CheckModeSignContainsText::Equals => Value::String("Equals".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String(" SignHasTxt ".to_string()));
        data.insert("block".to_string(), Value::String(" SignHasTxt ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeSignContainsText {
    fn default() -> Self {
        Self::Contains
    }
}
#[derive(Debug, Clone)]
pub enum RedstonePowerModeBlockIsPowered {
    Directpower,
    Indirectpower,
}
impl RedstonePowerModeBlockIsPowered {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RedstonePowerModeBlockIsPowered::Directpower => {
                    Value::String("Direct power".to_string())
                }
                RedstonePowerModeBlockIsPowered::Indirectpower => {
                    Value::String("Indirect power".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Redstone Power Mode".to_string()));
        data.insert("action".to_string(), Value::String("BlockPowered".to_string()));
        data.insert("block".to_string(), Value::String("BlockPowered".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RedstonePowerModeBlockIsPowered {
    fn default() -> Self {
        Self::Directpower
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseCommandArgumentEquals {
    True,
    False,
}
impl IgnoreCaseCommandArgumentEquals {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseCommandArgumentEquals::True => {
                    Value::String("True".to_string())
                }
                IgnoreCaseCommandArgumentEquals::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("CmdArgEquals".to_string()));
        data.insert("block".to_string(), Value::String("CmdArgEquals".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCaseCommandArgumentEquals {
    fn default() -> Self {
        Self::True
    }
}
