use icecube::mouse_area::MouseArea;
use icecube::palette::Color;
use icecube::quad::Quad;
use icecube::slider::Slider;
use icecube::text::Text;
use num_traits::ToBytes;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use icecube::button::Button;
use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::tree::Node;
use icecube::{col, font, row};

use crate::animation::{self};
use crate::buffer::Buffer;
use crate::color::Material;
use crate::constants::COLOR_DEPTH;
use crate::gui::color_picker::ColorPicker;
use crate::model::{draw, Model};

mod color_picker;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Invert,
    TimeElapsed(Duration),
    RotateX(f32),
    RotateY(f32),
    SelectColor(u8),
    ClickRender(usize, usize),
}

pub struct State {
    data: Vec<usize>,
    model: Model,
    buffer: Buffer,
    start_instant: Instant,
    x_rotation: f32,
    y_rotation: f32,
    selected_color: u8,
}

impl State {
    pub fn new(buffer: Buffer, model: Model) -> Self {
        Self {
            data: vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            model,
            buffer,
            start_instant: Instant::now(),
            x_rotation: 0.0,
            y_rotation: 0.0,
            selected_color: Default::default(),
        }
    }
}

impl State {
    fn invert(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|px| match px {
                0 => 1,
                1 => 0,
                _ => 0,
            })
            .collect();
    }
}

pub fn update(m: Message, state: &mut State) {
    match m {
        Message::Invert => state.invert(),
        Message::TimeElapsed(duration) => render(duration, state),
        Message::RotateX(radians) => state.x_rotation = radians,
        Message::RotateY(radians) => state.y_rotation = radians,
        Message::SelectColor(color) => state.selected_color = color,
        Message::ClickRender(x, y) => {
            let tri = state.buffer.tri_idx_at_pixel(x, y);
            if let Some(tri) = tri {
                let mat = state.model.cube.shape.triangles[tri].material_index;
                state.model.cube.shape.materials.0[mat] = Material {
                    shades: [state.selected_color; 9],
                };
            }
        }
    }
}

fn render(duration: Duration, state: &mut State) {
    // cache.hot_reload(); // TODO turn back on?
    // buffer.palette = palette_handle.read().colors;
    //TODO: figure out how to get Materials out of the AssetReadGuard without cloning
    // model.cube.shape.materials = NamedMaterials(material_handle.read().0.clone()).into();

    draw(&mut state.buffer, &state.model);

    // let timing = Timing {
    //     time_since_start: Instant::now() - state.start_instant,
    //     _delta: duration.as_secs_f32(),
    // };

    animation::update(state.x_rotation, state.y_rotation, &mut state.model);
}

pub fn view<'a>(state: &State) -> Node<'a, Message, Layout> {
    // TODO just store a [u8; 4] in buffer instead of u32?
    let render: Vec<[u8; 4]> = state.buffer.get_palette_rgb().clone().into_iter().collect();

    const RENDER_SCALE: usize = 2;
    let image = Node::new(
        Image::<[u8; 4]>::new(render, state.buffer.width(), state.buffer.height())
            .scale_factor(RENDER_SCALE),
    )
    .height(Length::Shrink)
    .width(Length::Shrink);

    let mut mouse_image_wrapper: Node<Message, _> = MouseArea::new()
        // .whenever_down(|pos| Message::BoardClick(pos))
        // .on_hover(|pos| Message::BoardHover(pos))
        // .on_exit(|| Message::BoardExit)
        .on_press(|pos| Message::ClickRender(pos.0 / RENDER_SCALE, pos.1 / RENDER_SCALE))
        .into();

    mouse_image_wrapper.push(image);

    let fill_color = ToBytes::to_be_bytes(&state.buffer.palette[3]);
    let border_color = ToBytes::to_be_bytes(&state.buffer.palette[20]);
    let text_color = ToBytes::to_be_bytes(&state.buffer.palette[8]);

    let rotation_label = Node::new(
        Text::new(format!(
            "Rotation: {:.0} Degrees",
            state.x_rotation * 360. / (2. * PI)
        ))
        .with_font(&font::BLACKLETTER)
        .with_color(index_to_icecube_color(
            state.selected_color,
            state.buffer.palette,
        )),
    );

    let x_rotation_slider: Node<_, _> = Slider::new(-PI..PI, state.x_rotation)
        .on_drag(Message::RotateX)
        .set_color(border_color, fill_color, text_color)
        .into();
    let y_rotation_slider: Node<_, _> = Slider::new(-PI..PI, state.y_rotation)
        .on_drag(Message::RotateY)
        .set_color(border_color, fill_color, text_color)
        .into();

    let img_data: Vec<Color> = state
        .buffer
        .palette
        .clone()
        .into_iter()
        .map(|px| ToBytes::to_be_bytes(&px))
        .collect();

    let color_picker = ColorPicker {
        w: 8,
        h: img_data.len() / 8,
        scale: 8,
        img_data,
        palette: state.buffer.palette.to_vec(),
    };

    row![
        Node::spacer(),
        col![
            Node::spacer(),
            rotation_label,
            x_rotation_slider.width(100).height(10),
            y_rotation_slider.width(100).height(10),
            color_picker.view(),
            Node::spacer()
        ]
        .spacing(10),
        Node::spacer(),
        col![Node::spacer(), mouse_image_wrapper, Node::spacer()],
        Node::spacer(),
    ]
    .height(Length::Grow)
}

fn _make_button<'a>(
    label: String,
    action: Message,
    fill_color: Color,
    border_color: Color,
    text_color: Color,
) -> Node<'a, Message, Layout> {
    let button_text = Node::new(
        Text::new(label)
            .with_font(&font::BLACKLETTER)
            .with_color(text_color),
    );
    let mut button_quad = Node::new(
        Quad::new()
            .fill(fill_color)
            .border_thickness(1)
            .border_color(border_color),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding([0, 6, 5, 6])
    .row();
    button_quad.push(button_text);

    let mut button_node = Node::new(Button::new().whenever_down(action))
        .width(Length::Shrink)
        .height(Length::Shrink);
    button_node.push(button_quad);
    button_node
}

fn index_to_icecube_color(color_idx: u8, palette: [u32; COLOR_DEPTH as usize]) -> Color {
    let color_u32 = palette[color_idx as usize];
    ToBytes::to_be_bytes(&color_u32)
}
