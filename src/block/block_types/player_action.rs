use either :: Either ; use crate :: types :: * ; pub enum PlayerAction { SetHotbarItems { items_to_set : Vec < Item > } , SetReducedDebugInfoEnabled { } , CloseInventory { } , GiveItems { items_to_give : Vec < Item > , amount_to_give : Option < Number > } , NoKeepInv { } , SetAllowHandCrafting { } , BossBar { } , DisplayParticleSphere { effect : Particle , center_location : Location , diameter : Option < Number > } , SetBaseAttackSpeed { attack_speed : Option < Number > } , SetVelocity { new_velocity : Vector } , DisplayParticleEffect { effect : Vec < Particle > , effect_location : Location } , AddInventoryMenuRow { items_to_display : Vec < Item > } , DisablePvp { } , NoNatRegen { } , DisplayLightningBolt { strike_location : Location } , PlaySoundfromEntity { sound_to_play : Vec < Sound > , target_uuid : Vec < Either < Text , MiniMessage > > } , ReplaceProj { } , Damage { damage_to_inflict : Number , uuid_of_damager_entity : Option < Either < Text , MiniMessage > > } , SetExperience { experience_to_set : Number } , SendAnimation { } , SetXPProgress { progress__0100 : Number } , SetInventoryItems { items_to_set : Vec < Item > } , TeleportSequence { locations_to : Vec < Location > , teleport_delay_ticks : Option < Number > } , Heal { amount_to_heal : Option < Number > } , SetSpawnPoint { the_new_spawn_location : Option < Location > } , DisplayParticleSpiral { effect : Particle , base_location : Location , length : Option < Number > , diameter : Option < Number > , effect_count : Option < Number > , rotations : Option < Number > } , SetInventoryKept { } , SetAllowFlight { } , LaunchUp { launch_power : Number } , SetMaximumHealth { maximum_health : Number } , RemoveBossBar { boss_bar_position : Option < Number > } , SetFogDistance { fog_distance_in_chunks : Option < Number > } , GetTargetEntity { } , SettoAdventureMode { } , ForceFlightMode { } , LoadSavedInventory { } , SettoSpectatorMode { } , ClearPotionEffects { } , Kick { } , SetPlayerListInfo { headerfooter_text : Vec < MiniMessage > } , EnablePvp { } , ProjColl { } , HideDisguise { } , SpectateTarget { target_uuid : Option < Either < Text , MiniMessage > > } , SendHurtAnimation { damage_source : Option < Location > } , SettoSurvivalMode { } , SetBossBar { title : Option < MiniMessage > , current_health : Option < Number > , maximum_health : Option < Number > , boss_bar_position : Option < Number > } , SetSkin { player_head : Option < Item > } , SetSpectatorCollision { } , SetStatus { game_status : Option < MiniMessage > } , SetNameTagVisible { } , SetInvulnerabilityTicks { ticks : Number } , SetCursorItem { item_to_set : Option < Item > } , SetAbsorptionHealth { absorption_health : Number } , SetFireTicks { ticks : Number } , SetGamemode { } , RemoveInventoryMenuRow { rows_to_remove : Option < Number > } , EnableFlight { } , SendWakeUpAnimation { } , SetBeeStingsStuck { sting_count : Option < Number > } , DisallowPlacingBreakingBlocks { blocks_to_disallow : Vec < Block > } , SetScoreboardObjectiveName { objective_name : MiniMessage } , LSetHealth { } , ParticleEffect { } , ClearInventory { } , SetFreezeTicks { ticks : Number } , SetGliding { } , SetRotation { pitch_90_to_90 : Number , yaw_180_to_180 : Number } , RemoveScoreboardScore { score_name : MiniMessage } , DisallowDrops { } , DisplayParticleCircle { effect : Particle , center_location : Location , diameter : Option < Number > } , ClearItems { items_to_clear : Vec < Item > } , DisplayBlock { block_to_display : Block , block_location : Location , end_of_region : Option < Location > , block_data : Vec < BlockTag > } , RideEntity { target_uuid : Vec < Either < Text , MiniMessage > > } , WeatherRain { } , SetFlying { } , RemoveWorldBorder { } , SendResourcePack { resource_pack_url : Text } , DisplayBlockOpenedState { block_location : Location } , SetHandItem { } , SendAdvancement { advancement_name : MiniMessage , advancement_icon : Item } , Teleport { new_position : Location } , ClearChat { } , SetPvPAllowed { } , SetInventoryMenuItem { slot : Number , item_to_set : Option < Item > } , LaunchTowardLocation { launch_destination : Location , launch_power : Option < Number > } , SetArmorItems { armor_to_set : Vec < Item > } , DisplayGatewayBeam { gateway_location : Location } , DisableFlight { } , GiveSaturation { saturation_to_give : Number } , SetVisualFire { } , SetOwnDisguiseVisibility { } , SetArrowsStuck { arrow_count : Option < Number > } , GiveExperience { experience_to_give : Number } , FaceLocation { location_to_face : Location } , ClearScoreboard { } , SetItems { } , KeepInv { } , ReplaceItems { items_to_replace : Vec < Item > , item_to_replace_with : Item , amount_of_items_to : Option < Number > } , ShowActionBarText { message_to_send : Vec < MiniMessage > } , SetChatTag { chat_tag : Vec < MiniMessage > } , SendMessage { message_to_send : Vec < MiniMessage > } , ShiftWorldBorder { new_radius : Number , blocks_per_second : Option < Number > } , SetIteminSlot { item_to_set : Option < Item > , slot_to_set : Number } , PlaySoundSequence { sounds_to_play : Vec < Sound > , sound_delay_ticks : Option < Number > , playback_location : Option < Location > } , DisplayAnimatedParticleLine { effect : Particle , start_location : Location , end_location : Location , effect_spacing : Option < Number > , animation_duration : Option < Number > } , DisplaySignText { sign_location : Location , text_lines : Vec < MiniMessage > } , Respawn { } , SetMovementSpeed { movement_speed : Number } , ExpandInventoryMenu { items_to_display : Vec < Item > } , SetInventoryMenuName { inventory_name : MiniMessage } , LaunchProjectile { projectile_to : Projectile , launch_point : Option < Location > , projectile_name : Option < MiniMessage > , speed : Option < Number > , inaccuracy : Option < Number > } , SetItemCooldown { item_type__to_affect : Item , cooldown_in_ticks : Number } , SetPlayerWeather { } , SendHover { } , SetVisualShoulderParrot { } , NoProjColl { } , ShowDisguise { } , SetRemainingAir { breath_ticks : Number } , DisplayPickUpAnimation { entity_uuid : Either < Text , MiniMessage > , collector_uuid : Either < Text , MiniMessage > } , DisplayAnimatedParticleCuboid { effect : Particle , corner_1 : Location , corner_2 : Location , effect_spacing : Option < Number > , animation_duration : Option < Number > } , SetChatColor { new_chat_color : Option < MiniMessage > } , SetWorldBorder { center_position : Location , radius_in_blocks : Number , warning_distance : Option < Number > } , SetPlayerTime { daylight_ticks : Option < Number > } , GiveFood { food_to_give : Number } , NatRegen { } , GivePotionEffect { effects : Vec < Potion > } , PlaySound { sound_to_play : Vec < Sound > , playback_location : Option < Location > } , SetCompassTarget { new_target : Location } , RemoveItems { items_to_remove : Vec < Item > } , BoostElytra { firework : Item } , RandomizedTeleport { locations_to : Vec < Location > } , SaveCurrentInventory { } , OpenBook { book_item : Item } , SetCurrentHealth { current_health : Number } , DisguiseasMob { mob_to_disguise_as : SpawnEgg , display_name : Option < MiniMessage > } , DisguiseasBlock { block_to_disguise_as : Block , display_name : Option < MiniMessage > } , RollBackBlockChanges { rollback_time : Option < Number > } , NoDeathDrops { } , AllowPlacingBreakingBlocks { blocks_to_allow : Vec < Block > } , SetWalkSpeed { of_normal : Number } , OpenContainerInventory { container_location : Location } , SetCollidable { } , LaunchForward { launch_power : Number } , DisplayAnimatedParticleCircle { effect : Particle , center_location : Location , diameter : Option < Number > , animation_duration : Option < Number > } , RemoveBossBarN { } , SetFallDistance { fall_distance_blocks : Number } , SettoCreativeMode { } , SetEquipmentItem { item_to_set : Option < Item > } , SendPlayerAttackAnimation { } , GiveRngItem { } , SetDeathDropsEnabled { } , RemovePotionEffect { effects : Vec < Potion > } , DisplayHologram { display_location : Location , text_to_display : Option < MiniMessage > } , DisplayBlockFracture { blocks_to : Vec < Location > , fracture_level : Option < Number > } , SetEntityHidden { entity_uuid : Either < Text , MiniMessage > } , SetSidebarVisible { } , AllowDrops { } , DeathDrops { } , DisplayVibrationEffect { origin_location : Location , target_location : Location , arrival_time : Option < Number > } , ShowInventoryMenu { items_to_display : Vec < Item > } , SetHotbarSlot { new_slot : Number } , DisplayParticleRay { effect : Particle , ray_location : Location , ray_vector : Vector , effect_spacing : Option < Number > } , SetFoodLevel { food_level : Number } , DisguiseasPlayer { player_name_to_disguise_as : MiniMessage , display_skin : Option < Item > } , DisplayParticleCuboid { effect : Particle , corner_1 : Location , corner_2 : Location , effect_spacing : Option < Number > } , SendMessageSequence { messages_to_send : Vec < MiniMessage > , message_delay_ticks : Option < Number > } , SetNamePrefixSuffix { prefixsuffix_text : Option < MiniMessage > } , SetSaturationLevel { saturation_level : Number } , WeatherClear { } , SetRainLevel { rain_level_ : Number , storm_level_ : Number } , Undisguise { } , DisplayAnimatedParticleSpiral { effect : Particle , base_location : Location , length : Option < Number > , diameter : Option < Number > , particle_count : Option < Number > , rotations : Option < Number > , animation_duration : Option < Number > } , ShowTitleText { title_text : MiniMessage , subtitle_text : Option < MiniMessage > , title_duration : Option < Number > , fade_in_length : Option < Number > , fade_out_length : Option < Number > } , SetInstantRespawn { } , SetScoreboardScore { score_name : MiniMessage , score_value : Option < Number > } , StopSounds { sounds_to_stop : Vec < Sound > } , SetNameColor { } , DisplayParticleLine { effect : Particle , start_location : Location , end_location : Location , effect_spacing : Option < Number > } , }