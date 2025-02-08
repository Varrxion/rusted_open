use crate::framework::graphics::util::master_graphics_list::MasterGraphicsList;

#[derive(Debug, PartialEq)]
pub struct CollisionEvent {
    pub object_name_1: String,
    pub object_name_2: String,
}

pub fn check_collisions(master_graphics_list: &MasterGraphicsList, object_name: &str) -> Vec<CollisionEvent> {
    let mut collision_events = Vec::new(); // Vector to hold collision events

    if let Some(object_1) = master_graphics_list.get_object(object_name) {
        let object_1_read = object_1.read().unwrap(); // Access the object through RwLock

        // Iterate over all objects in the MasterGraphicsList
        let all_objects = master_graphics_list.get_objects(); // Get the read-only reference
        for (name, object_2) in all_objects.read().unwrap().iter() {
            // Skip the object being checked against itself
            if name == &object_name {
                continue;
            }

            let object_2_read = object_2.read().unwrap(); // Lock for reading

            // Check for collision
            if object_1_read.is_colliding(&object_2_read) {
                // Create a CollisionEvent and push it into the vector
                collision_events.push(CollisionEvent {
                    object_name_1: object_name.to_string(),
                    object_name_2: name.clone(),
                });
            }
        }
    } else {
        println!("No object found with name: {}", object_name);
    }

    collision_events // Return the vector of collision events
}
