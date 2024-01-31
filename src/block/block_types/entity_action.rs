use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum EntityAction {
    SetDisplayRotationfromEulerAngles {
        pitch_c07c3607: Number,
        yaw_c07c3607: Number,
        roll_c07c3607: Number,
        rotation_type__tag: RotationTypeSetDisplayRotationfromEulerAngles,
    },
    SetParrotColor { parrot_color_tag: ParrotColorSetParrotColor },
    SetDisplayTranslation {
        x_translation: Number,
        y_translation: Number,
        z_translation: Either<Number, Vector>,
    },
    Remove {},
    SetVelocity {
        new_velocity: Vector,
        add_to_current_velocity_tag: AddtoCurrentVelocitySetVelocity,
    },
    SetTextDisplayBackground {
        color_hexadecimal: Option<Text>,
        opacity_in_percentage: Option<Number>,
    },
    SetDisplayCullingSize { width: Option<Number>, height: Option<Number> },
    SetGlowSquidDarkTicks { ticks: Number },
    HideName {},
    SetFrogType { frog_type__tag: FrogTypeSetFrogType },
    SetDisplayRotationfromAxisAngle {
        axis_vector: Vector,
        angle_c07c3607: Number,
        rotation_type__tag: RotationTypeSetDisplayRotationfromAxisAngle,
    },
    Damage {
        damage_to_inflict: Number,
        uuid_of_damager_entity: Option<Either<Text, MiniMessage>>,
    },
    SetSheepSheared { sheared_tag: ShearedSetSheepSheared },
    SetSitting { is_sitting_tag: IsSittingSetSitting },
    SetAxolotlPattern { axolotl_color_tag: AxolotlColorSetAxolotlPattern },
    SendMobAnimation { animation_type__tag: AnimationTypeSendMobAnimation },
    DisableGlowing {},
    SetWardenAngerLevel { anger_level: Number, entity_name: Either<Text, MiniMessage> },
    SetHorsePattern {
        horse_color_tag: HorseColorSetHorsePattern,
        horse_markings_tag: HorseMarkingsSetHorsePattern,
    },
    Heal { amount_to_heal: Option<Number> },
    SetAI { ai_tag: AISetAI },
    SetRiptiding { riptiding_tag: RiptidingSetRiptiding },
    SetProjectileShooter { shooter_uuid: Option<Either<Text, MiniMessage>> },
    SetFoxLeaping { leaping_tag: LeapingSetFoxLeaping },
    SetItemOwner {},
    SetPandaGene {
        set_gene_tag: SetGeneSetPandaGene,
        gene_type__tag: GeneTypeSetPandaGene,
    },
    SetDyeColor { dye_tag: DyeSetDyeColor },
    LaunchUp {
        launch_power: Number,
        add_to_current_velocity_tag: AddtoCurrentVelocityLaunchUp,
    },
    SetMaximumHealth {
        maximum_health: Number,
        heal_mob_to_max_health_tag: HealMobtoMaxHealthSetMaximumHealth,
    },
    SetAnimalAge { age: Number, age_lock_tag: AgeLockSetAnimalAge },
    SetFishingWaitTime { wait_time_ticks: Number },
    SetEndCrystalBeam { target: Option<Location> },
    EatTarget { target_uuid: Either<Text, MiniMessage> },
    SetDisplayBrightness {
        block_light_level_c07c157: Number,
        sky_light_level_c07c157: Option<Number>,
    },
    SetVillagerProfession { profession_tag: ProfessionSetVillagerProfession },
    NoGravity {},
    SetArmsRaised { arms_raised_tag: ArmsRaisedSetArmsRaised },
    ClearPotionEffects {},
    SetArmorStandParts {
        arms_tag: ArmsSetArmorStandParts,
        base_plate_tag: BasePlateSetArmorStandParts,
    },
    SetInvulnerable { invulnerable_tag: InvulnerableSetInvulnerable },
    ProjColl {},
    ArmorStandTags {
        is_visible_tag: IsVisibleArmorStandTags,
        is_marker_no_hitbox_tag: IsMarkerNoHitboxArmorStandTags,
        allow_item_taking__adding_tag: AllowItemTakingAddingArmorStandTags,
        has_physics__updates_tag: HasPhysicsUpdatesArmorStandTags,
        is_small_tag: IsSmallArmorStandTags,
        has_arms_tag: HasArmsArmorStandTags,
        has_base_plate_tag: HasBasePlateArmorStandTags,
    },
    SetPickupDelay { delay: Number },
    SetTarget { target_uuid: Vec<Either<Text, MiniMessage>> },
    DropItems {},
    SetTextDisplayTextShadow { text_shadow_tag: TextShadowSetTextDisplayTextShadow },
    SetCreeperExplosionPower { power_c07c257: Number },
    SetMarker { marker_tag: MarkerSetMarker },
    RemoveCustomTag { tag_name: Text },
    SetNameVisible { name_tag_visible_tag: NameTagVisibleSetNameVisible },
    SetInvulnerabilityTicks { ticks: Number },
    SetAbsorptionHealth { absorption_health: Number },
    SetPose { pose_tag: PoseSetPose },
    SetRearing { rearing_tag: RearingSetRearing },
    SetCreeperCharged { charged_tag: ChargedSetCreeperCharged },
    SetFireTicks { ticks: Number },
    SetPotionCloudRadius { radius: Number, shrinking_speed: Option<Number> },
    SetGravity { gravity_tag: GravitySetGravity },
    SetCustomName {
        custom_name: Option<MiniMessage>,
        hide_name_tag_tag: HideNameTagSetCustomName,
    },
    Jump {},
    SetWitherInvul { ticks: Number },
    SetBlockDisplayBlock { displayed_block: Block, block_data: Vec<BlockTag> },
    SetFreezeTicks { ticks: Number, ticking_locked_tag: TickingLockedSetFreezeTicks },
    Silence {},
    SetTextDisplaySeethrough { seethrough_tag: SeethroughSetTextDisplaySeethrough },
    SetGliding { gliding_tag: GlidingSetGliding },
    SetRotation { pitch_90_to_90: Number, yaw_180_to_180: Number },
    SetDisplayShadow {
        shadow_radius_in_blocks: Option<Number>,
        shadow_opacity_in: Option<Number>,
    },
    SetInteractionResponsive { responsive_tag: ResponsiveSetInteractionResponsive },
    UseItem { hand_tag: HandUseItem, use_item_tag: UseItemUseItem },
    SetTropicalFishPattern {
        pattern_color_tag: PatternColorSetTropicalFishPattern,
        body_color_tag: BodyColorSetTropicalFishPattern,
        pattern_tag: PatternSetTropicalFishPattern,
    },
    RideEntity { target_uuid: Vec<Either<Text, MiniMessage>> },
    SetDisplayTransformationMatrix { numbers_describing: List },
    NoDrops {},
    SetDisplayInterpolation {
        interpolation_duration: Option<Number>,
        interpolation_delay: Option<Number>,
    },
    SetSnifferState { behavior_tag: BehaviorSetSnifferState },
    SetHandItem { hand_slot_tag: HandSlotSetHandItem },
    EnableGlowing {},
    SetEndermanHeldBlock { block_to_hold: Block },
    Teleport {
        new_position: Location,
        keep_current_rotation_tag: KeepCurrentRotationTeleport,
    },
    LaunchTowardLocation {
        launch_destination: Location,
        launch_power: Option<Number>,
        add_to_current_velocity_tag: AddtoCurrentVelocityLaunchTowardLocation,
        ignore_distance_tag: IgnoreDistanceLaunchTowardLocation,
    },
    SetArmorItems { armor_to_set: Vec<Item> },
    SetDisplayGlowColor { color_hexadecimal: Option<Text> },
    SetVisualFire { on_fire_tag: OnFireSetVisualFire },
    SetAgeSize {},
    GetCustomTag { variable_to_set: VariableLiteral, tag_name: Text },
    SetInteractionSize { width: Option<Number>, height: Option<Number> },
    LSetArmor {},
    SetCatType { skin_type__tag: SkinTypeSetCatType },
    SetWearingSaddle { saddle_tag: SaddleSetWearingSaddle },
    SetShulkerBulletTarget { target_uuid: Option<Either<Text, MiniMessage>> },
    SetDisplayScale {
        x_scale: Number,
        y_scale: Number,
        z_scale: Either<Number, Vector>,
    },
    SetTextDisplayLineWidth { line_width: Option<Number> },
    SetTextDisplayTextAlignment {
        text_alignment_tag: TextAlignmentSetTextDisplayTextAlignment,
    },
    LaunchProjectile {
        projectile_to_launch: Projectile,
        launch_point: Option<Location>,
        projectile_name: Option<MiniMessage>,
        speed: Option<Number>,
        inaccuracy: Option<Number>,
    },
    SetDragonPhase { phase_tag: PhaseSetDragonPhase },
    SetLlamaColor { llama_color_tag: LlamaColorSetLlamaColor },
    SetVillagerBiome { biome_tag: BiomeSetVillagerBiome },
    SetCreeperFuse { fuse_ticks: Number },
    EnableAI {},
    SettoBabyAdult { baby_tag: BabySettoBabyAdult },
    SetMooshroomType { mooshroom_variant_tag: MooshroomVariantSetMooshroomType },
    SetInvisible { invisible_tag: InvisibleSetInvisible },
    SetDisplayBillboard { billboard_type__tag: BillboardTypeSetDisplayBillboard },
    NoProjColl {},
    EatGrass {},
    SetCatResting { resting_tag: RestingSetCatResting },
    GivePotionEffect {
        effects: Vec<Potion>,
        overwrite_effect_tag: OverwriteEffectGivePotionEffect,
        effect_particles_tag: EffectParticlesGivePotionEffect,
    },
    SetGoatHorns {
        left_horn_tag: LeftHornSetGoatHorns,
        right_horn_tag: RightHornSetGoatHorns,
    },
    Tame { owner_uuid: Option<Either<Text, MiniMessage>> },
    SetGlowing { glowing_tag: GlowingSetGlowing },
    SetGoatScreaming { screams_tag: ScreamsSetGoatScreaming },
    SetItemDisplayModelType { model_type__tag: ModelTypeSetItemDisplayModelType },
    SetCurrentHealth { current_health: Number },
    DisguiseasMob { mob_to_disguise_as: SpawnEgg, display_name: Option<MiniMessage> },
    DisguiseasBlock { block_to_disguise_as: Block, display_name: Option<MiniMessage> },
    SetMinecartBlock { block_to_show: Block, block_offset: Option<Number> },
    SetFoxSleeping { sleeping_tag: SleepingSetFoxSleeping },
    SetCollidable { collision_tag: CollisionSetCollidable },
    SetArmorStandPose {
        direction: Either<Vector, Number>,
        y_rotation_c07c3607: Option<Number>,
        z_rotation_c07c3607: Option<Number>,
        armor_stand_part_tag: ArmorStandPartSetArmorStandPose,
    },
    LaunchForward {
        launch_power: Number,
        add_to_current_velocity_tag: AddtoCurrentVelocityLaunchForward,
        launch_axis_tag: LaunchAxisLaunchForward,
    },
    SetFallDistance { fall_distance_blocks: Number },
    MovetoLocation { target_location: Location, walk_speed: Option<Number> },
    SetTextDisplayTextOpacity { text_opacity: Option<Number> },
    SetItemDisplayItem { displayed_item: Item },
    SetEquipmentItem {
        item_to_set: Option<Item>,
        equipment_slot_tag: EquipmentSlotSetEquipmentItem,
    },
    SendMobAttackAnimation { animation_arm_tag: AnimationArmSendMobAttackAnimation },
    SetSilenced { silenced_tag: SilencedSetSilenced },
    SetBeeHasNectar { has_nectar_tag: HasNectarSetBeeHasNectar },
    AttachLead { lead_holder_uuid: Either<Either<Text, MiniMessage>, Location> },
    SetSnowGolemPumpkin { pumpkin_tag: PumpkinSetSnowGolemPumpkin },
    SetCustomTag { tag_name: Text, tag_value: Either<Number, Text> },
    RemovePotionEffect { effects: Vec<Potion> },
    Gravity {},
    ShearSheep {},
    SetArmorStandSlotInteractions {
        interactions_tag: InteractionsSetArmorStandSlotInteractions,
        equipment_slot_tag: EquipmentSlotSetArmorStandSlotInteractions,
    },
    SetAllayDancing { dancing_tag: DancingSetAllayDancing },
    SetRabbitType { skin_type__tag: SkinTypeSetRabbitType },
    SetDisplayViewRange { view_range_in_blocks: Option<Number> },
    SetSize { size: Number },
    NoAI {},
    ShowName {},
    DisguiseasPlayer {
        player_name_to_disguise_as: MiniMessage,
        display_skin: Option<Item>,
    },
    SetAngry { angry_tag: AngrySetAngry },
    SetEntityItem { new_item: Item },
    Explode {},
    SetWardenDigging { digging_type__tag: DiggingTypeSetWardenDigging },
    MoveTo {},
    Undisguise {},
    SetDeathDropsEnabled { has_death_drops_tag: HasDeathDropsSetDeathDropsEnabled },
    SetPersistent { persistent_tag: PersistentSetPersistent },
    SetVillagerExperience { experience: Number },
    IgniteCreeper {},
    SetCelebrating { celebrate_tag: CelebrateSetCelebrating },
    SetProjectileDisplayItem { display_item: Item },
    SetTextDisplayText {
        displayed_text: Vec<MiniMessage>,
        text_value_merging_tag: TextValueMergingSetTextDisplayText,
        inherit_styles_tag: InheritStylesSetTextDisplayText,
    },
    SetHorseJumpStrength { strength: Number },
    SetNameColor { name_color_tag: NameColorSetNameColor },
    Unsilence {},
    SetCarryingChest { carrying_chest_tag: CarryingChestSetCarryingChest },
    RamTarget { target_uuid: Either<Text, MiniMessage> },
    SetFoxType { fox_type__tag: FoxTypeSetFoxType },
}
impl EntityAction {
    pub fn compile(&self) -> Value {
        match self {
            EntityAction::SetDisplayRotationfromEulerAngles {
                pitch_c07c3607,
                yaw_c07c3607,
                roll_c07c3607,
                rotation_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        pitch_c07c3607.json(), yaw_c07c3607.json(), roll_c07c3607.json()
                    ],
                    vec![rotation_type__tag.json()],
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
            EntityAction::SetParrotColor { parrot_color_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![parrot_color_tag.json()]);
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Remove".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetVelocity { new_velocity, add_to_current_velocity_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
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
            EntityAction::SetTextDisplayBackground {
                color_hexadecimal,
                opacity_in_percentage,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![color_hexadecimal.json(), opacity_in_percentage.json()],
                    vec![],
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
                let item_args = compile(vec![width.json(), height.json()], vec![]);
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
                let item_args = compile(vec![ticks.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HideName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFrogType { frog_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![frog_type__tag.json()]);
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
                rotation_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![axis_vector.json(), angle_c07c3607.json()],
                    vec![rotation_type__tag.json()],
                );
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
            EntityAction::SetSheepSheared { sheared_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![sheared_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSheepSheared".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetSitting { is_sitting_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![is_sitting_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMobSitting".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetAxolotlPattern { axolotl_color_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![axolotl_color_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAxolotlColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SendMobAnimation { animation_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![animation_type__tag.json()]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(
                    vec![anger_level.json(), entity_name.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetWardenAnger".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetHorsePattern { horse_color_tag, horse_markings_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![horse_color_tag.json(), horse_markings_tag.json()],
                );
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
                let item_args = compile(vec![amount_to_heal.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Heal".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetAI { ai_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![ai_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAI".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetRiptiding { riptiding_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![riptiding_tag.json()]);
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
                let item_args = compile(vec![shooter_uuid.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetProjSource".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFoxLeaping { leaping_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![leaping_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemOwner".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetPandaGene { set_gene_tag, gene_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![set_gene_tag.json(), gene_type__tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPandaGene".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDyeColor { dye_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![dye_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDyeColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::LaunchUp { launch_power, add_to_current_velocity_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
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
            EntityAction::SetMaximumHealth {
                maximum_health,
                heal_mob_to_max_health_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![maximum_health.json()],
                    vec![heal_mob_to_max_health_tag.json()],
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
            EntityAction::SetAnimalAge { age, age_lock_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![age.json()], vec![age_lock_tag.json()]);
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
                let item_args = compile(vec![wait_time_ticks.json()], vec![]);
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
                let item_args = compile(vec![target.json()], vec![]);
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
                let item_args = compile(vec![target_uuid.json()], vec![]);
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
                    vec![],
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
            EntityAction::SetVillagerProfession { profession_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![profession_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NoGravity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetArmsRaised { arms_raised_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![arms_raised_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearPotions".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetArmorStandParts { arms_tag, base_plate_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![arms_tag.json(), base_plate_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ArmorStandParts".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetInvulnerable { invulnerable_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![invulnerable_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ProjColl".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::ArmorStandTags {
                is_visible_tag,
                is_marker_no_hitbox_tag,
                allow_item_taking__adding_tag,
                has_physics__updates_tag,
                is_small_tag,
                has_arms_tag,
                has_base_plate_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![
                        is_visible_tag.json(), is_marker_no_hitbox_tag.json(),
                        allow_item_taking__adding_tag.json(), has_physics__updates_tag
                        .json(), is_small_tag.json(), has_arms_tag.json(),
                        has_base_plate_tag.json()
                    ],
                );
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
                let item_args = compile(vec![delay.json()], vec![]);
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
                let item_args = compile(vec![target_uuid.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DropItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplayTextShadow { text_shadow_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![text_shadow_tag.json()]);
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
                let item_args = compile(vec![power_c07c257.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCreeperPower".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetMarker { marker_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![marker_tag.json()]);
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
                let item_args = compile(vec![tag_name.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveCustomTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetNameVisible { name_tag_visible_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![name_tag_visible_tag.json()]);
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
                let item_args = compile(vec![ticks.json()], vec![]);
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
                let item_args = compile(vec![absorption_health.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAbsorption".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetPose { pose_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![pose_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" SetPose ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetRearing { rearing_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![rearing_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetRearing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCreeperCharged { charged_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![charged_tag.json()]);
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
                let item_args = compile(vec![ticks.json()], vec![]);
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
                let item_args = compile(
                    vec![radius.json(), shrinking_speed.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCloudRadius".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGravity { gravity_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![gravity_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGravity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCustomName { custom_name, hide_name_tag_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![custom_name.json()],
                    vec![hide_name_tag_tag.json()],
                );
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![ticks.json()], vec![]);
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
                let item_args = compile(
                    vec![displayed_block.json(), block_data.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BDisplayBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFreezeTicks { ticks, ticking_locked_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
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
            EntityAction::Silence {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Silence".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplaySeethrough { seethrough_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![seethrough_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDisplaySeeThru".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGliding { gliding_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![gliding_tag.json()]);
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
            EntityAction::SetDisplayShadow {
                shadow_radius_in_blocks,
                shadow_opacity_in,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![shadow_radius_in_blocks.json(), shadow_opacity_in.json()],
                    vec![],
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
            EntityAction::SetInteractionResponsive { responsive_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![responsive_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InteractResponse".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::UseItem { hand_tag, use_item_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![hand_tag.json(), use_item_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("UseItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTropicalFishPattern {
                pattern_color_tag,
                body_color_tag,
                pattern_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![
                        pattern_color_tag.json(), body_color_tag.json(), pattern_tag
                        .json()
                    ],
                );
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
                let item_args = compile(vec![target_uuid.json()], vec![]);
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
                let item_args = compile(vec![numbers_describing.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
            EntityAction::SetSnifferState { behavior_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![behavior_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SnifferState".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetHandItem { hand_slot_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![hand_slot_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![block_to_hold.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetEndermanBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::Teleport { new_position, keep_current_rotation_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![new_position.json()],
                    vec![keep_current_rotation_tag.json()],
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
            EntityAction::LaunchTowardLocation {
                launch_destination,
                launch_power,
                add_to_current_velocity_tag,
                ignore_distance_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
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
            EntityAction::SetArmorItems { armor_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![armor_to_set.json()], vec![]);
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
                let item_args = compile(vec![color_hexadecimal.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DisplayGlowColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetVisualFire { on_fire_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![on_fire_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(
                    vec![variable_to_set.json(), tag_name.json()],
                    vec![],
                );
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
                let item_args = compile(vec![width.json(), height.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("L SetArmor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCatType { skin_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![skin_type__tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCatType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetWearingSaddle { saddle_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![saddle_tag.json()]);
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
                let item_args = compile(vec![target_uuid.json()], vec![]);
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
                    vec![],
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
                let item_args = compile(vec![line_width.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TDisplayLineWidth".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplayTextAlignment { text_alignment_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![text_alignment_tag.json()]);
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
            EntityAction::SetDragonPhase { phase_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![phase_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDragonPhase".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetLlamaColor { llama_color_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![llama_color_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetLlamaColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetVillagerBiome { biome_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![biome_tag.json()]);
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
                let item_args = compile(vec![fuse_ticks.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EnableAI".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SettoBabyAdult { baby_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![baby_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBaby".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetMooshroomType { mooshroom_variant_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![mooshroom_variant_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MooshroomType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetInvisible { invisible_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![invisible_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetInvisible".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDisplayBillboard { billboard_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![billboard_type__tag.json()]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SheepEat".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCatResting { resting_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![resting_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCatResting".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::GivePotionEffect {
                effects,
                overwrite_effect_tag,
                effect_particles_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![effects.json()],
                    vec![overwrite_effect_tag.json(), effect_particles_tag.json()],
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
            EntityAction::SetGoatHorns { left_horn_tag, right_horn_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![left_horn_tag.json(), right_horn_tag.json()],
                );
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
                let item_args = compile(vec![owner_uuid.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Tame".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGlowing { glowing_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![glowing_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGlowing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetGoatScreaming { screams_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![screams_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetGoatScreaming".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetItemDisplayModelType { model_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![model_type__tag.json()]);
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
                let item_args = compile(vec![current_health.json()], vec![]);
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
            EntityAction::DisguiseasBlock { block_to_disguise_as, display_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
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
            EntityAction::SetMinecartBlock { block_to_show, block_offset } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![block_to_show.json(), block_offset.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMinecartBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFoxSleeping { sleeping_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![sleeping_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FoxSleeping".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCollidable { collision_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![collision_tag.json()]);
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
                armor_stand_part_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        direction.json(), y_rotation_c07c3607.json(), z_rotation_c07c3607
                        .json()
                    ],
                    vec![armor_stand_part_tag.json()],
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
            EntityAction::LaunchForward {
                launch_power,
                add_to_current_velocity_tag,
                launch_axis_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
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
            EntityAction::SetFallDistance { fall_distance_blocks } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![fall_distance_blocks.json()], vec![]);
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
                let item_args = compile(
                    vec![target_location.json(), walk_speed.json()],
                    vec![],
                );
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
                let item_args = compile(vec![text_opacity.json()], vec![]);
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
                let item_args = compile(vec![displayed_item.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IDisplayItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetEquipmentItem { item_to_set, equipment_slot_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
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
            EntityAction::SendMobAttackAnimation { animation_arm_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![animation_arm_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AttackAnimation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetSilenced { silenced_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![silenced_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSilenced".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetBeeHasNectar { has_nectar_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![has_nectar_tag.json()]);
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
                let item_args = compile(vec![lead_holder_uuid.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AttachLead".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetSnowGolemPumpkin { pumpkin_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![pumpkin_tag.json()]);
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
                let item_args = compile(vec![tag_name.json(), tag_value.json()], vec![]);
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
                let item_args = compile(vec![effects.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShearSheep".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetArmorStandSlotInteractions {
                interactions_tag,
                equipment_slot_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![interactions_tag.json(), equipment_slot_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ArmorStandSlots".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetAllayDancing { dancing_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![dancing_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAllayDancing".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetRabbitType { skin_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![skin_type__tag.json()]);
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
                let item_args = compile(vec![view_range_in_blocks.json()], vec![]);
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
                let item_args = compile(vec![size.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
            EntityAction::SetAngry { angry_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![angry_tag.json()]);
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
                let item_args = compile(vec![new_item.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Explode".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetWardenDigging { digging_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![digging_type__tag.json()]);
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Undisguise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetDeathDropsEnabled { has_death_drops_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![has_death_drops_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDeathDrops".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetPersistent { persistent_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![persistent_tag.json()]);
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
                let item_args = compile(vec![experience.json()], vec![]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IgniteCreeper".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCelebrating { celebrate_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![celebrate_tag.json()]);
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
                let item_args = compile(vec![display_item.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ProjectileItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetTextDisplayText {
                displayed_text,
                text_value_merging_tag,
                inherit_styles_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![displayed_text.json()],
                    vec![text_value_merging_tag.json(), inherit_styles_tag.json()],
                );
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
                let item_args = compile(vec![strength.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHorseJump".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetNameColor { name_color_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![name_color_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Unsilence".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetCarryingChest { carrying_chest_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![carrying_chest_tag.json()]);
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
                let item_args = compile(vec![target_uuid.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Ram".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            EntityAction::SetFoxType { fox_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![fox_type__tag.json()]);
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
#[derive(Debug, Clone)]
pub enum RotationTypeSetDisplayRotationfromEulerAngles {
    LeftRotation,
    RightRotation,
}
impl RotationTypeSetDisplayRotationfromEulerAngles {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RotationTypeSetDisplayRotationfromEulerAngles::LeftRotation => {
                    Value::String("Left Rotation".to_string())
                }
                RotationTypeSetDisplayRotationfromEulerAngles::RightRotation => {
                    Value::String("Right Rotation".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Rotation Type".to_string()));
        data.insert(
            "action".to_string(),
            Value::String("DispRotationEuler".to_string()),
        );
        data.insert("block".to_string(), Value::String("DispRotationEuler".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RotationTypeSetDisplayRotationfromEulerAngles {
    fn default() -> Self {
        Self::LeftRotation
    }
}
#[derive(Debug, Clone)]
pub enum ParrotColorSetParrotColor {
    Red,
    Blue,
    Green,
    Cyan,
    Gray,
}
impl ParrotColorSetParrotColor {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ParrotColorSetParrotColor::Red => Value::String("Red".to_string()),
                ParrotColorSetParrotColor::Blue => Value::String("Blue".to_string()),
                ParrotColorSetParrotColor::Green => Value::String("Green".to_string()),
                ParrotColorSetParrotColor::Cyan => Value::String("Cyan".to_string()),
                ParrotColorSetParrotColor::Gray => Value::String("Gray".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Parrot Color".to_string()));
        data.insert("action".to_string(), Value::String("SetParrotColor".to_string()));
        data.insert("block".to_string(), Value::String("SetParrotColor".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ParrotColorSetParrotColor {
    fn default() -> Self {
        Self::Red
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
pub enum FrogTypeSetFrogType {
    Temperate,
    Warm,
    Cold,
}
impl FrogTypeSetFrogType {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FrogTypeSetFrogType::Temperate => Value::String("Temperate".to_string()),
                FrogTypeSetFrogType::Warm => Value::String("Warm".to_string()),
                FrogTypeSetFrogType::Cold => Value::String("Cold".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Frog Type".to_string()));
        data.insert("action".to_string(), Value::String("SetFrogType".to_string()));
        data.insert("block".to_string(), Value::String("SetFrogType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FrogTypeSetFrogType {
    fn default() -> Self {
        Self::Temperate
    }
}
#[derive(Debug, Clone)]
pub enum RotationTypeSetDisplayRotationfromAxisAngle {
    LeftRotation,
    RightRotation,
}
impl RotationTypeSetDisplayRotationfromAxisAngle {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RotationTypeSetDisplayRotationfromAxisAngle::LeftRotation => {
                    Value::String("Left Rotation".to_string())
                }
                RotationTypeSetDisplayRotationfromAxisAngle::RightRotation => {
                    Value::String("Right Rotation".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Rotation Type".to_string()));
        data.insert("action".to_string(), Value::String("DispRotAxisAngle".to_string()));
        data.insert("block".to_string(), Value::String("DispRotAxisAngle".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RotationTypeSetDisplayRotationfromAxisAngle {
    fn default() -> Self {
        Self::LeftRotation
    }
}
#[derive(Debug, Clone)]
pub enum ShearedSetSheepSheared {
    Enable,
    Disable,
}
impl ShearedSetSheepSheared {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShearedSetSheepSheared::Enable => Value::String("Enable".to_string()),
                ShearedSetSheepSheared::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sheared".to_string()));
        data.insert("action".to_string(), Value::String("SetSheepSheared".to_string()));
        data.insert("block".to_string(), Value::String("SetSheepSheared".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShearedSetSheepSheared {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum IsSittingSetSitting {
    Enable,
    Disable,
}
impl IsSittingSetSitting {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IsSittingSetSitting::Enable => Value::String("Enable".to_string()),
                IsSittingSetSitting::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Is Sitting".to_string()));
        data.insert("action".to_string(), Value::String("SetMobSitting".to_string()));
        data.insert("block".to_string(), Value::String("SetMobSitting".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IsSittingSetSitting {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum AxolotlColorSetAxolotlPattern {
    Pink,
    Brown,
    Yellow,
    Cyan,
    Blue,
}
impl AxolotlColorSetAxolotlPattern {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AxolotlColorSetAxolotlPattern::Pink => Value::String("Pink".to_string()),
                AxolotlColorSetAxolotlPattern::Brown => {
                    Value::String("Brown".to_string())
                }
                AxolotlColorSetAxolotlPattern::Yellow => {
                    Value::String("Yellow".to_string())
                }
                AxolotlColorSetAxolotlPattern::Cyan => Value::String("Cyan".to_string()),
                AxolotlColorSetAxolotlPattern::Blue => Value::String("Blue".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Axolotl Color".to_string()));
        data.insert("action".to_string(), Value::String("SetAxolotlColor".to_string()));
        data.insert("block".to_string(), Value::String("SetAxolotlColor".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AxolotlColorSetAxolotlPattern {
    fn default() -> Self {
        Self::Pink
    }
}
#[derive(Debug, Clone)]
pub enum AnimationTypeSendMobAnimation {
    Hurtanimation,
    Critparticles,
    Enchantedhitparticles,
}
impl AnimationTypeSendMobAnimation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AnimationTypeSendMobAnimation::Hurtanimation => {
                    Value::String("Hurt animation".to_string())
                }
                AnimationTypeSendMobAnimation::Critparticles => {
                    Value::String("Crit particles".to_string())
                }
                AnimationTypeSendMobAnimation::Enchantedhitparticles => {
                    Value::String("Enchanted hit particles".to_string())
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
impl Default for AnimationTypeSendMobAnimation {
    fn default() -> Self {
        Self::Hurtanimation
    }
}
#[derive(Debug, Clone)]
pub enum HorseColorSetHorsePattern {
    White,
    Buckskin,
    Flaxenchestnut,
    Bay,
    Black,
    Dapplegray,
    Darkbay,
    Dontchange,
}
impl HorseColorSetHorsePattern {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HorseColorSetHorsePattern::White => Value::String("White".to_string()),
                HorseColorSetHorsePattern::Buckskin => {
                    Value::String("Buckskin".to_string())
                }
                HorseColorSetHorsePattern::Flaxenchestnut => {
                    Value::String("Flaxen chestnut".to_string())
                }
                HorseColorSetHorsePattern::Bay => Value::String("Bay".to_string()),
                HorseColorSetHorsePattern::Black => Value::String("Black".to_string()),
                HorseColorSetHorsePattern::Dapplegray => {
                    Value::String("Dapple gray".to_string())
                }
                HorseColorSetHorsePattern::Darkbay => {
                    Value::String("Dark bay".to_string())
                }
                HorseColorSetHorsePattern::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Horse Color".to_string()));
        data.insert("action".to_string(), Value::String("SetHorsePattern".to_string()));
        data.insert("block".to_string(), Value::String("SetHorsePattern".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HorseColorSetHorsePattern {
    fn default() -> Self {
        Self::Flaxenchestnut
    }
}
#[derive(Debug, Clone)]
pub enum HorseMarkingsSetHorsePattern {
    Nomarkings,
    Stockingsandblaze,
    Paint,
    Snowflakeappaloosa,
    Sooty,
    Dontchange,
}
impl HorseMarkingsSetHorsePattern {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HorseMarkingsSetHorsePattern::Nomarkings => {
                    Value::String("No markings".to_string())
                }
                HorseMarkingsSetHorsePattern::Stockingsandblaze => {
                    Value::String("Stockings and blaze".to_string())
                }
                HorseMarkingsSetHorsePattern::Paint => Value::String("Paint".to_string()),
                HorseMarkingsSetHorsePattern::Snowflakeappaloosa => {
                    Value::String("Snowflake appaloosa".to_string())
                }
                HorseMarkingsSetHorsePattern::Sooty => Value::String("Sooty".to_string()),
                HorseMarkingsSetHorsePattern::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Horse Markings".to_string()));
        data.insert("action".to_string(), Value::String("SetHorsePattern".to_string()));
        data.insert("block".to_string(), Value::String("SetHorsePattern".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HorseMarkingsSetHorsePattern {
    fn default() -> Self {
        Self::Stockingsandblaze
    }
}
#[derive(Debug, Clone)]
pub enum AISetAI {
    Sentient,
    Insentient,
    None,
}
impl AISetAI {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AISetAI::Sentient => Value::String("Sentient".to_string()),
                AISetAI::Insentient => Value::String("Insentient".to_string()),
                AISetAI::None => Value::String("None".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("AI".to_string()));
        data.insert("action".to_string(), Value::String("SetAI".to_string()));
        data.insert("block".to_string(), Value::String("SetAI".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AISetAI {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone)]
pub enum RiptidingSetRiptiding {
    Enable,
    Disable,
}
impl RiptidingSetRiptiding {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RiptidingSetRiptiding::Enable => Value::String("Enable".to_string()),
                RiptidingSetRiptiding::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Riptiding".to_string()));
        data.insert("action".to_string(), Value::String("SetRiptiding".to_string()));
        data.insert("block".to_string(), Value::String("SetRiptiding".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RiptidingSetRiptiding {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum LeapingSetFoxLeaping {
    Enable,
    Disable,
}
impl LeapingSetFoxLeaping {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                LeapingSetFoxLeaping::Enable => Value::String("Enable".to_string()),
                LeapingSetFoxLeaping::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Leaping".to_string()));
        data.insert("action".to_string(), Value::String("SetFoxLeaping".to_string()));
        data.insert("block".to_string(), Value::String("SetFoxLeaping".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for LeapingSetFoxLeaping {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum SetGeneSetPandaGene {
    Maingene,
    Hiddengene,
    Both,
}
impl SetGeneSetPandaGene {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SetGeneSetPandaGene::Maingene => Value::String("Main gene".to_string()),
                SetGeneSetPandaGene::Hiddengene => {
                    Value::String("Hidden gene".to_string())
                }
                SetGeneSetPandaGene::Both => Value::String("Both".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Set Gene".to_string()));
        data.insert("action".to_string(), Value::String("SetPandaGene".to_string()));
        data.insert("block".to_string(), Value::String("SetPandaGene".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SetGeneSetPandaGene {
    fn default() -> Self {
        Self::Both
    }
}
#[derive(Debug, Clone)]
pub enum GeneTypeSetPandaGene {
    Aggressive,
    Lazy,
    Weak,
    Worried,
    Playful,
    Normal,
    Brown,
}
impl GeneTypeSetPandaGene {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GeneTypeSetPandaGene::Aggressive => {
                    Value::String("Aggressive".to_string())
                }
                GeneTypeSetPandaGene::Lazy => Value::String("Lazy".to_string()),
                GeneTypeSetPandaGene::Weak => Value::String("Weak".to_string()),
                GeneTypeSetPandaGene::Worried => Value::String("Worried".to_string()),
                GeneTypeSetPandaGene::Playful => Value::String("Playful".to_string()),
                GeneTypeSetPandaGene::Normal => Value::String("Normal".to_string()),
                GeneTypeSetPandaGene::Brown => Value::String("Brown".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Gene Type".to_string()));
        data.insert("action".to_string(), Value::String("SetPandaGene".to_string()));
        data.insert("block".to_string(), Value::String("SetPandaGene".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GeneTypeSetPandaGene {
    fn default() -> Self {
        Self::Aggressive
    }
}
#[derive(Debug, Clone)]
pub enum DyeSetDyeColor {
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
impl DyeSetDyeColor {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DyeSetDyeColor::White => Value::String("White".to_string()),
                DyeSetDyeColor::Orange => Value::String("Orange".to_string()),
                DyeSetDyeColor::Magenta => Value::String("Magenta".to_string()),
                DyeSetDyeColor::Lightblue => Value::String("Light blue".to_string()),
                DyeSetDyeColor::Yellow => Value::String("Yellow".to_string()),
                DyeSetDyeColor::Lime => Value::String("Lime".to_string()),
                DyeSetDyeColor::Pink => Value::String("Pink".to_string()),
                DyeSetDyeColor::Gray => Value::String("Gray".to_string()),
                DyeSetDyeColor::Lightgray => Value::String("Light gray".to_string()),
                DyeSetDyeColor::Cyan => Value::String("Cyan".to_string()),
                DyeSetDyeColor::Purple => Value::String("Purple".to_string()),
                DyeSetDyeColor::Blue => Value::String("Blue".to_string()),
                DyeSetDyeColor::Brown => Value::String("Brown".to_string()),
                DyeSetDyeColor::Green => Value::String("Green".to_string()),
                DyeSetDyeColor::Red => Value::String("Red".to_string()),
                DyeSetDyeColor::Black => Value::String("Black".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Dye".to_string()));
        data.insert("action".to_string(), Value::String("SetDyeColor".to_string()));
        data.insert("block".to_string(), Value::String("SetDyeColor".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DyeSetDyeColor {
    fn default() -> Self {
        Self::White
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
pub enum HealMobtoMaxHealthSetMaximumHealth {
    True,
    False,
}
impl HealMobtoMaxHealthSetMaximumHealth {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HealMobtoMaxHealthSetMaximumHealth::True => {
                    Value::String("True".to_string())
                }
                HealMobtoMaxHealthSetMaximumHealth::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Heal Mob to Max Health".to_string()),
        );
        data.insert("action".to_string(), Value::String("SetMaxHealth".to_string()));
        data.insert("block".to_string(), Value::String("SetMaxHealth".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HealMobtoMaxHealthSetMaximumHealth {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum AgeLockSetAnimalAge {
    Enable,
    Disable,
    Dontchange,
}
impl AgeLockSetAnimalAge {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AgeLockSetAnimalAge::Enable => Value::String("Enable".to_string()),
                AgeLockSetAnimalAge::Disable => Value::String("Disable".to_string()),
                AgeLockSetAnimalAge::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Age Lock".to_string()));
        data.insert("action".to_string(), Value::String("SetAge".to_string()));
        data.insert("block".to_string(), Value::String("SetAge".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AgeLockSetAnimalAge {
    fn default() -> Self {
        Self::Dontchange
    }
}
#[derive(Debug, Clone)]
pub enum ProfessionSetVillagerProfession {
    Unemployed,
    Armorer,
    Butcher,
    Cartographer,
    Cleric,
    Farmer,
    Fisherman,
    Fletcher,
    Leatherworker,
    Librarian,
    Mason,
    Nitwit,
    Shepherd,
    Toolsmith,
    Weaponsmith,
}
impl ProfessionSetVillagerProfession {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ProfessionSetVillagerProfession::Unemployed => {
                    Value::String("Unemployed".to_string())
                }
                ProfessionSetVillagerProfession::Armorer => {
                    Value::String("Armorer".to_string())
                }
                ProfessionSetVillagerProfession::Butcher => {
                    Value::String("Butcher".to_string())
                }
                ProfessionSetVillagerProfession::Cartographer => {
                    Value::String("Cartographer".to_string())
                }
                ProfessionSetVillagerProfession::Cleric => {
                    Value::String("Cleric".to_string())
                }
                ProfessionSetVillagerProfession::Farmer => {
                    Value::String("Farmer".to_string())
                }
                ProfessionSetVillagerProfession::Fisherman => {
                    Value::String("Fisherman".to_string())
                }
                ProfessionSetVillagerProfession::Fletcher => {
                    Value::String("Fletcher".to_string())
                }
                ProfessionSetVillagerProfession::Leatherworker => {
                    Value::String("Leatherworker".to_string())
                }
                ProfessionSetVillagerProfession::Librarian => {
                    Value::String("Librarian".to_string())
                }
                ProfessionSetVillagerProfession::Mason => {
                    Value::String("Mason".to_string())
                }
                ProfessionSetVillagerProfession::Nitwit => {
                    Value::String("Nitwit".to_string())
                }
                ProfessionSetVillagerProfession::Shepherd => {
                    Value::String("Shepherd".to_string())
                }
                ProfessionSetVillagerProfession::Toolsmith => {
                    Value::String("Toolsmith".to_string())
                }
                ProfessionSetVillagerProfession::Weaponsmith => {
                    Value::String("Weaponsmith".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Profession".to_string()));
        data.insert("action".to_string(), Value::String("SetProfession".to_string()));
        data.insert("block".to_string(), Value::String("SetProfession".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ProfessionSetVillagerProfession {
    fn default() -> Self {
        Self::Armorer
    }
}
#[derive(Debug, Clone)]
pub enum ArmsRaisedSetArmsRaised {
    Enable,
    Disable,
}
impl ArmsRaisedSetArmsRaised {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ArmsRaisedSetArmsRaised::Enable => Value::String("Enable".to_string()),
                ArmsRaisedSetArmsRaised::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Arms Raised".to_string()));
        data.insert("action".to_string(), Value::String("SetArmsRaised".to_string()));
        data.insert("block".to_string(), Value::String("SetArmsRaised".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ArmsRaisedSetArmsRaised {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum ArmsSetArmorStandParts {
    Enable,
    Disable,
    Dontchange,
}
impl ArmsSetArmorStandParts {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ArmsSetArmorStandParts::Enable => Value::String("Enable".to_string()),
                ArmsSetArmorStandParts::Disable => Value::String("Disable".to_string()),
                ArmsSetArmorStandParts::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Arms".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandParts".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandParts".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ArmsSetArmorStandParts {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum BasePlateSetArmorStandParts {
    Enable,
    Disable,
    Dontchange,
}
impl BasePlateSetArmorStandParts {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BasePlateSetArmorStandParts::Enable => {
                    Value::String("Enable".to_string())
                }
                BasePlateSetArmorStandParts::Disable => {
                    Value::String("Disable".to_string())
                }
                BasePlateSetArmorStandParts::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Base Plate".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandParts".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandParts".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BasePlateSetArmorStandParts {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum InvulnerableSetInvulnerable {
    Enable,
    Disable,
}
impl InvulnerableSetInvulnerable {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InvulnerableSetInvulnerable::Enable => {
                    Value::String("Enable".to_string())
                }
                InvulnerableSetInvulnerable::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Invulnerable".to_string()));
        data.insert("action".to_string(), Value::String("SetInvulnerable".to_string()));
        data.insert("block".to_string(), Value::String("SetInvulnerable".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InvulnerableSetInvulnerable {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum IsVisibleArmorStandTags {
    True,
    False,
    Dontchange,
}
impl IsVisibleArmorStandTags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IsVisibleArmorStandTags::True => Value::String("True".to_string()),
                IsVisibleArmorStandTags::False => Value::String("False".to_string()),
                IsVisibleArmorStandTags::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Is Visible".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandTags".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandTags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IsVisibleArmorStandTags {
    fn default() -> Self {
        Self::Dontchange
    }
}
#[derive(Debug, Clone)]
pub enum IsMarkerNoHitboxArmorStandTags {
    True,
    False,
    Dontchange,
}
impl IsMarkerNoHitboxArmorStandTags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IsMarkerNoHitboxArmorStandTags::True => Value::String("True".to_string()),
                IsMarkerNoHitboxArmorStandTags::False => {
                    Value::String("False".to_string())
                }
                IsMarkerNoHitboxArmorStandTags::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Is Marker (No Hitbox)".to_string()),
        );
        data.insert("action".to_string(), Value::String("ArmorStandTags".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandTags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IsMarkerNoHitboxArmorStandTags {
    fn default() -> Self {
        Self::Dontchange
    }
}
#[derive(Debug, Clone)]
pub enum AllowItemTakingAddingArmorStandTags {
    True,
    False,
    Dontchange,
}
impl AllowItemTakingAddingArmorStandTags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AllowItemTakingAddingArmorStandTags::True => {
                    Value::String("True".to_string())
                }
                AllowItemTakingAddingArmorStandTags::False => {
                    Value::String("False".to_string())
                }
                AllowItemTakingAddingArmorStandTags::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Allow Item Taking / Adding".to_string()),
        );
        data.insert("action".to_string(), Value::String("ArmorStandTags".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandTags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AllowItemTakingAddingArmorStandTags {
    fn default() -> Self {
        Self::Dontchange
    }
}
#[derive(Debug, Clone)]
pub enum HasPhysicsUpdatesArmorStandTags {
    True,
    False,
    Dontchange,
}
impl HasPhysicsUpdatesArmorStandTags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HasPhysicsUpdatesArmorStandTags::True => {
                    Value::String("True".to_string())
                }
                HasPhysicsUpdatesArmorStandTags::False => {
                    Value::String("False".to_string())
                }
                HasPhysicsUpdatesArmorStandTags::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Has Physics / Updates".to_string()),
        );
        data.insert("action".to_string(), Value::String("ArmorStandTags".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandTags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HasPhysicsUpdatesArmorStandTags {
    fn default() -> Self {
        Self::Dontchange
    }
}
#[derive(Debug, Clone)]
pub enum IsSmallArmorStandTags {
    True,
    False,
    Dontchange,
}
impl IsSmallArmorStandTags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IsSmallArmorStandTags::True => Value::String("True".to_string()),
                IsSmallArmorStandTags::False => Value::String("False".to_string()),
                IsSmallArmorStandTags::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Is Small".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandTags".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandTags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IsSmallArmorStandTags {
    fn default() -> Self {
        Self::Dontchange
    }
}
#[derive(Debug, Clone)]
pub enum HasArmsArmorStandTags {
    True,
    False,
    Dontchange,
}
impl HasArmsArmorStandTags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HasArmsArmorStandTags::True => Value::String("True".to_string()),
                HasArmsArmorStandTags::False => Value::String("False".to_string()),
                HasArmsArmorStandTags::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Has Arms".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandTags".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandTags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HasArmsArmorStandTags {
    fn default() -> Self {
        Self::Dontchange
    }
}
#[derive(Debug, Clone)]
pub enum HasBasePlateArmorStandTags {
    True,
    False,
    Dontchange,
}
impl HasBasePlateArmorStandTags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HasBasePlateArmorStandTags::True => Value::String("True".to_string()),
                HasBasePlateArmorStandTags::False => Value::String("False".to_string()),
                HasBasePlateArmorStandTags::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Has Base Plate".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandTags".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandTags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HasBasePlateArmorStandTags {
    fn default() -> Self {
        Self::Dontchange
    }
}
#[derive(Debug, Clone)]
pub enum TextShadowSetTextDisplayTextShadow {
    Enable,
    Disable,
}
impl TextShadowSetTextDisplayTextShadow {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextShadowSetTextDisplayTextShadow::Enable => {
                    Value::String("Enable".to_string())
                }
                TextShadowSetTextDisplayTextShadow::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Shadow".to_string()));
        data.insert("action".to_string(), Value::String("TDisplayShadow".to_string()));
        data.insert("block".to_string(), Value::String("TDisplayShadow".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextShadowSetTextDisplayTextShadow {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum MarkerSetMarker {
    Enable,
    Disable,
}
impl MarkerSetMarker {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                MarkerSetMarker::Enable => Value::String("Enable".to_string()),
                MarkerSetMarker::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Marker".to_string()));
        data.insert("action".to_string(), Value::String("SetMarker".to_string()));
        data.insert("block".to_string(), Value::String("SetMarker".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for MarkerSetMarker {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum NameTagVisibleSetNameVisible {
    Enable,
    Disable,
}
impl NameTagVisibleSetNameVisible {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                NameTagVisibleSetNameVisible::Enable => {
                    Value::String("Enable".to_string())
                }
                NameTagVisibleSetNameVisible::Disable => {
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
impl Default for NameTagVisibleSetNameVisible {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum PoseSetPose {
    Standing,
    Sleeping,
    Swimming,
    Sneaking,
}
impl PoseSetPose {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PoseSetPose::Standing => Value::String("Standing".to_string()),
                PoseSetPose::Sleeping => Value::String("Sleeping".to_string()),
                PoseSetPose::Swimming => Value::String("Swimming".to_string()),
                PoseSetPose::Sneaking => Value::String("Sneaking".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Pose".to_string()));
        data.insert("action".to_string(), Value::String(" SetPose ".to_string()));
        data.insert("block".to_string(), Value::String(" SetPose ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PoseSetPose {
    fn default() -> Self {
        Self::Standing
    }
}
#[derive(Debug, Clone)]
pub enum RearingSetRearing {
    Enable,
    Disable,
}
impl RearingSetRearing {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RearingSetRearing::Enable => Value::String("Enable".to_string()),
                RearingSetRearing::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Rearing".to_string()));
        data.insert("action".to_string(), Value::String("SetRearing".to_string()));
        data.insert("block".to_string(), Value::String("SetRearing".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RearingSetRearing {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum ChargedSetCreeperCharged {
    Enable,
    Disable,
}
impl ChargedSetCreeperCharged {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ChargedSetCreeperCharged::Enable => Value::String("Enable".to_string()),
                ChargedSetCreeperCharged::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Charged".to_string()));
        data.insert("action".to_string(), Value::String("CreeperCharged".to_string()));
        data.insert("block".to_string(), Value::String("CreeperCharged".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ChargedSetCreeperCharged {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum GravitySetGravity {
    Enable,
    Disable,
}
impl GravitySetGravity {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GravitySetGravity::Enable => Value::String("Enable".to_string()),
                GravitySetGravity::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Gravity".to_string()));
        data.insert("action".to_string(), Value::String("SetGravity".to_string()));
        data.insert("block".to_string(), Value::String("SetGravity".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GravitySetGravity {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum HideNameTagSetCustomName {
    True,
    False,
    Dontchange,
}
impl HideNameTagSetCustomName {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideNameTagSetCustomName::True => Value::String("True".to_string()),
                HideNameTagSetCustomName::False => Value::String("False".to_string()),
                HideNameTagSetCustomName::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Name Tag".to_string()));
        data.insert("action".to_string(), Value::String("SetName".to_string()));
        data.insert("block".to_string(), Value::String("SetName".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideNameTagSetCustomName {
    fn default() -> Self {
        Self::False
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
pub enum SeethroughSetTextDisplaySeethrough {
    Enable,
    Disable,
}
impl SeethroughSetTextDisplaySeethrough {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SeethroughSetTextDisplaySeethrough::Enable => {
                    Value::String("Enable".to_string())
                }
                SeethroughSetTextDisplaySeethrough::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("See-through".to_string()));
        data.insert("action".to_string(), Value::String("TDisplaySeeThru".to_string()));
        data.insert("block".to_string(), Value::String("TDisplaySeeThru".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SeethroughSetTextDisplaySeethrough {
    fn default() -> Self {
        Self::Enable
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
pub enum ResponsiveSetInteractionResponsive {
    Enable,
    Disable,
}
impl ResponsiveSetInteractionResponsive {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ResponsiveSetInteractionResponsive::Enable => {
                    Value::String("Enable".to_string())
                }
                ResponsiveSetInteractionResponsive::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Responsive".to_string()));
        data.insert("action".to_string(), Value::String("InteractResponse".to_string()));
        data.insert("block".to_string(), Value::String("InteractResponse".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ResponsiveSetInteractionResponsive {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum HandUseItem {
    MainHand,
    OffHand,
}
impl HandUseItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HandUseItem::MainHand => Value::String("Main Hand".to_string()),
                HandUseItem::OffHand => Value::String("Off Hand".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Hand".to_string()));
        data.insert("action".to_string(), Value::String("UseItem".to_string()));
        data.insert("block".to_string(), Value::String("UseItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HandUseItem {
    fn default() -> Self {
        Self::MainHand
    }
}
#[derive(Debug, Clone)]
pub enum UseItemUseItem {
    Enable,
    Disable,
}
impl UseItemUseItem {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                UseItemUseItem::Enable => Value::String("Enable".to_string()),
                UseItemUseItem::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Use Item".to_string()));
        data.insert("action".to_string(), Value::String("UseItem".to_string()));
        data.insert("block".to_string(), Value::String("UseItem".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for UseItemUseItem {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum PatternColorSetTropicalFishPattern {
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
    Dontchange,
}
impl PatternColorSetTropicalFishPattern {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PatternColorSetTropicalFishPattern::White => {
                    Value::String("White".to_string())
                }
                PatternColorSetTropicalFishPattern::Orange => {
                    Value::String("Orange".to_string())
                }
                PatternColorSetTropicalFishPattern::Magenta => {
                    Value::String("Magenta".to_string())
                }
                PatternColorSetTropicalFishPattern::Lightblue => {
                    Value::String("Light blue".to_string())
                }
                PatternColorSetTropicalFishPattern::Yellow => {
                    Value::String("Yellow".to_string())
                }
                PatternColorSetTropicalFishPattern::Lime => {
                    Value::String("Lime".to_string())
                }
                PatternColorSetTropicalFishPattern::Pink => {
                    Value::String("Pink".to_string())
                }
                PatternColorSetTropicalFishPattern::Gray => {
                    Value::String("Gray".to_string())
                }
                PatternColorSetTropicalFishPattern::Lightgray => {
                    Value::String("Light gray".to_string())
                }
                PatternColorSetTropicalFishPattern::Cyan => {
                    Value::String("Cyan".to_string())
                }
                PatternColorSetTropicalFishPattern::Purple => {
                    Value::String("Purple".to_string())
                }
                PatternColorSetTropicalFishPattern::Blue => {
                    Value::String("Blue".to_string())
                }
                PatternColorSetTropicalFishPattern::Brown => {
                    Value::String("Brown".to_string())
                }
                PatternColorSetTropicalFishPattern::Green => {
                    Value::String("Green".to_string())
                }
                PatternColorSetTropicalFishPattern::Red => {
                    Value::String("Red".to_string())
                }
                PatternColorSetTropicalFishPattern::Black => {
                    Value::String("Black".to_string())
                }
                PatternColorSetTropicalFishPattern::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Pattern Color".to_string()));
        data.insert("action".to_string(), Value::String("SetFishPattern".to_string()));
        data.insert("block".to_string(), Value::String("SetFishPattern".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PatternColorSetTropicalFishPattern {
    fn default() -> Self {
        Self::White
    }
}
#[derive(Debug, Clone)]
pub enum BodyColorSetTropicalFishPattern {
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
    Dontchange,
}
impl BodyColorSetTropicalFishPattern {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BodyColorSetTropicalFishPattern::White => {
                    Value::String("White".to_string())
                }
                BodyColorSetTropicalFishPattern::Orange => {
                    Value::String("Orange".to_string())
                }
                BodyColorSetTropicalFishPattern::Magenta => {
                    Value::String("Magenta".to_string())
                }
                BodyColorSetTropicalFishPattern::Lightblue => {
                    Value::String("Light blue".to_string())
                }
                BodyColorSetTropicalFishPattern::Yellow => {
                    Value::String("Yellow".to_string())
                }
                BodyColorSetTropicalFishPattern::Lime => {
                    Value::String("Lime".to_string())
                }
                BodyColorSetTropicalFishPattern::Pink => {
                    Value::String("Pink".to_string())
                }
                BodyColorSetTropicalFishPattern::Gray => {
                    Value::String("Gray".to_string())
                }
                BodyColorSetTropicalFishPattern::Lightgray => {
                    Value::String("Light gray".to_string())
                }
                BodyColorSetTropicalFishPattern::Cyan => {
                    Value::String("Cyan".to_string())
                }
                BodyColorSetTropicalFishPattern::Purple => {
                    Value::String("Purple".to_string())
                }
                BodyColorSetTropicalFishPattern::Blue => {
                    Value::String("Blue".to_string())
                }
                BodyColorSetTropicalFishPattern::Brown => {
                    Value::String("Brown".to_string())
                }
                BodyColorSetTropicalFishPattern::Green => {
                    Value::String("Green".to_string())
                }
                BodyColorSetTropicalFishPattern::Red => Value::String("Red".to_string()),
                BodyColorSetTropicalFishPattern::Black => {
                    Value::String("Black".to_string())
                }
                BodyColorSetTropicalFishPattern::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Body Color".to_string()));
        data.insert("action".to_string(), Value::String("SetFishPattern".to_string()));
        data.insert("block".to_string(), Value::String("SetFishPattern".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BodyColorSetTropicalFishPattern {
    fn default() -> Self {
        Self::White
    }
}
#[derive(Debug, Clone)]
pub enum PatternSetTropicalFishPattern {
    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
    Flopper,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish,
    Dontchange,
}
impl PatternSetTropicalFishPattern {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PatternSetTropicalFishPattern::Kob => Value::String("Kob".to_string()),
                PatternSetTropicalFishPattern::Sunstreak => {
                    Value::String("Sunstreak".to_string())
                }
                PatternSetTropicalFishPattern::Snooper => {
                    Value::String("Snooper".to_string())
                }
                PatternSetTropicalFishPattern::Dasher => {
                    Value::String("Dasher".to_string())
                }
                PatternSetTropicalFishPattern::Brinely => {
                    Value::String("Brinely".to_string())
                }
                PatternSetTropicalFishPattern::Spotty => {
                    Value::String("Spotty".to_string())
                }
                PatternSetTropicalFishPattern::Flopper => {
                    Value::String("Flopper".to_string())
                }
                PatternSetTropicalFishPattern::Stripey => {
                    Value::String("Stripey".to_string())
                }
                PatternSetTropicalFishPattern::Glitter => {
                    Value::String("Glitter".to_string())
                }
                PatternSetTropicalFishPattern::Blockfish => {
                    Value::String("Blockfish".to_string())
                }
                PatternSetTropicalFishPattern::Betty => {
                    Value::String("Betty".to_string())
                }
                PatternSetTropicalFishPattern::Clayfish => {
                    Value::String("Clayfish".to_string())
                }
                PatternSetTropicalFishPattern::Dontchange => {
                    Value::String("Don't change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Pattern".to_string()));
        data.insert("action".to_string(), Value::String("SetFishPattern".to_string()));
        data.insert("block".to_string(), Value::String("SetFishPattern".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PatternSetTropicalFishPattern {
    fn default() -> Self {
        Self::Kob
    }
}
#[derive(Debug, Clone)]
pub enum BehaviorSetSnifferState {
    Idle,
    FeelingHappy,
    Scenting,
    Sniffing,
    Searching,
    Digging,
}
impl BehaviorSetSnifferState {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BehaviorSetSnifferState::Idle => Value::String("Idle".to_string()),
                BehaviorSetSnifferState::FeelingHappy => {
                    Value::String("Feeling Happy".to_string())
                }
                BehaviorSetSnifferState::Scenting => {
                    Value::String("Scenting".to_string())
                }
                BehaviorSetSnifferState::Sniffing => {
                    Value::String("Sniffing".to_string())
                }
                BehaviorSetSnifferState::Searching => {
                    Value::String("Searching".to_string())
                }
                BehaviorSetSnifferState::Digging => Value::String("Digging".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Behavior".to_string()));
        data.insert("action".to_string(), Value::String("SnifferState".to_string()));
        data.insert("block".to_string(), Value::String("SnifferState".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BehaviorSetSnifferState {
    fn default() -> Self {
        Self::Idle
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
pub enum SkinTypeSetCatType {
    Tabby,
    Tuxedo,
    Red,
    Siamese,
    BritishShorthair,
    Calico,
    Persian,
    Ragdoll,
    White,
    Jellie,
    Black,
}
impl SkinTypeSetCatType {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SkinTypeSetCatType::Tabby => Value::String("Tabby".to_string()),
                SkinTypeSetCatType::Tuxedo => Value::String("Tuxedo".to_string()),
                SkinTypeSetCatType::Red => Value::String("Red".to_string()),
                SkinTypeSetCatType::Siamese => Value::String("Siamese".to_string()),
                SkinTypeSetCatType::BritishShorthair => {
                    Value::String("British Shorthair".to_string())
                }
                SkinTypeSetCatType::Calico => Value::String("Calico".to_string()),
                SkinTypeSetCatType::Persian => Value::String("Persian".to_string()),
                SkinTypeSetCatType::Ragdoll => Value::String("Ragdoll".to_string()),
                SkinTypeSetCatType::White => Value::String("White".to_string()),
                SkinTypeSetCatType::Jellie => Value::String("Jellie".to_string()),
                SkinTypeSetCatType::Black => Value::String("Black".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Skin Type".to_string()));
        data.insert("action".to_string(), Value::String("SetCatType".to_string()));
        data.insert("block".to_string(), Value::String("SetCatType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SkinTypeSetCatType {
    fn default() -> Self {
        Self::Tabby
    }
}
#[derive(Debug, Clone)]
pub enum SaddleSetWearingSaddle {
    Enable,
    Disable,
}
impl SaddleSetWearingSaddle {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SaddleSetWearingSaddle::Enable => Value::String("Enable".to_string()),
                SaddleSetWearingSaddle::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Saddle".to_string()));
        data.insert("action".to_string(), Value::String("SetSaddle".to_string()));
        data.insert("block".to_string(), Value::String("SetSaddle".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SaddleSetWearingSaddle {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum TextAlignmentSetTextDisplayTextAlignment {
    Center,
    Left,
    Right,
}
impl TextAlignmentSetTextDisplayTextAlignment {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextAlignmentSetTextDisplayTextAlignment::Center => {
                    Value::String("Center".to_string())
                }
                TextAlignmentSetTextDisplayTextAlignment::Left => {
                    Value::String("Left".to_string())
                }
                TextAlignmentSetTextDisplayTextAlignment::Right => {
                    Value::String("Right".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Alignment".to_string()));
        data.insert("action".to_string(), Value::String("TDisplayAlign".to_string()));
        data.insert("block".to_string(), Value::String("TDisplayAlign".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextAlignmentSetTextDisplayTextAlignment {
    fn default() -> Self {
        Self::Center
    }
}
#[derive(Debug, Clone)]
pub enum PhaseSetDragonPhase {
    Flying,
    Hovering,
    Breathattack,
    Dying,
}
impl PhaseSetDragonPhase {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PhaseSetDragonPhase::Flying => Value::String("Flying".to_string()),
                PhaseSetDragonPhase::Hovering => Value::String("Hovering".to_string()),
                PhaseSetDragonPhase::Breathattack => {
                    Value::String("Breath attack".to_string())
                }
                PhaseSetDragonPhase::Dying => Value::String("Dying".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Phase".to_string()));
        data.insert("action".to_string(), Value::String("SetDragonPhase".to_string()));
        data.insert("block".to_string(), Value::String("SetDragonPhase".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PhaseSetDragonPhase {
    fn default() -> Self {
        Self::Flying
    }
}
#[derive(Debug, Clone)]
pub enum LlamaColorSetLlamaColor {
    Brown,
    Creamy,
    White,
    Gray,
}
impl LlamaColorSetLlamaColor {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                LlamaColorSetLlamaColor::Brown => Value::String("Brown".to_string()),
                LlamaColorSetLlamaColor::Creamy => Value::String("Creamy".to_string()),
                LlamaColorSetLlamaColor::White => Value::String("White".to_string()),
                LlamaColorSetLlamaColor::Gray => Value::String("Gray".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Llama Color".to_string()));
        data.insert("action".to_string(), Value::String("SetLlamaColor".to_string()));
        data.insert("block".to_string(), Value::String("SetLlamaColor".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for LlamaColorSetLlamaColor {
    fn default() -> Self {
        Self::Brown
    }
}
#[derive(Debug, Clone)]
pub enum BiomeSetVillagerBiome {
    Desert,
    Jungle,
    Plains,
    Savanna,
    Snow,
    Swamp,
    Taiga,
}
impl BiomeSetVillagerBiome {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BiomeSetVillagerBiome::Desert => Value::String("Desert".to_string()),
                BiomeSetVillagerBiome::Jungle => Value::String("Jungle".to_string()),
                BiomeSetVillagerBiome::Plains => Value::String("Plains".to_string()),
                BiomeSetVillagerBiome::Savanna => Value::String("Savanna".to_string()),
                BiomeSetVillagerBiome::Snow => Value::String("Snow".to_string()),
                BiomeSetVillagerBiome::Swamp => Value::String("Swamp".to_string()),
                BiomeSetVillagerBiome::Taiga => Value::String("Taiga".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Biome".to_string()));
        data.insert("action".to_string(), Value::String("SetVillagerBiome".to_string()));
        data.insert("block".to_string(), Value::String("SetVillagerBiome".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BiomeSetVillagerBiome {
    fn default() -> Self {
        Self::Desert
    }
}
#[derive(Debug, Clone)]
pub enum BabySettoBabyAdult {
    Enable,
    Disable,
}
impl BabySettoBabyAdult {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BabySettoBabyAdult::Enable => Value::String("Enable".to_string()),
                BabySettoBabyAdult::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Baby".to_string()));
        data.insert("action".to_string(), Value::String("SetBaby".to_string()));
        data.insert("block".to_string(), Value::String("SetBaby".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BabySettoBabyAdult {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum MooshroomVariantSetMooshroomType {
    Red,
    Brown,
}
impl MooshroomVariantSetMooshroomType {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                MooshroomVariantSetMooshroomType::Red => Value::String("Red".to_string()),
                MooshroomVariantSetMooshroomType::Brown => {
                    Value::String("Brown".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Mooshroom Variant".to_string()));
        data.insert("action".to_string(), Value::String("MooshroomType".to_string()));
        data.insert("block".to_string(), Value::String("MooshroomType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for MooshroomVariantSetMooshroomType {
    fn default() -> Self {
        Self::Red
    }
}
#[derive(Debug, Clone)]
pub enum InvisibleSetInvisible {
    Enable,
    Disable,
}
impl InvisibleSetInvisible {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InvisibleSetInvisible::Enable => Value::String("Enable".to_string()),
                InvisibleSetInvisible::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Invisible".to_string()));
        data.insert("action".to_string(), Value::String("SetInvisible".to_string()));
        data.insert("block".to_string(), Value::String("SetInvisible".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InvisibleSetInvisible {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum BillboardTypeSetDisplayBillboard {
    Fixed,
    Vertical,
    Horizontal,
    Center,
}
impl BillboardTypeSetDisplayBillboard {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BillboardTypeSetDisplayBillboard::Fixed => {
                    Value::String("Fixed".to_string())
                }
                BillboardTypeSetDisplayBillboard::Vertical => {
                    Value::String("Vertical".to_string())
                }
                BillboardTypeSetDisplayBillboard::Horizontal => {
                    Value::String("Horizontal".to_string())
                }
                BillboardTypeSetDisplayBillboard::Center => {
                    Value::String("Center".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Billboard Type".to_string()));
        data.insert("action".to_string(), Value::String("DisplayBillboard".to_string()));
        data.insert("block".to_string(), Value::String("DisplayBillboard".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BillboardTypeSetDisplayBillboard {
    fn default() -> Self {
        Self::Fixed
    }
}
#[derive(Debug, Clone)]
pub enum RestingSetCatResting {
    Enable,
    Disable,
}
impl RestingSetCatResting {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RestingSetCatResting::Enable => Value::String("Enable".to_string()),
                RestingSetCatResting::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Resting".to_string()));
        data.insert("action".to_string(), Value::String("SetCatResting".to_string()));
        data.insert("block".to_string(), Value::String("SetCatResting".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RestingSetCatResting {
    fn default() -> Self {
        Self::Enable
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
pub enum LeftHornSetGoatHorns {
    Show,
    Hide,
    NoChange,
}
impl LeftHornSetGoatHorns {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                LeftHornSetGoatHorns::Show => Value::String("Show".to_string()),
                LeftHornSetGoatHorns::Hide => Value::String("Hide".to_string()),
                LeftHornSetGoatHorns::NoChange => Value::String("No Change".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Left Horn".to_string()));
        data.insert("action".to_string(), Value::String("SetGoatHorns".to_string()));
        data.insert("block".to_string(), Value::String("SetGoatHorns".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for LeftHornSetGoatHorns {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum RightHornSetGoatHorns {
    Show,
    Hide,
    NoChange,
}
impl RightHornSetGoatHorns {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RightHornSetGoatHorns::Show => Value::String("Show".to_string()),
                RightHornSetGoatHorns::Hide => Value::String("Hide".to_string()),
                RightHornSetGoatHorns::NoChange => Value::String("No Change".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Right Horn".to_string()));
        data.insert("action".to_string(), Value::String("SetGoatHorns".to_string()));
        data.insert("block".to_string(), Value::String("SetGoatHorns".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RightHornSetGoatHorns {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum GlowingSetGlowing {
    Enable,
    Disable,
}
impl GlowingSetGlowing {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GlowingSetGlowing::Enable => Value::String("Enable".to_string()),
                GlowingSetGlowing::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Glowing".to_string()));
        data.insert("action".to_string(), Value::String("SetGlowing".to_string()));
        data.insert("block".to_string(), Value::String("SetGlowing".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GlowingSetGlowing {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum ScreamsSetGoatScreaming {
    Enable,
    Disable,
}
impl ScreamsSetGoatScreaming {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ScreamsSetGoatScreaming::Enable => Value::String("Enable".to_string()),
                ScreamsSetGoatScreaming::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Screams".to_string()));
        data.insert("action".to_string(), Value::String("SetGoatScreaming".to_string()));
        data.insert("block".to_string(), Value::String("SetGoatScreaming".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ScreamsSetGoatScreaming {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum ModelTypeSetItemDisplayModelType {
    None,
    FirstPersonLeftHand,
    FirstPersonRightHand,
    ThirdPersonLeftHand,
    ThirdPersonRightHand,
    Head,
    GUI,
    Ground,
    Fixed,
}
impl ModelTypeSetItemDisplayModelType {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ModelTypeSetItemDisplayModelType::None => {
                    Value::String("None".to_string())
                }
                ModelTypeSetItemDisplayModelType::FirstPersonLeftHand => {
                    Value::String("First Person Left Hand".to_string())
                }
                ModelTypeSetItemDisplayModelType::FirstPersonRightHand => {
                    Value::String("First Person Right Hand".to_string())
                }
                ModelTypeSetItemDisplayModelType::ThirdPersonLeftHand => {
                    Value::String("Third Person Left Hand".to_string())
                }
                ModelTypeSetItemDisplayModelType::ThirdPersonRightHand => {
                    Value::String("Third Person Right Hand".to_string())
                }
                ModelTypeSetItemDisplayModelType::Head => {
                    Value::String("Head".to_string())
                }
                ModelTypeSetItemDisplayModelType::GUI => Value::String("GUI".to_string()),
                ModelTypeSetItemDisplayModelType::Ground => {
                    Value::String("Ground".to_string())
                }
                ModelTypeSetItemDisplayModelType::Fixed => {
                    Value::String("Fixed".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Model Type".to_string()));
        data.insert(
            "action".to_string(),
            Value::String("IDisplayModelType".to_string()),
        );
        data.insert("block".to_string(), Value::String("IDisplayModelType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ModelTypeSetItemDisplayModelType {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone)]
pub enum SleepingSetFoxSleeping {
    Enable,
    Disable,
}
impl SleepingSetFoxSleeping {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SleepingSetFoxSleeping::Enable => Value::String("Enable".to_string()),
                SleepingSetFoxSleeping::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sleeping".to_string()));
        data.insert("action".to_string(), Value::String("FoxSleeping".to_string()));
        data.insert("block".to_string(), Value::String("FoxSleeping".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SleepingSetFoxSleeping {
    fn default() -> Self {
        Self::Enable
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
pub enum ArmorStandPartSetArmorStandPose {
    Head,
    Body,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}
impl ArmorStandPartSetArmorStandPose {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ArmorStandPartSetArmorStandPose::Head => {
                    Value::String("Head".to_string())
                }
                ArmorStandPartSetArmorStandPose::Body => {
                    Value::String("Body".to_string())
                }
                ArmorStandPartSetArmorStandPose::LeftArm => {
                    Value::String("Left Arm".to_string())
                }
                ArmorStandPartSetArmorStandPose::RightArm => {
                    Value::String("Right Arm".to_string())
                }
                ArmorStandPartSetArmorStandPose::LeftLeg => {
                    Value::String("Left Leg".to_string())
                }
                ArmorStandPartSetArmorStandPose::RightLeg => {
                    Value::String("Right Leg".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Armor Stand Part".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandPose".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandPose".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ArmorStandPartSetArmorStandPose {
    fn default() -> Self {
        Self::Head
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
pub enum EquipmentSlotSetEquipmentItem {
    Mainhand,
    Offhand,
    Head,
    Body,
    Legs,
    Feet,
    Saddle,
    Horsearmor,
    Decor,
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
                EquipmentSlotSetEquipmentItem::Body => Value::String("Body".to_string()),
                EquipmentSlotSetEquipmentItem::Legs => Value::String("Legs".to_string()),
                EquipmentSlotSetEquipmentItem::Feet => Value::String("Feet".to_string()),
                EquipmentSlotSetEquipmentItem::Saddle => {
                    Value::String("Saddle".to_string())
                }
                EquipmentSlotSetEquipmentItem::Horsearmor => {
                    Value::String("Horse armor".to_string())
                }
                EquipmentSlotSetEquipmentItem::Decor => {
                    Value::String("Decor".to_string())
                }
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
pub enum AnimationArmSendMobAttackAnimation {
    Swingmainarm,
    Swingoffarm,
}
impl AnimationArmSendMobAttackAnimation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AnimationArmSendMobAttackAnimation::Swingmainarm => {
                    Value::String("Swing main arm".to_string())
                }
                AnimationArmSendMobAttackAnimation::Swingoffarm => {
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
impl Default for AnimationArmSendMobAttackAnimation {
    fn default() -> Self {
        Self::Swingmainarm
    }
}
#[derive(Debug, Clone)]
pub enum SilencedSetSilenced {
    Enable,
    Disable,
}
impl SilencedSetSilenced {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SilencedSetSilenced::Enable => Value::String("Enable".to_string()),
                SilencedSetSilenced::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Silenced".to_string()));
        data.insert("action".to_string(), Value::String("SetSilenced".to_string()));
        data.insert("block".to_string(), Value::String("SetSilenced".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SilencedSetSilenced {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum HasNectarSetBeeHasNectar {
    Enable,
    Disable,
}
impl HasNectarSetBeeHasNectar {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HasNectarSetBeeHasNectar::Enable => Value::String("Enable".to_string()),
                HasNectarSetBeeHasNectar::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Has Nectar".to_string()));
        data.insert("action".to_string(), Value::String("SetBeeNectar".to_string()));
        data.insert("block".to_string(), Value::String("SetBeeNectar".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HasNectarSetBeeHasNectar {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum PumpkinSetSnowGolemPumpkin {
    Enable,
    Disable,
}
impl PumpkinSetSnowGolemPumpkin {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PumpkinSetSnowGolemPumpkin::Enable => Value::String("Enable".to_string()),
                PumpkinSetSnowGolemPumpkin::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Pumpkin".to_string()));
        data.insert("action".to_string(), Value::String("SnowmanPumpkin".to_string()));
        data.insert("block".to_string(), Value::String("SnowmanPumpkin".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PumpkinSetSnowGolemPumpkin {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum InteractionsSetArmorStandSlotInteractions {
    Takeswaporplaceitem,
    Takeorswapitem,
    Takeitem,
    Placeitem,
    None,
}
impl InteractionsSetArmorStandSlotInteractions {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InteractionsSetArmorStandSlotInteractions::Takeswaporplaceitem => {
                    Value::String("Take, swap or place item".to_string())
                }
                InteractionsSetArmorStandSlotInteractions::Takeorswapitem => {
                    Value::String("Take or swap item".to_string())
                }
                InteractionsSetArmorStandSlotInteractions::Takeitem => {
                    Value::String("Take item".to_string())
                }
                InteractionsSetArmorStandSlotInteractions::Placeitem => {
                    Value::String("Place item".to_string())
                }
                InteractionsSetArmorStandSlotInteractions::None => {
                    Value::String("None".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Interactions".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandSlots".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandSlots".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InteractionsSetArmorStandSlotInteractions {
    fn default() -> Self {
        Self::Takeswaporplaceitem
    }
}
#[derive(Debug, Clone)]
pub enum EquipmentSlotSetArmorStandSlotInteractions {
    All,
    Mainhand,
    Offhand,
    Head,
    Chest,
    Legs,
    Feet,
}
impl EquipmentSlotSetArmorStandSlotInteractions {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                EquipmentSlotSetArmorStandSlotInteractions::All => {
                    Value::String("All".to_string())
                }
                EquipmentSlotSetArmorStandSlotInteractions::Mainhand => {
                    Value::String("Main hand".to_string())
                }
                EquipmentSlotSetArmorStandSlotInteractions::Offhand => {
                    Value::String("Off hand".to_string())
                }
                EquipmentSlotSetArmorStandSlotInteractions::Head => {
                    Value::String("Head".to_string())
                }
                EquipmentSlotSetArmorStandSlotInteractions::Chest => {
                    Value::String("Chest".to_string())
                }
                EquipmentSlotSetArmorStandSlotInteractions::Legs => {
                    Value::String("Legs".to_string())
                }
                EquipmentSlotSetArmorStandSlotInteractions::Feet => {
                    Value::String("Feet".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Equipment Slot".to_string()));
        data.insert("action".to_string(), Value::String("ArmorStandSlots".to_string()));
        data.insert("block".to_string(), Value::String("ArmorStandSlots".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for EquipmentSlotSetArmorStandSlotInteractions {
    fn default() -> Self {
        Self::All
    }
}
#[derive(Debug, Clone)]
pub enum DancingSetAllayDancing {
    Enable,
    Disable,
}
impl DancingSetAllayDancing {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DancingSetAllayDancing::Enable => Value::String("Enable".to_string()),
                DancingSetAllayDancing::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Dancing".to_string()));
        data.insert("action".to_string(), Value::String("SetAllayDancing".to_string()));
        data.insert("block".to_string(), Value::String("SetAllayDancing".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DancingSetAllayDancing {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum SkinTypeSetRabbitType {
    Brown,
    White,
    Black,
    BlackandWhite,
    Gold,
    SaltandPepper,
    Killer,
}
impl SkinTypeSetRabbitType {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SkinTypeSetRabbitType::Brown => Value::String("Brown".to_string()),
                SkinTypeSetRabbitType::White => Value::String("White".to_string()),
                SkinTypeSetRabbitType::Black => Value::String("Black".to_string()),
                SkinTypeSetRabbitType::BlackandWhite => {
                    Value::String("Black and White".to_string())
                }
                SkinTypeSetRabbitType::Gold => Value::String("Gold".to_string()),
                SkinTypeSetRabbitType::SaltandPepper => {
                    Value::String("Salt and Pepper".to_string())
                }
                SkinTypeSetRabbitType::Killer => Value::String("Killer".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Skin Type".to_string()));
        data.insert("action".to_string(), Value::String("SetRabbitType".to_string()));
        data.insert("block".to_string(), Value::String("SetRabbitType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SkinTypeSetRabbitType {
    fn default() -> Self {
        Self::Brown
    }
}
#[derive(Debug, Clone)]
pub enum AngrySetAngry {
    Enable,
    Disable,
}
impl AngrySetAngry {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AngrySetAngry::Enable => Value::String("Enable".to_string()),
                AngrySetAngry::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Angry".to_string()));
        data.insert("action".to_string(), Value::String("SetAngry".to_string()));
        data.insert("block".to_string(), Value::String("SetAngry".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AngrySetAngry {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum DiggingTypeSetWardenDigging {
    Emerge,
    DigDown,
}
impl DiggingTypeSetWardenDigging {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DiggingTypeSetWardenDigging::Emerge => {
                    Value::String("Emerge".to_string())
                }
                DiggingTypeSetWardenDigging::DigDown => {
                    Value::String("Dig Down".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Digging Type".to_string()));
        data.insert("action".to_string(), Value::String("SetDigging".to_string()));
        data.insert("block".to_string(), Value::String("SetDigging".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DiggingTypeSetWardenDigging {
    fn default() -> Self {
        Self::Emerge
    }
}
#[derive(Debug, Clone)]
pub enum HasDeathDropsSetDeathDropsEnabled {
    Enable,
    Disable,
}
impl HasDeathDropsSetDeathDropsEnabled {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HasDeathDropsSetDeathDropsEnabled::Enable => {
                    Value::String("Enable".to_string())
                }
                HasDeathDropsSetDeathDropsEnabled::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Has Death Drops".to_string()));
        data.insert("action".to_string(), Value::String("SetDeathDrops".to_string()));
        data.insert("block".to_string(), Value::String("SetDeathDrops".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HasDeathDropsSetDeathDropsEnabled {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum PersistentSetPersistent {
    Enable,
    Disable,
}
impl PersistentSetPersistent {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PersistentSetPersistent::Enable => Value::String("Enable".to_string()),
                PersistentSetPersistent::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Persistent".to_string()));
        data.insert("action".to_string(), Value::String("SetPersistent".to_string()));
        data.insert("block".to_string(), Value::String("SetPersistent".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PersistentSetPersistent {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum CelebrateSetCelebrating {
    Enable,
    Disable,
}
impl CelebrateSetCelebrating {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CelebrateSetCelebrating::Enable => Value::String("Enable".to_string()),
                CelebrateSetCelebrating::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Celebrate".to_string()));
        data.insert("action".to_string(), Value::String("SetCelebrating".to_string()));
        data.insert("block".to_string(), Value::String("SetCelebrating".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CelebrateSetCelebrating {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum TextValueMergingSetTextDisplayText {
    Addspaces,
    Nospaces,
}
impl TextValueMergingSetTextDisplayText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextValueMergingSetTextDisplayText::Addspaces => {
                    Value::String("Add spaces".to_string())
                }
                TextValueMergingSetTextDisplayText::Nospaces => {
                    Value::String("No spaces".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Value Merging".to_string()));
        data.insert("action".to_string(), Value::String("TDisplayText".to_string()));
        data.insert("block".to_string(), Value::String("TDisplayText".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextValueMergingSetTextDisplayText {
    fn default() -> Self {
        Self::Nospaces
    }
}
#[derive(Debug, Clone)]
pub enum InheritStylesSetTextDisplayText {
    True,
    False,
}
impl InheritStylesSetTextDisplayText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InheritStylesSetTextDisplayText::True => {
                    Value::String("True".to_string())
                }
                InheritStylesSetTextDisplayText::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Inherit Styles".to_string()));
        data.insert("action".to_string(), Value::String("TDisplayText".to_string()));
        data.insert("block".to_string(), Value::String("TDisplayText".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InheritStylesSetTextDisplayText {
    fn default() -> Self {
        Self::True
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
#[derive(Debug, Clone)]
pub enum CarryingChestSetCarryingChest {
    Enable,
    Disable,
}
impl CarryingChestSetCarryingChest {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CarryingChestSetCarryingChest::Enable => {
                    Value::String("Enable".to_string())
                }
                CarryingChestSetCarryingChest::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Carrying Chest".to_string()));
        data.insert("action".to_string(), Value::String("SetCarryingChest".to_string()));
        data.insert("block".to_string(), Value::String("SetCarryingChest".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CarryingChestSetCarryingChest {
    fn default() -> Self {
        Self::Enable
    }
}
#[derive(Debug, Clone)]
pub enum FoxTypeSetFoxType {
    Red,
    Snow,
}
impl FoxTypeSetFoxType {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FoxTypeSetFoxType::Red => Value::String("Red".to_string()),
                FoxTypeSetFoxType::Snow => Value::String("Snow".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Fox Type".to_string()));
        data.insert("action".to_string(), Value::String("SetFoxType".to_string()));
        data.insert("block".to_string(), Value::String("SetFoxType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FoxTypeSetFoxType {
    fn default() -> Self {
        Self::Red
    }
}
