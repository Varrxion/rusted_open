use nalgebra::Vector3;
use crate::framework::graphics::internal_object::graphics_object::Generic2DGraphicsObject;

pub fn move_object(object: &mut Generic2DGraphicsObject, direction: Vector3<f32>, delta_time: f32) {
    let mut pos = object.get_position();

    // Apply movement in the given direction
    pos += direction * delta_time;

    // Update the position and model matrix
    object.set_position(pos);
}

// Rotate the object by a given angle (in radians).
pub fn rotate_object(object: &mut Generic2DGraphicsObject, angle: f32) {
    // Get the current rotation (in radians), assuming you have a method to retrieve it
    let mut current_rotation = object.get_rotation(); // This should return the current rotation in radians

    // Update the rotation by adding the angle
    current_rotation += angle;

    // Set the new rotation
    object.set_rotation(current_rotation); // This should update the object's rotation
}