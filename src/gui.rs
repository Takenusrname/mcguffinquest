use rltk::{ Point, RGB, Rltk, VirtualKeyCode};
use specs::prelude::*;

use rltk::Rect;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::colors::*;
use super::{ CombatStats, Equipped, game_log::GameLog, Hidden, HungerClock, HungerState, InBackpack, Map, Name, Player, Position, rex_assets::RexAssets, RunState, State, Viewshed };

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    let fg: RGB = return_u8_rgb(DEFAULT_FG);
    let bg: RGB = return_u8_rgb(DEFAULT_BG);

    let m_bg: RGB = return_u8_rgb(MOUSE_BG);

    let info_title = format!(" Player Info. ");
    let msg_title = format!(" Message Log ");

    let bg_rect = Rect::with_size(0, 40, 79, 49);

    ctx.fill_region(bg_rect, rltk::to_cp437(' '), fg, bg);

    ctx.draw_hollow_box(0, 40, 18, 9, fg, bg);
    ctx.draw_hollow_box(18, 40, 79-18, 9, fg, bg);

    ctx.set(18, 40, fg, bg, rltk::to_cp437('┬'));
    ctx.set(18, 49, fg, bg, rltk::to_cp437('┴'));

    ctx.set(1, 40, fg, bg, rltk::to_cp437('┤'));
    ctx.print_color(2, 40, fg, bg, info_title);
    ctx.set(16, 40, fg, bg, rltk::to_cp437('├'));

    ctx.set(19, 40, fg, bg, rltk::to_cp437('┤'));
    ctx.print_color(20, 40, fg, bg, msg_title);
    ctx.set(33, 40, fg, bg, rltk::to_cp437('├'));
    
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let hunger = ecs.read_storage::<HungerClock>();

    for (_player, stats, hc) in (&players, &combat_stats, &hunger).join() {
        let health = format!("HP: {} / {} ", stats.hp, stats.max_hp);        
        
        ctx.print_color(2, 42, fg, bg, &health);

        let bar_fg: RGB = return_u8_rgb(HEALTH_BAR_FG);
        let bar_bg: RGB = return_u8_rgb(DEFAULT_BG);

        ctx.draw_bar_horizontal(2, 43, 15, stats.hp, stats.max_hp, bar_fg, bar_bg);

        match hc.state {
            HungerState::WellFed => ctx.print_color(2, 44, return_u8_rgb(WELLFED), return_u8_rgb(DEFAULT_BG), "Well Fed"),
            HungerState::Normal => ctx.print_color(2, 44, return_u8_rgb(FED), return_u8_rgb(DEFAULT_BG), "Fed"),
            HungerState::Hungry => ctx.print_color(2, 44, return_u8_rgb(HUNGRY), return_u8_rgb(DEFAULT_BG), "Hungry"),
            HungerState::Starving => ctx.print_color(2, 44, return_u8_rgb(STARVING), return_u8_rgb(DEFAULT_BG), "Starving"),
        }
    }

    let map = ecs.fetch::<Map>();
    let depth = format!(" Depth: {} ", map.depth);
    ctx.print_color(1, 49, return_u8_rgb(DEFAULT_BG), return_u8_rgb(DEFAULT_FG), &depth);

    let log = ecs.fetch::<GameLog>();

    let mut y = 41;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print_color(20, y, fg, bg, s);}
        y += 1;
    }

    // Draw Mouse Cursor
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, m_bg);
    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let hidden = ecs.read_storage::<Hidden>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { return; }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position, _hidden) in (&names, &positions, !&hidden).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    let fg: RGB = return_u8_rgb(DEFAULT_FG);
    let bg: RGB = return_u8_rgb(TOOLTIP_BG);

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 { width = s.len() as i32; }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x, y, fg, bg, s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x - i, y, fg, bg, &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, fg, bg, &"->".to_string());
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y  = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, fg, bg, s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x + 1 + i, y, fg, bg, &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, fg, bg, &"<-".to_string());
        }
    }
}

fn inventory_frame(ctx: &mut Rltk, count: usize , x: i32, y: i32, w: i32, fg: RGB, bg: RGB, ctrl_fg: RGB, action: &str) {
    let bg_rect = Rect::with_size(x, y - 2, w, (count + 3) as i32);
    let menu_text: &str;

    if action == "unequip" {
        menu_text = " Unequip Which Item? ";
    } else if action == "drop" {
        menu_text = " Drop Which Item? ";
    } else if action == "use" {
        menu_text = " Use Which Item? ";
    } else {
        menu_text = "ERROR";
    }

    let start_char = rltk::to_cp437('┤');
    let end_char = rltk::to_cp437('├');

    ctx.fill_region(bg_rect, rltk::to_cp437(' '), fg, bg);
    ctx.draw_hollow_box(x, y - 2, w, (count + 3) as i32, fg, bg);
    ctx.print_color(x + 1, y - 2, bg, fg, menu_text);
    ctx.print_color(x + 15, y + count as i32 + 1, ctrl_fg, bg, " ESC ");
    ctx.print_color(x + 20, y + count as i32 + 1, fg, bg, "to cancel ");
    ctx.set(x + 14, y + count as i32 + 1, fg, bg, start_char);
    ctx.set(x + 30, y + count as i32 + 1, fg, bg, end_char);

}

