use std::path::Path;

use assets_manager::Handle;
use glam::{Affine3A, Vec3};

use crate::{buffer::Buffer, color::NamedMaterials, geo::Geo, obj};

pub struct Model {
    pub cube: Geo,
}

impl Model {
    pub fn new(material_handle: &Handle<NamedMaterials>) -> Model {
        let named_materials: NamedMaterials = NamedMaterials(material_handle.read().0.clone());

        // let mesh = obj::parse(Path::new("assets/porygon/model.obj"), named_materials).unwrap();
        let mesh = obj::parse(Path::new("assets/sphere.obj"), named_materials).unwrap();
        Model {
            cube: Geo::new(mesh, Affine3A::IDENTITY),
        }
    }
}

/// called every frame
pub fn draw(buffer: &mut Buffer, model: &Model) {
    buffer.clear_screen();

    model.cube.deferred_render(buffer);
    buffer.finalize_render(&model.cube.shape.materials.0, &model.cube.shape.triangles);
}
