use icecube::{
    image::Image,
    layout::{Layout, Length},
    mouse_area::MouseArea,
    palette::Color,
    tree::Node,
};

#[derive(Clone)]
pub struct ColorPicker {
    pub img_data: Vec<Color>,
    pub w: usize,
    pub h: usize,
    pub scale: usize,
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
                super::Message::SelectColor(self.img_data[px + py * self.w])
            })
            .into();
        mouse_image_wrapper.push(image);
        mouse_image_wrapper
    }
}
