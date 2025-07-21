#![allow(unused)]
mod db16;
use db16::*;
mod db32;
mod lcd;
use rltk::RGB;


pub fn return_u8_rgb(color_values: (f32,f32,f32)) -> RGB {
    let rgb: RGB = RGB::from_f32(color_values.0, color_values.1, color_values.2);
    return rgb
}

pub fn return_f32_rgb(color_values: (u8, u8, u8)) -> RGB {
    let rgb: RGB = RGB::from_u8(color_values.0, color_values.1, color_values.2);
    return rgb
}

// Defaults

pub const DEFAULT_FG: (f32, f32, f32) = DB16_LIGHT8_F32;
pub const DEFAULT_BG: (f32, f32, f32) = DB16_DARK1_F32;

// Screen burn
// dark5, dark7, light1, light2, light4, light6 preferred
pub const SCREENBURN_COLOR: (u8, u8, u8) = DB16_LIGHT1;

// Player
pub const PLAYER_FG: (f32, f32, f32) = DB16_LIGHT8_F32;

// Mobs, NPC's, etc.
pub const ENEMY_FG: (f32, f32, f32) = DB16_DARK7_F32;
pub const ORC_FG: (f32, f32, f32) = DB16_LIGHT3_F32;
pub const GOBLIN_FG: (f32, f32, f32) = DB16_LIGHT4_F32;

// Items
pub const HEALTH_POT_FG: (f32, f32, f32) = DB16_DARK7_F32;
pub const CONFUSION_FG: (f32, f32, f32) = DB16_LIGHT7_F32;
pub const MMS_FG: (f32, f32, f32) = DB16_LIGHT6_F32;
pub const FIREBALL_FG: (f32, f32, f32) = DB16_LIGHT2_F32;
pub const RATION_FG: (f32, f32, f32) = DB16_DARK6_F32;
pub const MAGICMAP_FG: (f32, f32, f32) = DB16_DARK2_F32;

// Equipment
pub const DAGGER_FG: (f32, f32, f32) = DB16_LIGHT6_F32;
pub const SHIELD_FG: (f32, f32, f32) = DB16_LIGHT6_F32;
pub const TOWER_S_FG: (f32, f32, f32) = DB16_LIGHT7_F32;
pub const SWORD_FG: (f32, f32, f32) = DB16_LIGHT7_F32;

// Dungeon
pub const WALL_COLOR: (f32, f32, f32) = DB16_LIGHT5_F32;
pub const FLOOR_COLOR: (f32, f32, f32) = DB16_DARK5_F32;
pub const STAIRS_FG: (f32, f32, f32) = DB16_LIGHT2_F32;
pub const OUT_OF_VIEW: (f32, f32, f32) = DB16_DARK2_F32;
pub const AETHER_FG: (f32,f32,f32) = DB16_DARK4_F32;
pub const BLOOD_BG: (f32, f32, f32) = DB16_DARK7_F32;
pub const BEARTRAP_FG: (f32, f32, f32) = DB16_DARK7_F32;

// User Interface
pub const HEALTH_BAR_FG: (f32, f32, f32) = DB16_DARK7_F32;
pub const MOUSE_BG: (f32, f32, f32) = DB16_LIGHT5_F32;
pub const TOOLTIP_BG: (f32, f32, f32) = DB16_DARK8_F32;
pub const TARGET_BG: (f32, f32, f32) = DB16_LIGHT1_F32;
pub const ERROR_BG: (f32,f32,f32) = DB16_DARK7_F32;
pub const TITLE_FG: (f32,f32,f32) = DB16_LIGHT7_F32;
pub const SELECT_FG: (f32, f32, f32) = DB16_LIGHT8_F32;
pub const NOTSELECT_FG: (f32, f32, f32) = DB16_LIGHT6_F32;

// Particles
pub const AOE_FG: (f32, f32, f32) = DB16_LIGHT2_F32;
pub const HEAL_FG: (f32, f32, f32) = DB16_LIGHT4_F32;
pub const POW_FG: (f32, f32, f32) = DB16_LIGHT2_F32;
pub const DMG_FG: (f32, f32, f32) = DB16_DARK7_F32;

// UI - Menu
pub const MENU_FG: (f32, f32, f32) = DB16_LIGHT8_F32;
pub const INV_BG: (f32, f32, f32) = DB16_LIGHT6_F32;
pub const DROP_BG: (f32, f32, f32) = DB16_DARK7_F32;
pub const REMOVE_BG: (f32, f32, f32) = DB16_LIGHT2_F32;
pub const HELP_FG: (f32, f32, f32) = DB16_LIGHT3_F32;
pub const HELP_BG: (f32, f32, f32) = DB16_LIGHT1_F32;
pub const CTRL_FG: (f32, f32, f32) = DB16_LIGHT7_F32;
pub const CN_FG: (f32, f32, f32) = DB16_LIGHT7_F32;
pub const CL_FG: (f32, f32, f32) = DB16_LIGHT6_F32;
pub const CQ_FG: (f32, f32, f32) = DB16_DARK7_F32;

// UI - Hunger Clock
pub const WELLFED: (f32, f32, f32) = DB16_LIGHT4_F32;
pub const FED: (f32, f32, f32) = DB16_DARK6_F32;
pub const HUNGRY: (f32, f32, f32) = DB16_LIGHT2_F32;
pub const STARVING: (f32, f32, f32) = DB16_DARK7_F32;

// UI - GameOver
pub const GAME_OVER_FG: (f32, f32, f32) = DB16_DARK7_F32;
