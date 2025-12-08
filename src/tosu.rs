use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TosuState {
    pub number: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TosuBeatmapStatsAr {
    pub converted: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TosuBeatmapStats {
    pub ar: TosuBeatmapStatsAr,
}

#[derive(Serialize, Deserialize)]
pub struct TosuBeatmap {
    pub stats: TosuBeatmapStats,
}

#[derive(Serialize, Deserialize)]
pub struct TosuApiResponse {
    pub state: TosuState,
    pub beatmap: TosuBeatmap,
}
