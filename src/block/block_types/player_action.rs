use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum PlayerAction {
    SetHotbarItems { items_to_set: Vec<Item> },
    SetReducedDebugInfoEnabled {},
    CloseInventory {},
    GiveItems { items_to_give: Vec<Item>, amount_to_give: Option<Number> },
    NoKeepInv {},
    SetAllowHandCrafting {},
    BossBar {},
    DisplayParticleSphere {
        effect: Particle,
        center_location: Location,
        diameter: Option<Number>,
    },
    SetBaseAttackSpeed { attack_speed: Option<Number> },
    SetVelocity { new_velocity: Vector },
    DisplayParticleEffect { effect: Vec<Particle>, effect_location: Location },
    AddInventoryMenuRow { items_to_display: Vec<Item> },
    DisablePvp {},
    NoNatRegen {},
    DisplayLightningBolt { strike_location: Location },
    PlaySoundfromEntity {
        sound_to_play: Vec<Sound>,
        target_uuid: Vec<Either<Text, MiniMessage>>,
    },
    ReplaceProj {},
    Damage {
        damage_to_inflict: Number,
        uuid_of_damager_entity: Option<Either<Text, MiniMessage>>,
    },
    SetExperience { experience_to_set: Number },
    SendAnimation {},
    SetXPProgress { progress__0100: Number },
    SetInventoryItems { items_to_set: Vec<Item> },
    TeleportSequence {
        locations_to: Vec<Location>,
        teleport_delay_ticks: Option<Number>,
    },
    Heal { amount_to_heal: Option<Number> },
    SetSpawnPoint { the_new_spawn_location: Option<Location> },
    DisplayParticleSpiral {
        effect: Particle,
        base_location: Location,
        length: Option<Number>,
        diameter: Option<Number>,
        effect_count: Option<Number>,
        rotations: Option<Number>,
    },
    SetInventoryKept {},
    SetAllowFlight {},
    LaunchUp { launch_power: Number },
    SetMaximumHealth { maximum_health: Number },
    RemoveBossBar { boss_bar_position: Option<Number> },
    SetFogDistance { fog_distance_in_chunks: Option<Number> },
    GetTargetEntity {},
    SettoAdventureMode {},
    ForceFlightMode {},
    LoadSavedInventory {},
    SettoSpectatorMode {},
    ClearPotionEffects {},
    Kick {},
    SetPlayerListInfo { headerfooter_text: Vec<MiniMessage> },
    EnablePvp {},
    ProjColl {},
    HideDisguise {},
    SpectateTarget { target_uuid: Option<Either<Text, MiniMessage>> },
    SendHurtAnimation { damage_source: Option<Location> },
    SettoSurvivalMode {},
    SetBossBar {
        title: Option<MiniMessage>,
        current_health: Option<Number>,
        maximum_health: Option<Number>,
        boss_bar_position: Option<Number>,
    },
    SetSkin { player_head: Option<Item> },
    SetSpectatorCollision {},
    SetStatus { game_status: Option<MiniMessage> },
    SetNameTagVisible {},
    SetInvulnerabilityTicks { ticks: Number },
    SetCursorItem { item_to_set: Option<Item> },
    SetAbsorptionHealth { absorption_health: Number },
    SetFireTicks { ticks: Number },
    SetGamemode {},
    RemoveInventoryMenuRow { rows_to_remove: Option<Number> },
    EnableFlight {},
    SendWakeUpAnimation {},
    SetBeeStingsStuck { sting_count: Option<Number> },
    DisallowPlacingBreakingBlocks { blocks_to_disallow: Vec<Block> },
    SetScoreboardObjectiveName { objective_name: MiniMessage },
    LSetHealth {},
    ParticleEffect {},
    ClearInventory {},
    SetFreezeTicks { ticks: Number },
    SetGliding {},
    SetRotation { pitch_90_to_90: Number, yaw_180_to_180: Number },
    RemoveScoreboardScore { score_name: MiniMessage },
    DisallowDrops {},
    DisplayParticleCircle {
        effect: Particle,
        center_location: Location,
        diameter: Option<Number>,
    },
    ClearItems { items_to_clear: Vec<Item> },
    DisplayBlock {
        block_to_display: Block,
        block_location: Location,
        end_of_region: Option<Location>,
        block_data: Vec<BlockTag>,
    },
    RideEntity { target_uuid: Vec<Either<Text, MiniMessage>> },
    WeatherRain {},
    SetFlying {},
    RemoveWorldBorder {},
    SendResourcePack { resource_pack_url: Text },
    DisplayBlockOpenedState { block_location: Location },
    SetHandItem {},
    SendAdvancement { advancement_name: MiniMessage, advancement_icon: Item },
    Teleport { new_position: Location },
    ClearChat {},
    SetPvPAllowed {},
    SetInventoryMenuItem { slot: Number, item_to_set: Option<Item> },
    LaunchTowardLocation { launch_destination: Location, launch_power: Option<Number> },
    SetArmorItems { armor_to_set: Vec<Item> },
    DisplayGatewayBeam { gateway_location: Location },
    DisableFlight {},
    GiveSaturation { saturation_to_give: Number },
    SetVisualFire {},
    SetOwnDisguiseVisibility {},
    SetArrowsStuck { arrow_count: Option<Number> },
    GiveExperience { experience_to_give: Number },
    FaceLocation { location_to_face: Location },
    ClearScoreboard {},
    SetItems {},
    KeepInv {},
    ReplaceItems {
        items_to_replace: Vec<Item>,
        item_to_replace_with: Item,
        amount_of_items_to: Option<Number>,
    },
    ShowActionBarText { message_to_send: Vec<MiniMessage> },
    SetChatTag { chat_tag: Vec<MiniMessage> },
    SendMessage { message_to_send: Vec<MiniMessage> },
    ShiftWorldBorder { new_radius: Number, blocks_per_second: Option<Number> },
    SetIteminSlot { item_to_set: Option<Item>, slot_to_set: Number },
    PlaySoundSequence {
        sounds_to_play: Vec<Sound>,
        sound_delay_ticks: Option<Number>,
        playback_location: Option<Location>,
    },
    DisplayAnimatedParticleLine {
        effect: Particle,
        start_location: Location,
        end_location: Location,
        effect_spacing: Option<Number>,
        animation_duration: Option<Number>,
    },
    DisplaySignText { sign_location: Location, text_lines: Vec<MiniMessage> },
    Respawn {},
    SetMovementSpeed { movement_speed: Number },
    ExpandInventoryMenu { items_to_display: Vec<Item> },
    SetInventoryMenuName { inventory_name: MiniMessage },
    LaunchProjectile {
        projectile_to: Projectile,
        launch_point: Option<Location>,
        projectile_name: Option<MiniMessage>,
        speed: Option<Number>,
        inaccuracy: Option<Number>,
    },
    SetItemCooldown { item_type__to_affect: Item, cooldown_in_ticks: Number },
    SetPlayerWeather {},
    SendHover {},
    SetVisualShoulderParrot {},
    NoProjColl {},
    ShowDisguise {},
    SetRemainingAir { breath_ticks: Number },
    DisplayPickUpAnimation {
        entity_uuid: Either<Text, MiniMessage>,
        collector_uuid: Either<Text, MiniMessage>,
    },
    DisplayAnimatedParticleCuboid {
        effect: Particle,
        corner_1: Location,
        corner_2: Location,
        effect_spacing: Option<Number>,
        animation_duration: Option<Number>,
    },
    SetChatColor { new_chat_color: Option<MiniMessage> },
    SetWorldBorder {
        center_position: Location,
        radius_in_blocks: Number,
        warning_distance: Option<Number>,
    },
    SetPlayerTime { daylight_ticks: Option<Number> },
    GiveFood { food_to_give: Number },
    NatRegen {},
    GivePotionEffect { effects: Vec<Potion> },
    PlaySound { sound_to_play: Vec<Sound>, playback_location: Option<Location> },
    SetCompassTarget { new_target: Location },
    RemoveItems { items_to_remove: Vec<Item> },
    BoostElytra { firework: Item },
    RandomizedTeleport { locations_to: Vec<Location> },
    SaveCurrentInventory {},
    OpenBook { book_item: Item },
    SetCurrentHealth { current_health: Number },
    DisguiseasMob { mob_to_disguise_as: SpawnEgg, display_name: Option<MiniMessage> },
    DisguiseasBlock { block_to_disguise_as: Block, display_name: Option<MiniMessage> },
    RollBackBlockChanges { rollback_time: Option<Number> },
    NoDeathDrops {},
    AllowPlacingBreakingBlocks { blocks_to_allow: Vec<Block> },
    SetWalkSpeed { of_normal: Number },
    OpenContainerInventory { container_location: Location },
    SetCollidable {},
    LaunchForward { launch_power: Number },
    DisplayAnimatedParticleCircle {
        effect: Particle,
        center_location: Location,
        diameter: Option<Number>,
        animation_duration: Option<Number>,
    },
    RemoveBossBarN {},
    SetFallDistance { fall_distance_blocks: Number },
    SettoCreativeMode {},
    SetEquipmentItem { item_to_set: Option<Item> },
    SendPlayerAttackAnimation {},
    GiveRngItem {},
    SetDeathDropsEnabled {},
    RemovePotionEffect { effects: Vec<Potion> },
    DisplayHologram { display_location: Location, text_to_display: Option<MiniMessage> },
    DisplayBlockFracture { blocks_to: Vec<Location>, fracture_level: Option<Number> },
    SetEntityHidden { entity_uuid: Either<Text, MiniMessage> },
    SetSidebarVisible {},
    AllowDrops {},
    DeathDrops {},
    DisplayVibrationEffect {
        origin_location: Location,
        target_location: Location,
        arrival_time: Option<Number>,
    },
    ShowInventoryMenu { items_to_display: Vec<Item> },
    SetHotbarSlot { new_slot: Number },
    DisplayParticleRay {
        effect: Particle,
        ray_location: Location,
        ray_vector: Vector,
        effect_spacing: Option<Number>,
    },
    SetFoodLevel { food_level: Number },
    DisguiseasPlayer {
        player_name_to_disguise_as: MiniMessage,
        display_skin: Option<Item>,
    },
    DisplayParticleCuboid {
        effect: Particle,
        corner_1: Location,
        corner_2: Location,
        effect_spacing: Option<Number>,
    },
    SendMessageSequence {
        messages_to_send: Vec<MiniMessage>,
        message_delay_ticks: Option<Number>,
    },
    SetNamePrefixSuffix { prefixsuffix_text: Option<MiniMessage> },
    SetSaturationLevel { saturation_level: Number },
    WeatherClear {},
    SetRainLevel { rain_level_: Number, storm_level_: Number },
    Undisguise {},
    DisplayAnimatedParticleSpiral {
        effect: Particle,
        base_location: Location,
        length: Option<Number>,
        diameter: Option<Number>,
        particle_count: Option<Number>,
        rotations: Option<Number>,
        animation_duration: Option<Number>,
    },
    ShowTitleText {
        title_text: MiniMessage,
        subtitle_text: Option<MiniMessage>,
        title_duration: Option<Number>,
        fade_in_length: Option<Number>,
        fade_out_length: Option<Number>,
    },
    SetInstantRespawn {},
    SetScoreboardScore { score_name: MiniMessage, score_value: Option<Number> },
    StopSounds { sounds_to_stop: Vec<Sound> },
    SetNameColor {},
    DisplayParticleLine {
        effect: Particle,
        start_location: Location,
        end_location: Location,
        effect_spacing: Option<Number>,
    },
}
impl PlayerAction {
    pub fn compile(&self) -> Value {
        match self {
            PlayerAction::SetHotbarItems { items_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![items_to_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHotbar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetReducedDebugInfoEnabled {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetReducedDebug".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::CloseInventory {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CloseInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GiveItems { items_to_give, amount_to_give } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![items_to_give.json(), amount_to_give.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GiveItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::NoKeepInv {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoKeepInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetAllowHandCrafting {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHandCrafting".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::BossBar {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BossBar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayParticleSphere {
                effect,
                center_location,
                diameter,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![effect.json(), center_location.json(), diameter.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleSphere".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetBaseAttackSpeed { attack_speed } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![attack_speed.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAtkSpeed".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetVelocity { new_velocity } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_velocity.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetVelocity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayParticleEffect { effect, effect_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![effect.json(), effect_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Particle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::AddInventoryMenuRow { items_to_display } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![items_to_display.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AddInvRow".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisablePvp {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisablePvp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::NoNatRegen {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoNatRegen".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayLightningBolt { strike_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![strike_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayLightning".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::PlaySoundfromEntity { sound_to_play, target_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![sound_to_play.json(), target_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlayEntitySound".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ReplaceProj {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ReplaceProj".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::Damage { damage_to_inflict, uuid_of_damager_entity } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![damage_to_inflict.json(), uuid_of_damager_entity.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Damage".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetExperience { experience_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![experience_to_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetExp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendAnimation {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SendAnimation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetXPProgress { progress__0100 } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![progress__0100.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetXPProg".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetInventoryItems { items_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![items_to_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetInventory".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::TeleportSequence { locations_to, teleport_delay_ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![locations_to.json(), teleport_delay_ticks.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TpSequence".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::Heal { amount_to_heal } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![amount_to_heal.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Heal".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetSpawnPoint { the_new_spawn_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![the_new_spawn_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSpawnPoint".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayParticleSpiral {
                effect,
                base_location,
                length,
                diameter,
                effect_count,
                rotations,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        effect.json(), base_location.json(), length.json(), diameter
                        .json(), effect_count.json(), rotations.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleSpiral".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetInventoryKept {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetInventoryKept".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetAllowFlight {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAllowFlight".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LaunchUp { launch_power } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![launch_power.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LaunchUp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetMaximumHealth { maximum_health } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![maximum_health.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMaxHealth".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RemoveBossBar { boss_bar_position } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![boss_bar_position.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" RemoveBossBar ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetFogDistance { fog_distance_in_chunks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![fog_distance_in_chunks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFogDistance".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GetTargetEntity {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetTargetEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SettoAdventureMode {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AdventureMode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ForceFlightMode {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ForceFlight".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LoadSavedInventory {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LoadInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SettoSpectatorMode {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpectatorMode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ClearPotionEffects {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearPotions".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::Kick {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Kick".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetPlayerListInfo { headerfooter_text } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![headerfooter_text.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetTabListInfo".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::EnablePvp {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EnablePvp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ProjColl {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ProjColl".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::HideDisguise {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HideDisguise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SpectateTarget { target_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![target_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpectateTarget".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendHurtAnimation { damage_source } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![damage_source.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HurtAnimation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SettoSurvivalMode {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SurvivalMode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetBossBar {
                title,
                current_health,
                maximum_health,
                boss_bar_position,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        title.json(), current_health.json(), maximum_health.json(),
                        boss_bar_position.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" SetBossBar ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetSkin { player_head } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![player_head.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSkin".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetSpectatorCollision {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SpectatorCollision".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetStatus { game_status } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![game_status.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetStatus".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetNameTagVisible {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetNameVisible".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetInvulnerabilityTicks { ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetInvulTicks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetCursorItem { item_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![item_to_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCursorItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetAbsorptionHealth { absorption_health } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![absorption_health.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAbsorption".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetFireTicks { ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFireTicks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetGamemode {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGamemode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RemoveInventoryMenuRow { rows_to_remove } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![rows_to_remove.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveInvRow".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::EnableFlight {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EnableFlight".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendWakeUpAnimation {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WakeUpAnimation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetBeeStingsStuck { sting_count } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![sting_count.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetStingsStuck".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisallowPlacingBreakingBlocks { blocks_to_disallow } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![blocks_to_disallow.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisableBlocks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetScoreboardObjectiveName { objective_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![objective_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetScoreObj".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LSetHealth {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("L SetHealth".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ParticleEffect {} => {
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
            PlayerAction::ClearInventory {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetFreezeTicks { ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFreezeTicks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetGliding {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGliding".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetRotation { pitch_90_to_90, yaw_180_to_180 } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![pitch_90_to_90.json(), yaw_180_to_180.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetRotation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RemoveScoreboardScore { score_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![score_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveScore".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisallowDrops {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisallowDrops".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayParticleCircle {
                effect,
                center_location,
                diameter,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![effect.json(), center_location.json(), diameter.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleCircle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ClearItems { items_to_clear } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![items_to_clear.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayBlock {
                block_to_display,
                block_location,
                end_of_region,
                block_data,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        block_to_display.json(), block_location.json(), end_of_region
                        .json(), block_data.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RideEntity { target_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![target_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RideEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::WeatherRain {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WeatherRain".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetFlying {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFlying".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RemoveWorldBorder {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RmWorldBorder".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendResourcePack { resource_pack_url } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![resource_pack_url.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ResourcePack".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayBlockOpenedState { block_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![block_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayBlockOpen".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetHandItem {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHandItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendAdvancement { advancement_name, advancement_icon } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![advancement_name.json(), advancement_icon.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SendAdvancement".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::Teleport { new_position } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_position.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Teleport".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ClearChat {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearChat".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetPvPAllowed {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAllowPVP".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetInventoryMenuItem { slot, item_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![slot.json(), item_to_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMenuItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LaunchTowardLocation { launch_destination, launch_power } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![launch_destination.json(), launch_power.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LaunchToward".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetArmorItems { armor_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![armor_to_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetArmor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayGatewayBeam { gateway_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![gateway_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayGateway".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisableFlight {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisableFlight".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GiveSaturation { saturation_to_give } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![saturation_to_give.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GiveSaturation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetVisualFire {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetVisualFire".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetOwnDisguiseVisibility {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDisguiseVisible".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetArrowsStuck { arrow_count } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![arrow_count.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetArrowsStuck".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GiveExperience { experience_to_give } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![experience_to_give.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GiveExp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::FaceLocation { location_to_face } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![location_to_face.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FaceLocation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ClearScoreboard {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearScoreboard".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetItems {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::KeepInv {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("KeepInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ReplaceItems {
                items_to_replace,
                item_to_replace_with,
                amount_of_items_to,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        items_to_replace.json(), item_to_replace_with.json(),
                        amount_of_items_to.json()
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
            PlayerAction::ShowActionBarText { message_to_send } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![message_to_send.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ActionBar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetChatTag { chat_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![chat_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetChatTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendMessage { message_to_send } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![message_to_send.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SendMessage".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ShiftWorldBorder { new_radius, blocks_per_second } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![new_radius.json(), blocks_per_second.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftWorldBorder".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetIteminSlot { item_to_set, slot_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![item_to_set.json(), slot_to_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSlotItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::PlaySoundSequence {
                sounds_to_play,
                sound_delay_ticks,
                playback_location,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        sounds_to_play.json(), sound_delay_ticks.json(),
                        playback_location.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlaySoundSeq".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayAnimatedParticleLine {
                effect,
                start_location,
                end_location,
                effect_spacing,
                animation_duration,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        effect.json(), start_location.json(), end_location.json(),
                        effect_spacing.json(), animation_duration.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleLineA".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplaySignText { sign_location, text_lines } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![sign_location.json(), text_lines.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplaySignText".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::Respawn {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Respawn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetMovementSpeed { movement_speed } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![movement_speed.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSpeed".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ExpandInventoryMenu { items_to_display } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![items_to_display.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ExpandInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetInventoryMenuName { inventory_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![inventory_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetInvName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LaunchProjectile {
                projectile_to,
                launch_point,
                projectile_name,
                speed,
                inaccuracy,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        projectile_to.json(), launch_point.json(), projectile_name
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
            PlayerAction::SetItemCooldown {
                item_type__to_affect,
                cooldown_in_ticks,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![item_type__to_affect.json(), cooldown_in_ticks.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemCooldown".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetPlayerWeather {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPlayerWeather".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendHover {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SendHover".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetVisualShoulderParrot {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetShoulder".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::NoProjColl {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoProjColl".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ShowDisguise {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShowDisguise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetRemainingAir { breath_ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![breath_ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAirTicks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayPickUpAnimation { entity_uuid, collector_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![entity_uuid.json(), collector_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayPickup".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayAnimatedParticleCuboid {
                effect,
                corner_1,
                corner_2,
                effect_spacing,
                animation_duration,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        effect.json(), corner_1.json(), corner_2.json(), effect_spacing
                        .json(), animation_duration.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleCuboidA".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetChatColor { new_chat_color } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_chat_color.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ChatColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetWorldBorder {
                center_position,
                radius_in_blocks,
                warning_distance,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        center_position.json(), radius_in_blocks.json(), warning_distance
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetWorldBorder".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetPlayerTime { daylight_ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![daylight_ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPlayerTime".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GiveFood { food_to_give } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![food_to_give.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GiveFood".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::NatRegen {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NatRegen".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GivePotionEffect { effects } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![effects.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GivePotion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::PlaySound { sound_to_play, playback_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![sound_to_play.json(), playback_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlaySound".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetCompassTarget { new_target } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_target.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCompass".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RemoveItems { items_to_remove } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![items_to_remove.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::BoostElytra { firework } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![firework.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BoostElytra".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RandomizedTeleport { locations_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![locations_to.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RngTeleport".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SaveCurrentInventory {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SaveInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::OpenBook { book_item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![book_item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("OpenBook".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetCurrentHealth { current_health } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![current_health.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHealth".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisguiseasMob { mob_to_disguise_as, display_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![mob_to_disguise_as.json(), display_name.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MobDisguise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisguiseasBlock { block_to_disguise_as, display_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![block_to_disguise_as.json(), display_name.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BlockDisguise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RollBackBlockChanges { rollback_time } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![rollback_time.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RollbackBlocks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::NoDeathDrops {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoDeathDrops".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::AllowPlacingBreakingBlocks { blocks_to_allow } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![blocks_to_allow.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EnableBlocks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetWalkSpeed { of_normal } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![of_normal.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WalkSpeed".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::OpenContainerInventory { container_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![container_location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("OpenBlockInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetCollidable {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCollidable".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LaunchForward { launch_power } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![launch_power.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LaunchFwd".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayAnimatedParticleCircle {
                effect,
                center_location,
                diameter,
                animation_duration,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        effect.json(), center_location.json(), diameter.json(),
                        animation_duration.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleCircleA".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RemoveBossBarN {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveBossBar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetFallDistance { fall_distance_blocks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![fall_distance_blocks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFallDistance".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SettoCreativeMode {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CreativeMode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetEquipmentItem { item_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![item_to_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEquipment".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendPlayerAttackAnimation {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AttackAnimation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GiveRngItem {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GiveRngItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetDeathDropsEnabled {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDropsEnabled".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RemovePotionEffect { effects } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![effects.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemovePotion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayHologram { display_location, text_to_display } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![display_location.json(), text_to_display.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayHologram".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayBlockFracture { blocks_to, fracture_level } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![blocks_to.json(), fracture_level.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayFracture".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetEntityHidden { entity_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![entity_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEntityHidden".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetSidebarVisible {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSidebar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::AllowDrops {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AllowDrops".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DeathDrops {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DeathDrops".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayVibrationEffect {
                origin_location,
                target_location,
                arrival_time,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        origin_location.json(), target_location.json(), arrival_time
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Vibration".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ShowInventoryMenu { items_to_display } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![items_to_display.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShowInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetHotbarSlot { new_slot } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_slot.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSlot".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayParticleRay {
                effect,
                ray_location,
                ray_vector,
                effect_spacing,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        effect.json(), ray_location.json(), ray_vector.json(),
                        effect_spacing.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleRay".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetFoodLevel { food_level } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![food_level.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFoodLevel".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisguiseasPlayer {
                player_name_to_disguise_as,
                display_skin,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![player_name_to_disguise_as.json(), display_skin.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlayerDisguise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayParticleCuboid {
                effect,
                corner_1,
                corner_2,
                effect_spacing,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        effect.json(), corner_1.json(), corner_2.json(), effect_spacing
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleCuboid".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendMessageSequence {
                messages_to_send,
                message_delay_ticks,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![messages_to_send.json(), message_delay_ticks.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SendMessageSeq".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetNamePrefixSuffix { prefixsuffix_text } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![prefixsuffix_text.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetNamePrefix".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetSaturationLevel { saturation_level } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![saturation_level.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSaturation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::WeatherClear {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WeatherClear".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetRainLevel { rain_level_, storm_level_ } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![rain_level_.json(), storm_level_.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetRainLevel".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::Undisguise {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Undisguise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayAnimatedParticleSpiral {
                effect,
                base_location,
                length,
                diameter,
                particle_count,
                rotations,
                animation_duration,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        effect.json(), base_location.json(), length.json(), diameter
                        .json(), particle_count.json(), rotations.json(),
                        animation_duration.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParticleSpiralA".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ShowTitleText {
                title_text,
                subtitle_text,
                title_duration,
                fade_in_length,
                fade_out_length,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        title_text.json(), subtitle_text.json(), title_duration.json(),
                        fade_in_length.json(), fade_out_length.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SendTitle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetInstantRespawn {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InstantRespawn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetScoreboardScore { score_name, score_value } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![score_name.json(), score_value.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetScore".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::StopSounds { sounds_to_stop } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![sounds_to_stop.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StopSound".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetNameColor {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetNameColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayParticleLine {
                effect,
                start_location,
                end_location,
                effect_spacing,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        effect.json(), start_location.json(), end_location.json(),
                        effect_spacing.json()
                    ],
                );
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
