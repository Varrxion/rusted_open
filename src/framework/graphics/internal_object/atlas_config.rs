use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct AtlasConfig {
    pub current_frame: usize,
    pub atlas_columns: usize,
    pub atlas_rows: usize,
    pub columns_wide: usize,
    pub rows_tall: usize,
}