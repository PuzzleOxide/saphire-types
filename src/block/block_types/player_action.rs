use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum PlayerAction {
    SetHotbarItems { items_to_set: Vec<Item> },
    SetReducedDebugInfoEnabled {
        reduced_debug_info_enabled_tag: ReducedDebugInfoEnabledSetReducedDebugInfoEnabled,
    },
    CloseInventory {},
    GiveItems { items_to_give: Vec<Item>, amount_to_give: Option<Number> },
    NoKeepInv {},
    SetAllowHandCrafting {
        allow_hand_crafting_tag: AllowHandCraftingSetAllowHandCrafting,
    },
    BossBar {
        bar_slot_tag: BarSlotBossBar,
        bar_style_tag: BarStyleBossBar,
        sky_effect_tag: SkyEffectBossBar,
        bar_color_tag: BarColorBossBar,
    },
    DisplayParticleSphere {
        effect: Particle,
        center_location: Location,
        diameter: Option<Number>,
    },
    SetBaseAttackSpeed { attack_speed: Option<Number> },
    SetVelocity {
        new_velocity: Vector,
        add_to_current_velocity_tag: AddtoCurrentVelocitySetVelocity,
    },
    DisplayParticleEffect { effect: Vec<Particle>, effect_location: Location },
    AddInventoryMenuRow {
        items_to_display: Vec<Item>,
        new_row_position_tag: NewRowPositionAddInventoryMenuRow,
    },
    DisablePvp {},
    NoNatRegen {},
    DisplayLightningBolt { strike_location: Location },
    PlaySoundfromEntity {
        sound_to_play: Vec<Sound>,
        target_uuid: Vec<Either<Text, MiniMessage>>,
        sound_source_tag: SoundSourcePlaySoundfromEntity,
    },
    ReplaceProj {},
    Damage {
        damage_to_inflict: Number,
        uuid_of_damager_entity: Option<Either<Text, MiniMessage>>,
    },
    SetExperience {
        experience_to_set: Number,
        set_experience_tag: SetExperienceSetExperience,
    },
    SendAnimation { animation_type__tag: AnimationTypeSendAnimation },
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
    SetInventoryKept { inventory_kept_tag: InventoryKeptSetInventoryKept },
    SetAllowFlight { allow_flight_tag: AllowFlightSetAllowFlight },
    LaunchUp {
        launch_power: Number,
        add_to_current_velocity_tag: AddtoCurrentVelocityLaunchUp,
    },
    SetMaximumHealth {
        maximum_health: Number,
        heal_player_to_max_health_tag: HealPlayertoMaxHealthSetMaximumHealth,
    },
    RemoveBossBar { boss_bar_position: Option<Number> },
    SetFogDistance { fog_distance_in_chunks: Option<Number> },
    GetTargetEntity { ignore_blocks_tag: IgnoreBlocksGetTargetEntity },
    SettoAdventureMode {},
    ForceFlightMode { flight_mode_tag: FlightModeForceFlightMode },
    LoadSavedInventory {},
    SettoSpectatorMode {},
    ClearPotionEffects {},
    Kick {},
    SetPlayerListInfo {
        headerfooter_text: Vec<MiniMessage>,
        player_list_field_tag: PlayerListFieldSetPlayerListInfo,
        text_value_merging_tag: TextValueMergingSetPlayerListInfo,
        inherit_styles_tag: InheritStylesSetPlayerListInfo,
    },
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
        bar_style_tag: BarStyleSetBossBar,
        sky_effect_tag: SkyEffectSetBossBar,
        bar_color_tag: BarColorSetBossBar,
    },
    SetSkin { player_head: Option<Item> },
    SetSpectatorCollision {
        spectator_collision_tag: SpectatorCollisionSetSpectatorCollision,
    },
    SetStatus { game_status: Option<MiniMessage> },
    SetNameTagVisible { name_tag_visible_tag: NameTagVisibleSetNameTagVisible },
    SetInvulnerabilityTicks { ticks: Number },
    SetCursorItem { item_to_set: Option<Item> },
    SetAbsorptionHealth { absorption_health: Number },
    SetFireTicks { ticks: Number },
    SetGamemode {
        flight_mode_tag: FlightModeSetGamemode,
        gamemode_tag: GamemodeSetGamemode,
    },
    RemoveInventoryMenuRow {
        rows_to_remove: Option<Number>,
        row_to_remove_tag: RowtoRemoveRemoveInventoryMenuRow,
    },
    EnableFlight {},
    SendWakeUpAnimation {},
    SetBeeStingsStuck { sting_count: Option<Number> },
    DisallowPlacingBreakingBlocks { blocks_to_disallow: Vec<Block> },
    SetScoreboardObjectiveName { objective_name: MiniMessage },
    LSetHealth { heal_type__tag: HealTypeLSetHealth },
    ParticleEffect {},
    ClearInventory {
        clear_mode_tag: ClearModeClearInventory,
        clear_crafting_and_cursor_tag: ClearCraftingandCursorClearInventory,
    },
    SetFreezeTicks { ticks: Number, ticking_locked_tag: TickingLockedSetFreezeTicks },
    SetGliding { gliding_tag: GlidingSetGliding },
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
    SetFlying { flying_tag: FlyingSetFlying },
    RemoveWorldBorder {},
    SendResourcePack { resource_pack_url: Text },
    DisplayBlockOpenedState {
        block_location: Location,
        container_state_tag: ContainerStateDisplayBlockOpenedState,
    },
    SetHandItem { hand_slot_tag: HandSlotSetHandItem },
    SendAdvancement {
        advancement_name: MiniMessage,
        advancement_icon: Item,
        toast_type__tag: ToastTypeSendAdvancement,
    },
    Teleport {
        new_position: Location,
        keep_current_rotation_tag: KeepCurrentRotationTeleport,
        keep_velocity_tag: KeepVelocityTeleport,
    },
    ClearChat {},
    SetPvPAllowed { pvp_tag: PVPSetPvPAllowed },
    SetInventoryMenuItem { slot: Number, item_to_set: Option<Item> },
    LaunchTowardLocation {
        launch_destination: Location,
        launch_power: Option<Number>,
        add_to_current_velocity_tag: AddtoCurrentVelocityLaunchTowardLocation,
        ignore_distance_tag: IgnoreDistanceLaunchTowardLocation,
    },
    SetArmorItems { armor_to_set: Vec<Item> },
    DisplayGatewayBeam {
        gateway_location: Location,
        animation_type__tag: AnimationTypeDisplayGatewayBeam,
    },
    DisableFlight {},
    GiveSaturation { saturation_to_give: Number },
    SetVisualFire { on_fire_tag: OnFireSetVisualFire },
    SetOwnDisguiseVisibility {
        disguise_visible_tag: DisguiseVisibleSetOwnDisguiseVisibility,
    },
    SetArrowsStuck { arrow_count: Option<Number> },
    GiveExperience {
        experience_to_give: Number,
        give_experience_tag: GiveExperienceGiveExperience,
    },
    FaceLocation { location_to_face: Location },
    ClearScoreboard {},
    SetItems {},
    KeepInv {},
    ReplaceItems {
        items_to_replace: Vec<Item>,
        item_to_replace_with: Item,
        amount_of_items_to: Option<Number>,
    },
    ShowActionBarText {
        message_to_send: Vec<MiniMessage>,
        text_value_merging_tag: TextValueMergingShowActionBarText,
        inherit_styles_tag: InheritStylesShowActionBarText,
    },
    SetChatTag { chat_tag: Vec<MiniMessage> },
    SendMessage {
        message_to_send: Vec<MiniMessage>,
        alignment_mode_tag: AlignmentModeSendMessage,
        text_value_merging_tag: TextValueMergingSendMessage,
        inherit_styles_tag: InheritStylesSendMessage,
    },
    ShiftWorldBorder { new_radius: Number, blocks_per_second: Option<Number> },
    SetIteminSlot { item_to_set: Option<Item>, slot_to_set: Number },
    PlaySoundSequence {
        sounds_to_play: Vec<Sound>,
        sound_delay_ticks: Option<Number>,
        playback_location: Option<Location>,
        sound_source_tag: SoundSourcePlaySoundSequence,
    },
    DisplayAnimatedParticleLine {
        effect: Particle,
        start_location: Location,
        end_location: Location,
        effect_spacing: Option<Number>,
        animation_duration: Option<Number>,
    },
    DisplaySignText {
        sign_location: Location,
        text_lines: Vec<MiniMessage>,
        sign_side_tag: SignSideDisplaySignText,
        text_color_tag: TextColorDisplaySignText,
        glowing_tag: GlowingDisplaySignText,
    },
    Respawn {},
    SetMovementSpeed {
        movement_speed: Number,
        speed_type__tag: SpeedTypeSetMovementSpeed,
    },
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
    SetPlayerWeather { weather_tag: WeatherSetPlayerWeather },
    SendHover {},
    SetVisualShoulderParrot {
        shoulder_tag: ShoulderSetVisualShoulderParrot,
        type__tag: TypeSetVisualShoulderParrot,
    },
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
        fill_type__tag: FillTypeDisplayAnimatedParticleCuboid,
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
    GivePotionEffect {
        effects: Vec<Potion>,
        show_icon_tag: ShowIconGivePotionEffect,
        overwrite_effect_tag: OverwriteEffectGivePotionEffect,
        effect_particles_tag: EffectParticlesGivePotionEffect,
    },
    PlaySound {
        sound_to_play: Vec<Sound>,
        playback_location: Option<Location>,
        sound_source_tag: SoundSourcePlaySound,
    },
    SetCompassTarget { new_target: Location },
    RemoveItems { items_to_remove: Vec<Item> },
    BoostElytra { firework: Item },
    RandomizedTeleport {
        locations_to: Vec<Location>,
        keep_current_rotation_tag: KeepCurrentRotationRandomizedTeleport,
    },
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
    SetCollidable { collision_tag: CollisionSetCollidable },
    LaunchForward {
        launch_power: Number,
        add_to_current_velocity_tag: AddtoCurrentVelocityLaunchForward,
        launch_axis_tag: LaunchAxisLaunchForward,
    },
    DisplayAnimatedParticleCircle {
        effect: Particle,
        center_location: Location,
        diameter: Option<Number>,
        animation_duration: Option<Number>,
    },
    RemoveBossBarN { boss_bar_slot_tag: BossBarSlotRemoveBossBarN },
    SetFallDistance { fall_distance_blocks: Number },
    SettoCreativeMode {},
    SetEquipmentItem {
        item_to_set: Option<Item>,
        equipment_slot_tag: EquipmentSlotSetEquipmentItem,
    },
    SendPlayerAttackAnimation {
        animation_arm_tag: AnimationArmSendPlayerAttackAnimation,
    },
    GiveRngItem {},
    SetDeathDropsEnabled { spawn_death_drops_tag: SpawnDeathDropsSetDeathDropsEnabled },
    RemovePotionEffect { effects: Vec<Potion> },
    DisplayHologram { display_location: Location, text_to_display: Option<MiniMessage> },
    DisplayBlockFracture {
        blocks_to: Vec<Location>,
        fracture_level: Option<Number>,
        overwrite_previous_fracture_tag: OverwritePreviousFractureDisplayBlockFracture,
    },
    SetEntityHidden {
        entity_uuid: Either<Text, MiniMessage>,
        hidden_tag: HiddenSetEntityHidden,
    },
    SetSidebarVisible { sidebar_tag: SidebarSetSidebarVisible },
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
        fill_type__tag: FillTypeDisplayParticleCuboid,
    },
    SendMessageSequence {
        messages_to_send: Vec<MiniMessage>,
        message_delay_ticks: Option<Number>,
        alignment_mode_tag: AlignmentModeSendMessageSequence,
    },
    SetNamePrefixSuffix {
        prefixsuffix_text: Option<MiniMessage>,
        text_type__tag: TextTypeSetNamePrefixSuffix,
    },
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
    SetInstantRespawn { instant_respawn_tag: InstantRespawnSetInstantRespawn },
    SetScoreboardScore { score_name: MiniMessage, score_value: Option<Number> },
    StopSounds { sounds_to_stop: Vec<Sound>, sound_source_tag: SoundSourceStopSounds },
    SetNameColor { name_color_tag: NameColorSetNameColor },
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
                let mut item_args = compile(vec![items_to_set.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHotbar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetReducedDebugInfoEnabled {
                reduced_debug_info_enabled_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![reduced_debug_info_enabled_tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![items_to_give.json(), amount_to_give.json()],
                    vec![],
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoKeepInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetAllowHandCrafting { allow_hand_crafting_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![allow_hand_crafting_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHandCrafting".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::BossBar {
                bar_slot_tag,
                bar_style_tag,
                sky_effect_tag,
                bar_color_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![
                        bar_slot_tag.json(), bar_style_tag.json(), sky_effect_tag.json(),
                        bar_color_tag.json()
                    ],
                );
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
                let mut item_args = compile(
                    vec![effect.json(), center_location.json(), diameter.json()],
                    vec![],
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
                let mut item_args = compile(vec![attack_speed.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAtkSpeed".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetVelocity { new_velocity, add_to_current_velocity_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![new_velocity.json()],
                    vec![add_to_current_velocity_tag.json()],
                );
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
                let mut item_args = compile(
                    vec![effect.json(), effect_location.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Particle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::AddInventoryMenuRow {
                items_to_display,
                new_row_position_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![items_to_display.json()],
                    vec![new_row_position_tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![strike_location.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayLightning".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::PlaySoundfromEntity {
                sound_to_play,
                target_uuid,
                sound_source_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![sound_to_play.json(), target_uuid.json()],
                    vec![sound_source_tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![damage_to_inflict.json(), uuid_of_damager_entity.json()],
                    vec![],
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
            PlayerAction::SetExperience { experience_to_set, set_experience_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![experience_to_set.json()],
                    vec![set_experience_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetExp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendAnimation { animation_type__tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![animation_type__tag.json()]);
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
                let mut item_args = compile(vec![progress__0100.json()], vec![]);
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
                let mut item_args = compile(vec![items_to_set.json()], vec![]);
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
                let mut item_args = compile(
                    vec![locations_to.json(), teleport_delay_ticks.json()],
                    vec![],
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
                let mut item_args = compile(vec![amount_to_heal.json()], vec![]);
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
                let mut item_args = compile(vec![the_new_spawn_location.json()], vec![]);
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
                let mut item_args = compile(
                    vec![
                        effect.json(), base_location.json(), length.json(), diameter
                        .json(), effect_count.json(), rotations.json()
                    ],
                    vec![],
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
            PlayerAction::SetInventoryKept { inventory_kept_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![inventory_kept_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetInventoryKept".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetAllowFlight { allow_flight_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![allow_flight_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAllowFlight".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LaunchUp { launch_power, add_to_current_velocity_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![launch_power.json()],
                    vec![add_to_current_velocity_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LaunchUp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetMaximumHealth {
                maximum_health,
                heal_player_to_max_health_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![maximum_health.json()],
                    vec![heal_player_to_max_health_tag.json()],
                );
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
                let mut item_args = compile(vec![boss_bar_position.json()], vec![]);
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
                let mut item_args = compile(vec![fog_distance_in_chunks.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFogDistance".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GetTargetEntity { ignore_blocks_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![ignore_blocks_tag.json()]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AdventureMode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::ForceFlightMode { flight_mode_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![flight_mode_tag.json()]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Kick".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetPlayerListInfo {
                headerfooter_text,
                player_list_field_tag,
                text_value_merging_tag,
                inherit_styles_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![headerfooter_text.json()],
                    vec![
                        player_list_field_tag.json(), text_value_merging_tag.json(),
                        inherit_styles_tag.json()
                    ],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![target_uuid.json()], vec![]);
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
                let mut item_args = compile(vec![damage_source.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                bar_style_tag,
                sky_effect_tag,
                bar_color_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        title.json(), current_health.json(), maximum_health.json(),
                        boss_bar_position.json()
                    ],
                    vec![
                        bar_style_tag.json(), sky_effect_tag.json(), bar_color_tag.json()
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
                let mut item_args = compile(vec![player_head.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSkin".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetSpectatorCollision { spectator_collision_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![spectator_collision_tag.json()],
                );
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
                let mut item_args = compile(vec![game_status.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetStatus".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetNameTagVisible { name_tag_visible_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![name_tag_visible_tag.json()]);
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
                let mut item_args = compile(vec![ticks.json()], vec![]);
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
                let mut item_args = compile(vec![item_to_set.json()], vec![]);
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
                let mut item_args = compile(vec![absorption_health.json()], vec![]);
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
                let mut item_args = compile(vec![ticks.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFireTicks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetGamemode { flight_mode_tag, gamemode_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![flight_mode_tag.json(), gamemode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGamemode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RemoveInventoryMenuRow {
                rows_to_remove,
                row_to_remove_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![rows_to_remove.json()],
                    vec![row_to_remove_tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![sting_count.json()], vec![]);
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
                let mut item_args = compile(vec![blocks_to_disallow.json()], vec![]);
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
                let mut item_args = compile(vec![objective_name.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetScoreObj".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LSetHealth { heal_type__tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![heal_type__tag.json()]);
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
            PlayerAction::ClearInventory {
                clear_mode_tag,
                clear_crafting_and_cursor_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![clear_mode_tag.json(), clear_crafting_and_cursor_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetFreezeTicks { ticks, ticking_locked_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![ticks.json()],
                    vec![ticking_locked_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFreezeTicks".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetGliding { gliding_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![gliding_tag.json()]);
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
                let mut item_args = compile(
                    vec![pitch_90_to_90.json(), yaw_180_to_180.json()],
                    vec![],
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
                let mut item_args = compile(vec![score_name.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![effect.json(), center_location.json(), diameter.json()],
                    vec![],
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
                let mut item_args = compile(vec![items_to_clear.json()], vec![]);
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
                let mut item_args = compile(
                    vec![
                        block_to_display.json(), block_location.json(), end_of_region
                        .json(), block_data.json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![target_uuid.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WeatherRain".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetFlying { flying_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![flying_tag.json()]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![resource_pack_url.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ResourcePack".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayBlockOpenedState {
                block_location,
                container_state_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![block_location.json()],
                    vec![container_state_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayBlockOpen".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetHandItem { hand_slot_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![hand_slot_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHandItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendAdvancement {
                advancement_name,
                advancement_icon,
                toast_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![advancement_name.json(), advancement_icon.json()],
                    vec![toast_type__tag.json()],
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
            PlayerAction::Teleport {
                new_position,
                keep_current_rotation_tag,
                keep_velocity_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![new_position.json()],
                    vec![keep_current_rotation_tag.json(), keep_velocity_tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearChat".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetPvPAllowed { pvp_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![pvp_tag.json()]);
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
                let mut item_args = compile(
                    vec![slot.json(), item_to_set.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMenuItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LaunchTowardLocation {
                launch_destination,
                launch_power,
                add_to_current_velocity_tag,
                ignore_distance_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![launch_destination.json(), launch_power.json()],
                    vec![add_to_current_velocity_tag.json(), ignore_distance_tag.json()],
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
                let mut item_args = compile(vec![armor_to_set.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetArmor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::DisplayGatewayBeam {
                gateway_location,
                animation_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![gateway_location.json()],
                    vec![animation_type__tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![saturation_to_give.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GiveSaturation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetVisualFire { on_fire_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![on_fire_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetVisualFire".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetOwnDisguiseVisibility { disguise_visible_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![disguise_visible_tag.json()]);
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
                let mut item_args = compile(vec![arrow_count.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetArrowsStuck".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GiveExperience { experience_to_give, give_experience_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![experience_to_give.json()],
                    vec![give_experience_tag.json()],
                );
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
                let mut item_args = compile(vec![location_to_face.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        items_to_replace.json(), item_to_replace_with.json(),
                        amount_of_items_to.json()
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
            PlayerAction::ShowActionBarText {
                message_to_send,
                text_value_merging_tag,
                inherit_styles_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![message_to_send.json()],
                    vec![text_value_merging_tag.json(), inherit_styles_tag.json()],
                );
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
                let mut item_args = compile(vec![chat_tag.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetChatTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendMessage {
                message_to_send,
                alignment_mode_tag,
                text_value_merging_tag,
                inherit_styles_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![message_to_send.json()],
                    vec![
                        alignment_mode_tag.json(), text_value_merging_tag.json(),
                        inherit_styles_tag.json()
                    ],
                );
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
                let mut item_args = compile(
                    vec![new_radius.json(), blocks_per_second.json()],
                    vec![],
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
                let mut item_args = compile(
                    vec![item_to_set.json(), slot_to_set.json()],
                    vec![],
                );
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
                sound_source_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        sounds_to_play.json(), sound_delay_ticks.json(),
                        playback_location.json()
                    ],
                    vec![sound_source_tag.json()],
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
                let mut item_args = compile(
                    vec![
                        effect.json(), start_location.json(), end_location.json(),
                        effect_spacing.json(), animation_duration.json()
                    ],
                    vec![],
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
            PlayerAction::DisplaySignText {
                sign_location,
                text_lines,
                sign_side_tag,
                text_color_tag,
                glowing_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![sign_location.json(), text_lines.json()],
                    vec![sign_side_tag.json(), text_color_tag.json(), glowing_tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Respawn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetMovementSpeed { movement_speed, speed_type__tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![movement_speed.json()],
                    vec![speed_type__tag.json()],
                );
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
                let mut item_args = compile(vec![items_to_display.json()], vec![]);
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
                let mut item_args = compile(vec![inventory_name.json()], vec![]);
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
                let mut item_args = compile(
                    vec![
                        projectile_to.json(), launch_point.json(), projectile_name
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
            PlayerAction::SetItemCooldown {
                item_type__to_affect,
                cooldown_in_ticks,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![item_type__to_affect.json(), cooldown_in_ticks.json()],
                    vec![],
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
            PlayerAction::SetPlayerWeather { weather_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![weather_tag.json()]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SendHover".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetVisualShoulderParrot { shoulder_tag, type__tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![shoulder_tag.json(), type__tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![breath_ticks.json()], vec![]);
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
                let mut item_args = compile(
                    vec![entity_uuid.json(), collector_uuid.json()],
                    vec![],
                );
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
                fill_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        effect.json(), corner_1.json(), corner_2.json(), effect_spacing
                        .json(), animation_duration.json()
                    ],
                    vec![fill_type__tag.json()],
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
                let mut item_args = compile(vec![new_chat_color.json()], vec![]);
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
                let mut item_args = compile(
                    vec![
                        center_position.json(), radius_in_blocks.json(), warning_distance
                        .json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![daylight_ticks.json()], vec![]);
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
                let mut item_args = compile(vec![food_to_give.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NatRegen".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::GivePotionEffect {
                effects,
                show_icon_tag,
                overwrite_effect_tag,
                effect_particles_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![effects.json()],
                    vec![
                        show_icon_tag.json(), overwrite_effect_tag.json(),
                        effect_particles_tag.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GivePotion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::PlaySound {
                sound_to_play,
                playback_location,
                sound_source_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![sound_to_play.json(), playback_location.json()],
                    vec![sound_source_tag.json()],
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
                let mut item_args = compile(vec![new_target.json()], vec![]);
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
                let mut item_args = compile(vec![items_to_remove.json()], vec![]);
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
                let mut item_args = compile(vec![firework.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BoostElytra".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::RandomizedTeleport {
                locations_to,
                keep_current_rotation_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![locations_to.json()],
                    vec![keep_current_rotation_tag.json()],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![book_item.json()], vec![]);
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
                let mut item_args = compile(vec![current_health.json()], vec![]);
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
                let mut item_args = compile(
                    vec![mob_to_disguise_as.json(), display_name.json()],
                    vec![],
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
                let mut item_args = compile(
                    vec![block_to_disguise_as.json(), display_name.json()],
                    vec![],
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
                let mut item_args = compile(vec![rollback_time.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![blocks_to_allow.json()], vec![]);
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
                let mut item_args = compile(vec![of_normal.json()], vec![]);
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
                let mut item_args = compile(vec![container_location.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("OpenBlockInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetCollidable { collision_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![collision_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCollidable".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::LaunchForward {
                launch_power,
                add_to_current_velocity_tag,
                launch_axis_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![launch_power.json()],
                    vec![add_to_current_velocity_tag.json(), launch_axis_tag.json()],
                );
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
                let mut item_args = compile(
                    vec![
                        effect.json(), center_location.json(), diameter.json(),
                        animation_duration.json()
                    ],
                    vec![],
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
            PlayerAction::RemoveBossBarN { boss_bar_slot_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![boss_bar_slot_tag.json()]);
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
                let mut item_args = compile(vec![fall_distance_blocks.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CreativeMode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetEquipmentItem { item_to_set, equipment_slot_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![item_to_set.json()],
                    vec![equipment_slot_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEquipment".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SendPlayerAttackAnimation { animation_arm_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![animation_arm_tag.json()]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GiveRngItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetDeathDropsEnabled { spawn_death_drops_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![spawn_death_drops_tag.json()]);
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
                let mut item_args = compile(vec![effects.json()], vec![]);
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
                let mut item_args = compile(
                    vec![display_location.json(), text_to_display.json()],
                    vec![],
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
            PlayerAction::DisplayBlockFracture {
                blocks_to,
                fracture_level,
                overwrite_previous_fracture_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![blocks_to.json(), fracture_level.json()],
                    vec![overwrite_previous_fracture_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayFracture".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetEntityHidden { entity_uuid, hidden_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![entity_uuid.json()],
                    vec![hidden_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEntityHidden".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetSidebarVisible { sidebar_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![sidebar_tag.json()]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        origin_location.json(), target_location.json(), arrival_time
                        .json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![items_to_display.json()], vec![]);
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
                let mut item_args = compile(vec![new_slot.json()], vec![]);
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
                let mut item_args = compile(
                    vec![
                        effect.json(), ray_location.json(), ray_vector.json(),
                        effect_spacing.json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![food_level.json()], vec![]);
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
                let mut item_args = compile(
                    vec![player_name_to_disguise_as.json(), display_skin.json()],
                    vec![],
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
                fill_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        effect.json(), corner_1.json(), corner_2.json(), effect_spacing
                        .json()
                    ],
                    vec![fill_type__tag.json()],
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
                alignment_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![messages_to_send.json(), message_delay_ticks.json()],
                    vec![alignment_mode_tag.json()],
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
            PlayerAction::SetNamePrefixSuffix { prefixsuffix_text, text_type__tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![prefixsuffix_text.json()],
                    vec![text_type__tag.json()],
                );
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
                let mut item_args = compile(vec![saturation_level.json()], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![rain_level_.json(), storm_level_.json()],
                    vec![],
                );
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        effect.json(), base_location.json(), length.json(), diameter
                        .json(), particle_count.json(), rotations.json(),
                        animation_duration.json()
                    ],
                    vec![],
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
                let mut item_args = compile(
                    vec![
                        title_text.json(), subtitle_text.json(), title_duration.json(),
                        fade_in_length.json(), fade_out_length.json()
                    ],
                    vec![],
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
            PlayerAction::SetInstantRespawn { instant_respawn_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![instant_respawn_tag.json()]);
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
                let mut item_args = compile(
                    vec![score_name.json(), score_value.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetScore".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::StopSounds { sounds_to_stop, sound_source_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![sounds_to_stop.json()],
                    vec![sound_source_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StopSound".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerAction::SetNameColor { name_color_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![name_color_tag.json()]);
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
                let mut item_args = compile(
                    vec![
                        effect.json(), start_location.json(), end_location.json(),
                        effect_spacing.json()
                    ],
                    vec![],
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
#[derive(Debug, Clone)]
pub enum ReducedDebugInfoEnabledSetReducedDebugInfoEnabled {
    True,
    False,
}
impl ReducedDebugInfoEnabledSetReducedDebugInfoEnabled {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ReducedDebugInfoEnabledSetReducedDebugInfoEnabled::True => {
                    Value::String("True".to_string())
                }
                ReducedDebugInfoEnabledSetReducedDebugInfoEnabled::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Reduced Debug Info Enabled".to_string()),
        );
        data.insert("action".to_string(), Value::String("SetReducedDebug".to_string()));
        data.insert("block".to_string(), Value::String("SetReducedDebug".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ReducedDebugInfoEnabledSetReducedDebugInfoEnabled {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum AllowHandCraftingSetAllowHandCrafting {
    Enable,
    Disable,
}
impl AllowHandCraftingSetAllowHandCrafting {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AllowHandCraftingSetAllowHandCrafting::Enable => {
                    Value::String("Enable".to_string())
                }
                AllowHandCraftingSetAllowHandCrafting::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Allow Hand Crafting".to_string()));
        data.insert("action".to_string(), Value::String("SetHandCrafting".to_string()));
        data.insert("block".to_string(), Value::String("SetHandCrafting".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AllowHandCraftingSetAllowHandCrafting {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum BarSlotBossBar {
    SlotOne,
    SlotTwo,
    SlotThree,
    SlotFour,
    SlotFive,
    SlotSix,
    SlotSeven,
    SlotEight,
    SlotNine,
}
impl BarSlotBossBar {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BarSlotBossBar::SlotOne => Value::String("Slot 1".to_string()),
                BarSlotBossBar::SlotTwo => Value::String("Slot 2".to_string()),
                BarSlotBossBar::SlotThree => Value::String("Slot 3".to_string()),
                BarSlotBossBar::SlotFour => Value::String("Slot 4".to_string()),
                BarSlotBossBar::SlotFive => Value::String("Slot 5".to_string()),
                BarSlotBossBar::SlotSix => Value::String("Slot 6".to_string()),
                BarSlotBossBar::SlotSeven => Value::String("Slot 7".to_string()),
                BarSlotBossBar::SlotEight => Value::String("Slot 8".to_string()),
                BarSlotBossBar::SlotNine => Value::String("Slot 9".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Bar Slot".to_string()));
        data.insert("action".to_string(), Value::String("BossBar".to_string()));
        data.insert("block".to_string(), Value::String("BossBar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BarSlotBossBar {
    fn default() -> Self {
        Self::SlotOne
    }
}
#[derive(Debug, Clone)]
pub enum BarStyleBossBar {
    Solid,
    Sixsegments,
    OneZerosegments,
    OneTwosegments,
    TwoZerosegments,
}
impl BarStyleBossBar {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BarStyleBossBar::Solid => Value::String("Solid".to_string()),
                BarStyleBossBar::Sixsegments => Value::String("6 segments".to_string()),
                BarStyleBossBar::OneZerosegments => {
                    Value::String("10 segments".to_string())
                }
                BarStyleBossBar::OneTwosegments => {
                    Value::String("12 segments".to_string())
                }
                BarStyleBossBar::TwoZerosegments => {
                    Value::String("20 segments".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Bar Style".to_string()));
        data.insert("action".to_string(), Value::String("BossBar".to_string()));
        data.insert("block".to_string(), Value::String("BossBar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BarStyleBossBar {
    fn default() -> Self {
        Self::Solid
    }
}
#[derive(Debug, Clone)]
pub enum SkyEffectBossBar {
    None,
    Createfog,
    Darkensky,
    Both,
}
impl SkyEffectBossBar {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SkyEffectBossBar::None => Value::String("None".to_string()),
                SkyEffectBossBar::Createfog => Value::String("Create fog".to_string()),
                SkyEffectBossBar::Darkensky => Value::String("Darken sky".to_string()),
                SkyEffectBossBar::Both => Value::String("Both".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sky Effect".to_string()));
        data.insert("action".to_string(), Value::String("BossBar".to_string()));
        data.insert("block".to_string(), Value::String("BossBar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SkyEffectBossBar {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone)]
pub enum BarColorBossBar {
    Red,
    Purple,
    Pink,
    Blue,
    Green,
    Yellow,
    White,
}
impl BarColorBossBar {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BarColorBossBar::Red => Value::String("Red".to_string()),
                BarColorBossBar::Purple => Value::String("Purple".to_string()),
                BarColorBossBar::Pink => Value::String("Pink".to_string()),
                BarColorBossBar::Blue => Value::String("Blue".to_string()),
                BarColorBossBar::Green => Value::String("Green".to_string()),
                BarColorBossBar::Yellow => Value::String("Yellow".to_string()),
                BarColorBossBar::White => Value::String("White".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Bar Color".to_string()));
        data.insert("action".to_string(), Value::String("BossBar".to_string()));
        data.insert("block".to_string(), Value::String("BossBar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BarColorBossBar {
    fn default() -> Self {
        Self::Purple
    }
}
#[derive(Debug, Clone)]
pub enum AddtoCurrentVelocitySetVelocity {
    True,
    False,
}
impl AddtoCurrentVelocitySetVelocity {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AddtoCurrentVelocitySetVelocity::True => {
                    Value::String("True".to_string())
                }
                AddtoCurrentVelocitySetVelocity::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Add to Current Velocity".to_string()),
        );
        data.insert("action".to_string(), Value::String("SetVelocity".to_string()));
        data.insert("block".to_string(), Value::String("SetVelocity".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AddtoCurrentVelocitySetVelocity {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum NewRowPositionAddInventoryMenuRow {
    Toprow,
    Bottomrow,
}
impl NewRowPositionAddInventoryMenuRow {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                NewRowPositionAddInventoryMenuRow::Toprow => {
                    Value::String("Top row".to_string())
                }
                NewRowPositionAddInventoryMenuRow::Bottomrow => {
                    Value::String("Bottom row".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("New Row Position".to_string()));
        data.insert("action".to_string(), Value::String("AddInvRow".to_string()));
        data.insert("block".to_string(), Value::String("AddInvRow".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for NewRowPositionAddInventoryMenuRow {
    fn default() -> Self {
        Self::Bottomrow
    }
}
#[derive(Debug, Clone)]
pub enum SoundSourcePlaySoundfromEntity {
    Master,
    Music,
    JukeboxNoteBlocks,
    Weather,
    Blocks,
    HostileCreatures,
    FriendlyCreatures,
    Players,
    AmbientEnvironment,
    VoiceSpeech,
}
impl SoundSourcePlaySoundfromEntity {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SoundSourcePlaySoundfromEntity::Master => {
                    Value::String("Master".to_string())
                }
                SoundSourcePlaySoundfromEntity::Music => {
                    Value::String("Music".to_string())
                }
                SoundSourcePlaySoundfromEntity::JukeboxNoteBlocks => {
                    Value::String("Jukebox/Note Blocks".to_string())
                }
                SoundSourcePlaySoundfromEntity::Weather => {
                    Value::String("Weather".to_string())
                }
                SoundSourcePlaySoundfromEntity::Blocks => {
                    Value::String("Blocks".to_string())
                }
                SoundSourcePlaySoundfromEntity::HostileCreatures => {
                    Value::String("Hostile Creatures".to_string())
                }
                SoundSourcePlaySoundfromEntity::FriendlyCreatures => {
                    Value::String("Friendly Creatures".to_string())
                }
                SoundSourcePlaySoundfromEntity::Players => {
                    Value::String("Players".to_string())
                }
                SoundSourcePlaySoundfromEntity::AmbientEnvironment => {
                    Value::String("Ambient/Environment".to_string())
                }
                SoundSourcePlaySoundfromEntity::VoiceSpeech => {
                    Value::String("Voice/Speech".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sound Source".to_string()));
        data.insert("action".to_string(), Value::String("PlayEntitySound".to_string()));
        data.insert("block".to_string(), Value::String("PlayEntitySound".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SoundSourcePlaySoundfromEntity {
    fn default() -> Self {
        Self::Master
    }
}
#[derive(Debug, Clone)]
pub enum SetExperienceSetExperience {
    Points,
    Level,
    LevelPercentage,
}
impl SetExperienceSetExperience {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SetExperienceSetExperience::Points => Value::String("Points".to_string()),
                SetExperienceSetExperience::Level => Value::String("Level".to_string()),
                SetExperienceSetExperience::LevelPercentage => {
                    Value::String("Level Percentage".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Set Experience".to_string()));
        data.insert("action".to_string(), Value::String("SetExp".to_string()));
        data.insert("block".to_string(), Value::String("SetExp".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SetExperienceSetExperience {
    fn default() -> Self {
        Self::Level
    }
}
#[derive(Debug, Clone)]
pub enum AnimationTypeSendAnimation {
    Hurtanimation,
    Wakeupfadeeffect,
}
impl AnimationTypeSendAnimation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AnimationTypeSendAnimation::Hurtanimation => {
                    Value::String("Hurt animation".to_string())
                }
                AnimationTypeSendAnimation::Wakeupfadeeffect => {
                    Value::String("Wake up (fade effect)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Animation Type".to_string()));
        data.insert("action".to_string(), Value::String("SendAnimation".to_string()));
        data.insert("block".to_string(), Value::String("SendAnimation".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AnimationTypeSendAnimation {
    fn default() -> Self {
        Self::Hurtanimation
    }
}
#[derive(Debug, Clone)]
pub enum InventoryKeptSetInventoryKept {
    Enable,
    Disable,
}
impl InventoryKeptSetInventoryKept {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InventoryKeptSetInventoryKept::Enable => {
                    Value::String("Enable".to_string())
                }
                InventoryKeptSetInventoryKept::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Inventory Kept".to_string()));
        data.insert("action".to_string(), Value::String("SetInventoryKept".to_string()));
        data.insert("block".to_string(), Value::String("SetInventoryKept".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InventoryKeptSetInventoryKept {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum AllowFlightSetAllowFlight {
    Enable,
    Disable,
}
impl AllowFlightSetAllowFlight {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AllowFlightSetAllowFlight::Enable => Value::String("Enable".to_string()),
                AllowFlightSetAllowFlight::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Allow Flight".to_string()));
        data.insert("action".to_string(), Value::String("SetAllowFlight".to_string()));
        data.insert("block".to_string(), Value::String("SetAllowFlight".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AllowFlightSetAllowFlight {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum AddtoCurrentVelocityLaunchUp {
    True,
    False,
}
impl AddtoCurrentVelocityLaunchUp {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AddtoCurrentVelocityLaunchUp::True => Value::String("True".to_string()),
                AddtoCurrentVelocityLaunchUp::False => Value::String("False".to_string()),
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Add to Current Velocity".to_string()),
        );
        data.insert("action".to_string(), Value::String("LaunchUp".to_string()));
        data.insert("block".to_string(), Value::String("LaunchUp".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AddtoCurrentVelocityLaunchUp {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum HealPlayertoMaxHealthSetMaximumHealth {
    True,
    False,
}
impl HealPlayertoMaxHealthSetMaximumHealth {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HealPlayertoMaxHealthSetMaximumHealth::True => {
                    Value::String("True".to_string())
                }
                HealPlayertoMaxHealthSetMaximumHealth::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Heal Player to Max Health".to_string()),
        );
        data.insert("action".to_string(), Value::String("SetMaxHealth".to_string()));
        data.insert("block".to_string(), Value::String("SetMaxHealth".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HealPlayertoMaxHealthSetMaximumHealth {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreBlocksGetTargetEntity {
    True,
    False,
}
impl IgnoreBlocksGetTargetEntity {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreBlocksGetTargetEntity::True => Value::String("True".to_string()),
                IgnoreBlocksGetTargetEntity::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Blocks".to_string()));
        data.insert("action".to_string(), Value::String("GetTargetEntity".to_string()));
        data.insert("block".to_string(), Value::String("GetTargetEntity".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreBlocksGetTargetEntity {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum FlightModeForceFlightMode {
    StartFlight,
    StopFlight,
}
impl FlightModeForceFlightMode {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FlightModeForceFlightMode::StartFlight => {
                    Value::String("Start Flight".to_string())
                }
                FlightModeForceFlightMode::StopFlight => {
                    Value::String("Stop Flight".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Flight Mode".to_string()));
        data.insert("action".to_string(), Value::String("ForceFlight".to_string()));
        data.insert("block".to_string(), Value::String("ForceFlight".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FlightModeForceFlightMode {
    fn default() -> Self {
        Self::StartFlight
    }
}
#[derive(Debug, Clone)]
pub enum PlayerListFieldSetPlayerListInfo {
    Header,
    Footer,
}
impl PlayerListFieldSetPlayerListInfo {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PlayerListFieldSetPlayerListInfo::Header => {
                    Value::String("Header".to_string())
                }
                PlayerListFieldSetPlayerListInfo::Footer => {
                    Value::String("Footer".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Player List Field".to_string()));
        data.insert("action".to_string(), Value::String("SetTabListInfo".to_string()));
        data.insert("block".to_string(), Value::String("SetTabListInfo".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PlayerListFieldSetPlayerListInfo {
    fn default() -> Self {
        Self::Header
    }
}
#[derive(Debug, Clone)]
pub enum TextValueMergingSetPlayerListInfo {
    Addspaces,
    Nospaces,
}
impl TextValueMergingSetPlayerListInfo {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextValueMergingSetPlayerListInfo::Addspaces => {
                    Value::String("Add spaces".to_string())
                }
                TextValueMergingSetPlayerListInfo::Nospaces => {
                    Value::String("No spaces".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Value Merging".to_string()));
        data.insert("action".to_string(), Value::String("SetTabListInfo".to_string()));
        data.insert("block".to_string(), Value::String("SetTabListInfo".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextValueMergingSetPlayerListInfo {
    fn default() -> Self {
        Self::Nospaces
    }
}
#[derive(Debug, Clone)]
pub enum InheritStylesSetPlayerListInfo {
    True,
    False,
}
impl InheritStylesSetPlayerListInfo {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InheritStylesSetPlayerListInfo::True => Value::String("True".to_string()),
                InheritStylesSetPlayerListInfo::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Inherit Styles".to_string()));
        data.insert("action".to_string(), Value::String("SetTabListInfo".to_string()));
        data.insert("block".to_string(), Value::String("SetTabListInfo".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InheritStylesSetPlayerListInfo {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum BarStyleSetBossBar {
    Solid,
    Sixsegments,
    OneZerosegments,
    OneTwosegments,
    TwoZerosegments,
}
impl BarStyleSetBossBar {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BarStyleSetBossBar::Solid => Value::String("Solid".to_string()),
                BarStyleSetBossBar::Sixsegments => {
                    Value::String("6 segments".to_string())
                }
                BarStyleSetBossBar::OneZerosegments => {
                    Value::String("10 segments".to_string())
                }
                BarStyleSetBossBar::OneTwosegments => {
                    Value::String("12 segments".to_string())
                }
                BarStyleSetBossBar::TwoZerosegments => {
                    Value::String("20 segments".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Bar Style".to_string()));
        data.insert("action".to_string(), Value::String(" SetBossBar ".to_string()));
        data.insert("block".to_string(), Value::String(" SetBossBar ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BarStyleSetBossBar {
    fn default() -> Self {
        Self::Solid
    }
}
#[derive(Debug, Clone)]
pub enum SkyEffectSetBossBar {
    None,
    Createfog,
    Darkensky,
    Both,
}
impl SkyEffectSetBossBar {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SkyEffectSetBossBar::None => Value::String("None".to_string()),
                SkyEffectSetBossBar::Createfog => Value::String("Create fog".to_string()),
                SkyEffectSetBossBar::Darkensky => Value::String("Darken sky".to_string()),
                SkyEffectSetBossBar::Both => Value::String("Both".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sky Effect".to_string()));
        data.insert("action".to_string(), Value::String(" SetBossBar ".to_string()));
        data.insert("block".to_string(), Value::String(" SetBossBar ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SkyEffectSetBossBar {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone)]
pub enum BarColorSetBossBar {
    Red,
    Purple,
    Pink,
    Blue,
    Green,
    Yellow,
    White,
}
impl BarColorSetBossBar {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BarColorSetBossBar::Red => Value::String("Red".to_string()),
                BarColorSetBossBar::Purple => Value::String("Purple".to_string()),
                BarColorSetBossBar::Pink => Value::String("Pink".to_string()),
                BarColorSetBossBar::Blue => Value::String("Blue".to_string()),
                BarColorSetBossBar::Green => Value::String("Green".to_string()),
                BarColorSetBossBar::Yellow => Value::String("Yellow".to_string()),
                BarColorSetBossBar::White => Value::String("White".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Bar Color".to_string()));
        data.insert("action".to_string(), Value::String(" SetBossBar ".to_string()));
        data.insert("block".to_string(), Value::String(" SetBossBar ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BarColorSetBossBar {
    fn default() -> Self {
        Self::Purple
    }
}
#[derive(Debug, Clone)]
pub enum SpectatorCollisionSetSpectatorCollision {
    Enable,
    Disable,
}
impl SpectatorCollisionSetSpectatorCollision {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SpectatorCollisionSetSpectatorCollision::Enable => {
                    Value::String("Enable".to_string())
                }
                SpectatorCollisionSetSpectatorCollision::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Spectator Collision".to_string()));
        data.insert(
            "action".to_string(),
            Value::String("SpectatorCollision".to_string()),
        );
        data.insert(
            "block".to_string(),
            Value::String("SpectatorCollision".to_string()),
        );
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SpectatorCollisionSetSpectatorCollision {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum NameTagVisibleSetNameTagVisible {
    Enable,
    Disable,
}
impl NameTagVisibleSetNameTagVisible {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                NameTagVisibleSetNameTagVisible::Enable => {
                    Value::String("Enable".to_string())
                }
                NameTagVisibleSetNameTagVisible::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Name Tag Visible".to_string()));
        data.insert("action".to_string(), Value::String("SetNameVisible".to_string()));
        data.insert("block".to_string(), Value::String("SetNameVisible".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for NameTagVisibleSetNameTagVisible {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum FlightModeSetGamemode {
    RespectGamemode,
    KeepOriginal,
}
impl FlightModeSetGamemode {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FlightModeSetGamemode::RespectGamemode => {
                    Value::String("Respect Gamemode".to_string())
                }
                FlightModeSetGamemode::KeepOriginal => {
                    Value::String("Keep Original".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Flight Mode".to_string()));
        data.insert("action".to_string(), Value::String("SetGamemode".to_string()));
        data.insert("block".to_string(), Value::String("SetGamemode".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FlightModeSetGamemode {
    fn default() -> Self {
        Self::RespectGamemode
    }
}
#[derive(Debug, Clone)]
pub enum GamemodeSetGamemode {
    Survival,
    Creative,
    Adventure,
}
impl GamemodeSetGamemode {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GamemodeSetGamemode::Survival => Value::String("Survival".to_string()),
                GamemodeSetGamemode::Creative => Value::String("Creative".to_string()),
                GamemodeSetGamemode::Adventure => Value::String("Adventure".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Gamemode".to_string()));
        data.insert("action".to_string(), Value::String("SetGamemode".to_string()));
        data.insert("block".to_string(), Value::String("SetGamemode".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GamemodeSetGamemode {
    fn default() -> Self {
        Self::Survival
    }
}
#[derive(Debug, Clone)]
pub enum RowtoRemoveRemoveInventoryMenuRow {
    Toprow,
    Bottomrow,
}
impl RowtoRemoveRemoveInventoryMenuRow {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RowtoRemoveRemoveInventoryMenuRow::Toprow => {
                    Value::String("Top row".to_string())
                }
                RowtoRemoveRemoveInventoryMenuRow::Bottomrow => {
                    Value::String("Bottom row".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Row to Remove".to_string()));
        data.insert("action".to_string(), Value::String("RemoveInvRow".to_string()));
        data.insert("block".to_string(), Value::String("RemoveInvRow".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RowtoRemoveRemoveInventoryMenuRow {
    fn default() -> Self {
        Self::Bottomrow
    }
}
#[derive(Debug, Clone)]
pub enum HealTypeLSetHealth {
    RegularHealth,
    AbsorptionHealth,
    CombinedHealth,
}
impl HealTypeLSetHealth {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HealTypeLSetHealth::RegularHealth => {
                    Value::String("Regular Health".to_string())
                }
                HealTypeLSetHealth::AbsorptionHealth => {
                    Value::String("Absorption Health".to_string())
                }
                HealTypeLSetHealth::CombinedHealth => {
                    Value::String("Combined Health".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Heal Type".to_string()));
        data.insert("action".to_string(), Value::String("L SetHealth".to_string()));
        data.insert("block".to_string(), Value::String("L SetHealth".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HealTypeLSetHealth {
    fn default() -> Self {
        Self::RegularHealth
    }
}
#[derive(Debug, Clone)]
pub enum ClearModeClearInventory {
    Entireinventory,
    Maininventory,
    Upperinventory,
    Hotbar,
    Armor,
}
impl ClearModeClearInventory {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ClearModeClearInventory::Entireinventory => {
                    Value::String("Entire inventory".to_string())
                }
                ClearModeClearInventory::Maininventory => {
                    Value::String("Main inventory".to_string())
                }
                ClearModeClearInventory::Upperinventory => {
                    Value::String("Upper inventory".to_string())
                }
                ClearModeClearInventory::Hotbar => Value::String("Hotbar".to_string()),
                ClearModeClearInventory::Armor => Value::String("Armor".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Clear Mode".to_string()));
        data.insert("action".to_string(), Value::String("ClearInv".to_string()));
        data.insert("block".to_string(), Value::String("ClearInv".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ClearModeClearInventory {
    fn default() -> Self {
        Self::Entireinventory
    }
}
#[derive(Debug, Clone)]
pub enum ClearCraftingandCursorClearInventory {
    True,
    False,
}
impl ClearCraftingandCursorClearInventory {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ClearCraftingandCursorClearInventory::True => {
                    Value::String("True".to_string())
                }
                ClearCraftingandCursorClearInventory::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Clear Crafting and Cursor".to_string()),
        );
        data.insert("action".to_string(), Value::String("ClearInv".to_string()));
        data.insert("block".to_string(), Value::String("ClearInv".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ClearCraftingandCursorClearInventory {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum TickingLockedSetFreezeTicks {
    Enable,
    Disable,
}
impl TickingLockedSetFreezeTicks {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TickingLockedSetFreezeTicks::Enable => {
                    Value::String("Enable".to_string())
                }
                TickingLockedSetFreezeTicks::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Ticking Locked".to_string()));
        data.insert("action".to_string(), Value::String("SetFreezeTicks".to_string()));
        data.insert("block".to_string(), Value::String("SetFreezeTicks".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TickingLockedSetFreezeTicks {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum GlidingSetGliding {
    Enable,
    Disable,
}
impl GlidingSetGliding {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GlidingSetGliding::Enable => Value::String("Enable".to_string()),
                GlidingSetGliding::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Gliding".to_string()));
        data.insert("action".to_string(), Value::String("SetGliding".to_string()));
        data.insert("block".to_string(), Value::String("SetGliding".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GlidingSetGliding {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum FlyingSetFlying {
    Enable,
    Disable,
}
impl FlyingSetFlying {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FlyingSetFlying::Enable => Value::String("Enable".to_string()),
                FlyingSetFlying::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Flying".to_string()));
        data.insert("action".to_string(), Value::String("SetFlying".to_string()));
        data.insert("block".to_string(), Value::String("SetFlying".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FlyingSetFlying {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum ContainerStateDisplayBlockOpenedState {
    Open,
    Closed,
}
impl ContainerStateDisplayBlockOpenedState {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ContainerStateDisplayBlockOpenedState::Open => {
                    Value::String("Open".to_string())
                }
                ContainerStateDisplayBlockOpenedState::Closed => {
                    Value::String("Closed".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Container State".to_string()));
        data.insert("action".to_string(), Value::String("DisplayBlockOpen".to_string()));
        data.insert("block".to_string(), Value::String("DisplayBlockOpen".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ContainerStateDisplayBlockOpenedState {
    fn default() -> Self {
        Self::Open
    }
}
#[derive(Debug, Clone)]
pub enum HandSlotSetHandItem {
    MainHand,
    OffHand,
}
impl HandSlotSetHandItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HandSlotSetHandItem::MainHand => Value::String("Main Hand".to_string()),
                HandSlotSetHandItem::OffHand => Value::String("Off Hand".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Hand Slot".to_string()));
        data.insert("action".to_string(), Value::String("SetHandItem".to_string()));
        data.insert("block".to_string(), Value::String("SetHandItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HandSlotSetHandItem {
    fn default() -> Self {
        Self::MainHand
    }
}
#[derive(Debug, Clone)]
pub enum ToastTypeSendAdvancement {
    Advancement,
    Goal,
    Challenge,
}
impl ToastTypeSendAdvancement {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ToastTypeSendAdvancement::Advancement => {
                    Value::String("Advancement".to_string())
                }
                ToastTypeSendAdvancement::Goal => Value::String("Goal".to_string()),
                ToastTypeSendAdvancement::Challenge => {
                    Value::String("Challenge".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Toast Type".to_string()));
        data.insert("action".to_string(), Value::String("SendAdvancement".to_string()));
        data.insert("block".to_string(), Value::String("SendAdvancement".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ToastTypeSendAdvancement {
    fn default() -> Self {
        Self::Advancement
    }
}
#[derive(Debug, Clone)]
pub enum KeepCurrentRotationTeleport {
    True,
    False,
}
impl KeepCurrentRotationTeleport {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                KeepCurrentRotationTeleport::True => Value::String("True".to_string()),
                KeepCurrentRotationTeleport::False => Value::String("False".to_string()),
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Keep Current Rotation".to_string()),
        );
        data.insert("action".to_string(), Value::String("Teleport".to_string()));
        data.insert("block".to_string(), Value::String("Teleport".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for KeepCurrentRotationTeleport {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum KeepVelocityTeleport {
    True,
    False,
}
impl KeepVelocityTeleport {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                KeepVelocityTeleport::True => Value::String("True".to_string()),
                KeepVelocityTeleport::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Keep Velocity".to_string()));
        data.insert("action".to_string(), Value::String("Teleport".to_string()));
        data.insert("block".to_string(), Value::String("Teleport".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for KeepVelocityTeleport {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum PVPSetPvPAllowed {
    Enable,
    Disable,
}
impl PVPSetPvPAllowed {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PVPSetPvPAllowed::Enable => Value::String("Enable".to_string()),
                PVPSetPvPAllowed::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("PVP".to_string()));
        data.insert("action".to_string(), Value::String("SetAllowPVP".to_string()));
        data.insert("block".to_string(), Value::String("SetAllowPVP".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PVPSetPvPAllowed {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum AddtoCurrentVelocityLaunchTowardLocation {
    True,
    False,
}
impl AddtoCurrentVelocityLaunchTowardLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AddtoCurrentVelocityLaunchTowardLocation::True => {
                    Value::String("True".to_string())
                }
                AddtoCurrentVelocityLaunchTowardLocation::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Add to Current Velocity".to_string()),
        );
        data.insert("action".to_string(), Value::String("LaunchToward".to_string()));
        data.insert("block".to_string(), Value::String("LaunchToward".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AddtoCurrentVelocityLaunchTowardLocation {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreDistanceLaunchTowardLocation {
    True,
    False,
}
impl IgnoreDistanceLaunchTowardLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreDistanceLaunchTowardLocation::True => {
                    Value::String("True".to_string())
                }
                IgnoreDistanceLaunchTowardLocation::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Distance".to_string()));
        data.insert("action".to_string(), Value::String("LaunchToward".to_string()));
        data.insert("block".to_string(), Value::String("LaunchToward".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreDistanceLaunchTowardLocation {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum AnimationTypeDisplayGatewayBeam {
    Initialbeam,
    Periodicbeam,
}
impl AnimationTypeDisplayGatewayBeam {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AnimationTypeDisplayGatewayBeam::Initialbeam => {
                    Value::String("Initial beam".to_string())
                }
                AnimationTypeDisplayGatewayBeam::Periodicbeam => {
                    Value::String("Periodic beam".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Animation Type".to_string()));
        data.insert("action".to_string(), Value::String("DisplayGateway".to_string()));
        data.insert("block".to_string(), Value::String("DisplayGateway".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AnimationTypeDisplayGatewayBeam {
    fn default() -> Self {
        Self::Initialbeam
    }
}
#[derive(Debug, Clone)]
pub enum OnFireSetVisualFire {
    True,
    False,
}
impl OnFireSetVisualFire {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                OnFireSetVisualFire::True => Value::String("True".to_string()),
                OnFireSetVisualFire::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("On Fire".to_string()));
        data.insert("action".to_string(), Value::String("SetVisualFire".to_string()));
        data.insert("block".to_string(), Value::String("SetVisualFire".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for OnFireSetVisualFire {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum DisguiseVisibleSetOwnDisguiseVisibility {
    Enable,
    Disable,
}
impl DisguiseVisibleSetOwnDisguiseVisibility {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DisguiseVisibleSetOwnDisguiseVisibility::Enable => {
                    Value::String("Enable".to_string())
                }
                DisguiseVisibleSetOwnDisguiseVisibility::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Disguise Visible".to_string()));
        data.insert(
            "action".to_string(),
            Value::String("SetDisguiseVisible".to_string()),
        );
        data.insert(
            "block".to_string(),
            Value::String("SetDisguiseVisible".to_string()),
        );
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DisguiseVisibleSetOwnDisguiseVisibility {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum GiveExperienceGiveExperience {
    Points,
    Levels,
    Levelpercentage,
}
impl GiveExperienceGiveExperience {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GiveExperienceGiveExperience::Points => {
                    Value::String("Points".to_string())
                }
                GiveExperienceGiveExperience::Levels => {
                    Value::String("Levels".to_string())
                }
                GiveExperienceGiveExperience::Levelpercentage => {
                    Value::String("Level percentage".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Give Experience".to_string()));
        data.insert("action".to_string(), Value::String("GiveExp".to_string()));
        data.insert("block".to_string(), Value::String("GiveExp".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GiveExperienceGiveExperience {
    fn default() -> Self {
        Self::Points
    }
}
#[derive(Debug, Clone)]
pub enum TextValueMergingShowActionBarText {
    Addspaces,
    Nospaces,
}
impl TextValueMergingShowActionBarText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextValueMergingShowActionBarText::Addspaces => {
                    Value::String("Add spaces".to_string())
                }
                TextValueMergingShowActionBarText::Nospaces => {
                    Value::String("No spaces".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Value Merging".to_string()));
        data.insert("action".to_string(), Value::String("ActionBar".to_string()));
        data.insert("block".to_string(), Value::String("ActionBar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextValueMergingShowActionBarText {
    fn default() -> Self {
        Self::Nospaces
    }
}
#[derive(Debug, Clone)]
pub enum InheritStylesShowActionBarText {
    True,
    False,
}
impl InheritStylesShowActionBarText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InheritStylesShowActionBarText::True => Value::String("True".to_string()),
                InheritStylesShowActionBarText::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Inherit Styles".to_string()));
        data.insert("action".to_string(), Value::String("ActionBar".to_string()));
        data.insert("block".to_string(), Value::String("ActionBar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InheritStylesShowActionBarText {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum AlignmentModeSendMessage {
    Regular,
    Centered,
}
impl AlignmentModeSendMessage {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AlignmentModeSendMessage::Regular => Value::String("Regular".to_string()),
                AlignmentModeSendMessage::Centered => {
                    Value::String("Centered".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Alignment Mode".to_string()));
        data.insert("action".to_string(), Value::String("SendMessage".to_string()));
        data.insert("block".to_string(), Value::String("SendMessage".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AlignmentModeSendMessage {
    fn default() -> Self {
        Self::Regular
    }
}
#[derive(Debug, Clone)]
pub enum TextValueMergingSendMessage {
    Addspaces,
    Nospaces,
}
impl TextValueMergingSendMessage {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextValueMergingSendMessage::Addspaces => {
                    Value::String("Add spaces".to_string())
                }
                TextValueMergingSendMessage::Nospaces => {
                    Value::String("No spaces".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Value Merging".to_string()));
        data.insert("action".to_string(), Value::String("SendMessage".to_string()));
        data.insert("block".to_string(), Value::String("SendMessage".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextValueMergingSendMessage {
    fn default() -> Self {
        Self::Addspaces
    }
}
#[derive(Debug, Clone)]
pub enum InheritStylesSendMessage {
    True,
    False,
}
impl InheritStylesSendMessage {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InheritStylesSendMessage::True => Value::String("True".to_string()),
                InheritStylesSendMessage::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Inherit Styles".to_string()));
        data.insert("action".to_string(), Value::String("SendMessage".to_string()));
        data.insert("block".to_string(), Value::String("SendMessage".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InheritStylesSendMessage {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum SoundSourcePlaySoundSequence {
    Master,
    Music,
    JukeboxNoteBlocks,
    Weather,
    Blocks,
    HostileCreatures,
    FriendlyCreatures,
    Players,
    AmbientEnvironment,
    VoiceSpeech,
}
impl SoundSourcePlaySoundSequence {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SoundSourcePlaySoundSequence::Master => {
                    Value::String("Master".to_string())
                }
                SoundSourcePlaySoundSequence::Music => Value::String("Music".to_string()),
                SoundSourcePlaySoundSequence::JukeboxNoteBlocks => {
                    Value::String("Jukebox/Note Blocks".to_string())
                }
                SoundSourcePlaySoundSequence::Weather => {
                    Value::String("Weather".to_string())
                }
                SoundSourcePlaySoundSequence::Blocks => {
                    Value::String("Blocks".to_string())
                }
                SoundSourcePlaySoundSequence::HostileCreatures => {
                    Value::String("Hostile Creatures".to_string())
                }
                SoundSourcePlaySoundSequence::FriendlyCreatures => {
                    Value::String("Friendly Creatures".to_string())
                }
                SoundSourcePlaySoundSequence::Players => {
                    Value::String("Players".to_string())
                }
                SoundSourcePlaySoundSequence::AmbientEnvironment => {
                    Value::String("Ambient/Environment".to_string())
                }
                SoundSourcePlaySoundSequence::VoiceSpeech => {
                    Value::String("Voice/Speech".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sound Source".to_string()));
        data.insert("action".to_string(), Value::String("PlaySoundSeq".to_string()));
        data.insert("block".to_string(), Value::String("PlaySoundSeq".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SoundSourcePlaySoundSequence {
    fn default() -> Self {
        Self::Master
    }
}
#[derive(Debug, Clone)]
pub enum SignSideDisplaySignText {
    Front,
    Back,
}
impl SignSideDisplaySignText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignSideDisplaySignText::Front => Value::String("Front".to_string()),
                SignSideDisplaySignText::Back => Value::String("Back".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Side".to_string()));
        data.insert("action".to_string(), Value::String("DisplaySignText".to_string()));
        data.insert("block".to_string(), Value::String("DisplaySignText".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignSideDisplaySignText {
    fn default() -> Self {
        Self::Front
    }
}
#[derive(Debug, Clone)]
pub enum TextColorDisplaySignText {
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
impl TextColorDisplaySignText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextColorDisplaySignText::White => Value::String("White".to_string()),
                TextColorDisplaySignText::Orange => Value::String("Orange".to_string()),
                TextColorDisplaySignText::Magenta => Value::String("Magenta".to_string()),
                TextColorDisplaySignText::Lightblue => {
                    Value::String("Light blue".to_string())
                }
                TextColorDisplaySignText::Yellow => Value::String("Yellow".to_string()),
                TextColorDisplaySignText::Lime => Value::String("Lime".to_string()),
                TextColorDisplaySignText::Pink => Value::String("Pink".to_string()),
                TextColorDisplaySignText::Gray => Value::String("Gray".to_string()),
                TextColorDisplaySignText::Lightgray => {
                    Value::String("Light gray".to_string())
                }
                TextColorDisplaySignText::Cyan => Value::String("Cyan".to_string()),
                TextColorDisplaySignText::Purple => Value::String("Purple".to_string()),
                TextColorDisplaySignText::Blue => Value::String("Blue".to_string()),
                TextColorDisplaySignText::Brown => Value::String("Brown".to_string()),
                TextColorDisplaySignText::Green => Value::String("Green".to_string()),
                TextColorDisplaySignText::Red => Value::String("Red".to_string()),
                TextColorDisplaySignText::Black => Value::String("Black".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Text Color".to_string()));
        data.insert("action".to_string(), Value::String("DisplaySignText".to_string()));
        data.insert("block".to_string(), Value::String("DisplaySignText".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextColorDisplaySignText {
    fn default() -> Self {
        Self::Black
    }
}
#[derive(Debug, Clone)]
pub enum GlowingDisplaySignText {
    Enable,
    Disable,
}
impl GlowingDisplaySignText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GlowingDisplaySignText::Enable => Value::String("Enable".to_string()),
                GlowingDisplaySignText::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Glowing".to_string()));
        data.insert("action".to_string(), Value::String("DisplaySignText".to_string()));
        data.insert("block".to_string(), Value::String("DisplaySignText".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GlowingDisplaySignText {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum SpeedTypeSetMovementSpeed {
    Groundspeed,
    Flightspeed,
    Both,
}
impl SpeedTypeSetMovementSpeed {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SpeedTypeSetMovementSpeed::Groundspeed => {
                    Value::String("Ground speed".to_string())
                }
                SpeedTypeSetMovementSpeed::Flightspeed => {
                    Value::String("Flight speed".to_string())
                }
                SpeedTypeSetMovementSpeed::Both => Value::String("Both".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Speed Type".to_string()));
        data.insert("action".to_string(), Value::String("SetSpeed".to_string()));
        data.insert("block".to_string(), Value::String("SetSpeed".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SpeedTypeSetMovementSpeed {
    fn default() -> Self {
        Self::Groundspeed
    }
}
#[derive(Debug, Clone)]
pub enum WeatherSetPlayerWeather {
    Clear,
    Downfall,
}
impl WeatherSetPlayerWeather {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                WeatherSetPlayerWeather::Clear => Value::String("Clear".to_string()),
                WeatherSetPlayerWeather::Downfall => {
                    Value::String("Downfall".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Weather".to_string()));
        data.insert("action".to_string(), Value::String("SetPlayerWeather".to_string()));
        data.insert("block".to_string(), Value::String("SetPlayerWeather".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for WeatherSetPlayerWeather {
    fn default() -> Self {
        Self::Downfall
    }
}
#[derive(Debug, Clone)]
pub enum ShoulderSetVisualShoulderParrot {
    Left,
    Right,
}
impl ShoulderSetVisualShoulderParrot {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShoulderSetVisualShoulderParrot::Left => {
                    Value::String("Left".to_string())
                }
                ShoulderSetVisualShoulderParrot::Right => {
                    Value::String("Right".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Shoulder".to_string()));
        data.insert("action".to_string(), Value::String("SetShoulder".to_string()));
        data.insert("block".to_string(), Value::String("SetShoulder".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShoulderSetVisualShoulderParrot {
    fn default() -> Self {
        Self::Left
    }
}
#[derive(Debug, Clone)]
pub enum TypeSetVisualShoulderParrot {
    Remove,
    Red,
    Blue,
    Green,
    Cyan,
    Gray,
}
impl TypeSetVisualShoulderParrot {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TypeSetVisualShoulderParrot::Remove => {
                    Value::String("Remove".to_string())
                }
                TypeSetVisualShoulderParrot::Red => Value::String("Red".to_string()),
                TypeSetVisualShoulderParrot::Blue => Value::String("Blue".to_string()),
                TypeSetVisualShoulderParrot::Green => Value::String("Green".to_string()),
                TypeSetVisualShoulderParrot::Cyan => Value::String("Cyan".to_string()),
                TypeSetVisualShoulderParrot::Gray => Value::String("Gray".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Type".to_string()));
        data.insert("action".to_string(), Value::String("SetShoulder".to_string()));
        data.insert("block".to_string(), Value::String("SetShoulder".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TypeSetVisualShoulderParrot {
    fn default() -> Self {
        Self::Remove
    }
}
#[derive(Debug, Clone)]
pub enum FillTypeDisplayAnimatedParticleCuboid {
    Wireframe,
    Hollow,
    Solid,
}
impl FillTypeDisplayAnimatedParticleCuboid {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FillTypeDisplayAnimatedParticleCuboid::Wireframe => {
                    Value::String("Wireframe".to_string())
                }
                FillTypeDisplayAnimatedParticleCuboid::Hollow => {
                    Value::String("Hollow".to_string())
                }
                FillTypeDisplayAnimatedParticleCuboid::Solid => {
                    Value::String("Solid".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Fill Type".to_string()));
        data.insert("action".to_string(), Value::String("ParticleCuboidA".to_string()));
        data.insert("block".to_string(), Value::String("ParticleCuboidA".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FillTypeDisplayAnimatedParticleCuboid {
    fn default() -> Self {
        Self::Wireframe
    }
}
#[derive(Debug, Clone)]
pub enum ShowIconGivePotionEffect {
    True,
    False,
}
impl ShowIconGivePotionEffect {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShowIconGivePotionEffect::True => Value::String("True".to_string()),
                ShowIconGivePotionEffect::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Show Icon".to_string()));
        data.insert("action".to_string(), Value::String("GivePotion".to_string()));
        data.insert("block".to_string(), Value::String("GivePotion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShowIconGivePotionEffect {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum OverwriteEffectGivePotionEffect {
    True,
    False,
}
impl OverwriteEffectGivePotionEffect {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                OverwriteEffectGivePotionEffect::True => {
                    Value::String("True".to_string())
                }
                OverwriteEffectGivePotionEffect::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Overwrite Effect".to_string()));
        data.insert("action".to_string(), Value::String("GivePotion".to_string()));
        data.insert("block".to_string(), Value::String("GivePotion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for OverwriteEffectGivePotionEffect {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum EffectParticlesGivePotionEffect {
    Regular,
    Ambient,
    None,
}
impl EffectParticlesGivePotionEffect {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                EffectParticlesGivePotionEffect::Regular => {
                    Value::String("Regular".to_string())
                }
                EffectParticlesGivePotionEffect::Ambient => {
                    Value::String("Ambient".to_string())
                }
                EffectParticlesGivePotionEffect::None => {
                    Value::String("None".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Effect Particles".to_string()));
        data.insert("action".to_string(), Value::String("GivePotion".to_string()));
        data.insert("block".to_string(), Value::String("GivePotion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for EffectParticlesGivePotionEffect {
    fn default() -> Self {
        Self::Regular
    }
}
#[derive(Debug, Clone)]
pub enum SoundSourcePlaySound {
    Master,
    Music,
    JukeboxNoteBlocks,
    Weather,
    Blocks,
    HostileCreatures,
    FriendlyCreatures,
    Players,
    AmbientEnvironment,
    VoiceSpeech,
}
impl SoundSourcePlaySound {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SoundSourcePlaySound::Master => Value::String("Master".to_string()),
                SoundSourcePlaySound::Music => Value::String("Music".to_string()),
                SoundSourcePlaySound::JukeboxNoteBlocks => {
                    Value::String("Jukebox/Note Blocks".to_string())
                }
                SoundSourcePlaySound::Weather => Value::String("Weather".to_string()),
                SoundSourcePlaySound::Blocks => Value::String("Blocks".to_string()),
                SoundSourcePlaySound::HostileCreatures => {
                    Value::String("Hostile Creatures".to_string())
                }
                SoundSourcePlaySound::FriendlyCreatures => {
                    Value::String("Friendly Creatures".to_string())
                }
                SoundSourcePlaySound::Players => Value::String("Players".to_string()),
                SoundSourcePlaySound::AmbientEnvironment => {
                    Value::String("Ambient/Environment".to_string())
                }
                SoundSourcePlaySound::VoiceSpeech => {
                    Value::String("Voice/Speech".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sound Source".to_string()));
        data.insert("action".to_string(), Value::String("PlaySound".to_string()));
        data.insert("block".to_string(), Value::String("PlaySound".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SoundSourcePlaySound {
    fn default() -> Self {
        Self::Master
    }
}
#[derive(Debug, Clone)]
pub enum KeepCurrentRotationRandomizedTeleport {
    True,
    False,
}
impl KeepCurrentRotationRandomizedTeleport {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                KeepCurrentRotationRandomizedTeleport::True => {
                    Value::String("True".to_string())
                }
                KeepCurrentRotationRandomizedTeleport::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Keep Current Rotation".to_string()),
        );
        data.insert("action".to_string(), Value::String("RngTeleport".to_string()));
        data.insert("block".to_string(), Value::String("RngTeleport".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for KeepCurrentRotationRandomizedTeleport {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum CollisionSetCollidable {
    Enable,
    Disable,
}
impl CollisionSetCollidable {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CollisionSetCollidable::Enable => Value::String("Enable".to_string()),
                CollisionSetCollidable::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Collision".to_string()));
        data.insert("action".to_string(), Value::String("SetCollidable".to_string()));
        data.insert("block".to_string(), Value::String("SetCollidable".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CollisionSetCollidable {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum AddtoCurrentVelocityLaunchForward {
    True,
    False,
}
impl AddtoCurrentVelocityLaunchForward {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AddtoCurrentVelocityLaunchForward::True => {
                    Value::String("True".to_string())
                }
                AddtoCurrentVelocityLaunchForward::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Add to Current Velocity".to_string()),
        );
        data.insert("action".to_string(), Value::String("LaunchFwd".to_string()));
        data.insert("block".to_string(), Value::String("LaunchFwd".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AddtoCurrentVelocityLaunchForward {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum LaunchAxisLaunchForward {
    PitchandYaw,
    YawOnly,
}
impl LaunchAxisLaunchForward {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                LaunchAxisLaunchForward::PitchandYaw => {
                    Value::String("Pitch and Yaw".to_string())
                }
                LaunchAxisLaunchForward::YawOnly => Value::String("Yaw Only".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Launch Axis".to_string()));
        data.insert("action".to_string(), Value::String("LaunchFwd".to_string()));
        data.insert("block".to_string(), Value::String("LaunchFwd".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for LaunchAxisLaunchForward {
    fn default() -> Self {
        Self::PitchandYaw
    }
}
#[derive(Debug, Clone)]
pub enum BossBarSlotRemoveBossBarN {
    Allbossbars,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl BossBarSlotRemoveBossBarN {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BossBarSlotRemoveBossBarN::Allbossbars => {
                    Value::String("All boss bars".to_string())
                }
                BossBarSlotRemoveBossBarN::One => Value::String("1".to_string()),
                BossBarSlotRemoveBossBarN::Two => Value::String("2".to_string()),
                BossBarSlotRemoveBossBarN::Three => Value::String("3".to_string()),
                BossBarSlotRemoveBossBarN::Four => Value::String("4".to_string()),
                BossBarSlotRemoveBossBarN::Five => Value::String("5".to_string()),
                BossBarSlotRemoveBossBarN::Six => Value::String("6".to_string()),
                BossBarSlotRemoveBossBarN::Seven => Value::String("7".to_string()),
                BossBarSlotRemoveBossBarN::Eight => Value::String("8".to_string()),
                BossBarSlotRemoveBossBarN::Nine => Value::String("9".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Boss Bar Slot".to_string()));
        data.insert("action".to_string(), Value::String("RemoveBossBar".to_string()));
        data.insert("block".to_string(), Value::String("RemoveBossBar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BossBarSlotRemoveBossBarN {
    fn default() -> Self {
        Self::Allbossbars
    }
}
#[derive(Debug, Clone)]
pub enum EquipmentSlotSetEquipmentItem {
    Mainhand,
    Offhand,
    Head,
    Chest,
    Legs,
    Feet,
}
impl EquipmentSlotSetEquipmentItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                EquipmentSlotSetEquipmentItem::Mainhand => {
                    Value::String("Main hand".to_string())
                }
                EquipmentSlotSetEquipmentItem::Offhand => {
                    Value::String("Off hand".to_string())
                }
                EquipmentSlotSetEquipmentItem::Head => Value::String("Head".to_string()),
                EquipmentSlotSetEquipmentItem::Chest => {
                    Value::String("Chest".to_string())
                }
                EquipmentSlotSetEquipmentItem::Legs => Value::String("Legs".to_string()),
                EquipmentSlotSetEquipmentItem::Feet => Value::String("Feet".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Equipment Slot".to_string()));
        data.insert("action".to_string(), Value::String("SetEquipment".to_string()));
        data.insert("block".to_string(), Value::String("SetEquipment".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for EquipmentSlotSetEquipmentItem {
    fn default() -> Self {
        Self::Mainhand
    }
}
#[derive(Debug, Clone)]
pub enum AnimationArmSendPlayerAttackAnimation {
    Swingmainarm,
    Swingoffarm,
}
impl AnimationArmSendPlayerAttackAnimation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AnimationArmSendPlayerAttackAnimation::Swingmainarm => {
                    Value::String("Swing main arm".to_string())
                }
                AnimationArmSendPlayerAttackAnimation::Swingoffarm => {
                    Value::String("Swing off arm".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Animation Arm".to_string()));
        data.insert("action".to_string(), Value::String("AttackAnimation".to_string()));
        data.insert("block".to_string(), Value::String("AttackAnimation".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AnimationArmSendPlayerAttackAnimation {
    fn default() -> Self {
        Self::Swingmainarm
    }
}
#[derive(Debug, Clone)]
pub enum SpawnDeathDropsSetDeathDropsEnabled {
    Enable,
    Disable,
}
impl SpawnDeathDropsSetDeathDropsEnabled {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SpawnDeathDropsSetDeathDropsEnabled::Enable => {
                    Value::String("Enable".to_string())
                }
                SpawnDeathDropsSetDeathDropsEnabled::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Spawn Death Drops".to_string()));
        data.insert("action".to_string(), Value::String("SetDropsEnabled".to_string()));
        data.insert("block".to_string(), Value::String("SetDropsEnabled".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SpawnDeathDropsSetDeathDropsEnabled {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum OverwritePreviousFractureDisplayBlockFracture {
    True,
    False,
}
impl OverwritePreviousFractureDisplayBlockFracture {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                OverwritePreviousFractureDisplayBlockFracture::True => {
                    Value::String("True".to_string())
                }
                OverwritePreviousFractureDisplayBlockFracture::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Overwrite Previous Fracture".to_string()),
        );
        data.insert("action".to_string(), Value::String("DisplayFracture".to_string()));
        data.insert("block".to_string(), Value::String("DisplayFracture".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for OverwritePreviousFractureDisplayBlockFracture {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum HiddenSetEntityHidden {
    Enable,
    Disable,
}
impl HiddenSetEntityHidden {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HiddenSetEntityHidden::Enable => Value::String("Enable".to_string()),
                HiddenSetEntityHidden::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Hidden".to_string()));
        data.insert("action".to_string(), Value::String("SetEntityHidden".to_string()));
        data.insert("block".to_string(), Value::String("SetEntityHidden".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HiddenSetEntityHidden {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum SidebarSetSidebarVisible {
    Enable,
    Disable,
}
impl SidebarSetSidebarVisible {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SidebarSetSidebarVisible::Enable => Value::String("Enable".to_string()),
                SidebarSetSidebarVisible::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sidebar".to_string()));
        data.insert("action".to_string(), Value::String("SetSidebar".to_string()));
        data.insert("block".to_string(), Value::String("SetSidebar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SidebarSetSidebarVisible {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum FillTypeDisplayParticleCuboid {
    Wireframe,
    Hollow,
    Solid,
}
impl FillTypeDisplayParticleCuboid {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FillTypeDisplayParticleCuboid::Wireframe => {
                    Value::String("Wireframe".to_string())
                }
                FillTypeDisplayParticleCuboid::Hollow => {
                    Value::String("Hollow".to_string())
                }
                FillTypeDisplayParticleCuboid::Solid => {
                    Value::String("Solid".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Fill Type".to_string()));
        data.insert("action".to_string(), Value::String("ParticleCuboid".to_string()));
        data.insert("block".to_string(), Value::String("ParticleCuboid".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FillTypeDisplayParticleCuboid {
    fn default() -> Self {
        Self::Wireframe
    }
}
#[derive(Debug, Clone)]
pub enum AlignmentModeSendMessageSequence {
    Regular,
    Centered,
}
impl AlignmentModeSendMessageSequence {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AlignmentModeSendMessageSequence::Regular => {
                    Value::String("Regular".to_string())
                }
                AlignmentModeSendMessageSequence::Centered => {
                    Value::String("Centered".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Alignment Mode".to_string()));
        data.insert("action".to_string(), Value::String("SendMessageSeq".to_string()));
        data.insert("block".to_string(), Value::String("SendMessageSeq".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AlignmentModeSendMessageSequence {
    fn default() -> Self {
        Self::Regular
    }
}
#[derive(Debug, Clone)]
pub enum TextTypeSetNamePrefixSuffix {
    Prefix,
    Suffix,
}
impl TextTypeSetNamePrefixSuffix {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextTypeSetNamePrefixSuffix::Prefix => {
                    Value::String("Prefix".to_string())
                }
                TextTypeSetNamePrefixSuffix::Suffix => {
                    Value::String("Suffix".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Type".to_string()));
        data.insert("action".to_string(), Value::String("SetNamePrefix".to_string()));
        data.insert("block".to_string(), Value::String("SetNamePrefix".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextTypeSetNamePrefixSuffix {
    fn default() -> Self {
        Self::Prefix
    }
}
#[derive(Debug, Clone)]
pub enum InstantRespawnSetInstantRespawn {
    Enable,
    Disable,
}
impl InstantRespawnSetInstantRespawn {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InstantRespawnSetInstantRespawn::Enable => {
                    Value::String("Enable".to_string())
                }
                InstantRespawnSetInstantRespawn::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Instant Respawn".to_string()));
        data.insert("action".to_string(), Value::String("InstantRespawn".to_string()));
        data.insert("block".to_string(), Value::String("InstantRespawn".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InstantRespawnSetInstantRespawn {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum SoundSourceStopSounds {
    Master,
    Music,
    JukeboxNoteBlocks,
    Weather,
    Blocks,
    HostileCreatures,
    FriendlyCreatures,
    Players,
    AmbientEnvironment,
    VoiceSpeech,
}
impl SoundSourceStopSounds {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SoundSourceStopSounds::Master => Value::String("Master".to_string()),
                SoundSourceStopSounds::Music => Value::String("Music".to_string()),
                SoundSourceStopSounds::JukeboxNoteBlocks => {
                    Value::String("Jukebox/Note Blocks".to_string())
                }
                SoundSourceStopSounds::Weather => Value::String("Weather".to_string()),
                SoundSourceStopSounds::Blocks => Value::String("Blocks".to_string()),
                SoundSourceStopSounds::HostileCreatures => {
                    Value::String("Hostile Creatures".to_string())
                }
                SoundSourceStopSounds::FriendlyCreatures => {
                    Value::String("Friendly Creatures".to_string())
                }
                SoundSourceStopSounds::Players => Value::String("Players".to_string()),
                SoundSourceStopSounds::AmbientEnvironment => {
                    Value::String("Ambient/Environment".to_string())
                }
                SoundSourceStopSounds::VoiceSpeech => {
                    Value::String("Voice/Speech".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sound Source".to_string()));
        data.insert("action".to_string(), Value::String("StopSound".to_string()));
        data.insert("block".to_string(), Value::String("StopSound".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SoundSourceStopSounds {
    fn default() -> Self {
        Self::Master
    }
}
#[derive(Debug, Clone)]
pub enum NameColorSetNameColor {
    Black,
    Darkblue,
    Darkgreen,
    Darkaqua,
    Darkred,
    Darkpurple,
    Gold,
    Gray,
    Darkgray,
    Blue,
    Green,
    Aqua,
    Red,
    Lightpurple,
    Yellow,
    White,
    None,
}
impl NameColorSetNameColor {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                NameColorSetNameColor::Black => Value::String("Black".to_string()),
                NameColorSetNameColor::Darkblue => Value::String("Dark blue".to_string()),
                NameColorSetNameColor::Darkgreen => {
                    Value::String("Dark green".to_string())
                }
                NameColorSetNameColor::Darkaqua => Value::String("Dark aqua".to_string()),
                NameColorSetNameColor::Darkred => Value::String("Dark red".to_string()),
                NameColorSetNameColor::Darkpurple => {
                    Value::String("Dark purple".to_string())
                }
                NameColorSetNameColor::Gold => Value::String("Gold".to_string()),
                NameColorSetNameColor::Gray => Value::String("Gray".to_string()),
                NameColorSetNameColor::Darkgray => Value::String("Dark gray".to_string()),
                NameColorSetNameColor::Blue => Value::String("Blue".to_string()),
                NameColorSetNameColor::Green => Value::String("Green".to_string()),
                NameColorSetNameColor::Aqua => Value::String("Aqua".to_string()),
                NameColorSetNameColor::Red => Value::String("Red".to_string()),
                NameColorSetNameColor::Lightpurple => {
                    Value::String("Light purple".to_string())
                }
                NameColorSetNameColor::Yellow => Value::String("Yellow".to_string()),
                NameColorSetNameColor::White => Value::String("White".to_string()),
                NameColorSetNameColor::None => Value::String("None".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Name Color".to_string()));
        data.insert("action".to_string(), Value::String("SetNameColor".to_string()));
        data.insert("block".to_string(), Value::String("SetNameColor".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for NameColorSetNameColor {
    fn default() -> Self {
        Self::Black
    }
}
