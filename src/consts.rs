use std::error::Error;
use std::io::BufReader;
use bevy::app::{App, Plugin};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Resource, Serialize, Deserialize)]
pub struct Consts {
    pub window_height: f32,
    pub window_width: f32,
    pub window_name: String,

    pub pebble_width: f32,
    pub pebble_height: f32,
    pub pebble_start_y_range: std::ops::Range<f32>,
    pub pebble_default_velocity: f32,
    pub g_force_acceleration: f32,

    pub moai_width: f32,
    pub moai_height: f32,
    pub moai_horizontal_distance: f32,
    pub moai_vertical_distance: f32,
    pub moai_height_range: std::ops::Range<f32>,
    pub moai_move_speed: f32,
}

pub struct ConstsPlugin;

impl Plugin for ConstsPlugin {
    fn build(&self, app: &mut App) {
        let consts = ConstsPlugin::load().unwrap_or_else(|err| {
            eprintln!("Error reading consts: {}", err);
            let consts = Consts::default();
            if let Err(error) = ConstsPlugin::save(&consts) {
                eprintln!("Error saving consts: {}", error);
            } else {
                println!("Save success");
            }
            consts
        });

        app.insert_resource(consts);
    }
}

const PATH: &str = "assets/consts.ron";

impl ConstsPlugin {
    fn load() -> Result<Consts, Box<dyn Error>> {
        let file = std::fs::File::open(PATH)?;
        let reader = BufReader::new(file);
        let consts = ron::de::from_reader(reader)?;
        Ok(consts)
    }

    fn save(consts: &Consts) -> Result<(), Box<dyn Error>> {
        let consts_ron = ron::ser::to_string_pretty(consts, ron::ser::PrettyConfig::default())?;
        std::fs::write(PATH, consts_ron.as_bytes())?;
        Ok(())
    }
}