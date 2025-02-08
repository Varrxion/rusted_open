use std::sync::{Arc, RwLock};
use nalgebra::Vector3;
use crate::framework::graphics::internal_object::graphics_object::Generic2DGraphicsObject;

pub fn move_object(object: Arc<RwLock<Generic2DGraphicsObject>>, direction: Vector3<f32>, speed: f32, delta_time: f32) {
    let mut object = object.write().unwrap();
    let mut pos = object.get_position();

    // Apply movement in the given direction
    pos += direction * speed * delta_time;

    // Update the position and model matrix
    object.set_position(pos);
    object.update_model_matrix();
}

// Rotate the object by a given angle (in radians).
pub fn rotate_object(object: Arc<RwLock<Generic2DGraphicsObject>>, angle: f32) {
    let mut object = object.write().unwrap();
    
    // Get the current rotation (in radians), assuming you have a method to retrieve it
    let mut current_rotation = object.get_rotation(); // This should return the current rotation in radians

    // Update the rotation by adding the angle
    current_rotation += angle;

    // Set the new rotation
    object.set_rotation(current_rotation); // This should update the object's rotation

    // Update the model matrix to reflect the new rotation
    object.update_model_matrix();
}