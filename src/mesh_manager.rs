use alloc::boxed::Box;

use vector::Vector;
use collection::Collection;
use stack::Stack;
use remove::Remove;
use shared::Shared;
use scene_graph::{Scene, Component, ComponentManager, Id};

use mesh::Mesh;


struct MeshManagerData {
    scene: Option<Scene>,
    components: Vector<Mesh>,
}


#[derive(Clone)]
pub struct MeshManager {
    data: Shared<MeshManagerData>,
}

impl MeshManager {
    pub fn new() -> MeshManager {
        MeshManager {
            data: Shared::new(MeshManagerData {
                scene: None,
                components: Vector::new(),
            })
        }
    }

    pub fn for_each<F>(&mut self, func: F) where F: Fn(&mut Mesh) {
        for mesh in self.data.components.iter_mut() {
            func(mesh);
        }
    }
}

impl ComponentManager for MeshManager {

    fn get_id(&self) -> Id { Id::of::<MeshManager>() }

    fn get_scene(&self) -> Option<Scene> {
        match self.data.scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&mut self, scene: Option<Scene>) {
        self.data.scene = scene;
    }

    fn get_order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        self.data.components.is_empty()
    }

    fn clear(&mut self) {}
    fn init(&mut self) {}
    fn update(&mut self) {}

    fn add_component(&mut self, component: &mut Box<Component>) {
        let ref mut component = component.downcast_mut::<Mesh>().unwrap();
        self.data.components.push(component.clone());
    }
    fn remove_component(&mut self, component: &mut Box<Component>) {
        let component = component.downcast_ref::<Mesh>().unwrap();

        match self.data.components.iter().position(|c| c == component) {
            Some(i) => {
                self.data.components.remove(&i);
            },
            None => (),
        }
    }
}
