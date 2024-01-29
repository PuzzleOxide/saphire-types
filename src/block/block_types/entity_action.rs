use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum EntityAction {
    SetDisplayRotationfromEulerAngles {
        pitch_c07c3607: Number,
        yaw_c07c3607: Number,
        roll_c07c3607: Number,
    },
    SetParrotColor {},
    SetDisplayTranslation {
        x_translation: Number,
        y_translation: Number,
        z_translation: Either<Number, Vector>,
    },
    Remove {},
    SetVelocity { new_velocity: Vector },
    SetTextDisplayBackground {
        color_hexadecimal: Option<Text>,
        opacity_in_percentage: Option<Number>,
    },
    SetDisplayCullingSize { width: Option<Number>, height: Option<Number> },
    SetGlowSquidDarkTicks { ticks: Number },
    HideName {},
    SetFrogType {},
    SetDisplayRotationfromAxisAngle { axis_vector: Vector, angle_c07c3607: Number },
    Damage {
        damage_to_inflict: Number,
        uuid_of_damager_entity: Option<Either<Text, MiniMessage>>,
    },
    SetSheepSheared {},
    SetSitting {},
    SetAxolotlPattern {},
    SendMobAnimation {},
    DisableGlowing {},
    SetWardenAngerLevel { anger_level: Number, entity_name: Either<Text, MiniMessage> },
    SetHorsePattern {},
    Heal { amount_to_heal: Option<Number> },
    SetAI {},
    SetRiptiding {},
    SetProjectileShooter { shooter_uuid: Option<Either<Text, MiniMessage>> },
    SetFoxLeaping {},
    SetItemOwner {},
    SetPandaGene {},
    SetDyeColor {},
    LaunchUp { launch_power: Number },
    SetMaximumHealth { maximum_health: Number },
    SetAnimalAge { age: Number },
    SetFishingWaitTime { wait_time_ticks: Number },
    SetEndCrystalBeam { target: Option<Location> },
    EatTarget { target_uuid: Either<Text, MiniMessage> },
    SetDisplayBrightness {
        block_light_level_c07c157: Number,
        sky_light_level_c07c157: Option<Number>,
    },
    SetVillagerProfession {},
    NoGravity {},
    SetArmsRaised {},
    ClearPotionEffects {},
    SetArmorStandParts {},
    SetInvulnerable {},
    ProjColl {},
    ArmorStandTags {},
    SetPickupDelay { delay: Number },
    SetTarget { target_uuid: Vec<Either<Text, MiniMessage>> },
    DropItems {},
    SetTextDisplayTextShadow {},
    SetCreeperExplosionPower { power_c07c257: Number },
    SetMarker {},
    RemoveCustomTag { tag_name: Text },
    SetNameVisible {},
    SetInvulnerabilityTicks { ticks: Number },
    SetAbsorptionHealth { absorption_health: Number },
    SetPose {},
    SetRearing {},
    SetCreeperCharged {},
    SetFireTicks { ticks: Number },
    SetPotionCloudRadius { radius: Number, shrinking_speed: Option<Number> },
    SetGravity {},
    SetCustomName { custom_name: Option<MiniMessage> },
    Jump {},
    SetWitherInvul { ticks: Number },
    SetBlockDisplayBlock { displayed_block: Block, block_data: Vec<BlockTag> },
    SetFreezeTicks { ticks: Number },
    Silence {},
    SetTextDisplaySeethrough {},
    SetGliding {},
    SetRotation { pitch_90_to_90: Number, yaw_180_to_180: Number },
    SetDisplayShadow {
        shadow_radius_in_blocks: Option<Number>,
        shadow_opacity_in: Option<Number>,
    },
    SetInteractionResponsive {},
    UseItem {},
    SetTropicalFishPattern {},
    RideEntity { target_uuid: Vec<Either<Text, MiniMessage>> },
    SetDisplayTransformationMatrix { numbers_describing: List },
    NoDrops {},
    SetDisplayInterpolation {
        interpolation_duration: Option<Number>,
        interpolation_delay: Option<Number>,
    },
    SetSnifferState {},
    SetHandItem {},
    EnableGlowing {},
    SetEndermanHeldBlock { block_to_hold: Block },
    Teleport { new_position: Location },
    LaunchTowardLocation { launch_destination: Location, launch_power: Option<Number> },
    SetArmorItems { armor_to_set: Vec<Item> },
    SetDisplayGlowColor { color_hexadecimal: Option<Text> },
    SetVisualFire {},
    SetAgeSize {},
    GetCustomTag { variable_to_set: Variable, tag_name: Text },
    SetInteractionSize { width: Option<Number>, height: Option<Number> },
    LSetArmor {},
    SetCatType {},
    SetWearingSaddle {},
    SetShulkerBulletTarget { target_uuid: Option<Either<Text, MiniMessage>> },
    SetDisplayScale {
        x_scale: Number,
        y_scale: Number,
        z_scale: Either<Number, Vector>,
    },
    SetTextDisplayLineWidth { line_width: Option<Number> },
    SetTextDisplayTextAlignment {},
    LaunchProjectile {
        projectile_to_launch: Projectile,
        launch_point: Option<Location>,
        projectile_name: Option<MiniMessage>,
        speed: Option<Number>,
        inaccuracy: Option<Number>,
    },
    SetDragonPhase {},
    SetLlamaColor {},
    SetVillagerBiome {},
    SetCreeperFuse { fuse_ticks: Number },
    EnableAI {},
    SettoBabyAdult {},
    SetMooshroomType {},
    SetInvisible {},
    SetDisplayBillboard {},
    NoProjColl {},
    EatGrass {},
    SetCatResting {},
    GivePotionEffect { effects: Vec<Potion> },
    SetGoatHorns {},
    Tame { owner_uuid: Option<Either<Text, MiniMessage>> },
    SetGlowing {},
    SetGoatScreaming {},
    SetItemDisplayModelType {},
    SetCurrentHealth { current_health: Number },
    DisguiseasMob { mob_to_disguise_as: SpawnEgg, display_name: Option<MiniMessage> },
    DisguiseasBlock { block_to_disguise_as: Block, display_name: Option<MiniMessage> },
    SetMinecartBlock { block_to_show: Block, block_offset: Option<Number> },
    SetFoxSleeping {},
    SetCollidable {},
    SetArmorStandPose {
        direction: Either<Vector, Number>,
        y_rotation_c07c3607: Option<Number>,
        z_rotation_c07c3607: Option<Number>,
    },
    LaunchForward { launch_power: Number },
    SetFallDistance { fall_distance_blocks: Number },
    MovetoLocation { target_location: Location, walk_speed: Option<Number> },
    SetTextDisplayTextOpacity { text_opacity: Option<Number> },
    SetItemDisplayItem { displayed_item: Item },
    SetEquipmentItem { item_to_set: Option<Item> },
    SendMobAttackAnimation {},
    SetSilenced {},
    SetBeeHasNectar {},
    AttachLead { lead_holder_uuid: Either<Either<Text, MiniMessage>, Location> },
    SetSnowGolemPumpkin {},
    SetCustomTag { tag_name: Text, tag_value: Either<Number, Text> },
    RemovePotionEffect { effects: Vec<Potion> },
    Gravity {},
    ShearSheep {},
    SetArmorStandSlotInteractions {},
    SetAllayDancing {},
    SetRabbitType {},
    SetDisplayViewRange { view_range_in_blocks: Option<Number> },
    SetSize { size: Number },
    NoAI {},
    ShowName {},
    DisguiseasPlayer {
        player_name_to_disguise_as: MiniMessage,
        display_skin: Option<Item>,
    },
    SetAngry {},
    SetEntityItem { new_item: Item },
    Explode {},
    SetWardenDigging {},
    MoveTo {},
    Undisguise {},
    SetDeathDropsEnabled {},
    SetPersistent {},
    SetVillagerExperience { experience: Number },
    IgniteCreeper {},
    SetCelebrating {},
    SetProjectileDisplayItem { display_item: Item },
    SetTextDisplayText { displayed_text: Vec<MiniMessage> },
    SetHorseJumpStrength { strength: Number },
    SetNameColor {},
    Unsilence {},
    SetCarryingChest {},
    RamTarget { target_uuid: Either<Text, MiniMessage> },
    SetFoxType {},
}
impl EntityAction {
    pub fn compile(&self) -> Value {
        match self {
            EntityAction::SetDisplayRotationfromEulerAngles {
                pitch_c07c3607,
                yaw_c07c3607,
                roll_c07c3607,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        pitch_c07c3607.json(), yaw_c07c3607.json(), roll_c07c3607.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DispRotationEuler".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetParrotColor {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParrotColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayTranslation {
                x_translation,
                y_translation,
                z_translation,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        x_translation.json(), y_translation.json(), z_translation.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DispTranslation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Remove {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Remove".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetVelocity { new_velocity } => {
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
            EntityAction::SetTextDisplayBackground {
                color_hexadecimal,
                opacity_in_percentage,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![color_hexadecimal.json(), opacity_in_percentage.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDispBackground".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayCullingSize { width, height } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![width.json(), height.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayCullingSize".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGlowSquidDarkTicks { ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGlowSquidDark".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::HideName {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HideName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFrogType {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFrogType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayRotationfromAxisAngle {
                axis_vector,
                angle_c07c3607,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![axis_vector.json(), angle_c07c3607.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DispRotAxisAngle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Damage { damage_to_inflict, uuid_of_damager_entity } => {
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
            EntityAction::SetSheepSheared {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSheepSheared".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetSitting {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMobSitting".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetAxolotlPattern {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAxolotlColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SendMobAnimation {} => {
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
            EntityAction::DisableGlowing {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisableGlowing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetWardenAngerLevel { anger_level, entity_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![anger_level.json(), entity_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetWardenAnger".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetHorsePattern {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHorsePattern".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Heal { amount_to_heal } => {
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
            EntityAction::SetAI {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAI".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetRiptiding {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetRiptiding".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetProjectileShooter { shooter_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![shooter_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetProjSource".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFoxLeaping {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFoxLeaping".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetItemOwner {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemOwner".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetPandaGene {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPandaGene".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDyeColor {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDyeColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::LaunchUp { launch_power } => {
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
            EntityAction::SetMaximumHealth { maximum_health } => {
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
            EntityAction::SetAnimalAge { age } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![age.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAge".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFishingWaitTime { wait_time_ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![wait_time_ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFishingTime".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetEndCrystalBeam { target } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![target.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EndCrystalBeam".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::EatTarget { target_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![target_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FrogEat".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayBrightness {
                block_light_level_c07c157,
                sky_light_level_c07c157,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        block_light_level_c07c157.json(), sky_light_level_c07c157.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayBrightness".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetVillagerProfession {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetProfession".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::NoGravity {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoGravity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetArmsRaised {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetArmsRaised".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::ClearPotionEffects {} => {
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
            EntityAction::SetArmorStandParts {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ArmorStandParts".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetInvulnerable {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetInvulnerable".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::ProjColl {} => {
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
            EntityAction::ArmorStandTags {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ArmorStandTags".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetPickupDelay { delay } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![delay.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPickupDelay".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTarget { target_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![target_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetTarget".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::DropItems {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DropItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplayTextShadow {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDisplayShadow".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCreeperExplosionPower { power_c07c257 } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![power_c07c257.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCreeperPower".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetMarker {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMarker".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::RemoveCustomTag { tag_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![tag_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveCustomTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetNameVisible {} => {
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
            EntityAction::SetInvulnerabilityTicks { ticks } => {
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
            EntityAction::SetAbsorptionHealth { absorption_health } => {
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
            EntityAction::SetPose {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" SetPose ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetRearing {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetRearing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCreeperCharged {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CreeperCharged".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFireTicks { ticks } => {
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
            EntityAction::SetPotionCloudRadius { radius, shrinking_speed } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![radius.json(), shrinking_speed.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCloudRadius".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGravity {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGravity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCustomName { custom_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![custom_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Jump {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Jump".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetWitherInvul { ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetWitherInvul".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetBlockDisplayBlock { displayed_block, block_data } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![displayed_block.json(), block_data.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BDisplayBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFreezeTicks { ticks } => {
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
            EntityAction::Silence {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Silence".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplaySeethrough {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDisplaySeeThru".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGliding {} => {
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
            EntityAction::SetRotation { pitch_90_to_90, yaw_180_to_180 } => {
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
            EntityAction::SetDisplayShadow {
                shadow_radius_in_blocks,
                shadow_opacity_in,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![shadow_radius_in_blocks.json(), shadow_opacity_in.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayShadow".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetInteractionResponsive {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InteractResponse".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::UseItem {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("UseItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTropicalFishPattern {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFishPattern".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::RideEntity { target_uuid } => {
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
            EntityAction::SetDisplayTransformationMatrix { numbers_describing } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![numbers_describing.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayMatrix".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::NoDrops {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoDrops".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayInterpolation {
                interpolation_duration,
                interpolation_delay,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![interpolation_duration.json(), interpolation_delay.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DispInterpolation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetSnifferState {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SnifferState".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetHandItem {} => {
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
            EntityAction::EnableGlowing {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EnableGlowing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetEndermanHeldBlock { block_to_hold } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![block_to_hold.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEndermanBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Teleport { new_position } => {
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
            EntityAction::LaunchTowardLocation { launch_destination, launch_power } => {
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
            EntityAction::SetArmorItems { armor_to_set } => {
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
            EntityAction::SetDisplayGlowColor { color_hexadecimal } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![color_hexadecimal.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayGlowColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetVisualFire {} => {
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
            EntityAction::SetAgeSize {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAge/Size".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::GetCustomTag { variable_to_set, tag_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), tag_name.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetCustomTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetInteractionSize { width, height } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![width.json(), height.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InteractionSize".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::LSetArmor {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("L SetArmor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCatType {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCatType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetWearingSaddle {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSaddle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetShulkerBulletTarget { target_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![target_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBulletTarget".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayScale { x_scale, y_scale, z_scale } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![x_scale.json(), y_scale.json(), z_scale.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayScale".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplayLineWidth { line_width } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![line_width.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDisplayLineWidth".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplayTextAlignment {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDisplayAlign".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::LaunchProjectile {
                projectile_to_launch,
                launch_point,
                projectile_name,
                speed,
                inaccuracy,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        projectile_to_launch.json(), launch_point.json(), projectile_name
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
            EntityAction::SetDragonPhase {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDragonPhase".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetLlamaColor {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetLlamaColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetVillagerBiome {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetVillagerBiome".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCreeperFuse { fuse_ticks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![fuse_ticks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCreeperFuse".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::EnableAI {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EnableAI".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SettoBabyAdult {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBaby".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetMooshroomType {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MooshroomType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetInvisible {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetInvisible".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayBillboard {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayBillboard".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::NoProjColl {} => {
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
            EntityAction::EatGrass {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SheepEat".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCatResting {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCatResting".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::GivePotionEffect { effects } => {
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
            EntityAction::SetGoatHorns {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGoatHorns".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Tame { owner_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![owner_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Tame".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGlowing {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGlowing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGoatScreaming {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGoatScreaming".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetItemDisplayModelType {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IDisplayModelType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCurrentHealth { current_health } => {
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
            EntityAction::DisguiseasMob { mob_to_disguise_as, display_name } => {
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
            EntityAction::DisguiseasBlock { block_to_disguise_as, display_name } => {
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
            EntityAction::SetMinecartBlock { block_to_show, block_offset } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![block_to_show.json(), block_offset.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMinecartBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFoxSleeping {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FoxSleeping".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCollidable {} => {
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
            EntityAction::SetArmorStandPose {
                direction,
                y_rotation_c07c3607,
                z_rotation_c07c3607,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        direction.json(), y_rotation_c07c3607.json(), z_rotation_c07c3607
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ArmorStandPose".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::LaunchForward { launch_power } => {
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
            EntityAction::SetFallDistance { fall_distance_blocks } => {
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
            EntityAction::MovetoLocation { target_location, walk_speed } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![target_location.json(), walk_speed.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MoveToLoc".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplayTextOpacity { text_opacity } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![text_opacity.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDisplayOpacity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetItemDisplayItem { displayed_item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![displayed_item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IDisplayItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetEquipmentItem { item_to_set } => {
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
            EntityAction::SendMobAttackAnimation {} => {
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
            EntityAction::SetSilenced {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSilenced".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetBeeHasNectar {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBeeNectar".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::AttachLead { lead_holder_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![lead_holder_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AttachLead".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetSnowGolemPumpkin {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SnowmanPumpkin".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCustomTag { tag_name, tag_value } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![tag_name.json(), tag_value.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCustomTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::RemovePotionEffect { effects } => {
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
            EntityAction::Gravity {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Gravity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::ShearSheep {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShearSheep".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetArmorStandSlotInteractions {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ArmorStandSlots".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetAllayDancing {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAllayDancing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetRabbitType {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetRabbitType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayViewRange { view_range_in_blocks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![view_range_in_blocks.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayViewRange".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetSize { size } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![size.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSize".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::NoAI {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoAI".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::ShowName {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShowName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::DisguiseasPlayer {
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
            EntityAction::SetAngry {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAngry".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetEntityItem { new_item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![new_item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Explode {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Explode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetWardenDigging {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDigging".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::MoveTo {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MoveTo".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Undisguise {} => {
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
            EntityAction::SetDeathDropsEnabled {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDeathDrops".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetPersistent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPersistent".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetVillagerExperience { experience } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![experience.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetVillagerExp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::IgniteCreeper {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IgniteCreeper".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCelebrating {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCelebrating".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetProjectileDisplayItem { display_item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![display_item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ProjectileItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplayText { displayed_text } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![displayed_text.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDisplayText".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetHorseJumpStrength { strength } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![strength.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHorseJump".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetNameColor {} => {
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
            EntityAction::Unsilence {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Unsilence".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCarryingChest {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCarryingChest".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::RamTarget { target_uuid } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![target_uuid.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Ram".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFoxType {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetFoxType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
