use std::{error::Error, process::Command};

use log::{error, info};
use websockets::{Frame, WebSocket};

mod tosu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Running");
    let mut ws = WebSocket::connect("ws://127.0.0.1:24050/websocket/v2").await?;
    info!("Connected");

    ctrlc::set_handler(|| {
        let _ = Command::new("hyprshade").arg("off").spawn();
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
            && let Ok(value) = serde_json::from_str::<'_, tosu::TosuApiResponse>(&payload)
        {
            if last_state.is_none_or(|state| state != value.state.number) {
                let state = value.state.number;
                last_state = Some(state);
                info!(
                    "[STATE CHANGE] num={} name={}",
                    value.state.number, value.state.name
                );

                if state == 2
                    && let Some(ar) = last_ar
                {
                    if let Err(e) = Command::new("hyprshade")
                        .args([
                            "on",
                            "/home/brad/.local/share/hyprshade/shaders/osu.glsl.mustache",
                            "--var",
                            format!("value={ar}").as_str(),
                        ])
                        .spawn()
                    {
                        error!("{e:?}");
                    }
                } else if let Err(e) = Command::new("hyprshade").arg("off").spawn() {
                    error!("{e:?}");
                }
            } else if last_ar.is_none_or(|ar| ar != value.beatmap.stats.ar.converted)
                && value.state.number == 5
            {
                last_ar = Some(value.beatmap.stats.ar.converted);
                info!("[AR CHANGE] ar={}", value.beatmap.stats.ar.converted);
            }
            // info!("{{TEXT}} payload=\"{val}\" continuation={continuation} fin={fin}")
        }
    }

    let _ = Command::new("hyprshade").arg("off").spawn();

    Ok(())
}
