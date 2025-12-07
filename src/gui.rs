use icecube::palette::{Color, BLUE_DARK, BLUE_LIGHT};
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

use crate::animation::{self, Timing};
use crate::buffer::Buffer;
use crate::model::{draw, Model};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Invert,
    TimeElapsed(Duration),
    Rotate(f32),
}

pub struct State {
    data: Vec<usize>,
    model: Model,
    buffer: Buffer,
    start_instant: Instant,
    rotation: f32,
}

impl State {
    pub fn new(buffer: Buffer, model: Model) -> Self {
        Self {
            data: vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            model,
            buffer,
            start_instant: Instant::now(),
            rotation: 0.0,
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
        Message::Rotate(radians) => state.rotation = radians,
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

    animation::update(state.rotation, &mut state.model);
}

pub fn view(state: &State) -> Node<Message, Layout> {
    // TODO just store a [u8; 4] in buffer instead of u32?
    let render: Vec<[u8; 4]> = state
        .buffer
        .rgb_pixels()
        .clone()
        .into_iter()
        .map(|px| ToBytes::to_be_bytes(&px))
        .collect();

    let image = Node::new(
        Image::<[u8; 4]>::new(render, state.buffer.width(), state.buffer.height()).scale_factor(2),
    )
    .height(Length::Shrink)
    .width(Length::Shrink);

    let _fill_color = ToBytes::to_be_bytes(&state.buffer.palette[1]);
    let _border_color = ToBytes::to_be_bytes(&state.buffer.palette[2]);
    let text_color = ToBytes::to_be_bytes(&state.buffer.palette[15]);

    let rotation_label = Node::new(
        Text::new(format!(
            "Rotation: {:.0} Degrees",
            state.rotation * 360. / (2. * PI)
        ))
        .with_font(&font::BLACKLETTER)
        .with_color(text_color),
    );

    let rotation_slider: Node<_, _> = Slider::new(-PI..PI, state.rotation)
        .on_drag(Message::Rotate)
        .into();

    row![
        Node::spacer(),
        col![
            Node::spacer(),
            rotation_label,
            rotation_slider.width(100).height(10),
            Node::spacer()
        ]
        .spacing(10),
        Node::spacer(),
        col![Node::spacer(), image, Node::spacer()],
        Node::spacer(),
    ]
    .height(Length::Grow)
}

fn _make_button(
    label: String,
    action: Message,
    fill_color: Color,
    border_color: Color,
    text_color: Color,
) -> Node<Message, Layout> {
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
/*
fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT, |_| None)
}
*/
