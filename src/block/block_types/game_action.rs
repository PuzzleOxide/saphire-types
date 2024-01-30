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
    BoneMealBlock {
        blocks_to_bone_meal: Vec<Location>,
        number_of_uses: Option<Number>,
        show_particles_tag: ShowParticlesBoneMealBlock,
    },
    DebugStackTrace {},
    SpawnFallingBlock {
        block_location: Location,
        block_material: Block,
        block_data: Vec<BlockTag>,
        hurt_hit_entities_tag: HurtHitEntitiesSpawnFallingBlock,
        reform_on_impact_tag: ReformonImpactSpawnFallingBlock,
    },
    SendDiscordWebhookMessage { webhook_url: Text, message_content: Text },
    ChangeSignText {
        sign_location: Location,
        line_number: Number,
        new_text: Option<MiniMessage>,
        sign_side_tag: SignSideChangeSignText,
    },
    RandomTickBlock { blocks_to_tick: Vec<Location>, number_of_ticks: Option<Number> },
    SendWebRequest {
        url_to_request: Text,
        content_body: Option<Text>,
        request_method_tag: RequestMethodSendWebRequest,
        content_type__tag: ContentTypeSendWebRequest,
    },
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
        end_of_lifespan_tag: EndofLifespanSpawnEyeofEnder,
    },
    ShowSidebar {},
    SpawnAreaofEffectCloud {
        spawn_location: Location,
        effect_to_apply: Vec<Potion>,
        radius_blocks: Option<Number>,
        duration_ticks: Option<Number>,
    },
    ParticleSpiral {},
    SetBlockData {
        location: Vec<Location>,
        block_data: Vec<BlockTag>,
        overwrite_existing_data_tag: OverwriteExistingDataSetBlockData,
    },
    LaunchGameProjectile {
        projectile_to_launch: Projectile,
        launch_point: Location,
        custom_name: Option<MiniMessage>,
        speed: Option<Number>,
        inaccuracy: Option<Number>,
    },
    SpawnFirework {
        firework_rocket: Item,
        spawn_location: Location,
        instant_tag: InstantSpawnFirework,
        movement_tag: MovementSpawnFirework,
    },
    SetBlock {
        block_to_set: Block,
        block_locations: Vec<Location>,
        block_data: Vec<BlockTag>,
    },
    SpawnItemDisp { spawn_location: Location, displayed_item: Item },
    SetEventDamage { new_damage_amount: Number },
    SetBlockGrowth {
        block_location: Location,
        growth_stage: Option<Number>,
        growth_unit_tag: GrowthUnitSetBlockGrowth,
    },
    Wait { delay_unit_tag: DelayUnitWait },
    SetContainerName { container_location: Location, name: MiniMessage },
    SpawnItem {
        items_to_spawn: Vec<Item>,
        spawn_location: Location,
        custom_name: Option<MiniMessage>,
        apply_item_motion_tag: ApplyItemMotionSpawnItem,
    },
    SetSignTextColor {
        sign_location: Location,
        sign_side_tag: SignSideSetSignTextColor,
        text_color_tag: TextColorSetSignTextColor,
        glowing_tag: GlowingSetSignTextColor,
    },
    SetPlayerHead { head_location: Location, head: Either<Item, Text> },
    RemoveHologram {},
    RemoveContainerItems { container_location: Location, items_to_remove: Vec<Item> },
    SpawnShulkerBullet { spawn_location: Location },
    SpawnRngItem { apply_item_motion_tag: ApplyItemMotionSpawnRngItem },
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
        responsive_tag: ResponsiveSpawnInteractionEntity,
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
        ignore_air_tag: IgnoreAirCloneRegion,
        clone_block_entities_tag: CloneBlockEntitiesCloneRegion,
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
        visibility_tag: VisibilitySpawnArmorStand,
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
    GenerateTree {
        tree_location_bottom: Location,
        tree_type__tag: TreeTypeGenerateTree,
    },
    StopLoop {},
    ClearContainer { container_location: Location },
    SetScObj {},
    CancelEvent {},
    SpawnEndCrystal {
        spawn_location: Location,
        custom_name: Option<MiniMessage>,
        show_bottom_tag: ShowBottomSpawnEndCrystal,
    },
    ParticleEffect {},
    SpawnEvokerFangs { spawn_location: Location, custom_name: Option<MiniMessage> },
    SetEventSound { new_sound: Sound },
    SetCampfireItem {
        campfire_location: Location,
        campfire_item: Item,
        cooking_time_ticks: Option<Number>,
        campfire_slot_tag: CampfireSlotSetCampfireItem,
    },
    SpawnTextDisplay {
        spawn_location: Location,
        displayed_text: Vec<MiniMessage>,
        text_value_merging_tag: TextValueMergingSpawnTextDisplay,
        inherit_styles_tag: InheritStylesSpawnTextDisplay,
    },
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![furnace_location.json(), ticks.json()],
                    vec![],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![container_location.json(), items_to_fill_with.json()],
                    vec![],
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
                let mut item_args = compile(vec![blocks_to_break.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleSphere".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::BoneMealBlock {
                blocks_to_bone_meal,
                number_of_uses,
                show_particles_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![blocks_to_bone_meal.json(), number_of_uses.json()],
                    vec![show_particles_tag.json()],
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
                let mut item_args = compile(vec![], vec![]);
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
                hurt_hit_entities_tag,
                reform_on_impact_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        block_location.json(), block_material.json(), block_data.json()
                    ],
                    vec![hurt_hit_entities_tag.json(), reform_on_impact_tag.json()],
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
                let mut item_args = compile(
                    vec![webhook_url.json(), message_content.json()],
                    vec![],
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
            GameAction::ChangeSignText {
                sign_location,
                line_number,
                new_text,
                sign_side_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![sign_location.json(), line_number.json(), new_text.json()],
                    vec![sign_side_tag.json()],
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
                let mut item_args = compile(
                    vec![blocks_to_tick.json(), number_of_ticks.json()],
                    vec![],
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
            GameAction::SendWebRequest {
                url_to_request,
                content_body,
                request_method_tag,
                content_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![url_to_request.json(), content_body.json()],
                    vec![request_method_tag.json(), content_type__tag.json()],
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        container_location.json(), items_to_replace.json(),
                        item_to_replace_with.json(), amount_of_items_to.json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![projectile_to_launch.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![explosion.json(), explosion_power_c07c47.json()],
                    vec![],
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
                let mut item_args = compile(
                    vec![
                        mob_type_.json(), spawn_location.json(), health.json(),
                        custom_name.json(), effects.json(), equipment.json()
                    ],
                    vec![],
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
                let mut item_args = compile(
                    vec![block_location.json(), item.json()],
                    vec![],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                end_of_lifespan_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        location_to_spawn_at.json(), destination.json(), lifespan_ticks
                        .json(), custom_name.json()
                    ],
                    vec![end_of_lifespan_tag.json()],
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        spawn_location.json(), effect_to_apply.json(), radius_blocks
                        .json(), duration_ticks.json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleSpiral".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetBlockData {
                location,
                block_data,
                overwrite_existing_data_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![location.json(), block_data.json()],
                    vec![overwrite_existing_data_tag.json()],
                );
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
                let mut item_args = compile(
                    vec![
                        projectile_to_launch.json(), launch_point.json(), custom_name
                        .json(), speed.json(), inaccuracy.json()
                    ],
                    vec![],
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
            GameAction::SpawnFirework {
                firework_rocket,
                spawn_location,
                instant_tag,
                movement_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![firework_rocket.json(), spawn_location.json()],
                    vec![instant_tag.json(), movement_tag.json()],
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
                let mut item_args = compile(
                    vec![block_to_set.json(), block_locations.json(), block_data.json()],
                    vec![],
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
                let mut item_args = compile(
                    vec![spawn_location.json(), displayed_item.json()],
                    vec![],
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
                let mut item_args = compile(vec![new_damage_amount.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEventDamage".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SetBlockGrowth {
                block_location,
                growth_stage,
                growth_unit_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![block_location.json(), growth_stage.json()],
                    vec![growth_unit_tag.json()],
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
            GameAction::Wait { delay_unit_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![delay_unit_tag.json()]);
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
                let mut item_args = compile(
                    vec![container_location.json(), name.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetContainerName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnItem {
                items_to_spawn,
                spawn_location,
                custom_name,
                apply_item_motion_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        items_to_spawn.json(), spawn_location.json(), custom_name.json()
                    ],
                    vec![apply_item_motion_tag.json()],
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
            GameAction::SetSignTextColor {
                sign_location,
                sign_side_tag,
                text_color_tag,
                glowing_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![sign_location.json()],
                    vec![sign_side_tag.json(), text_color_tag.json(), glowing_tag.json()],
                );
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
                let mut item_args = compile(
                    vec![head_location.json(), head.json()],
                    vec![],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![container_location.json(), items_to_remove.json()],
                    vec![],
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
                let mut item_args = compile(vec![spawn_location.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShulkerBullet".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnRngItem { apply_item_motion_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![apply_item_motion_tag.json()]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        block_to_set.json(), corner_1.json(), corner_2.json(), block_data
                        .json()
                    ],
                    vec![],
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
                let mut item_args = compile(
                    vec![container_location.json(), items_to_set.json()],
                    vec![],
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
                responsive_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        spawn_location.json(), hitbox_width.json(), hitbox_height.json()
                    ],
                    vec![responsive_tag.json()],
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
                let mut item_args = compile(
                    vec![container_location.json(), item_to_set.json(), slot.json()],
                    vec![],
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
                let mut item_args = compile(vec![], vec![]);
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
                ignore_air_tag,
                clone_block_entities_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        corner_1.json(), corner_2.json(), position_to_copy_from.json(),
                        position_to_paste_to.json()
                    ],
                    vec![ignore_air_tag.json(), clone_block_entities_tag.json()],
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        lectern_location.json(), book_to_put.json(), displayed_page
                        .json()
                    ],
                    vec![],
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
                let mut item_args = compile(
                    vec![
                        spawn_location.json(), tnt_power_c07c47.json(), fuse_duration
                        .json(), custom_name.json()
                    ],
                    vec![],
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
            GameAction::SpawnArmorStand {
                spawn_location,
                custom_name,
                equipment,
                visibility_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![spawn_location.json(), custom_name.json(), equipment.json()],
                    vec![visibility_tag.json()],
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
                let mut item_args = compile(
                    vec![
                        spawn_location.json(), experience_amount.json(), custom_name
                        .json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![new_healing_amount.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        spawn_location.json(), displayed_block.json(), block_data.json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleRay".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::GenerateTree { tree_location_bottom, tree_type__tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![tree_location_bottom.json()],
                    vec![tree_type__tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![container_location.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CancelEvent".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            GameAction::SpawnEndCrystal {
                spawn_location,
                custom_name,
                show_bottom_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![spawn_location.json(), custom_name.json()],
                    vec![show_bottom_tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![spawn_location.json(), custom_name.json()],
                    vec![],
                );
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
                let mut item_args = compile(vec![new_sound.json()], vec![]);
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
                campfire_slot_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        campfire_location.json(), campfire_item.json(),
                        cooking_time_ticks.json()
                    ],
                    vec![campfire_slot_tag.json()],
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
            GameAction::SpawnTextDisplay {
                spawn_location,
                displayed_text,
                text_value_merging_tag,
                inherit_styles_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![spawn_location.json(), displayed_text.json()],
                    vec![text_value_merging_tag.json(), inherit_styles_tag.json()],
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
                let mut item_args = compile(vec![experience.json()], vec![]);
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
                let mut item_args = compile(
                    vec![container_location.json(), lock_key.json()],
                    vec![],
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
                let mut item_args = compile(
                    vec![
                        vehicle_type_.json(), spawn_location.json(), custom_name.json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![impact_location.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![container_location.json(), items_to_clear.json()],
                    vec![],
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
                let mut item_args = compile(vec![], vec![]);
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
#[derive(Debug, Clone)]
pub enum ShowParticlesBoneMealBlock {
    True,
    False,
}
impl ShowParticlesBoneMealBlock {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShowParticlesBoneMealBlock::True => Value::String("True".to_string()),
                ShowParticlesBoneMealBlock::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Show Particles".to_string()));
        data.insert("action".to_string(), Value::String("BoneMeal".to_string()));
        data.insert("block".to_string(), Value::String("BoneMeal".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShowParticlesBoneMealBlock {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum HurtHitEntitiesSpawnFallingBlock {
    True,
    False,
}
impl HurtHitEntitiesSpawnFallingBlock {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HurtHitEntitiesSpawnFallingBlock::True => {
                    Value::String("True".to_string())
                }
                HurtHitEntitiesSpawnFallingBlock::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hurt Hit Entities".to_string()));
        data.insert("action".to_string(), Value::String("FallingBlock".to_string()));
        data.insert("block".to_string(), Value::String("FallingBlock".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HurtHitEntitiesSpawnFallingBlock {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum ReformonImpactSpawnFallingBlock {
    True,
    False,
}
impl ReformonImpactSpawnFallingBlock {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ReformonImpactSpawnFallingBlock::True => {
                    Value::String("True".to_string())
                }
                ReformonImpactSpawnFallingBlock::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Reform on Impact".to_string()));
        data.insert("action".to_string(), Value::String("FallingBlock".to_string()));
        data.insert("block".to_string(), Value::String("FallingBlock".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ReformonImpactSpawnFallingBlock {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum SignSideChangeSignText {
    Front,
    Back,
}
impl SignSideChangeSignText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignSideChangeSignText::Front => Value::String("Front".to_string()),
                SignSideChangeSignText::Back => Value::String("Back".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Side".to_string()));
        data.insert("action".to_string(), Value::String("ChangeSign".to_string()));
        data.insert("block".to_string(), Value::String("ChangeSign".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignSideChangeSignText {
    fn default() -> Self {
        Self::Front
    }
}
#[derive(Debug, Clone)]
pub enum RequestMethodSendWebRequest {
    Post,
    Get,
    Put,
    Delete,
}
impl RequestMethodSendWebRequest {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RequestMethodSendWebRequest::Post => Value::String("Post".to_string()),
                RequestMethodSendWebRequest::Get => Value::String("Get".to_string()),
                RequestMethodSendWebRequest::Put => Value::String("Put".to_string()),
                RequestMethodSendWebRequest::Delete => {
                    Value::String("Delete".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Request Method".to_string()));
        data.insert("action".to_string(), Value::String("WebRequest".to_string()));
        data.insert("block".to_string(), Value::String("WebRequest".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RequestMethodSendWebRequest {
    fn default() -> Self {
        Self::Post
    }
}
#[derive(Debug, Clone)]
pub enum ContentTypeSendWebRequest {
    textplain,
    applicationjson,
}
impl ContentTypeSendWebRequest {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ContentTypeSendWebRequest::textplain => {
                    Value::String("text/plain".to_string())
                }
                ContentTypeSendWebRequest::applicationjson => {
                    Value::String("application/json".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Content Type".to_string()));
        data.insert("action".to_string(), Value::String("WebRequest".to_string()));
        data.insert("block".to_string(), Value::String("WebRequest".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ContentTypeSendWebRequest {
    fn default() -> Self {
        Self::textplain
    }
}
#[derive(Debug, Clone)]
pub enum EndofLifespanSpawnEyeofEnder {
    Dropitem,
    Shatter,
    Random,
}
impl EndofLifespanSpawnEyeofEnder {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                EndofLifespanSpawnEyeofEnder::Dropitem => {
                    Value::String("Drop item".to_string())
                }
                EndofLifespanSpawnEyeofEnder::Shatter => {
                    Value::String("Shatter".to_string())
                }
                EndofLifespanSpawnEyeofEnder::Random => {
                    Value::String("Random".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("End of Lifespan".to_string()));
        data.insert("action".to_string(), Value::String("SpawnEnderEye".to_string()));
        data.insert("block".to_string(), Value::String("SpawnEnderEye".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for EndofLifespanSpawnEyeofEnder {
    fn default() -> Self {
        Self::Random
    }
}
#[derive(Debug, Clone)]
pub enum OverwriteExistingDataSetBlockData {
    True,
    False,
}
impl OverwriteExistingDataSetBlockData {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                OverwriteExistingDataSetBlockData::True => {
                    Value::String("True".to_string())
                }
                OverwriteExistingDataSetBlockData::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Overwrite Existing Data".to_string()),
        );
        data.insert("action".to_string(), Value::String("SetBlockData".to_string()));
        data.insert("block".to_string(), Value::String("SetBlockData".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for OverwriteExistingDataSetBlockData {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum InstantSpawnFirework {
    True,
    False,
}
impl InstantSpawnFirework {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InstantSpawnFirework::True => Value::String("True".to_string()),
                InstantSpawnFirework::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Instant".to_string()));
        data.insert("action".to_string(), Value::String("Firework".to_string()));
        data.insert("block".to_string(), Value::String("Firework".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InstantSpawnFirework {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum MovementSpawnFirework {
    Upwards,
    Directional,
}
impl MovementSpawnFirework {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                MovementSpawnFirework::Upwards => Value::String("Upwards".to_string()),
                MovementSpawnFirework::Directional => {
                    Value::String("Directional".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Movement".to_string()));
        data.insert("action".to_string(), Value::String("Firework".to_string()));
        data.insert("block".to_string(), Value::String("Firework".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for MovementSpawnFirework {
    fn default() -> Self {
        Self::Upwards
    }
}
#[derive(Debug, Clone)]
pub enum GrowthUnitSetBlockGrowth {
    GrowthStageNumber,
    GrowthPercentage,
}
impl GrowthUnitSetBlockGrowth {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GrowthUnitSetBlockGrowth::GrowthStageNumber => {
                    Value::String("Growth Stage Number".to_string())
                }
                GrowthUnitSetBlockGrowth::GrowthPercentage => {
                    Value::String("Growth Percentage".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Growth Unit".to_string()));
        data.insert("action".to_string(), Value::String("SetBlockGrowth".to_string()));
        data.insert("block".to_string(), Value::String("SetBlockGrowth".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GrowthUnitSetBlockGrowth {
    fn default() -> Self {
        Self::GrowthStageNumber
    }
}
#[derive(Debug, Clone)]
pub enum DelayUnitWait {
    Ticks,
    Seconds,
    Minutes,
}
impl DelayUnitWait {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DelayUnitWait::Ticks => Value::String("Ticks".to_string()),
                DelayUnitWait::Seconds => Value::String("Seconds".to_string()),
                DelayUnitWait::Minutes => Value::String("Minutes".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Delay Unit".to_string()));
        data.insert("action".to_string(), Value::String("Wait".to_string()));
        data.insert("block".to_string(), Value::String("Wait".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DelayUnitWait {
    fn default() -> Self {
        Self::Ticks
    }
}
#[derive(Debug, Clone)]
pub enum ApplyItemMotionSpawnItem {
    True,
    False,
}
impl ApplyItemMotionSpawnItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ApplyItemMotionSpawnItem::True => Value::String("True".to_string()),
                ApplyItemMotionSpawnItem::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Apply Item Motion".to_string()));
        data.insert("action".to_string(), Value::String("SpawnItem".to_string()));
        data.insert("block".to_string(), Value::String("SpawnItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ApplyItemMotionSpawnItem {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum SignSideSetSignTextColor {
    Front,
    Back,
}
impl SignSideSetSignTextColor {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignSideSetSignTextColor::Front => Value::String("Front".to_string()),
                SignSideSetSignTextColor::Back => Value::String("Back".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Side".to_string()));
        data.insert("action".to_string(), Value::String("SignColor".to_string()));
        data.insert("block".to_string(), Value::String("SignColor".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignSideSetSignTextColor {
    fn default() -> Self {
        Self::Front
    }
}
#[derive(Debug, Clone)]
pub enum TextColorSetSignTextColor {
    White,
    Orange,
    Magenta,
    Lightblue,
    Yellow,
    Lime,
    Pink,
    Gray,
    Lightgray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}
impl TextColorSetSignTextColor {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextColorSetSignTextColor::White => Value::String("White".to_string()),
                TextColorSetSignTextColor::Orange => Value::String("Orange".to_string()),
                TextColorSetSignTextColor::Magenta => {
                    Value::String("Magenta".to_string())
                }
                TextColorSetSignTextColor::Lightblue => {
                    Value::String("Light blue".to_string())
                }
                TextColorSetSignTextColor::Yellow => Value::String("Yellow".to_string()),
                TextColorSetSignTextColor::Lime => Value::String("Lime".to_string()),
                TextColorSetSignTextColor::Pink => Value::String("Pink".to_string()),
                TextColorSetSignTextColor::Gray => Value::String("Gray".to_string()),
                TextColorSetSignTextColor::Lightgray => {
                    Value::String("Light gray".to_string())
                }
                TextColorSetSignTextColor::Cyan => Value::String("Cyan".to_string()),
                TextColorSetSignTextColor::Purple => Value::String("Purple".to_string()),
                TextColorSetSignTextColor::Blue => Value::String("Blue".to_string()),
                TextColorSetSignTextColor::Brown => Value::String("Brown".to_string()),
                TextColorSetSignTextColor::Green => Value::String("Green".to_string()),
                TextColorSetSignTextColor::Red => Value::String("Red".to_string()),
                TextColorSetSignTextColor::Black => Value::String("Black".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Text Color".to_string()));
        data.insert("action".to_string(), Value::String("SignColor".to_string()));
        data.insert("block".to_string(), Value::String("SignColor".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextColorSetSignTextColor {
    fn default() -> Self {
        Self::Black
    }
}
#[derive(Debug, Clone)]
pub enum GlowingSetSignTextColor {
    Enable,
    Disable,
}
impl GlowingSetSignTextColor {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GlowingSetSignTextColor::Enable => Value::String("Enable".to_string()),
                GlowingSetSignTextColor::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Glowing".to_string()));
        data.insert("action".to_string(), Value::String("SignColor".to_string()));
        data.insert("block".to_string(), Value::String("SignColor".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GlowingSetSignTextColor {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum ApplyItemMotionSpawnRngItem {
    True,
    False,
}
impl ApplyItemMotionSpawnRngItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ApplyItemMotionSpawnRngItem::True => Value::String("True".to_string()),
                ApplyItemMotionSpawnRngItem::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Apply Item Motion".to_string()));
        data.insert("action".to_string(), Value::String("SpawnRngItem".to_string()));
        data.insert("block".to_string(), Value::String("SpawnRngItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ApplyItemMotionSpawnRngItem {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum ResponsiveSpawnInteractionEntity {
    Enable,
    Disable,
}
impl ResponsiveSpawnInteractionEntity {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ResponsiveSpawnInteractionEntity::Enable => {
                    Value::String("Enable".to_string())
                }
                ResponsiveSpawnInteractionEntity::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Responsive".to_string()));
        data.insert("action".to_string(), Value::String("SpawnInteraction".to_string()));
        data.insert("block".to_string(), Value::String("SpawnInteraction".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ResponsiveSpawnInteractionEntity {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreAirCloneRegion {
    True,
    False,
}
impl IgnoreAirCloneRegion {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreAirCloneRegion::True => Value::String("True".to_string()),
                IgnoreAirCloneRegion::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Air".to_string()));
        data.insert("action".to_string(), Value::String("CloneRegion".to_string()));
        data.insert("block".to_string(), Value::String("CloneRegion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreAirCloneRegion {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum CloneBlockEntitiesCloneRegion {
    True,
    False,
}
impl CloneBlockEntitiesCloneRegion {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CloneBlockEntitiesCloneRegion::True => Value::String("True".to_string()),
                CloneBlockEntitiesCloneRegion::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Clone Block Entities".to_string()),
        );
        data.insert("action".to_string(), Value::String("CloneRegion".to_string()));
        data.insert("block".to_string(), Value::String("CloneRegion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CloneBlockEntitiesCloneRegion {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum VisibilitySpawnArmorStand {
    Visible,
    VisibleNohitbox,
    Invisible,
    InvisibleNohitbox,
}
impl VisibilitySpawnArmorStand {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                VisibilitySpawnArmorStand::Visible => {
                    Value::String("Visible".to_string())
                }
                VisibilitySpawnArmorStand::VisibleNohitbox => {
                    Value::String("Visible (No hitbox)".to_string())
                }
                VisibilitySpawnArmorStand::Invisible => {
                    Value::String("Invisible".to_string())
                }
                VisibilitySpawnArmorStand::InvisibleNohitbox => {
                    Value::String("Invisible (No hitbox)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Visibility".to_string()));
        data.insert("action".to_string(), Value::String("SpawnArmorStand".to_string()));
        data.insert("block".to_string(), Value::String("SpawnArmorStand".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for VisibilitySpawnArmorStand {
    fn default() -> Self {
        Self::Visible
    }
}
#[derive(Debug, Clone)]
pub enum TreeTypeGenerateTree {
    OakTree,
    BigOakTree,
    SwampTree,
    SpruceTree,
    SlightlyTallerSpruceTree,
    BigSpruceTree,
    BirchTree,
    TallBirchTree,
    JungleTree,
    BigJungleTree,
    JungleBush,
    AcaciaTree,
    DarkOakTree,
    MangroveTree,
    TallMangroveTree,
    CherryTree,
    AzaleaTree,
    RedMushroom,
    BrownMushroom,
    CrimsonFungus,
    WarpedFungus,
    ChorusPlant,
}
impl TreeTypeGenerateTree {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TreeTypeGenerateTree::OakTree => Value::String("Oak Tree".to_string()),
                TreeTypeGenerateTree::BigOakTree => {
                    Value::String("Big Oak Tree".to_string())
                }
                TreeTypeGenerateTree::SwampTree => {
                    Value::String("Swamp Tree".to_string())
                }
                TreeTypeGenerateTree::SpruceTree => {
                    Value::String("Spruce Tree".to_string())
                }
                TreeTypeGenerateTree::SlightlyTallerSpruceTree => {
                    Value::String("Slightly Taller Spruce Tree".to_string())
                }
                TreeTypeGenerateTree::BigSpruceTree => {
                    Value::String("Big Spruce Tree".to_string())
                }
                TreeTypeGenerateTree::BirchTree => {
                    Value::String("Birch Tree".to_string())
                }
                TreeTypeGenerateTree::TallBirchTree => {
                    Value::String("Tall Birch Tree".to_string())
                }
                TreeTypeGenerateTree::JungleTree => {
                    Value::String("Jungle Tree".to_string())
                }
                TreeTypeGenerateTree::BigJungleTree => {
                    Value::String("Big Jungle Tree".to_string())
                }
                TreeTypeGenerateTree::JungleBush => {
                    Value::String("Jungle Bush".to_string())
                }
                TreeTypeGenerateTree::AcaciaTree => {
                    Value::String("Acacia Tree".to_string())
                }
                TreeTypeGenerateTree::DarkOakTree => {
                    Value::String("Dark Oak Tree".to_string())
                }
                TreeTypeGenerateTree::MangroveTree => {
                    Value::String("Mangrove Tree".to_string())
                }
                TreeTypeGenerateTree::TallMangroveTree => {
                    Value::String("Tall Mangrove Tree".to_string())
                }
                TreeTypeGenerateTree::CherryTree => {
                    Value::String("Cherry Tree".to_string())
                }
                TreeTypeGenerateTree::AzaleaTree => {
                    Value::String("Azalea Tree".to_string())
                }
                TreeTypeGenerateTree::RedMushroom => {
                    Value::String("Red Mushroom".to_string())
                }
                TreeTypeGenerateTree::BrownMushroom => {
                    Value::String("Brown Mushroom".to_string())
                }
                TreeTypeGenerateTree::CrimsonFungus => {
                    Value::String("Crimson Fungus".to_string())
                }
                TreeTypeGenerateTree::WarpedFungus => {
                    Value::String("Warped Fungus".to_string())
                }
                TreeTypeGenerateTree::ChorusPlant => {
                    Value::String("Chorus Plant".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Tree Type".to_string()));
        data.insert("action".to_string(), Value::String("GenerateTree".to_string()));
        data.insert("block".to_string(), Value::String("GenerateTree".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TreeTypeGenerateTree {
    fn default() -> Self {
        Self::OakTree
    }
}
#[derive(Debug, Clone)]
pub enum ShowBottomSpawnEndCrystal {
    True,
    False,
}
impl ShowBottomSpawnEndCrystal {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShowBottomSpawnEndCrystal::True => Value::String("True".to_string()),
                ShowBottomSpawnEndCrystal::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Show Bottom".to_string()));
        data.insert("action".to_string(), Value::String("SpawnCrystal".to_string()));
        data.insert("block".to_string(), Value::String("SpawnCrystal".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShowBottomSpawnEndCrystal {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum CampfireSlotSetCampfireItem {
    One,
    Two,
    Three,
    Four,
}
impl CampfireSlotSetCampfireItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CampfireSlotSetCampfireItem::One => Value::String("1".to_string()),
                CampfireSlotSetCampfireItem::Two => Value::String("2".to_string()),
                CampfireSlotSetCampfireItem::Three => Value::String("3".to_string()),
                CampfireSlotSetCampfireItem::Four => Value::String("4".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Campfire Slot".to_string()));
        data.insert("action".to_string(), Value::String("SetCampfireItem".to_string()));
        data.insert("block".to_string(), Value::String("SetCampfireItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CampfireSlotSetCampfireItem {
    fn default() -> Self {
        Self::One
    }
}
#[derive(Debug, Clone)]
pub enum TextValueMergingSpawnTextDisplay {
    Addspaces,
    Nospaces,
}
impl TextValueMergingSpawnTextDisplay {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextValueMergingSpawnTextDisplay::Addspaces => {
                    Value::String("Add spaces".to_string())
                }
                TextValueMergingSpawnTextDisplay::Nospaces => {
                    Value::String("No spaces".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Value Merging".to_string()));
        data.insert("action".to_string(), Value::String("SpawnTextDisplay".to_string()));
        data.insert("block".to_string(), Value::String("SpawnTextDisplay".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextValueMergingSpawnTextDisplay {
    fn default() -> Self {
        Self::Nospaces
    }
}
#[derive(Debug, Clone)]
pub enum InheritStylesSpawnTextDisplay {
    True,
    False,
}
impl InheritStylesSpawnTextDisplay {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InheritStylesSpawnTextDisplay::True => Value::String("True".to_string()),
                InheritStylesSpawnTextDisplay::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Inherit Styles".to_string()));
        data.insert("action".to_string(), Value::String("SpawnTextDisplay".to_string()));
        data.insert("block".to_string(), Value::String("SpawnTextDisplay".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InheritStylesSpawnTextDisplay {
    fn default() -> Self {
        Self::True
    }
}
