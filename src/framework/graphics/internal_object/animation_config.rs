use std::ops::Range;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct AnimationConfig {
    pub looping: bool,
    pub mode: String,
    pub frame_range: Range<usize>,
    pub frame_duration: f32,
}