fn inventory_selection(ctx: &mut Rltk, x: i32, y: i32, fg: RGB, bg: RGB, ctrl_fg: RGB, glyph: rltk::FontCharType, selection_name: &String) {

    ctx.set(x, y, fg, bg, rltk::to_cp437('('));
    ctx.set(x + 1, y, ctrl_fg, bg, glyph);
    ctx.set(x + 2, y, fg, bg, rltk::to_cp437(')'));

    ctx.print(x + 4, y, selection_name);
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult { Cancel, NoResponse, Selected }

pub fn show_drop_use_inventory(gs: &mut State, ctx: &mut Rltk, action: &str) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names).join().filter(|item| item.0.owner == *player_entity );
    let count = inventory.count();

    let x: i32 = 15;
    let mut y = (25 - (count / 2)) as i32;
    let w: i32 = 31;

    let fg: RGB;
    let bg: RGB;
    let ctrl_fg: RGB = return_u8_rgb(CTRL_FG);

    if action == "drop" {
        fg = return_u8_rgb(MENU_FG);
        bg = return_u8_rgb(DROP_BG);
    } else if action == "use" {
        fg = return_u8_rgb(MENU_FG);
        bg = return_u8_rgb(INV_BG);
    } else {
        fg = return_u8_rgb(DEFAULT_FG);
        bg = return_u8_rgb(DEFAULT_BG);
    }

    inventory_frame(ctx, count, x, y, w, fg, bg, ctrl_fg, action);
    

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity) {
        let glyph = 97 + j;
        inventory_selection(ctx, x + 2, y, fg, bg, ctrl_fg, glyph, &name.name.to_string());

        equippable.push(entity);
        y += 1;
        j += 1; 
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        return (ItemMenuResult::Selected, Some(equippable[selection as usize]));
                    }
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }

}

pub fn remove_item_menu(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<Equipped>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names).join().filter(|item| item.0.owner == *player_entity );
    let count = inventory.count();

    let fg: RGB = return_u8_rgb(MENU_FG);
    let bg: RGB = return_u8_rgb(REMOVE_BG);
    let ctrl_fg: RGB = return_u8_rgb(CTRL_FG);
    
    let x: i32 = 15;
    let mut y = (25 - (count / 2)) as i32;
    let w: i32 = 31;

    inventory_frame(ctx, count, x, y, w, fg, bg, ctrl_fg, "unequip");

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity) {
        let glyph = 97 + j;
        inventory_selection(ctx, x + 2, y, fg, bg, ctrl_fg, glyph, &name.name.to_string());

        equippable.push(entity);
        y += 1;
        j += 1; 
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        return (ItemMenuResult::Selected, Some(equippable[selection as usize]));
                    }
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }

}

#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoSelection, QuitToMenu    
}

pub fn game_over(ctx: &mut Rltk) -> GameOverResult {
    let y: i32 = 15;
    let game_over_fg: RGB = return_u8_rgb(GAME_OVER_FG);
    let fg: RGB = return_u8_rgb(DEFAULT_FG);
    let bg: RGB = return_u8_rgb(DEFAULT_BG);
    let ctrl_fg: RGB = return_u8_rgb(CTRL_FG);

    ctx.print_color_centered(y, game_over_fg, bg, "Your journey has ended!");
    ctx.print_color_centered(y + 2, fg, bg, "You have failed your Quest to Collect the McGuffin");

    ctx.print_color_centered(y + 4, ctrl_fg, bg, "Press any key to return to the menu.");

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(_) => GameOverResult::QuitToMenu
    }

}

