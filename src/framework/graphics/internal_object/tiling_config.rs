use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct TilingConfig {
    pub horizontal_scalar: f32,
    pub vertical_scalar: f32,
}