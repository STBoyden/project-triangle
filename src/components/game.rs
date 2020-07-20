use raylib::prelude::*;
use std::collections::HashMap;

use super::{entity::Entity, map::Map, map_gen::load_map, menu::*, pause_menu::*};
use crate::gui::gui_cursor::*;
use crate::physics::physics_body::PhysicsBody;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameStates {
    Menu,
    Paused,
    Playing,
    Resetting,
    Quitting,
}

pub struct Game<'a> {
    player: &'a mut Entity,
    player_initial: Entity,
    cursor: &'a mut Cursor,
    title: &'a mut str,
    width: i32,
    height: i32,
    pub map: Map,
    pub current_state: &'a mut GameStates,
}

impl Game<'_> {
    pub fn new<'a>(
        player: &'a mut Entity,
        cursor: &'a mut Cursor,
        title: &'a mut str,
        width: i32,
        height: i32,
        current_state: &'a mut GameStates,
    ) -> Game<'a> {
        Game {
            player_initial: player.clone(),
            player,
            cursor,
            title,
            width,
            height,
            current_state,
            map: load_map("maps/map-test.json").ok().unwrap(),
        }
    }

    fn handle_keys(&mut self, rl_handler: &mut RaylibHandle) {
        if rl_handler.is_key_released(KeyboardKey::KEY_F11) {
            rl_handler.toggle_fullscreen();
        }

        match *self.current_state {
            GameStates::Playing => {
                if rl_handler.is_key_released(KeyboardKey::KEY_ESCAPE) {
                    *self.current_state = GameStates::Paused;
                }

                let move_speed = 10;
                if rl_handler.is_key_down(KeyboardKey::KEY_A) {
                    self.player.try_move((-move_speed, 0), &self.map.objects);
                } else if rl_handler.is_key_down(KeyboardKey::KEY_D) {
                    self.player.try_move((move_speed, 0), &self.map.objects);
                }

                #[cfg(debug_assertions)]
                if rl_handler.is_key_down(KeyboardKey::KEY_R) {
                    self.reload_map();
                }

                #[cfg(debug_assertions)]
                if rl_handler.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
                    && rl_handler.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON)
                {
                    let mut position = self.cursor.position;
                    let (offset_x, offset_y) = (position.0 % 10, position.1 % 10);

                    position.0 -= offset_x;
                    position.1 -= offset_y;

                    self.player.set_pos(position);
                }
            }
            GameStates::Paused => {
                if rl_handler.is_key_released(KeyboardKey::KEY_ESCAPE) {
                    *self.current_state = GameStates::Playing;
                }
            }
            GameStates::Menu => {}
            _ => {}
        }
    }

    pub fn initialise(&mut self) {
        let (mut rl_handler, rl_thread) = raylib::init()
            .size(self.width, self.height)
            .title(self.title)
            .build();

        rl_handler.set_exit_key(Option::None);
        rl_handler.set_target_fps(60);

        self.player_initial.set_pos(self.map.spawn_point);

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
        #[allow(unused_variables)]
        let mut should_draw_cursor = true;

        while !handler.window_should_close() {
            if *self.current_state == GameStates::Quitting {
                break;
            }

            self.handle_keys(handler);

            #[cfg(not(debug_assertions))]
            if should_draw_cursor {
                handler.show_cursor();
            } else {
                handler.hide_cursor();
            }

            let mut draw_func = handler.begin_drawing(thread);
            draw_func.clear_background(Color::WHITE);

            #[allow(unused_assignments)]
            match self.current_state {
                GameStates::Menu => {
                    should_draw_cursor = true;
                    let mut menu = Menu::new(&mut self.current_state);
                    menu.draw(&self.cursor, &mut draw_func);
                    if *self.player != self.player_initial {
                        self.reset();
                    }
                    self.cursor.draw(&mut draw_func, tex_map["cursor"], false);
                }
                GameStates::Paused => {
                    should_draw_cursor = true;
                    let mut pause_menu = PauseMenu::new(&mut self.current_state);
                    for object in self.map.objects.iter_mut() {
                        object.draw(&mut draw_func);
                    }
                    self.player.draw(&mut draw_func);
                    pause_menu.draw(&self.cursor, &mut draw_func);
                    self.cursor.draw(&mut draw_func, tex_map["cursor"], false);
                }
                GameStates::Playing => {
                    should_draw_cursor = false;

                    if self.map.objects.len() > 0 {
                        for object in self.map.objects.iter_mut() {
                            object.draw(&mut draw_func);
                        }

                        self.player.try_fall(&self.map.objects);
                    }

                    if self.map.entities.len() > 0 {
                        for entity in self.map.entities.iter_mut() {
                            entity.draw(&mut draw_func);
                        }

                        self.player.try_fall(&self.map.entities);
                    }

                    self.player.draw(&mut draw_func);

                    #[cfg(debug_assertions)]
                    self.cursor.draw(&mut draw_func, tex_map["cursor"], false);
                }
                GameStates::Resetting => {
                    self.reload_map();

                    *self.current_state = GameStates::Playing;
                }
                _ => {}
            }

            draw_func.draw_fps(0, 0);
        }
    }

    fn reload_map(&mut self) {
        self.player.set_pos(self.map.spawn_point);
    }

    fn reset(&mut self) {
        *self.player = self.player_initial;
    }
}
