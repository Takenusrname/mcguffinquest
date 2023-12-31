use rltk::{Algorithm2D, BaseMap, Point, RGB, Rltk, RandomNumberGenerator};
use std::cmp::{max, min};
use serde::{Serialize, Deserialize};
use specs::prelude::*;
use std::collections::HashSet;

use super::colors::*;
use super::glyph_index::{AETHER_GLYPH, FLOOR_GLYPH, STAIRS_GLYPH};
use super::rect::Rect;

pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 40;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall, Floor, DownStairs
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub bloodstains: HashSet<usize>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>
}

impl Map {
    pub fn xy_idx(&self, x: i32, y:i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1  + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32){
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 { return false; }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn new_map_rooms_and_corridors(new_depth: i32) -> Map {
        let mut map = Map{
            tiles: vec![TileType::Wall; MAPCOUNT],
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            tile_content: vec![Vec::new(); MAPCOUNT],
            depth: new_depth,
            bloodstains: HashSet::new()
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;
    
        let mut rng = RandomNumberGenerator::new();
    
        for _ in 0..MAX_ROOMS {
            let w: i32 = rng.range(MIN_SIZE, MAX_SIZE);
            let h: i32 = rng.range(MIN_SIZE, MAX_SIZE);
            let x: i32 = rng.range(2, map.width - w - 1) - 1;
            let y: i32 = rng.range(2, map.height - h - 1) - 1;
    
            let new_room: Rect = Rect::new(x, y, w, h);
            let mut ok: bool = true;
    
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                map.apply_room_to_map(&new_room);
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
    
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }
                map.rooms.push(new_room);
            }
        }

        let stairs_position = map.rooms[map.rooms.len() - 1].center();
        let stairs_idx = map.xy_idx(stairs_position.0, stairs_position.1);
        map.tiles[stairs_idx] = TileType::DownStairs;
    
        map
    }

}

pub fn is_inbounds(map: &Map, x: i32, y: i32) -> bool {
    if x < 0 || x > map.width - 1 || y < 0 || y > map.height - 1 { return false; } else { return true;}
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> rltk::FontCharType {
    if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 as i32 { return 35; }
    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask += 1; }
    if is_revealed_and_wall(map, x, y + 1) { mask += 2; }
    if is_revealed_and_wall(map, x - 1, y) { mask += 4; }
    if is_revealed_and_wall(map, x + 1, y) { mask += 8; }
    
    match mask {
        0 => { 9 } // ○ pillar
        1 => { 208 } // ╨ wall only to north
        2 => { 210 } // ╥ wall only to south
        3 => { 186 } // ║ wall to north and south
        4 => { 181 } // ╡ wall only to west 
        5 => { 188 } // ╝ wall to north and west
        6 => { 187 } // ╗ Wall to south and west
        7 => { 185 } // ╣ wall to north, south and west
        8 => { 198 } // ╞ wall to the east
        9 => { 200 } // ╚ wall to north and east
        10 => { 201 } // ╔ wall to south and east
        11 => { 204 } // ╠ wall to north, south and east
        12 => { 205 } // ═ wall to east and west 
        13 => { 202 } // ╩ wall to east, west and north
        14 => { 203 } // ╦ wall to east west and south
        15 => { 206 } // ╬ wall to north, east, south and west
        _ => { 35 } // # missed one?

    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall        
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);

        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();

        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx - w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx + w, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) { exits.push(((idx - w) - 1, 1.45)) };
        if self.is_exit_valid(x + 1, y - 1) { exits.push(((idx - w) + 1, 1.45)) };
        if self.is_exit_valid(x - 1, y + 1) { exits.push(((idx + w) - 1, 1.45)) };
        if self.is_exit_valid(x + 1, y + 1) { exits.push(((idx + w) + 1, 1.45)) };

        exits   
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {

    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg: RGB;
            let mut bg: RGB = return_rgb(DEFAULT_BG);
            // Render a tile depending upon the tile type
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437(FLOOR_GLYPH);
                    fg = return_rgb(FLOOR_COLOR);
                }
                TileType::Wall => {
                    glyph = wall_glyph(&*map, x, y);
                    fg = return_rgb(WALL_COLOR);
                }
                TileType::DownStairs => {
                    glyph = rltk::to_cp437(STAIRS_GLYPH);
                    fg = return_rgb(STAIRS_FG);
                }
            }
            if map.bloodstains.contains(&idx) { bg = return_rgb(BLOOD_BG);}
            if !map.visible_tiles[idx] { 
                fg = return_rgb(OUT_OF_VIEW);
                bg = return_rgb(DEFAULT_BG);
            } 
            ctx.set(x, y, fg, bg, glyph);
        } else {
            let glyph;
            let fg = return_rgb(AETHER_FG);
            let bg = return_rgb(DEFAULT_BG);
            glyph = rltk::to_cp437(AETHER_GLYPH);
            ctx.set(x, y, fg, bg, glyph);
        }
        
        // Move the coordinates
        x += 1;
        if x > MAPWIDTH as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}