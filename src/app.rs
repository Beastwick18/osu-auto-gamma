use std::{error::Error, process::Command};

use log::{error, info};
use websockets::{Frame, WebSocket};

use crate::{config::Config, tosu};

pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: Config::from_config_file().overwrite_with_cli(),
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let config = self.config;
        info!("Attempting to connect to {}", config.tosu_ws);
        let mut ws = WebSocket::connect(&config.tosu_ws).await?;
        info!("Connected to {}", config.tosu_ws);
        info!("Press Ctrl+C to exit");

        let glsl_path = shellexpand::tilde(&config.shader_path);

        // Handle Ctrl-C so we can disable hyprshade on interrupt
        ctrlc::set_handler(|| {
            info!("Resetting gamma");
            let _ = Command::new("hyprshade").arg("off").spawn();
            info!("Exiting...");
            std::process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        let mut last_ar: Option<f64> = None;
        let mut last_state: Option<i32> = None;

        while let Ok(recv) = ws.receive().await {
            if let Frame::Text {
                payload,
                continuation: _,
                fin: _,
            } = recv
            {
                let Ok(msg) = serde_json::from_str::<'_, tosu::TosuApiResponse>(&payload) else {
                    error!("Failed to parse websocket JSON");
                    continue;
                };
                if last_state.is_none_or(|state| state != msg.state.number) {
                    let state = msg.state.number;
                    last_state = Some(state);
                    info!(
                        "[STATE CHANGE] num={} name={}",
                        msg.state.number, msg.state.name
                    );

                    if state == 2
                        && let Some(ar) = last_ar
                    {
                        info!("[PLAYING] gamma=on ar={}", msg.beatmap.stats.ar.converted);
                        if let Err(e) = Command::new("hyprshade")
                            .args(["on", &glsl_path, "--var", &format!("value={ar}")])
                            .spawn()
                        {
                            error!("{e:?}");
                        }
                    } else {
                        info!("[MENU] gamma=off");
                        if let Err(e) = Command::new("hyprshade").arg("off").spawn() {
                            error!("{e:?}");
                        }
                    }
                } else if last_ar.is_none_or(|ar| {
                    ar != msg.beatmap.stats.ar.converted
                        && !(ar == 10.0 && msg.beatmap.stats.ar.converted == 0.0)
                }) && (msg.state.number == 5)
                    || (msg.state.number == 11)
                {
                    let ar = match msg.beatmap.stats.ar.converted {
                        0.0 => 10.0,
                        ar => ar,
                    };
                    info!("[AR CHANGE] last_ar={last_ar:?} ar={ar}");
                    last_ar = Some(ar);
                }
            }
        }

        info!("Resetting gamma");
        let _ = Command::new("hyprshade").arg("off").spawn();
        info!("Exiting...");

        Ok(())
    }
}
