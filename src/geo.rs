use glam::{Affine3A, Vec3};

use crate::{
    buffer::Buffer,
    color::Materials,
    poly::{self, Tri},
};

pub trait Shape {
    fn render(&self, buffer: &mut Buffer, transform: Affine3A);
}

type Vertex = Vec3;

#[derive(Debug)]
pub struct IndexedTriangle {
    /// indices that correspond to vertiecs in Mesh
    pub index: (usize, usize, usize),
    pub material_index: usize,
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<IndexedTriangle>,
    pub materials: Materials,
}

impl Shape for Mesh {
    fn render(&self, buffer: &mut Buffer, transform: Affine3A) {
        let transformed_verts: Vec<Vertex> = (&self.vertices)
            .into_iter()
            .map(|v| transform.transform_point3(*v))
            .collect();

        for (i, triangle) in self.triangles.iter().enumerate() {
            let (t1, t2, t3) = triangle.index;
            let vert_tri = Tri::new(
                transformed_verts[t1],
                transformed_verts[t2],
                transformed_verts[t3],
                self.materials.0[triangle.material_index],
                i,
            );
            poly::draw_tri(buffer, &vert_tri);
        }
    }
}

pub struct Geo {
    pub shape: Mesh, //Box<dyn Shape>,
    pub transform: Affine3A,
    _children: Vec<Geo>,
}

impl Geo {
    pub fn new(shape: Mesh, transform: Affine3A) -> Geo {
        Geo {
            shape,
            transform,
            _children: Vec::new(),
        }
    }
    pub fn render(&self, buffer: &mut Buffer) {
        self.shape.render(buffer, self.transform);
    }
}
