use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum IfPlayer {
    IsLookingatBlock {
        block_to_check_for: Vec<Either<Block, Location>>,
        maximum_distance_from: Option<Number>,
        fluid_mode_tag: FluidModeIsLookingatBlock,
    },
    IsInWorldBorder { location_to_check: Option<Location> },
    HasRoomForItem {
        items_to_check_with: Option<Item>,
        check_mode_tag: CheckModeHasRoomForItem,
        checked_slots_tag: CheckedSlotsHasRoomForItem,
    },
    IsHoldingOff {},
    IsUsingResourcePack {},
    ItemIsNotOnCooldown { item_type_s_to_check: Vec<Item> },
    IsUsingItem { items_to_check: Vec<Item> },
    HasAllItems {},
    IsSwimming {},
    HasItem { items_to_check_for: Vec<Item>, check_mode_tag: CheckModeHasItem },
    BlockEquals {},
    IsWearingItem {
        items_to_check_for: Vec<Item>,
        check_mode_tag: CheckModeIsWearingItem,
    },
    IsNearLocation {
        center_location: Vec<Location>,
        radius: Option<Number>,
        shape_tag: ShapeIsNearLocation,
    },
    IsRiding { compare_text_to_tag: CompareTextToIsRiding },
    StandingOn {},
    CmdEquals {
        check_mode_tag: CheckModeCmdEquals,
        ignore_case_tag: IgnoreCaseCmdEquals,
    },
    IsStandingonBlock { block_to_check_for: Vec<Either<Block, Location>> },
    IsGrounded {},
    CursorItemEquals { itemss_to_check_for: Vec<Item> },
    HotbarSlotEquals { slot_id_to_check: Number },
    ItemEquals {},
    IsHoldingMain {},
    IsHoldingItem {
        items_to_check_for: Vec<Item>,
        hand_slot_tag: HandSlotIsHoldingItem,
    },
    InventoryMenuSlotEquals {
        slots_to_check: Vec<Number>,
        items_to_check_for: Vec<Item>,
    },
    IsBlocking {},
    HasPlotPermission { permission_tag: PermissionHasPlotPermission },
    IsRidingEntity { spawn_egg: Vec<Either<Either<EntityType, Text>, MiniMessage>> },
    IsSneaking {},
    IsFlying {},
    HasPotionEffect {
        effects: Vec<Potion>,
        check_properties_tag: CheckPropertiesHasPotionEffect,
        check_mode_tag: CheckModeHasPotionEffect,
    },
    NameEquals { names_to_check_for: Vec<Text> },
    InventoryTypeOpen { inventory_type__tag: InventoryTypeInventoryTypeOpen },
    HasIteminSlot { slots_to_check: Vec<Number>, items_to_check_for: Vec<Item> },
    IsSprinting {},
    IsGliding {},
    CmdArgEquals { ignore_case_tag: IgnoreCaseCmdArgEquals },
}
impl IfPlayer {
    pub fn compile(&self) -> Value {
        match self {
            IfPlayer::IsLookingatBlock {
                block_to_check_for,
                maximum_distance_from,
                fluid_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![block_to_check_for.json(), maximum_distance_from.json()],
                    vec![fluid_mode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsLookingAt".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsInWorldBorder { location_to_check } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![location_to_check.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InWorldBorder".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::HasRoomForItem {
                items_to_check_with,
                check_mode_tag,
                checked_slots_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![items_to_check_with.json()],
                    vec![check_mode_tag.json(), checked_slots_tag.json()],
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
            IfPlayer::IsHoldingOff {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsHoldingOff".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsUsingResourcePack {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("UsingPack".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::ItemIsNotOnCooldown { item_type_s_to_check } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![item_type_s_to_check.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoItemCooldown".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsUsingItem { items_to_check } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![items_to_check.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsUsingItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::HasAllItems {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasAllItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsSwimming {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsSwimming".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::HasItem { items_to_check_for, check_mode_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![items_to_check_for.json()],
                    vec![check_mode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::BlockEquals {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BlockEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsWearingItem { items_to_check_for, check_mode_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![items_to_check_for.json()],
                    vec![check_mode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsWearing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsNearLocation { center_location, radius, shape_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![center_location.json(), radius.json()],
                    vec![shape_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsNear".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsRiding { compare_text_to_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![compare_text_to_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsRiding".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::StandingOn {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StandingOn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::CmdEquals { check_mode_tag, ignore_case_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![check_mode_tag.json(), ignore_case_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CmdEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsStandingonBlock { block_to_check_for } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![block_to_check_for.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" StandingOn ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsGrounded {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsGrounded".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::CursorItemEquals { itemss_to_check_for } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![itemss_to_check_for.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CursorItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::HotbarSlotEquals { slot_id_to_check } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![slot_id_to_check.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SlotEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::ItemEquals {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ItemEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsHoldingMain {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsHoldingMain".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsHoldingItem { items_to_check_for, hand_slot_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![items_to_check_for.json()],
                    vec![hand_slot_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsHolding".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::InventoryMenuSlotEquals { slots_to_check, items_to_check_for } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![slots_to_check.json(), items_to_check_for.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MenuSlotEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsBlocking {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsBlocking".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::HasPlotPermission { permission_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![permission_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasPermission".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsRidingEntity { spawn_egg } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![spawn_egg.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" IsRiding ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsSneaking {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsSneaking".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsFlying {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsFlying".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::HasPotionEffect {
                effects,
                check_properties_tag,
                check_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![effects.json()],
                    vec![check_properties_tag.json(), check_mode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasPotion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::NameEquals { names_to_check_for } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![names_to_check_for.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NameEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::InventoryTypeOpen { inventory_type__tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![inventory_type__tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InvOpen".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::HasIteminSlot { slots_to_check, items_to_check_for } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![slots_to_check.json(), items_to_check_for.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasSlotItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsSprinting {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsSprinting".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::IsGliding {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsGliding".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfPlayer::CmdArgEquals { ignore_case_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![ignore_case_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CmdArgEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum FluidModeIsLookingatBlock {
    Ignorefluids,
    Detectfluids,
}
impl FluidModeIsLookingatBlock {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FluidModeIsLookingatBlock::Ignorefluids => {
                    Value::String("Ignore fluids".to_string())
                }
                FluidModeIsLookingatBlock::Detectfluids => {
                    Value::String("Detect fluids".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Fluid Mode".to_string()));
        data.insert("action".to_string(), Value::String("IsLookingAt".to_string()));
        data.insert("block".to_string(), Value::String("IsLookingAt".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FluidModeIsLookingatBlock {
    fn default() -> Self {
        Self::Ignorefluids
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeHasRoomForItem {
    HasRoomforAnyItem,
    HasRoomforAllItems,
}
impl CheckModeHasRoomForItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeHasRoomForItem::HasRoomforAnyItem => {
                    Value::String("Has Room for Any Item".to_string())
                }
                CheckModeHasRoomForItem::HasRoomforAllItems => {
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
impl Default for CheckModeHasRoomForItem {
    fn default() -> Self {
        Self::HasRoomforAnyItem
    }
}
#[derive(Debug, Clone)]
pub enum CheckedSlotsHasRoomForItem {
    Entireinventory,
    Maininventory,
    Upperinventory,
    Hotbar,
    Armor,
}
impl CheckedSlotsHasRoomForItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckedSlotsHasRoomForItem::Entireinventory => {
                    Value::String("Entire inventory".to_string())
                }
                CheckedSlotsHasRoomForItem::Maininventory => {
                    Value::String("Main inventory".to_string())
                }
                CheckedSlotsHasRoomForItem::Upperinventory => {
                    Value::String("Upper inventory".to_string())
                }
                CheckedSlotsHasRoomForItem::Hotbar => Value::String("Hotbar".to_string()),
                CheckedSlotsHasRoomForItem::Armor => Value::String("Armor".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Checked Slots".to_string()));
        data.insert("action".to_string(), Value::String("HasRoomForItem".to_string()));
        data.insert("block".to_string(), Value::String("HasRoomForItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckedSlotsHasRoomForItem {
    fn default() -> Self {
        Self::Maininventory
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeHasItem {
    HasAnyItem,
    HasAllItems,
}
impl CheckModeHasItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeHasItem::HasAnyItem => Value::String("Has Any Item".to_string()),
                CheckModeHasItem::HasAllItems => {
                    Value::String("Has All Items".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("HasItem".to_string()));
        data.insert("block".to_string(), Value::String("HasItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeHasItem {
    fn default() -> Self {
        Self::HasAnyItem
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeIsWearingItem {
    IsWearingSome,
    IsWearingAll,
}
impl CheckModeIsWearingItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeIsWearingItem::IsWearingSome => {
                    Value::String("Is Wearing Some".to_string())
                }
                CheckModeIsWearingItem::IsWearingAll => {
                    Value::String("Is Wearing All".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("IsWearing".to_string()));
        data.insert("block".to_string(), Value::String("IsWearing".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeIsWearingItem {
    fn default() -> Self {
        Self::IsWearingSome
    }
}
#[derive(Debug, Clone)]
pub enum ShapeIsNearLocation {
    Sphere,
    Circle,
    Cube,
    Square,
}
impl ShapeIsNearLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShapeIsNearLocation::Sphere => Value::String("Sphere".to_string()),
                ShapeIsNearLocation::Circle => Value::String("Circle".to_string()),
                ShapeIsNearLocation::Cube => Value::String("Cube".to_string()),
                ShapeIsNearLocation::Square => Value::String("Square".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Shape".to_string()));
        data.insert("action".to_string(), Value::String("IsNear".to_string()));
        data.insert("block".to_string(), Value::String("IsNear".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShapeIsNearLocation {
    fn default() -> Self {
        Self::Sphere
    }
}
#[derive(Debug, Clone)]
pub enum CompareTextToIsRiding {
    Entitytype,
    NameorUUID,
}
impl CompareTextToIsRiding {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CompareTextToIsRiding::Entitytype => {
                    Value::String("Entity type".to_string())
                }
                CompareTextToIsRiding::NameorUUID => {
                    Value::String("Name or UUID".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Compare Text To".to_string()));
        data.insert("action".to_string(), Value::String("IsRiding".to_string()));
        data.insert("block".to_string(), Value::String("IsRiding".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CompareTextToIsRiding {
    fn default() -> Self {
        Self::Entitytype
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeCmdEquals {
    CheckEntireCommand,
    CheckFirstWord,
}
impl CheckModeCmdEquals {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeCmdEquals::CheckEntireCommand => {
                    Value::String("Check Entire Command".to_string())
                }
                CheckModeCmdEquals::CheckFirstWord => {
                    Value::String("Check First Word".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("CmdEquals".to_string()));
        data.insert("block".to_string(), Value::String("CmdEquals".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeCmdEquals {
    fn default() -> Self {
        Self::CheckEntireCommand
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseCmdEquals {
    True,
    False,
}
impl IgnoreCaseCmdEquals {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseCmdEquals::True => Value::String("True".to_string()),
                IgnoreCaseCmdEquals::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("CmdEquals".to_string()));
        data.insert("block".to_string(), Value::String("CmdEquals".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCaseCmdEquals {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum HandSlotIsHoldingItem {
    Eitherhand,
    Mainhand,
    Offhand,
}
impl HandSlotIsHoldingItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HandSlotIsHoldingItem::Eitherhand => {
                    Value::String("Either hand".to_string())
                }
                HandSlotIsHoldingItem::Mainhand => Value::String("Main hand".to_string()),
                HandSlotIsHoldingItem::Offhand => Value::String("Off hand".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Hand Slot".to_string()));
        data.insert("action".to_string(), Value::String("IsHolding".to_string()));
        data.insert("block".to_string(), Value::String("IsHolding".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HandSlotIsHoldingItem {
    fn default() -> Self {
        Self::Eitherhand
    }
}
#[derive(Debug, Clone)]
pub enum PermissionHasPlotPermission {
    Owner,
    Developer,
    Builder,
    Developerorbuilder,
    Whitelisted,
}
impl PermissionHasPlotPermission {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PermissionHasPlotPermission::Owner => Value::String("Owner".to_string()),
                PermissionHasPlotPermission::Developer => {
                    Value::String("Developer".to_string())
                }
                PermissionHasPlotPermission::Builder => {
                    Value::String("Builder".to_string())
                }
                PermissionHasPlotPermission::Developerorbuilder => {
                    Value::String("Developer or builder".to_string())
                }
                PermissionHasPlotPermission::Whitelisted => {
                    Value::String("Whitelisted".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Permission".to_string()));
        data.insert("action".to_string(), Value::String("HasPermission".to_string()));
        data.insert("block".to_string(), Value::String("HasPermission".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PermissionHasPlotPermission {
    fn default() -> Self {
        Self::Developerorbuilder
    }
}
#[derive(Debug, Clone)]
pub enum CheckPropertiesHasPotionEffect {
    None,
    Amplifier,
    Duration,
    Amplifierandduration,
}
impl CheckPropertiesHasPotionEffect {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckPropertiesHasPotionEffect::None => Value::String("None".to_string()),
                CheckPropertiesHasPotionEffect::Amplifier => {
                    Value::String("Amplifier".to_string())
                }
                CheckPropertiesHasPotionEffect::Duration => {
                    Value::String("Duration".to_string())
                }
                CheckPropertiesHasPotionEffect::Amplifierandduration => {
                    Value::String("Amplifier and duration".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Properties".to_string()));
        data.insert("action".to_string(), Value::String("HasPotion".to_string()));
        data.insert("block".to_string(), Value::String("HasPotion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckPropertiesHasPotionEffect {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeHasPotionEffect {
    Hasanyeffect,
    Hasalleffects,
}
impl CheckModeHasPotionEffect {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeHasPotionEffect::Hasanyeffect => {
                    Value::String("Has any effect".to_string())
                }
                CheckModeHasPotionEffect::Hasalleffects => {
                    Value::String("Has all effects".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("HasPotion".to_string()));
        data.insert("block".to_string(), Value::String("HasPotion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeHasPotionEffect {
    fn default() -> Self {
        Self::Hasanyeffect
    }
}
#[derive(Debug, Clone)]
pub enum InventoryTypeInventoryTypeOpen {
    AnyInventory,
    PlotMenu,
    CraftingTable,
    Chest,
    DoubleChest,
    EnderChest,
    ShulkerBox,
    Barrel,
    Furnaceany,
    Furnace,
    BlastFurnace,
    Smoker,
    Dropper,
    Dispenser,
    Beacon,
    Hopper,
    Anvil,
    BrewingStand,
    CartographyTable,
    Loom,
    Grindstone,
    Stonecutter,
    EnchantingTable,
    TraderMenuany,
    VillagerMenu,
    WanderingTraderMenu,
    HorseInventory,
    LlamaInventory,
}
impl InventoryTypeInventoryTypeOpen {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InventoryTypeInventoryTypeOpen::AnyInventory => {
                    Value::String("Any Inventory".to_string())
                }
                InventoryTypeInventoryTypeOpen::PlotMenu => {
                    Value::String("Plot Menu".to_string())
                }
                InventoryTypeInventoryTypeOpen::CraftingTable => {
                    Value::String("Crafting Table".to_string())
                }
                InventoryTypeInventoryTypeOpen::Chest => {
                    Value::String("Chest".to_string())
                }
                InventoryTypeInventoryTypeOpen::DoubleChest => {
                    Value::String("Double Chest".to_string())
                }
                InventoryTypeInventoryTypeOpen::EnderChest => {
                    Value::String("Ender Chest".to_string())
                }
                InventoryTypeInventoryTypeOpen::ShulkerBox => {
                    Value::String("Shulker Box".to_string())
                }
                InventoryTypeInventoryTypeOpen::Barrel => {
                    Value::String("Barrel".to_string())
                }
                InventoryTypeInventoryTypeOpen::Furnaceany => {
                    Value::String("Furnace (any)".to_string())
                }
                InventoryTypeInventoryTypeOpen::Furnace => {
                    Value::String("Furnace".to_string())
                }
                InventoryTypeInventoryTypeOpen::BlastFurnace => {
                    Value::String("Blast Furnace".to_string())
                }
                InventoryTypeInventoryTypeOpen::Smoker => {
                    Value::String("Smoker".to_string())
                }
                InventoryTypeInventoryTypeOpen::Dropper => {
                    Value::String("Dropper".to_string())
                }
                InventoryTypeInventoryTypeOpen::Dispenser => {
                    Value::String("Dispenser".to_string())
                }
                InventoryTypeInventoryTypeOpen::Beacon => {
                    Value::String("Beacon".to_string())
                }
                InventoryTypeInventoryTypeOpen::Hopper => {
                    Value::String("Hopper".to_string())
                }
                InventoryTypeInventoryTypeOpen::Anvil => {
                    Value::String("Anvil".to_string())
                }
                InventoryTypeInventoryTypeOpen::BrewingStand => {
                    Value::String("Brewing Stand".to_string())
                }
                InventoryTypeInventoryTypeOpen::CartographyTable => {
                    Value::String("Cartography Table".to_string())
                }
                InventoryTypeInventoryTypeOpen::Loom => Value::String("Loom".to_string()),
                InventoryTypeInventoryTypeOpen::Grindstone => {
                    Value::String("Grindstone".to_string())
                }
                InventoryTypeInventoryTypeOpen::Stonecutter => {
                    Value::String("Stonecutter".to_string())
                }
                InventoryTypeInventoryTypeOpen::EnchantingTable => {
                    Value::String("Enchanting Table".to_string())
                }
                InventoryTypeInventoryTypeOpen::TraderMenuany => {
                    Value::String("Trader Menu (any)".to_string())
                }
                InventoryTypeInventoryTypeOpen::VillagerMenu => {
                    Value::String("Villager Menu".to_string())
                }
                InventoryTypeInventoryTypeOpen::WanderingTraderMenu => {
                    Value::String("Wandering Trader Menu".to_string())
                }
                InventoryTypeInventoryTypeOpen::HorseInventory => {
                    Value::String("Horse Inventory".to_string())
                }
                InventoryTypeInventoryTypeOpen::LlamaInventory => {
                    Value::String("Llama Inventory".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Inventory Type".to_string()));
        data.insert("action".to_string(), Value::String("InvOpen".to_string()));
        data.insert("block".to_string(), Value::String("InvOpen".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InventoryTypeInventoryTypeOpen {
    fn default() -> Self {
        Self::AnyInventory
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseCmdArgEquals {
    True,
    False,
}
impl IgnoreCaseCmdArgEquals {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseCmdArgEquals::True => Value::String("True".to_string()),
                IgnoreCaseCmdArgEquals::False => Value::String("False".to_string()),
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
impl Default for IgnoreCaseCmdArgEquals {
    fn default() -> Self {
        Self::True
    }
}
