use nalgebra::Vector3;
use super::util::master_graphics_list::MasterGraphicsList;

pub struct Camera {
    position: Vector3<f32>,
    tracking_target: Option<String>,
    smoothing_factor: f32, // Owned smoothing factor
}

impl Camera {
    // Constructor to initialize Camera with a smoothing factor
    pub fn new(smoothing_factor: f32) -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 1.0),
            tracking_target: None,
            smoothing_factor,
        }
    }

    pub fn update_position(&mut self, graphics_list: &MasterGraphicsList) {
        if let Some(ref tracking_target) = self.tracking_target {
            if let Some(target) = graphics_list.get_object(tracking_target) {
                let target_position = target.read().unwrap().get_position();
                self.position.x += (target_position.x - self.position.x) * self.smoothing_factor;
                self.position.y += (target_position.y - self.position.y) * self.smoothing_factor;
                return;
            }
        }
        // If no tracking target, stay at the default position (0,0)
    }

    pub fn reset_position(&mut self) {
        self.position = Vector3::new(0.0, 0.0, 0.0);
    }
    
    pub fn set_tracking_target(&mut self, tracking_target: Option<String>) {
        self.tracking_target = tracking_target;
    }

    pub fn set_smoothing_factor(&mut self, smoothing_factor: f32) {
        self.smoothing_factor = smoothing_factor;
    }

    pub fn get_position(&self) -> Vector3<f32>{
        return self.position;
    }

    // Zoom Functions (Using Z as Zoom)
    pub fn set_zoom(&mut self, zoom: f32) {
        self.position.z = zoom.clamp(0.1,5.0);
    }

    pub fn get_zoom(&self) -> f32 {
        self.position.z
    }
}
