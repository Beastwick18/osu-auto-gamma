use std::{fs::File, io::Read};

use clap::Parser;
use log::warn;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_tosu_ws")]
    pub tosu_ws: String,

    #[serde(default = "default_shader_path")]
    pub shader_path: String,
}

fn default_tosu_ws() -> String {
    String::from("ws://127.0.0.1:24050/websocket/v2")
}

fn default_shader_path() -> String {
    String::from("./osu.glsl.mustache")
}

#[derive(Deserialize, Parser, Debug)]
pub struct Args {
    #[arg(short = 't', long)]
    /// Defaults to "ws://127.0.0.1:24050/websocket/v2"
    pub tosu_ws: Option<String>,

    #[arg(short = 's', long)]
    /// Defaults to "./osu.glsl.mustache"
    pub shader_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tosu_ws: default_tosu_ws(),
            shader_path: default_shader_path(),
        }
    }
}

impl Config {
    pub fn from_config_file() -> Self {
        Self::try_from_config_file().unwrap_or_default()
    }

    fn try_from_config_file() -> Option<Self> {
        let project = directories::ProjectDirs::from("rs", "", "osu-auto-gamma")?;
        let config_file = project.config_dir().join("config.toml");
        match File::open(config_file) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).ok()?;
                let config = toml::from_str(&contents).ok()?;
                Some(config)
            }
            Err(_) => {
                warn!("No config found");
                None
            }
        }
    }

    pub fn overwrite_with_cli(self) -> Self {
        let args = Args::parse();
        Config {
            tosu_ws: args.tosu_ws.unwrap_or(self.tosu_ws),
            shader_path: args.shader_path.unwrap_or(self.shader_path),
        }
    }
}