pub fn ranged_target(gs: &mut State, ctx: &mut Rltk, range: i32) -> (ItemMenuResult, Option<Point>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    // Targeting message
    let fg: RGB = return_u8_rgb(CTRL_FG);
    let bg: RGB = return_u8_rgb(DEFAULT_BG);
    let msg = "Select Target";
    ctx.print_color(5, 0, fg, bg, msg);

    // Highlight available target cells
    let target_bg: RGB = return_u8_rgb(TARGET_BG);
    let mut available_cells = Vec::new();
    let visible = viewsheds.get(*player_entity);
    if let Some(visible) = visible {
        // We have a viewshed
        for idx in visible.visible_tiles.iter() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance <= range as f32 {
                ctx.set_bg(idx.x, idx.y, target_bg);
                available_cells.push(idx);
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    // Draw mouse cursor
    let valid_bg: RGB = return_u8_rgb(MOUSE_BG);
    let invalid_bg: RGB = return_u8_rgb(ERROR_BG);
    let mouse_pos = ctx.mouse_pos();
    let mut valid_target = false;
    for idx in available_cells.iter() { if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 { valid_target = true; } }
    if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, valid_bg);
        if ctx.left_click {
            return (ItemMenuResult::Selected, Some(Point::new(mouse_pos.0, mouse_pos.1)));
        }
    } else {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, invalid_bg);
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }
    (ItemMenuResult::NoResponse, None)
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection { NewGame, LoadGame, Quit }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult { NoSelection{ selected: MainMenuSelection}, Selected{ selected: MainMenuSelection} }

pub fn main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {

    let save_exists = super::saveload_system::does_save_exist();
    let runstate = gs.ecs.fetch::<RunState>();

    let title_fg: RGB = return_u8_rgb(TITLE_FG);
    let bg: RGB = return_u8_rgb(DEFAULT_BG);

    let select_fg: RGB = return_u8_rgb(SELECT_FG);
    let notselet_fg: RGB = return_u8_rgb(NOTSELECT_FG);

    let select_cn_fg: RGB = return_u8_rgb(CN_FG);
    let select_cl_fg: RGB = return_u8_rgb(CL_FG);
    let select_cq_fg: RGB = return_u8_rgb(CQ_FG);

    let sel_glyph = rltk::to_cp437('►');

    let assets = gs.ecs.fetch::<RexAssets>();
    ctx.render_xp_sprite(&assets.menu, 0, 0);

    let x: i32 = 5;
    let mut y = 23;
    
    ctx.draw_hollow_box_double(x - 3, y - 2, 27, 11, title_fg, bg);
    ctx.print_color(x - 1, y, select_fg, bg, "Use ▲/▼ arrows and Enter");
    ctx.print_color(x - 1, y + 1, select_fg, bg, "to make selection." );

    if let RunState::MainMenu { menu_selection: selection } = *runstate {
        y += 3;
        if selection == MainMenuSelection::NewGame {
            ctx.set(x - 1, y, select_cn_fg, bg, sel_glyph);
            ctx.print_color(x + 1, y, select_fg, bg, "New Game");
        } else {
            ctx.print_color(x + 1, y, notselet_fg, bg, "New Game");
        }
        y += 2;
        if save_exists {
            if selection == MainMenuSelection::LoadGame {
                ctx.set(x - 1, y, select_cl_fg, bg, sel_glyph);
                ctx.print_color(x + 1, y, select_fg, bg, "Load Game");
            } else {
                ctx.print_color(x + 1, y, notselet_fg, bg, "Load Game");
            }
            y += 2;
        }
        if selection == MainMenuSelection::Quit {
            ctx.set(x - 1, y, select_cq_fg, bg, sel_glyph);
            ctx.print_color(x + 1, y, select_fg, bg, "Quit");
        } else {
            ctx.print_color(x + 1, y, notselet_fg, bg, "Quit");
        }

        match ctx.key {
            None => return MainMenuResult::NoSelection { selected: selection },
            Some(key) => {
                match key {
                    VirtualKeyCode::Escape => { return MainMenuResult::NoSelection { selected: MainMenuSelection::Quit } }
                    VirtualKeyCode::Up => {
                        let mut newselection;
                        match selection {
                            MainMenuSelection::NewGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::NewGame,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::LoadGame
                        }
                        if newselection == MainMenuSelection::LoadGame && !save_exists {
                            newselection = MainMenuSelection::NewGame;
                        }
                        return MainMenuResult::NoSelection { selected: newselection }
                    }
                    VirtualKeyCode::Down => {
                        let mut newselection;
                        match selection {
                            MainMenuSelection::NewGame => newselection = MainMenuSelection::LoadGame,
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::NewGame
                        }
                        if newselection == MainMenuSelection::LoadGame && !save_exists {
                            newselection = MainMenuSelection::Quit;
                        }
                        return MainMenuResult::NoSelection { selected: newselection }
                    }
                    VirtualKeyCode::Return => return MainMenuResult::Selected { selected: selection },
                    _ => return MainMenuResult::NoSelection { selected: selection }
                }
            }
            
        }
    }

    MainMenuResult::NoSelection { selected: MainMenuSelection::NewGame }
}