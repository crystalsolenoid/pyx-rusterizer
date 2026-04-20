use assets_manager::Asset;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{buffer::Buffer, constants::CLEAR_COLOR, poly::Tri};

#[derive(Clone, Deserialize, Debug, Asset)]
#[asset_format = "toml"]
pub struct Palette {
    pub colors: [u32; 32],
}

/// TODO: don't clone/copy this
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Material {
    pub shades: [u8; 9],
}

#[derive(Deserialize, Debug, Clone, Asset)]
#[asset_format = "toml"]
#[serde(transparent)]
pub struct NamedMaterials(pub HashMap<String, Material>);

#[derive(Debug)]
pub struct Materials(pub Vec<Material>);

impl From<NamedMaterials> for Materials {
    fn from(value: NamedMaterials) -> Self {
        let mut unsorted_materials = value.0.clone().into_iter().collect::<Vec<_>>();
        // Using a manual sort_by_key here, to satisfy lifetime weirdness
        unsorted_materials.sort_by(|x, y| x.0.cmp(&(y.0)));
        Materials(unsorted_materials.into_iter().map(|(_k, v)| v).collect())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Blue0,
    Blue1,
    Blue2,
    Blue3,
    Blue4,
    Blue5,
    Blue6,
    White,
    Purple,
    Pink0,
    Pink1,
    Red,
    Coral,
    Orange,
    Brown0,
    Brown1,
    Brown2,
    Dijon,
    Avacado,
    Yellow,
    Green0,
    Green1,
    Green2,
    Aqua0,
    Aqua1,
    Aqua2,
    Cyan0,
    Cyan1,
    Cyan2,
    Pink2,
    Pink3,
}

/// Convert a float between 0.0 and 1.0 to a grayscale indexed color.
/// Overflowing values are white
pub fn grayscale(value: f32) -> Color {
    let scaled = 2.0f32.powf(3.0 * value.clamp(0., 1.));
    match (scaled).floor() as u8 {
        0 => Color::Black,
        1 => Color::Blue0,
        2 => Color::Blue1,
        3 => Color::Blue2,
        4 => Color::Blue3,
        5 => Color::Blue4,
        6 => Color::Blue5,
        7 => Color::Blue6,
        _ => Color::White,
    }
}

/// Convert a float between 0.0 and 1.0 and a color to a lit color
pub fn lit_color(value: f32, base_color: Material) -> u8 {
    let scaled = 2.0f32.powf(3.0 * value.clamp(0., 1.));
    let index = scaled.floor() as usize;
    let shades = base_color.shades;
    shades[index]
}

pub fn flat_lit_shader(_x: usize, _y: usize, material: Option<&Material>, light: f32) -> u8 {
    material
        .map(|m| lit_color(light, *m))
        .unwrap_or(CLEAR_COLOR)
}

fn color_index(v: f32, num_colors: usize) -> usize {
    ((v * num_colors as f32).floor() as usize).clamp(0, num_colors - 1)
}

fn color_rounded_index(v: f32, num_colors: usize) -> usize {
    // in index units, the transition point closest to v
    (v * num_colors as f32).round() as usize
}

fn closest_transition(v: f32, num_colors: usize) -> f32 {
    // in light units, the transition point closest to v
    color_rounded_index(v, num_colors) as f32 - 1. / num_colors as f32
}

fn transition_distance(v: f32, num_colors: usize) -> f32 {
    (v - closest_transition(v, num_colors)).abs()
}

const DITHER_RATIO: f32 = 0.5;
pub fn dither_mask_shader(x: usize, y: usize, material: Option<&Material>, light: f32) -> u8 {
    material
        .map(|m| {
            // TODO which light scaling should we do?
            // let scaled = 2.0f32.powf(3.0 * light.clamp(0., 1.)) / 9.;
            let num_shades = m.shades.len();
            let num_shades = 2;
            let scaled = light.clamp(0., 1.);
            let c2 = color_rounded_index(closest_transition(scaled, num_shades), num_shades);
            let c1 = (c2.saturating_sub(1)).clamp(0, num_shades - 1);
            let transition_width = 1. / num_shades as f32;
            if transition_distance(scaled, num_shades) < transition_width * DITHER_RATIO / 2. {
                if (x ^ y) % 2 == 0 {
                    m.shades[c2]
                } else {
                    m.shades[c1]
                }
            } else {
                m.shades[color_index(scaled, num_shades)]
            }
        })
        .unwrap_or(CLEAR_COLOR)
}

pub fn lit_color_old(value: f32, base_color: Color) -> Color {
    let scaled = 2.0f32.powf(3.0 * value.clamp(0., 1.));
    match base_color {
        Color::Red => match (scaled).floor() as u8 {
            0 => Color::Brown0,
            1 => Color::Purple,
            2 => Color::Pink0,
            3..=6 => Color::Red,
            7 => Color::Coral,
            _ => Color::Orange,
        },
        Color::White => match (scaled).floor() as u8 {
            0 => Color::Blue3,
            1 => Color::Blue4,
            2 => Color::Blue5,
            3..6 => Color::Blue6,
            _ => Color::White,
        },
        Color::Black => match (scaled).floor() as u8 {
            0..=3 => Color::Black,
            4..=6 => Color::Blue0,
            _ => Color::Blue1,
        },
        Color::Cyan2 => match (scaled).floor() as u8 {
            0..=1 => Color::Blue2,
            2..=3 => Color::Cyan1,
            4..=6 => Color::Cyan2,
            _ => Color::White,
        },
        _ => match (scaled).floor() as u8 {
            0 => Color::Black,
            1 => Color::Blue0,
            2 => Color::Blue1,
            3 => Color::Blue2,
            4 => Color::Blue3,
            5 => Color::Blue4,
            6 => Color::Blue5,
            7 => Color::Blue6,
            _ => Color::White,
        },
    }
}
