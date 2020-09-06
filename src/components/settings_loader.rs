use ptgui::prelude::Dimensions;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, remove_file, File};
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct GameSettings {
    pub resolution: Dimensions,
    pub is_fullscreen: bool,
    pub show_fps: bool,
}

pub fn load_settings() -> std::result::Result<GameSettings, String> {
    match Path::new("settings/settings.ron").exists() {
        true => {
            let mut file = File::open("settings/settings.ron").unwrap();
            let mut buffer = vec![];
            let _ = file.read_to_end(&mut buffer).unwrap();
            let buffer_string = String::from_utf8(buffer).unwrap();

            let settings: GameSettings = ron::from_str(buffer_string.as_str()).unwrap();

            Ok(settings)
        }
        false => Err("Could not read settings file".to_string()),
    }
}
pub fn read_settings(settings: &mut GameSettings) -> std::result::Result<(), String> {
    match Path::new("settings/settings.ron").exists() {
        true => {
            let mut file = File::open("settings/settings.ron").unwrap();

            let mut buffer = vec![];
            let _ = file.read_to_end(&mut buffer).unwrap();
            let buffer_string = String::from_utf8(buffer).unwrap();
            *settings = ron::from_str(buffer_string.as_str()).unwrap();

            Ok(())
        }
        false => Err("Could not read settings file".to_string()),
    }
}

pub fn update_settings(settings: &GameSettings) -> std::result::Result<(), String> {
    match Path::new("settings/settings.ron").exists() {
        true => {
            let mut file = File::create("settings/settings.ron").unwrap();
            let data: String = ron::to_string(settings).unwrap();
            file.write_all(data.as_bytes()).unwrap();

            Ok(())
        }
        false => Err("Could not read settings file".to_string()),
    }
}

pub fn check_settings() -> bool {
    match Path::new("settings/settings.ron").exists() {
        true => {
            let mut file = File::open("settings/settings.ron").unwrap();
            let mut buffer = vec![];
            let _ = file.read_to_end(&mut buffer).unwrap();
            let buffer_string = String::from_utf8(buffer).unwrap();

            let _ = match ron::from_str::<GameSettings>(buffer_string.as_str()) {
                Ok(_) => return true,
                Err(_) => {
                    remove_file("settings/settings.ron").unwrap();
                    return false;
                }
            };
        }
        false => return false,
    }
}

pub fn create_settings() {
    match Path::new("settings/settings.ron").exists() {
        true => {}
        false => {
            if !Path::new("settings").exists() {
                create_dir_all("settings").unwrap();
            }

            let mut file = File::create("settings/settings.ron").unwrap();
            let data: String = ron::to_string(&GameSettings {
                resolution: (1280, 720),
                is_fullscreen: false,
                show_fps: false,
            })
            .unwrap();
            file.write_all(data.as_bytes()).unwrap();
        }
    }
}
