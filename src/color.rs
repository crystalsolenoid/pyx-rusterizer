use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Palette {
    pub colors: [u32; 32],
}

/// TODO: don't clone/copy this
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Material {
    pub shades: [u8; 9],
}

pub type NamedMaterials = HashMap<String, Material>;
pub type Materials = Vec<Material>;

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
pub fn lit_color(value: f32, base_color: Material) -> Color {
    let scaled = 2.0f32.powf(3.0 * value.clamp(0., 1.));
    let index = scaled.floor() as usize;
    let shades = base_color.shades;
    //shades[index]
    Color::Blue1
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
