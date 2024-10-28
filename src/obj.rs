use std::{fs::read_to_string, io::Error, path::Path};

use glam::Vec3;

use crate::{
    color::Color,
    geo::{IndexedTriangle, Mesh},
};

#[derive(Debug)]
#[allow(dead_code)]
enum Line {
    ObjectName(String),
    Vertex(f32, f32, f32),
    Face(Vec<usize>, Color),
    UseMtl(String, Color), // Color might be removed later
}
pub fn parse(path: &Path) -> Result<Mesh, Error> {
    let obj_string = read_to_string(path)?;
    let mut current_material = Color::Red;
    let data: Vec<_> = obj_string
        .lines()
        .filter_map(|line| {
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("o") => Some(
                    tokens
                        .next()
                        .ok_or(Error::other("Missing object name"))
                        .map(|name| Line::ObjectName(name.to_string())),
                ),
                Some("v") => Some(
                    tokens
                        .map(|s| {
                            s.parse().map_err(|_| {
                                Error::other(std::format!("Expected Float; found {s}"))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .and_then(|vs| match vs.len() {
                            3 => Ok(Line::Vertex(vs[0], vs[1], vs[2])),
                            _ => Err(Error::other(std::format!(
                                "Expected 3 Floats; found {vs:?}"
                            ))),
                        }),
                ),
                Some("f") => Some(
                    tokens
                        .map(|s| {
                            s.split("/")
                                .next()
                                .ok_or(Error::other("Missing vertex index"))
                                .and_then(|vertex_index| {
                                    vertex_index.parse::<usize>().map_err(|_| {
                                        Error::other(std::format!(
                                            "Expected usize index; found {vertex_index}"
                                        ))
                                    })
                                })
                                .and_then(|vertex_index| {
                                    vertex_index.checked_sub(1).ok_or(Error::other(
                                        "Expected non-zero vertex index; obj indices start at 1!",
                                    ))
                                })
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .and_then(|fs| match fs.len() {
                            3.. => Ok(Line::Face(fs, current_material)),
                            _ => Err(Error::other(std::format!(
                                "Expected 3 or more vertex indices, found {fs:?}"
                            ))),
                        }),
                ),
                Some("g") => None,
                Some("vn") => None,
                Some("vt") => None,
                Some("#") => None,
                Some("usemtl") => Some(
                    tokens
                        .next()
                        .ok_or(Error::other("Missing object name"))
                        .map(|name| {
                            current_material = match name {
                                "mat4" => Color::Cyan2,
                                "mat8" => Color::Red,
                                "mat21" => Color::White,
                                "mat23" => Color::Black,
                                _ => Color::Pink0,
                            };
                            Line::UseMtl(name.to_string(), current_material)
                        }),
                ),
                Some("mtllib") => None, // Not sure what this is!
                None => None,
                _ => todo!("{:?}", tokens),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let vertices = data
        .iter()
        .filter_map(|line| match line {
            Line::Vertex(v1, v2, v3) => Some(Vec3::new(*v1, *v2, *v3)),
            _ => None,
        })
        .collect();

    let triangles = data
        .iter()
        .filter_map(|line| match line {
            Line::Face(fs, color) => match fs.len() {
                0..=2 => panic!(),
                3 => Some(vec![IndexedTriangle {
                    index: (fs[0], fs[1], fs[2]),
                    color: *color,
                }]),
                4 => Some(vec![
                    IndexedTriangle {
                        index: (fs[0], fs[1], fs[2]),
                        color: *color,
                    },
                    IndexedTriangle {
                        index: (fs[2], fs[3], fs[0]),
                        color: *color,
                    },
                ]),
                _ => Some(
                    fs[1..]
                        .windows(2)
                        .map(|window_f| IndexedTriangle {
                            index: (fs[0], window_f[0], window_f[1]),
                            color: *color,
                        })
                        .collect::<Vec<_>>(),
                ),
            },
            _ => None,
        })
        .flatten()
        .collect();

    Ok(Mesh {
        triangles,
        vertices,
    })
}
