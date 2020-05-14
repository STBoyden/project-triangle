use std::collections::HashMap;

use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;

pub enum GameStates {
    Menu = 0,
    Paused = 1,
    Playing = 2,
    Quitting = 3,
}

pub struct Game<'a> {
    pub player: &'a mut super::player::Player,
    pub cursor: &'a mut super::gui::gui_cursor::Cursor,
    pub title: &'a mut str,
    pub menu: super::menu::Menu,
}

impl Game<'_> {
    pub fn initialise(&mut self, width: i32, height: i32) {
        let (mut rl_handler, rl_thread) = raylib::init()
            .size(width, height)
            .title(self.title)
            .build();

        rl_handler.disable_cursor();
        rl_handler.set_exit_key(Option::None);

        let mut texture_map = HashMap::new();
        let cursor_texture =
            rl_handler.load_texture(&rl_thread, "assets/cursor.png")
                .expect("Could not load cursor image asset");

        texture_map.insert("cursor".to_string(), &cursor_texture);

        self.update(&mut rl_handler, &rl_thread, &texture_map);
    }

    fn update(&mut self, handler: &mut RaylibHandle, thread: &RaylibThread,
              tex_map: &HashMap<String, &Texture2D>) {
        let mut do_quit;

        match self.menu.current_state {
            GameStates::Quitting => do_quit = true,
            _ => do_quit = false
        }

        while !handler.window_should_close() {
            if do_quit {
                break
            }
            if handler.is_key_released(KEY_TAB) {
                match self.menu.current_state {
                    GameStates::Menu => self.menu.change_state("playing"),
                    _ => self.menu.change_state("menu")
                }
            }

            let mut draw_func = handler.begin_drawing(thread);
            draw_func.clear_background(Color::WHITE);

            self.menu.draw(&self.cursor, &mut draw_func, &mut do_quit);
            self.cursor.draw(&mut draw_func, tex_map["cursor"]);
            // match self.menu.current_state {
            //     GameStates::Menu => {
            //     }
            //     _ => self.player.draw(&mut draw_func)
            // }
        }
    }
}
