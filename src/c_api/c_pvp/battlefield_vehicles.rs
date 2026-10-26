//! Supplied battlefield map observations; no native vehicle catalog is seeded.

/// Fields declared by Mists PvpInfoDocumentation's BattlefieldVehicleInfo.
#[derive(Debug, Clone)]
pub struct BattlefieldVehicle {
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub is_occupied: bool,
    pub atlas: String,
    pub texture_width: f64,
    pub texture_height: f64,
    pub facing: f64,
    pub is_player: bool,
    pub is_alive: bool,
    pub should_draw_below_player_blips: bool,
}
