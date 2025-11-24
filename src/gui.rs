use std::time::{Duration, Instant};

use icecube::button::Button;
use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::palette::MAIN_LIGHT;
use icecube::quad::Quad;
use icecube::tree::Node;
use icecube::{col, row};

use crate::animation::{self, Timing};
use crate::buffer::Buffer;
use crate::model::{draw, Model};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Invert,
    TimeElapsed(Duration),
}

pub struct State {
    data: Vec<usize>,
    model: Model,
    buffer: Buffer,
    start_instant: Instant,
}

impl State {
    pub fn new(buffer: Buffer, model: Model) -> Self {
        Self {
            data: vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            model,
            buffer,
            start_instant: Instant::now(),
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
    }
}

fn render(duration: Duration, state: &mut State) {
    // cache.hot_reload(); // TODO turn back on?
    // buffer.palette = palette_handle.read().colors;
    //TODO: figure out how to get Materials out of the AssetReadGuard without cloning
    // model.cube.shape.materials = NamedMaterials(material_handle.read().0.clone()).into();

    draw(&mut state.buffer, &state.model);

    let timing = Timing {
        time_since_start: Instant::now() - state.start_instant,
        _delta: duration.as_secs_f32(),
    };

    animation::update(timing, &mut state.model);
}

pub fn view(state: &State) -> Node<Message, Layout> {
    let render = state
        .buffer
        .canvas
        .clone()
        .into_iter()
        .map(|px| match px {
            0 => 0,
            1..4 => 1,
            4..8 => 2,
            8.. => 3,
        } as usize)
        .collect();

    let image =
        Node::new(Image::new(render, state.buffer.width(), state.buffer.height()).scale_factor(2))
            .height(Length::Shrink)
            .width(Length::Shrink);

    let mut button = Node::new(Button::new().on_press(Message::Invert))
        .height(Length::Shrink)
        .width(Length::Shrink);
    button.push(image);

    row![
        Node::spacer(),
        col![Node::spacer(), button, Node::spacer()],
        Node::spacer()
    ]
    .height(Length::Grow)
}

/*
fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT, |_| None)
}
*/
