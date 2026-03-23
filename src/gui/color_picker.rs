use icecube::{
    image::Image,
    layout::{Layout, Length},
    mouse_area::MouseArea,
    palette::Color,
    tree::Node,
};
use num_traits::FromBytes;

use crate::gui::Message;

#[derive(Clone)]
pub struct PixelPicker<F>
where
    F: Fn(usize, usize, Option<usize>) -> Message + 'static,
{
    pub img_data: Vec<Color>,
    pub w: usize,
    pub h: usize,
    pub scale: usize,
    pub palette: Vec<u32>,
    pub on_press: F,
}

impl<F> PixelPicker<F>
where
    F: Fn(usize, usize, Option<usize>) -> Message + 'static,
{
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
                (self.on_press)(px, py, idx)
            })
            .into();
        mouse_image_wrapper.push(image);
        mouse_image_wrapper
    }
}
