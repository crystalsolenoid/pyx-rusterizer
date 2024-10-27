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
    match (value * 8.).floor() as u8 {
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
