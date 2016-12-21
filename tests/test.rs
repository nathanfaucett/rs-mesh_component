#![no_std]

extern crate mesh_component;
extern crate scene_graph;

extern crate geometry;
extern crate material;


use mesh_component::Mesh;
use scene_graph::{Scene, Entity};

use geometry::Geometry;
use material::Material;

#[test]
fn test_scene() {
    let mut scene = Scene::new();
    let mut entity = Entity::new();
    let mesh = Mesh::new(Geometry::new(), Material::new());

    entity.add_component(mesh);
    scene.add_entity(&mut entity);
}
