use super::{Map, rect::Rect, TileType, Position, SHOW_MAPGEN_VISUALIZER, spawner};

mod simple_map;
use simple_map::SimpleMapBuilder;
mod common;
use common::*;
mod bsp_dungeon;
use bsp_dungeon::BspDungeonBuilder;
mod bsp_interiors;
use bsp_interiors::BspInteriorBuilder;
mod cellular_automata;
use cellular_automata::CellularAutomataBuilder;
mod drunkard;
use drunkard::DrunkardsWalkBuilder;

use specs::prelude::*;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    
    // For testing Purposes 
    //Box::new(DrunkardsWalkBuilder::winding_passage(new_depth))
    // /*
    let mut rng = rltk::RandomNumberGenerator::new();
    let builder = rng.roll_dice(1, 7);
    match builder {
        1 => Box::new(BspDungeonBuilder::new(new_depth)),
        2 => Box::new(BspInteriorBuilder::new(new_depth)),
        3 => Box::new(CellularAutomataBuilder::new(new_depth)),
        4 => Box::new(DrunkardsWalkBuilder::open_area(new_depth)),
        5 => Box::new(DrunkardsWalkBuilder::open_halls(new_depth)),
        6 => Box::new(DrunkardsWalkBuilder::winding_passage(new_depth)),
        _ => Box::new(SimpleMapBuilder::new(new_depth))
    }
    // */
}