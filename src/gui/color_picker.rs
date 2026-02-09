use icecube::{
    image::Image,
    layout::{Layout, Length},
    mouse_area::MouseArea,
    palette::Color,
    tree::Node,
};
use num_traits::FromBytes;

use crate::{color::Palette, constants::COLOR_DEPTH};

#[derive(Clone)]
pub struct ColorPicker {
    pub img_data: Vec<Color>,
    pub w: usize,
    pub h: usize,
    pub scale: usize,
    pub palette: Vec<u32>,
}

impl ColorPicker {
    pub fn view<'a>(self) -> Node<'a, super::Message, Layout> {
        let image =
            Node::new(Image::new(self.img_data.clone(), self.w, self.h).scale_factor(self.scale))
                .height(Length::Shrink)
                .width(Length::Shrink);

        let mut mouse_image_wrapper: Node<'a, super::Message, _> = MouseArea::new()
            .on_press(move |pos| {
                let (px, py) = (pos.0 / self.scale, pos.1 / self.scale);
                let icecube_color: Color = self.img_data[px + py * self.w];
                let idx: Option<usize> = self.palette.iter().position(|&x| {
                    let query: u32 = FromBytes::from_be_bytes(&icecube_color);
                    x == query
                });
                super::Message::SelectColor(idx.unwrap_or_default() as u8)
            })
            .into();
        mouse_image_wrapper.push(image);
        mouse_image_wrapper
    }
}
