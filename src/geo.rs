use glam::{Affine3A, Vec3};

use crate::{
    buffer::Buffer,
    poly::{self, Tri},
};

pub trait Shape {
    fn render(&self, buffer: &mut Buffer, transform: Affine3A);
}

type Vertex = Vec3;

pub struct Triangle {
    /// indices that correspond to vertiecs in Mesh
    pub index: (usize, usize, usize),
    pub color: u8,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Shape for Mesh {
    fn render(&self, buffer: &mut Buffer, transform: Affine3A) {
        let transformed_verts: Vec<Vertex> = (&self.vertices)
            .into_iter()
            .map(|v| transform.transform_point3(*v))
            .collect();

        for triangle in &self.triangles {
            let (t1, t2, t3) = triangle.index;
            let vert_tri = Tri {
                v1: transformed_verts[t1],
                v2: transformed_verts[t2],
                v3: transformed_verts[t3],
            };
            poly::draw_tri(buffer, &vert_tri, triangle.color);
        }
    }
}

pub struct Geo {
    pub shape: Box<dyn Shape>,
    pub transform: Affine3A,
    _children: Vec<Geo>,
}

impl Geo {
    pub fn new(shape: Box<dyn Shape>, transform: Affine3A) -> Geo {
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
