use std::{fs::read_to_string, path::Path};

use glam::Vec3;

use crate::{
    color::Color,
    geo::{Mesh, Triangle},
};

#[derive(Debug)]
#[allow(dead_code)]
enum Line {
    ObjectName(String),
    Vertex(f32, f32, f32),
    Face(Vec<usize>, Color),
}

pub fn parse(path: &Path) -> Mesh {
    let obj_string = read_to_string(path).unwrap();
    let mut current_material = Color::Red;
    let data: Vec<_> = obj_string
        .lines()
        .filter_map(|line| {
            let mut tokens = line.split_whitespace();
            //let head = tokens.next();
            //let tail = tokens.collect();
            match tokens.next() {
                Some("o") => {
                    let name = tokens.next().unwrap().to_string();
                    Some(Line::ObjectName(name))
                }
                Some("v") => {
                    let vs: Vec<f32> = tokens.map(|s| s.parse().unwrap()).collect();
                    assert!(vs.len() == 3);
                    Some(Line::Vertex(vs[0], vs[1], vs[2]))
                }
                Some("f") => {
                    let fs: Vec<usize> = tokens
                        .map(|s| s.split("/").next().unwrap().parse::<usize>().unwrap() - 1)
                        .collect();
                    assert!(fs.len() > 2);
                    Some(Line::Face(fs, current_material))
                }
                Some("g") => None,
                Some("vn") => None,
                Some("vt") => None,
                Some("#") => None,
                Some("usemtl") => {
                    let name = tokens.next().unwrap().to_string();
                    current_material = match name.as_str() {
                        "mat4" => Color::Cyan2,
                        "mat8" => Color::Red,
                        "mat21" => Color::White,
                        "mat23" => Color::Black,
                        _ => Color::Pink0,
                    };
                    None
                }
                Some("mtllib") => None, // Not sure what this is!
                None => None,
                _ => todo!("{:?}", tokens),
            }
        })
        .collect();

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
                3 => Some(vec![Triangle {
                    index: (fs[0], fs[1], fs[2]),
                    color: *color,
                }]),
                4 => Some(vec![
                    Triangle {
                        index: (fs[0], fs[1], fs[2]),
                        color: *color,
                    },
                    Triangle {
                        index: (fs[2], fs[3], fs[0]),
                        color: *color,
                    },
                ]),
                _ => Some(
                    fs[1..]
                        .windows(2)
                        .map(|window_f| Triangle {
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

    Mesh {
        triangles,
        vertices,
    }
}
