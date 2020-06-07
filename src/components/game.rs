use std::collections::HashMap;
use raylib::prelude::*;
use std::collections::HashMap;

use super::player::Player;
use crate::gui::gui_cursor::Cursor;
use super::{menu::*, pause_menu::*, player::Player};
use crate::gui::gui_cursor::*;

#[repr(u32)]
#[derive(Debug)]
pub enum GameStates {
    Menu = 0,
    Paused = 1,
    Playing = 2,
    Quitting = 3,
}

pub struct Game<'a> {
    player: &'a mut Player,
    cursor: &'a mut Cursor,
    title: &'a mut str,
    pub current_state: &'a mut GameStates,
}

impl Game<'_> {
    pub fn new<'a>(player: &'a mut Player, cursor: &'a mut Cursor, title: &'a mut str,
                   current_state: &'a mut GameStates) -> Game<'a> {
        Game {
            player,
            cursor,
            title,
            current_state,
        }
    }

    pub fn initialise(&mut self, width: i32, height: i32) {
        let (mut rl_handler, rl_thread) = raylib::init()
            .size(width, height)
            .title(self.title)
            .build();

        rl_handler.disable_cursor();
        rl_handler.set_exit_key(Option::None);

        let mut initial_state = GameStates::Menu;
        let mut menu = super::menu::Menu::new(&mut initial_state);
        let mut texture_map = HashMap::new();
        let cursor_texture =
            rl_handler.load_texture(&rl_thread, "assets/cursor.png")
                .expect("Could not load cursor image asset");

        texture_map.insert("cursor".to_string(), &cursor_texture);

        self.update(&mut rl_handler, &rl_thread, &texture_map, &mut menu);
    }

    fn update(&mut self, handler: &mut RaylibHandle, thread: &RaylibThread,
              tex_map: &HashMap<String, &Texture2D>, menu: &mut super::menu::Menu) {
        let mut do_quit;

        match menu.current_state {
            GameStates::Quitting => do_quit = true,
            _ => do_quit = false
        }

        while !handler.window_should_close() {
            if do_quit {
                break
            }

            let mut draw_func = handler.begin_drawing(thread);
            draw_func.clear_background(Color::WHITE);


            match menu.current_state {
                GameStates::Menu => {
                    menu.draw(&self.cursor, &mut draw_func, &mut do_quit);
                    self.cursor.draw(&mut draw_func, tex_map["cursor"]);
                }
                _ => {
                    self.player.draw(&mut draw_func);
                }
            }
        }
    }
}
