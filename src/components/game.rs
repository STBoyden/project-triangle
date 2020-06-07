use raylib::prelude::*;
use std::collections::HashMap;

use super::{menu::*, pause_menu::*, player::Player};
use crate::gui::gui_cursor::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameStates {
    Menu,
    Paused,
    Playing,
    Quitting,
}

pub struct Game<'a> {
    player: &'a mut Player,
    cursor: &'a mut Cursor,
    title: &'a mut str,
    pub current_state: &'a mut GameStates,
}

impl Game<'_> {
    pub fn new<'a>(
        player: &'a mut Player,
        cursor: &'a mut Cursor,
        title: &'a mut str,
        current_state: &'a mut GameStates,
    ) -> Game<'a> {
        Game {
            player,
            cursor,
            title,
            current_state,
        }
    }

    pub fn initialise(&mut self, width: i32, height: i32) {
        let (mut rl_handler, rl_thread) =
            raylib::init().size(width, height).title(self.title).build();

        rl_handler.disable_cursor();
        rl_handler.set_exit_key(Option::None);

        let mut texture_map = HashMap::new();
        let cursor_texture = rl_handler
            .load_texture(&rl_thread, "assets/cursor.png")
            .expect("Could not load cursor image asset");

        texture_map.insert("cursor".to_string(), &cursor_texture);

        self.update(&mut rl_handler, &rl_thread, &texture_map);
    }

    fn update(
        &mut self,
        handler: &mut RaylibHandle,
        thread: &RaylibThread,
        tex_map: &HashMap<String, &Texture2D>,
    ) {
        while !handler.window_should_close() {
            if *self.current_state == GameStates::Quitting {
                break;
            }

            let mut draw_func = handler.begin_drawing(thread);
            draw_func.clear_background(Color::WHITE);

            match self.current_state {
                GameStates::Menu => {
                    let mut menu = Menu::new(&mut self.current_state);
                    menu.draw(&self.cursor, &mut draw_func);
                    self.cursor.draw(&mut draw_func, tex_map["cursor"]);
                }
                GameStates::Paused => {
                    let mut pause_menu = PauseMenu::new(&mut self.current_state);
                    pause_menu.draw(&self.cursor, &mut draw_func);
                    self.cursor.draw(&mut draw_func, tex_map["cursor"]);

                    if draw_func.is_key_released(KeyboardKey::KEY_ESCAPE) {
                        *self.current_state = GameStates::Playing;
                    }
                }
                GameStates::Playing => {
                    self.player.draw(&mut draw_func);

                    if draw_func.is_key_released(KeyboardKey::KEY_ESCAPE) {
                        *self.current_state = GameStates::Paused;
                    }
                }
                _ => {}
            }
        }
    }
}
