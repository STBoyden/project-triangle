use ptgui::prelude::*;
use raylib::prelude::*;
use std::collections::HashMap;

use crate::physics::{physics_body::PhysicsBody, physics_collider::PhysicsCollider};

use super::{
    entity::Entity,
    map::Map,
    map_gen::load_map,
    settings_loader::{load_settings, read_settings, GameSettings},
};

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
    pub current_state: GameStates,
    main_menu: GuiHandler<GameStates>,
    pause_menu: GuiHandler<GameStates>,
    settings_menu: GuiHandler<GameStates>,
    clear_colour: Colour,
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
            main_menu: GuiHandler::new(clear_colour),
            pause_menu: GuiHandler::new(clear_colour),
            settings_menu: GuiHandler::new(clear_colour),
            clear_colour,
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

        self.main_menu
            .add_button_with_position("Start", "play", button_position)
            .add_button("Settings", "settings_menu")
            .add_button("Quit", "quit")
            .set_button_fix_widths(true)
            .set_button_action_function(|state, action| super::change_state(state, action));

        self.pause_menu
            .add_button_with_position("Resume", "play", button_position)
            .add_button("Reset level", "play_reset")
            .add_button("Quit to menu", "menu")
            .set_button_fix_widths(true)
            .set_button_action_function(|state, action| super::change_state(state, action));

        self.settings_menu
            .add_button_with_position("Toggle FPS", "settings_menu_toggle_fps", button_position)
            .add_button("Toggle fullscreen", "settings_menu_toggle_fullscreen")
            .add_button("Resolution", "")
            .add_button("Back to menu", "menu")
            .set_button_fix_widths(true)
            .set_button_action_function(|state, action| super::change_state(state, action));

        self.update(&mut rl_handler, &rl_thread, player, settings);
    }

    fn update(
        &mut self,
        handler: &mut RaylibHandle,
        thread: &RaylibThread,
        player: &mut Entity,
        settings: GameSettings,
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
                handler.enable_cursor();
            } else {
                handler.hide_cursor();
                handler.disable_cursor();
            }

            let mut draw_func = handler.begin_drawing(thread);
            draw_func.clear_background(Color::WHITE);

            #[allow(unused_assignments)]
            match self.current_state {
                GameStates::Menu => {
                    should_draw_cursor = true;
                    let mut draw_func = self
                        .main_menu
                        .execute_actions(&mut self.current_state)
                        .draw(handler, thread)
                        .unwrap();
                    if player.get_pos() != self.map.spawn_point {
                        self.reset(player);
                    }
                    if settings.show_fps {
                        draw_func.draw_fps(0, 0);
                    }
                    self.cursor.draw(&mut draw_func, &tex_map["cursor"], false);
                }
                GameStates::Paused => {
                    should_draw_cursor = true;
                    self.pause_menu.clear_external_draws();
                    for objects in self.map.objects.iter_mut() {
                        self.pause_menu.add_external_draw(Box::new(*objects));
                    }
                    let mut draw_func = self
                        .pause_menu
                        .execute_actions(&mut self.current_state)
                        .add_external_draw(Box::new(*player))
                        .draw(handler, thread)
                        .unwrap();

                    if settings.show_fps {
                        draw_func.draw_fps(0, 0);
                    }
                    self.player.draw(&mut draw_func);
                    pause_menu.draw(&self.cursor, &mut draw_func, &tex_map["vignette"]);
                    self.cursor.draw(&mut draw_func, &tex_map["cursor"], false);
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

                    if settings.show_fps {
                        draw_func.draw_fps(0, 0);
                    }
                }
                GameStates::Settings => {
                    should_draw_cursor = true;

                    let mut draw_func = self
                        .settings_menu
                        .execute_actions(&mut self.current_state)
                        .draw(handler, thread)
                        .unwrap();

                    if settings.show_fps {
                        draw_func.draw_fps(0, 0);
                    }
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

    fn load_textures(
        &self,
        texture_map: &mut HashMap<String, Texture2D>,
        texture_names: Vec<String>,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        for texture_name in texture_names {
            let error = format!("Could not load \"{}\"", texture_name);
            let file_name = format!("assets/{}.png", texture_name);
            let texture = handle
                .load_texture(thread, file_name.as_str())
                .expect(error.as_str());

            texture_map.insert(texture_name, texture);
        }
    }

    fn reload_map(&mut self) {
        self.player.set_pos(self.map.spawn_point);
    }

    fn reset(&mut self) {
        *self.player = self.player_initial;
    }
}
