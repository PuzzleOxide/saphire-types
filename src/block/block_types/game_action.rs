use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum GameAction {
    StartLoop {},
    SetFurnaceCookTime { furnace_location: Location, ticks: Number },
    EnableBlockDrops {},
    FillContainer { container_location: Location, items_to_fill_with: Vec<Item> },
    BreakBlock { blocks_to_break: Vec<Location> },
    LPFXSpiral {},
    ParticleSphere {},
    BoneMealBlock { blocks_to_bone_meal: Vec<Location>, number_of_uses: Option<Number> },
    DebugStackTrace {},
    SpawnFallingBlock {
        block_location: Location,
        block_material: Block,
        block_data: Vec<BlockTag>,
    },
    SendDiscordWebhookMessage { webhook_url: Text, message_content: Text },
    ChangeSignText {
        sign_location: Location,
        line_number: Number,
        new_text: Option<MiniMessage>,
    },
    RandomTickBlock { blocks_to_tick: Vec<Location>, number_of_ticks: Option<Number> },
    SendWebRequest { url_to_request: Text, content_body: Option<Text> },
    ClearScBoard {},
    HideSidebar {},
    ReplaceContainerItems {
        container_location: Location,
        items_to_replace: Vec<Item>,
        item_to_replace_with: Item,
        amount_of_items_to: Option<Number>,
    },
    SetEventProjectile { projectile_to_launch: Option<Projectile> },
    SpawnItemDisplay {},
    CreateExplosion { explosion: Location, explosion_power_c07c47: Option<Number> },
    SpawnMob {
        mob_type_: SpawnEgg,
        spawn_location: Location,
        health: Option<Number>,
        custom_name: Option<MiniMessage>,
        effects: Vec<Potion>,
        equipment: Vec<Item>,
    },
    SetIteminBrushableBlock { block_location: Location, item: Option<Item> },
    ParticleLineA {},
    SpawnEyeofEnder {
        location_to_spawn_at: Location,
        destination: Option<Location>,
        lifespan_ticks: Option<Number>,
        custom_name: Option<MiniMessage>,
    },
    ShowSidebar {},
    SpawnAreaofEffectCloud {
        spawn_location: Location,
        effect_to_apply: Vec<Potion>,
        radius_blocks: Option<Number>,
        duration_ticks: Option<Number>,
    },
    ParticleSpiral {},
    SetBlockData { location: Vec<Location>, block_data: Vec<BlockTag> },
    LaunchGameProjectile {
        projectile_to_launch: Projectile,
        launch_point: Location,
        custom_name: Option<MiniMessage>,
        speed: Option<Number>,
        inaccuracy: Option<Number>,
    },
    SpawnFirework { firework_rocket: Item, spawn_location: Location },
    SetBlock {
        block_to_set: Block,
        block_locations: Vec<Location>,
        block_data: Vec<BlockTag>,
    },
    SpawnItemDisp { spawn_location: Location, displayed_item: Item },
    SetEventDamage { new_damage_amount: Number },
    SetBlockGrowth { block_location: Location, growth_stage: Option<Number> },
    Wait {},
    SetContainerName { container_location: Location, name: MiniMessage },
    SpawnItem {
        items_to_spawn: Vec<Item>,
        spawn_location: Location,
        custom_name: Option<MiniMessage>,
    },
    SetSignTextColor { sign_location: Location },
    SetPlayerHead { head_location: Location, head: Either<Item, Text> },
    RemoveHologram {},
    RemoveContainerItems { container_location: Location, items_to_remove: Vec<Item> },
    SpawnShulkerBullet { spawn_location: Location },
    SpawnRngItem {},
    FireworkEffect {},
    SetRegion {
        block_to_set: Block,
        corner_1: Location,
        corner_2: Location,
        block_data: Option<BlockTag>,
    },
    SetContainerContents { container_location: Location, items_to_set: Vec<Item> },
    SpawnInteractionEntity {
        spawn_location: Location,
        hitbox_width: Option<Number>,
        hitbox_height: Option<Number>,
    },
    SetIteminContainerSlot {
        container_location: Location,
        item_to_set: Option<Item>,
        slot: Number,
    },
    ParticleCircleA {},
    CloneRegion {
        corner_1: Location,
        corner_2: Location,
        position_to_copy_from: Location,
        position_to_paste_to: Location,
    },
    UncancelEvent {},
    SetLecternBook {
        lectern_location: Location,
        book_to_put: Option<Item>,
        displayed_page: Option<Number>,
    },
    SpawnPrimedTNT {
        spawn_location: Location,
        tnt_power_c07c47: Option<Number>,
        fuse_duration: Option<Number>,
        custom_name: Option<MiniMessage>,
    },
    SpawnArmorStand {
        spawn_location: Location,
        custom_name: Option<MiniMessage>,
        equipment: Vec<Item>,
    },
    SpawnExperienceOrb {
        spawn_location: Location,
        experience_amount: Option<Number>,
        custom_name: Option<MiniMessage>,
    },
    SetEventHealAmount { new_healing_amount: Number },
    PFXPath {},
    SpawnBlockDisplay {
        spawn_location: Location,
        displayed_block: Block,
        block_data: Vec<BlockTag>,
    },
    ParticleRay {},
    GenerateTree { tree_location_bottom: Location },
    StopLoop {},
    ClearContainer { container_location: Location },
    SetScObj {},
    CancelEvent {},
    SpawnEndCrystal { spawn_location: Location, custom_name: Option<MiniMessage> },
    ParticleEffect {},
    SpawnEvokerFangs { spawn_location: Location, custom_name: Option<MiniMessage> },
    SetEventSound { new_sound: Sound },
    SetCampfireItem {
        campfire_location: Location,
        campfire_item: Item,
        cooking_time_ticks: Option<Number>,
    },
    SpawnTextDisplay { spawn_location: Location, displayed_text: Vec<MiniMessage> },
    SetEventExperience { experience: Number },
    SetContainerLock { container_location: Location, lock_key: Option<Text> },
    SpawnVehicle {
        vehicle_type_: Vehicle,
        spawn_location: Location,
        custom_name: Option<MiniMessage>,
    },
    SummonLightning { impact_location: Location },
    ParticleSpiralA {},
    RemoveScore {},
    CreateHologram {},
    SetScore {},
    ParticleCluster {},
    ParticleCircle {},
    DisableBlockDrops {},
    PFXLineA {},
    ClearContainerItems { container_location: Location, items_to_clear: Vec<Item> },
    ParticleLine {},
}
impl GameAction {
    pub fn compile(&self) -> Value {
        match self {
            GameAction::StartLoop {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StartLoop".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetFurnaceCookTime { furnace_location, ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![furnace_location.json(), ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFurnaceSpeed".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::EnableBlockDrops {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BlockDropsOn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::FillContainer { container_location, items_to_fill_with } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), items_to_fill_with.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FillContainer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::BreakBlock { blocks_to_break } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![blocks_to_break.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BreakBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::LPFXSpiral {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("L PFX Spiral".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleSphere {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleSphere".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::BoneMealBlock { blocks_to_bone_meal, number_of_uses } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![blocks_to_bone_meal.json(), number_of_uses.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BoneMeal".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::DebugStackTrace {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DebugStackTrace".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnFallingBlock {
                block_location,
                block_material,
                block_data,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![block_location.json(), block_material.json(), block_data.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FallingBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SendDiscordWebhookMessage { webhook_url, message_content } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![webhook_url.json(), message_content.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DiscordWebhook".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ChangeSignText { sign_location, line_number, new_text } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![sign_location.json(), line_number.json(), new_text.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ChangeSign".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::RandomTickBlock { blocks_to_tick, number_of_ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![blocks_to_tick.json(), number_of_ticks.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TickBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SendWebRequest { url_to_request, content_body } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![url_to_request.json(), content_body.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WebRequest".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ClearScBoard {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearScBoard".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::HideSidebar {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HideSidebar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ReplaceContainerItems {
                container_location,
                items_to_replace,
                item_to_replace_with,
                amount_of_items_to,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        container_location.json(), items_to_replace.json(),
                        item_to_replace_with.json(), amount_of_items_to.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ReplaceItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetEventProjectile { projectile_to_launch } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![projectile_to_launch.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEventProj".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnItemDisplay {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnItemDisplay".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::CreateExplosion { explosion, explosion_power_c07c47 } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![explosion.json(), explosion_power_c07c47.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Explosion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnMob {
                mob_type_,
                spawn_location,
                health,
                custom_name,
                effects,
                equipment,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        mob_type_.json(), spawn_location.json(), health.json(),
                        custom_name.json(), effects.json(), equipment.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnMob".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetIteminBrushableBlock { block_location, item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![block_location.json(), item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBrushableItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleLineA {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleLineA".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnEyeofEnder {
                location_to_spawn_at,
                destination,
                lifespan_ticks,
                custom_name,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        location_to_spawn_at.json(), destination.json(), lifespan_ticks
                        .json(), custom_name.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnEnderEye".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ShowSidebar {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShowSidebar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnAreaofEffectCloud {
                spawn_location,
                effect_to_apply,
                radius_blocks,
                duration_ticks,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        spawn_location.json(), effect_to_apply.json(), radius_blocks
                        .json(), duration_ticks.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnPotionCloud".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleSpiral {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleSpiral".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetBlockData { location, block_data } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![location.json(), block_data.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBlockData".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::LaunchGameProjectile {
                projectile_to_launch,
                launch_point,
                custom_name,
                speed,
                inaccuracy,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        projectile_to_launch.json(), launch_point.json(), custom_name
                        .json(), speed.json(), inaccuracy.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LaunchProj".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnFirework { firework_rocket, spawn_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![firework_rocket.json(), spawn_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Firework".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetBlock { block_to_set, block_locations, block_data } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![block_to_set.json(), block_locations.json(), block_data.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" SetBlock ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnItemDisp { spawn_location, displayed_item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![spawn_location.json(), displayed_item.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnItemDisp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetEventDamage { new_damage_amount } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_damage_amount.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEventDamage".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetBlockGrowth { block_location, growth_stage } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![block_location.json(), growth_stage.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBlockGrowth".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::Wait {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Wait".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetContainerName { container_location, name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![container_location.json(), name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetContainerName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnItem { items_to_spawn, spawn_location, custom_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        items_to_spawn.json(), spawn_location.json(), custom_name.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetSignTextColor { sign_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![sign_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SignColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetPlayerHead { head_location, head } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![head_location.json(), head.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHead".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::RemoveHologram {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveHologram".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::RemoveContainerItems { container_location, items_to_remove } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), items_to_remove.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnShulkerBullet { spawn_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![spawn_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShulkerBullet".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnRngItem {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnRngItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::FireworkEffect {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FireworkEffect".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetRegion { block_to_set, corner_1, corner_2, block_data } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        block_to_set.json(), corner_1.json(), corner_2.json(), block_data
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetRegion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetContainerContents { container_location, items_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), items_to_set.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetContainer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnInteractionEntity {
                spawn_location,
                hitbox_width,
                hitbox_height,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        spawn_location.json(), hitbox_width.json(), hitbox_height.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnInteraction".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetIteminContainerSlot {
                container_location,
                item_to_set,
                slot,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), item_to_set.json(), slot.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemInSlot".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleCircleA {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleCircleA".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::CloneRegion {
                corner_1,
                corner_2,
                position_to_copy_from,
                position_to_paste_to,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        corner_1.json(), corner_2.json(), position_to_copy_from.json(),
                        position_to_paste_to.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CloneRegion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::UncancelEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("UncancelEvent".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetLecternBook {
                lectern_location,
                book_to_put,
                displayed_page,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        lectern_location.json(), book_to_put.json(), displayed_page
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetLecternBook".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnPrimedTNT {
                spawn_location,
                tnt_power_c07c47,
                fuse_duration,
                custom_name,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        spawn_location.json(), tnt_power_c07c47.json(), fuse_duration
                        .json(), custom_name.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnTNT".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnArmorStand { spawn_location, custom_name, equipment } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![spawn_location.json(), custom_name.json(), equipment.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnArmorStand".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnExperienceOrb {
                spawn_location,
                experience_amount,
                custom_name,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        spawn_location.json(), experience_amount.json(), custom_name
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnExpOrb".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetEventHealAmount { new_healing_amount } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_healing_amount.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEventHeal".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::PFXPath {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PFX Path".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnBlockDisplay {
                spawn_location,
                displayed_block,
                block_data,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        spawn_location.json(), displayed_block.json(), block_data.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnBlockDisp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleRay {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleRay".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::GenerateTree { tree_location_bottom } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![tree_location_bottom.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GenerateTree".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::StopLoop {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StopLoop".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ClearContainer { container_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![container_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearContainer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetScObj {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetScObj".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::CancelEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CancelEvent".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnEndCrystal { spawn_location, custom_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![spawn_location.json(), custom_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnCrystal".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleEffect {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleEffect".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnEvokerFangs { spawn_location, custom_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![spawn_location.json(), custom_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnFangs".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetEventSound { new_sound } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_sound.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEventSound".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetCampfireItem {
                campfire_location,
                campfire_item,
                cooking_time_ticks,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        campfire_location.json(), campfire_item.json(),
                        cooking_time_ticks.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCampfireItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnTextDisplay { spawn_location, displayed_text } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![spawn_location.json(), displayed_text.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnTextDisplay".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetEventExperience { experience } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![experience.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEventXP".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetContainerLock { container_location, lock_key } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), lock_key.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LockContainer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnVehicle { vehicle_type_, spawn_location, custom_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![vehicle_type_.json(), spawn_location.json(), custom_name.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpawnVehicle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SummonLightning { impact_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![impact_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Lightning".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleSpiralA {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleSpiralA".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::RemoveScore {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveScore".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::CreateHologram {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CreateHologram".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetScore {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetScore".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleCluster {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleCluster".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleCircle {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleCircle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::DisableBlockDrops {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BlockDropsOff".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::PFXLineA {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PFX Line [A]".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ClearContainerItems { container_location, items_to_clear } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![container_location.json(), items_to_clear.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::ParticleLine {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleLine".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
