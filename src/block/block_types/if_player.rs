use either :: Either ; use crate :: types :: * ; pub enum IfPlayer { IsLookingatBlock { block_to_check_for : Vec < Either < Block , Location > > , maximum_distance_from : Option < Number > } , IsInWorldBorder { location_to_check : Option < Location > } , HasRoomForItem { items_to_check_with : Option < Item > } , IsHoldingOff { } , IsUsingResourcePack { } , ItemIsNotOnCooldown { item_type_s_to_check : Vec < Item > } , IsUsingItem { items_to_check : Vec < Item > } , HasAllItems { } , IsSwimming { } , HasItem { items_to_check_for : Vec < Item > } , BlockEquals { } , IsWearingItem { items_to_check_for : Vec < Item > } , IsNearLocation { center_location : Vec < Location > , radius : Option < Number > } , IsRiding { } , StandingOn { } , CmdEquals { } , IsStandingonBlock { block_to_check_for : Vec < Either < Block , Location > > } , IsGrounded { } , CursorItemEquals { itemss_to_check_for : Vec < Item > } , HotbarSlotEquals { slot_id_to_check : Number } , ItemEquals { } , IsHoldingMain { } , IsHoldingItem { items_to_check_for : Vec < Item > } , InventoryMenuSlotEquals { slots_to_check : Vec < Number > , items_to_check_for : Vec < Item > } , IsBlocking { } , HasPlotPermission { } , IsRidingEntity { spawn_egg : Vec < Either < Either < EntityType , Text > , MiniMessage > > } , IsSneaking { } , IsFlying { } , HasPotionEffect { effects : Vec < Potion > } , NameEquals { names_to_check_for : Vec < Text > } , InventoryTypeOpen { } , HasIteminSlot { slots_to_check : Vec < Number > , items_to_check_for : Vec < Item > } , IsSprinting { } , IsGliding { } , CmdArgEquals { } , }