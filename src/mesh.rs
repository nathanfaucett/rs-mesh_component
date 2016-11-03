use alloc::boxed::Box;

use shared::Shared;
use scene_graph::{Entity, Component, ComponentManager, Id};
use geometry::Geometry;
use material::Material;

use mesh_manager::MeshManager;


struct MeshData {
    entity: Option<Entity>,
    geometry: Geometry,
    material: Material,
}


#[derive(Clone)]
pub struct Mesh {
    data: Shared<MeshData>,
}

impl Mesh {
    pub fn new(geometry: Geometry, material: Material) -> Self {
        Mesh {
            data: Shared::new(MeshData {
                entity: None,
                geometry: geometry,
                material: material,
            })
        }
    }

    pub fn get_geometry(&self) -> &Geometry {&self.data.geometry}
    pub fn get_geometry_mut(&mut self) -> &mut Geometry {&mut self.data.geometry}

    pub fn get_material(&self) -> &Material {&self.data.material}
    pub fn get_material_mut(&mut self) -> &mut Material {&mut self.data.material}
}

impl Component for Mesh {
    fn get_id(&self) -> Id {
        Id::of::<Mesh>()
    }
    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(MeshManager::new())
    }
    fn get_component_manager_id(&self) -> Id {
        Id::of::<MeshManager>()
    }
    fn get_entity(&self) -> Option<Entity> {
        self.data.entity.clone()
    }
    fn set_entity(&mut self, entity: Option<Entity>) {
        self.data.entity = entity;
    }
}

impl PartialEq<Mesh> for Mesh {
    fn eq(&self, other: &Mesh) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &Mesh) -> bool {
        !self.eq(other)
    }
}
