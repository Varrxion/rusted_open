use std::{collections::HashMap, sync::{Arc, RwLock}};
use nalgebra::Matrix4;

use crate::framework::graphics::internal_object::graphics_object::Generic2DGraphicsObject;

pub struct MasterGraphicsList {
    objects: Arc<RwLock<HashMap<String, Arc<RwLock<Generic2DGraphicsObject>>>>>, // Change key type to String
}

impl MasterGraphicsList {
    /// Initialize a new MasterGraphicsList
    pub fn new() -> Self {
        MasterGraphicsList {
            objects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add an object to the list using its name as the key
    pub fn add_object(&self, obj: Arc<RwLock<Generic2DGraphicsObject>>) {
        let binding = obj.read().unwrap();
        let name = binding.get_name();
        let mut objects = self.objects.write().unwrap();
        objects.insert(name.to_owned(), obj.clone());
    }

    /// Get an object by name
    pub fn get_object(&self, name: &str) -> Option<Arc<RwLock<Generic2DGraphicsObject>>> {
        let objects = self.objects.read().unwrap();
        objects.get(name).cloned()
    }

    /// Returns a pointer to the entire object list
    pub fn get_objects(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Generic2DGraphicsObject>>>>> {
        Arc::clone(&self.objects) // Return a clone of the Arc to allow shared access
    }

    /// Draw all objects in the list
    pub fn draw_all(&self, projection_matrix: &Matrix4<f32>) {
        let objects = self.objects.read().unwrap(); // Lock for reading the list
        for obj in objects.values() {
            if let Ok(mut obj) = obj.write() { // Lock each object for writing (to update model matrix)
                obj.update_model_matrix(); // Update the model matrix first
                obj.apply_transform(projection_matrix); // Apply the projection matrix
                obj.draw(); // Now draw the object
            }
        }
    }

    /// If we want to print ALL info for ALL objects
    pub fn debug_all(&self) {
        let objects = self.objects.read().unwrap(); // Lock for reading the list
        for obj in objects.values() {
            if let Ok(obj) = obj.read() { // Lock each object for writing (to update model matrix)
                obj.print_debug();
            }
        }
    }
    
    /// Remove an object by name
    pub fn remove_object(&self, name: &str) {
        let mut objects = self.objects.write().unwrap();
        objects.remove(name);
    }

    /// Remove all objects from the list
    pub fn remove_all(&self) {
        let mut objects = self.objects.write().unwrap();
        objects.clear();
    }
}
