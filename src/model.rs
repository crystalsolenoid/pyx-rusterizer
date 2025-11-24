use std::path::Path;

use assets_manager::Handle;
use glam::Affine3A;

use crate::{color::NamedMaterials, geo::Geo, obj};

pub struct Model {
    pub cube: Geo,
}

impl Model {
    pub fn new(material_handle: &Handle<NamedMaterials>) -> Model {
        let named_materials: NamedMaterials = NamedMaterials(material_handle.read().0.clone());

        let mesh = obj::parse(Path::new("assets/porygon/model.obj"), named_materials).unwrap();
        Model {
            cube: Geo::new(mesh, Affine3A::IDENTITY),
        }
    }
}
