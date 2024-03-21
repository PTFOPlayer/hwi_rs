use std::rc::Rc;

use iced::{
    widget::{text, Column, Container},
    Border, Color, Shadow,
};
use plotters::{
    series::LineSeries,
    style::{self, IntoTextStyle},
};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

pub use crate::Message;

#[derive(Clone)]
pub struct Graph {
    state: Vec<(usize, f32)>,
    max_value: f32,
    len: usize,
    name: Rc<str>,
}

impl Graph {
    pub fn new(max_value: f32, name: &str, len: usize) -> Graph {
        let mut state = vec![(0usize, 0f32); len];
        for i in 0..len {
            state[i].0 = i;
        }
        Graph {
            state,
            max_value,
            len,
            name: name.into(),
        }
    }

    pub fn update(&mut self, entry: f32) {
        if entry > self.max_value {
            self.max_value = entry;
        }
        for i in 0..(self.len - 1) {
            self.state[i] = self.state[i + 1];
            self.state[i].0 -= 1;
        }
        self.state[self.len - 1] = (self.len - 1, entry);
    }

    pub fn resize(&mut self, mut len: usize) {
        if len == 0 {
            len += 1;
        }
        self.len = len;
        self.state = vec![(0usize, 0f32); len];
        for i in 0..len {
            self.state[i].0 = i;
        }
    }

    pub fn into_view(&self) -> Container<'_, Message> {
        let chart = ChartWidget::new(self.clone())
            .height(250.into())
            .width(750.into());

        Container::new(
            Column::new()
                .spacing(10)
                .push(text(self.name.clone()).size(31))
                .push(chart),
        )
        .style(|_: &_| iced::widget::container::Appearance {
            border: Border {
                color: Color::from_rgba8(10, 10, 10, 1.),
                width: 3.,
                radius: [12., 12., 12., 12.].into(),
            },
            text_color: None,
            background: None,
            shadow: Shadow::default(),
        })
        .padding(14)
    }
}

impl Chart<Message> for Graph {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .margin(5)
            .set_left_and_bottom_label_area_size(60)
            .build_cartesian_2d(0..self.len, 0f32..self.max_value)
            .unwrap();
        chart
            .configure_mesh()
            .axis_style(style::RED)
            .bold_line_style(style::full_palette::GREY_A400)
            .label_style(("Calibri", 12, style::FontStyle::Normal).with_color(style::WHITE))
            .draw()
            .unwrap();
        chart.configure_series_labels().draw().unwrap();
        chart
            .draw_series(LineSeries::new(self.state.clone().into_iter(), style::CYAN))
            .unwrap();
    }
}
