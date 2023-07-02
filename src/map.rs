use rltk::{RGB, Rltk, RandomNumberGenerator};
use std::cmp::{max, min};
use specs::prelude::*;

use super::colors::*;
use super::glyph_index::FLOOR_GLYPH;
use super::rect::Rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32
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

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map{
            tiles: vec![TileType::Wall; 80*50],
            rooms: Vec::new(),
            width: 80,
            height: 50
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 12;
    
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
    
        map
    }

}

pub fn is_inbounds(map: &Map, x: i32, y: i32) -> bool {
    if x < 0 || x > map.width - 1 || y < 0 || y > map.height - 1 { 
        return false
    } else {
        return true
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> rltk::FontCharType {
    if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 { return 35; }
    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) {
            mask += 1;
        }
    if is_revealed_and_wall(map, x, y + 1) {
            mask += 2;
        }
    if is_revealed_and_wall(map, x - 1, y) {
          mask += 4;}

    if is_revealed_and_wall(map, x + 1, y) {
            mask += 8;
    }
    
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
        13 => { 202 } // ╩ wall to east, west and south
        14 => { 203 } // ╦ wall to east west and north
        15 => { 206 } // ╬ wall to north, east, south and west
        _ => { 35 } // # missed one?

    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let mut y = 0;
    let mut x = 0;

    for tile in map.tiles.iter() {
        let glyph;
        let fg: (f32, f32, f32);
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                glyph = rltk::to_cp437(FLOOR_GLYPH);
                fg = FLOOR_COLOR;
            }
            TileType::Wall => {
                glyph = wall_glyph(&*map, x, y);
                fg = WALL_COLOR;
            }
        }

        ctx.set(x, y, RGB::from_f32(fg.0,fg.1,fg.2), RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.0, DEFAULT_BG.0), glyph);

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}