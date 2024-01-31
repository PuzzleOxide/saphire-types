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
    FilterSelectionbyDistance {
        location_to: Location,
        selection_size: Option<Number>,
        ignore_yaxis_tag: IgnoreYAxisFilterSelectionbyDistance,
        compare_mode_tag: CompareModeFilterSelectionbyDistance,
    },
    FilterSelectionbyRaycast {
        gets_the_end_or: Option<VariableLiteral>,
        ray_origin: Location,
        ray_distance: Number,
        ray_width: Option<Number>,
        selection_size: Option<Number>,
        block_collision_tag: BlockCollisionFilterSelectionbyRaycast,
    },
    ResetSelection {},
    SelectEventTarget { event_target_tag: EventTargetSelectEventTarget },
    Killer {},
    Victim {},
    SelectEntitiesbyCondition { subaction: SelectEntity },
    SelectAllPlayers {},
    InvertSelection {},
    RandomEntity {},
    FilterSelectionbyCondition { subaction: AllSubactions },
    MobsCond { subaction: SelectEntity },
    FilterSelectionbySort {
        value_to: AnyType,
        selection_size: Option<Number>,
        sort_order_tag: SortOrderFilterSelectionbySort,
    },
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![selection_size.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![uuid_to_check_for.json()], vec![]);
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
                let item_args = compile(vec![selection_size.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![name_or_uuid.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Damager".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::FilterSelectionbyDistance {
                location_to,
                selection_size,
                ignore_yaxis_tag,
                compare_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![location_to.json(), selection_size.json()],
                    vec![ignore_yaxis_tag.json(), compare_mode_tag.json()],
                );
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
                block_collision_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        gets_the_end_or.json(), ray_origin.json(), ray_distance.json(),
                        ray_width.json(), selection_size.json()
                    ],
                    vec![block_collision_tag.json()],
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Reset".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SelectObject::SelectEventTarget { event_target_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![event_target_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                subaction
            }
            SelectObject::SelectAllPlayers {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                subaction
            }
            SelectObject::MobsCond { subaction } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
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
                subaction
            }
            SelectObject::FilterSelectionbySort {
                value_to,
                selection_size,
                sort_order_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![value_to.json(), selection_size.json()],
                    vec![sort_order_tag.json()],
                );
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                subaction
            }
            SelectObject::MobName {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
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
#[derive(Debug, Clone)]
pub enum IgnoreYAxisFilterSelectionbyDistance {
    True,
    False,
}
impl IgnoreYAxisFilterSelectionbyDistance {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreYAxisFilterSelectionbyDistance::True => {
                    Value::String("True".to_string())
                }
                IgnoreYAxisFilterSelectionbyDistance::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Y-Axis".to_string()));
        data.insert("action".to_string(), Value::String("FilterDistance".to_string()));
        data.insert("block".to_string(), Value::String("FilterDistance".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreYAxisFilterSelectionbyDistance {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum CompareModeFilterSelectionbyDistance {
    Nearest,
    Farthest,
}
impl CompareModeFilterSelectionbyDistance {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CompareModeFilterSelectionbyDistance::Nearest => {
                    Value::String("Nearest".to_string())
                }
                CompareModeFilterSelectionbyDistance::Farthest => {
                    Value::String("Farthest".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Compare Mode".to_string()));
        data.insert("action".to_string(), Value::String("FilterDistance".to_string()));
        data.insert("block".to_string(), Value::String("FilterDistance".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CompareModeFilterSelectionbyDistance {
    fn default() -> Self {
        Self::Nearest
    }
}
#[derive(Debug, Clone)]
pub enum BlockCollisionFilterSelectionbyRaycast {
    Allblocks,
    Nonfluidblocks,
    Solidblocks,
    None,
}
impl BlockCollisionFilterSelectionbyRaycast {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BlockCollisionFilterSelectionbyRaycast::Allblocks => {
                    Value::String("All blocks".to_string())
                }
                BlockCollisionFilterSelectionbyRaycast::Nonfluidblocks => {
                    Value::String("Non-fluid blocks".to_string())
                }
                BlockCollisionFilterSelectionbyRaycast::Solidblocks => {
                    Value::String("Solid blocks".to_string())
                }
                BlockCollisionFilterSelectionbyRaycast::None => {
                    Value::String("None".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Block Collision".to_string()));
        data.insert("action".to_string(), Value::String("FilterRay".to_string()));
        data.insert("block".to_string(), Value::String("FilterRay".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BlockCollisionFilterSelectionbyRaycast {
    fn default() -> Self {
        Self::Solidblocks
    }
}
#[derive(Debug, Clone)]
pub enum EventTargetSelectEventTarget {
    Default,
    Killer,
    Damager,
    Victim,
    Shooter,
    Projectile,
}
impl EventTargetSelectEventTarget {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                EventTargetSelectEventTarget::Default => {
                    Value::String("Default".to_string())
                }
                EventTargetSelectEventTarget::Killer => {
                    Value::String("Killer".to_string())
                }
                EventTargetSelectEventTarget::Damager => {
                    Value::String("Damager".to_string())
                }
                EventTargetSelectEventTarget::Victim => {
                    Value::String("Victim".to_string())
                }
                EventTargetSelectEventTarget::Shooter => {
                    Value::String("Shooter".to_string())
                }
                EventTargetSelectEventTarget::Projectile => {
                    Value::String("Projectile".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Event Target".to_string()));
        data.insert("action".to_string(), Value::String("EventTarget".to_string()));
        data.insert("block".to_string(), Value::String("EventTarget".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for EventTargetSelectEventTarget {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Debug, Clone)]
pub enum SortOrderFilterSelectionbySort {
    Ascending,
    Descending,
}
impl SortOrderFilterSelectionbySort {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SortOrderFilterSelectionbySort::Ascending => {
                    Value::String("Ascending".to_string())
                }
                SortOrderFilterSelectionbySort::Descending => {
                    Value::String("Descending".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sort Order".to_string()));
        data.insert("action".to_string(), Value::String("FilterSort".to_string()));
        data.insert("block".to_string(), Value::String("FilterSort".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SortOrderFilterSelectionbySort {
    fn default() -> Self {
        Self::Ascending
    }
}
