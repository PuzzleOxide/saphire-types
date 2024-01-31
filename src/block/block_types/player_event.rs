use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum PlayerEvent {
    PlayerCloseInventoryEvent {},
    PlayerStartFlightEvent {},
    PlayerBreakBlockEvent {},
    PlayerStartSprintEvent {},
    MobKillPlayerEvent {},
    PlayerShootBowEvent {},
    PlayerStopFlightEvent {},
    PlayerTameMobEvent {},
    PlayerLeftClickEvent {},
    PlayerTakeDamageEvent {},
    PlayerProjectileHitEvent {},
    PlayerKillPlayerEvent {},
    PlayerVehicleJumpEvent {},
    PlayerClickInventorySlotEvent {},
    PlayerRespawnEvent {},
    PlayerSwapHandsEvent {},
    ResourcePackLoad {},
    PlayerDamageEntityEvent {},
    PlayerSneakEvent {},
    PlayerHealEvent {},
    PlayerRightClickPlayerEvent {},
    PlayerConsumeItemEvent {},
    PlayerDeathEvent {},
    PlayerPlaceBlockEvent {},
    PlayerWalkEvent {},
    PlayerPickUpItemEvent {},
    PlayerDismountEvent {},
    PotionCloudImbuePlayerEvent {},
    PlayerLeaveGameEvent {},
    PlayerDropItemEvent {},
    PlayerChangeSlotEvent {},
    PlayerRightClickEntityEvent {},
    PlayerHorseJumpEvent {},
    PlayerShootProjectileEvent {},
    PlayerUnsneakEvent {},
    PlayerFishEvent {},
    FallDamage {},
    PlayerBreakItemEvent {},
    LoopEvent {},
    PlayerResurrectEvent {},
    PlayerRightClickEvent {},
    PlayerClickMenuSlotEvent {},
    PlayerRiptideEvent {},
    PlayerKillMobEvent {},
    PlayerJoinGameEvent {},
    EntityDamagePlayerEvent {},
    PlayerStopSprintingEvent {},
    PlayerJumpEvent {},
    ProjectileDamagePlayerEvent {},
    PlayerCommandEvent {},
    ResourcePackDecline {},
    PlayerDamagePlayerEvent {},
}
impl PlayerEvent {
    pub fn compile(&self) -> Value {
        match self {
            PlayerEvent::PlayerCloseInventoryEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CloseInv".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerStartFlightEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StartFly".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerBreakBlockEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BreakBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerStartSprintEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StartSprint".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::MobKillPlayerEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MobKillPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerShootBowEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShootBow".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerStopFlightEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StopFly".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerTameMobEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TameEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerLeftClickEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LeftClick".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerTakeDamageEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlayerTakeDmg".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerProjectileHitEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ProjHit".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerKillPlayerEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("KillPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerVehicleJumpEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("VehicleJump".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerClickInventorySlotEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClickInvSlot".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerRespawnEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Respawn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerSwapHandsEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SwapHands".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::ResourcePackLoad {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PackLoad".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerDamageEntityEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DamageEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerSneakEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Sneak".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerHealEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlayerHeal".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerRightClickPlayerEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClickPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerConsumeItemEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Consume".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerDeathEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Death".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerPlaceBlockEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlaceBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerWalkEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Walk".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerPickUpItemEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PickUpItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerDismountEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Dismount".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PotionCloudImbuePlayerEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CloudImbuePlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerLeaveGameEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Leave".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerDropItemEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DropItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerChangeSlotEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ChangeSlot".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerRightClickEntityEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClickEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerHorseJumpEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HorseJump".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerShootProjectileEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShootProjectile".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerUnsneakEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Unsneak".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerFishEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Fish".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::FallDamage {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FallDamage".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerBreakItemEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("BreakItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::LoopEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LoopEvent".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerResurrectEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlayerResurrect".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerRightClickEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RightClick".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerClickMenuSlotEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClickMenuSlot".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerRiptideEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Riptide".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerKillMobEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("KillMob".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerJoinGameEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Join".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::EntityDamagePlayerEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EntityDmgPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerStopSprintingEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StopSprint".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerJumpEvent {} => {
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
            PlayerEvent::ProjectileDamagePlayerEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ProjDmgPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerCommandEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Command".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::ResourcePackDecline {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PackDecline".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            PlayerEvent::PlayerDamagePlayerEvent {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PlayerDmgPlayer".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
