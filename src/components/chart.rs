use std::rc::Rc;

use iced::{
    widget::{text, Column, Text},
    Renderer, Theme,
};
use plotters::{series::LineSeries, style};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::Message;

#[derive( Clone)]
pub struct Graph {
    state: [(u32, f64); 50],
    max_value: f64,
    name: Rc<str>,
}

impl Graph {
    pub fn new(max_value: f64, name: &str) -> Graph {
        let mut state: [(u32, f64); 50] = [(0u32, 0f64); 50];
        for i in 0..50 {
            state[i].0 = i as u32;
        }
        Graph {
            state,
            max_value,
            name: "".into(),
        }
    }

    pub fn modify_graph(&mut self, entry: f64) {
        for i in 0..49 {
            self.state[i] = self.state[i + 1];
            self.state[i].0 -= 1;
        }
        self.state[49] = (50, entry);
    }

    pub fn into_view<'a>(&self) -> Column<'a, Message, Renderer<Theme>> {
        Column::new()
            .spacing(10)
            .push(text(self.name.clone()))
            .push(ChartWidget::new(self.clone()).height(iced::Length::Fixed(250.)))
    }
}

impl Chart<Message> for Graph {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .margin(5)
            .build_cartesian_2d(0u32..50u32, 0f64..self.max_value)
            .unwrap();
        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(self.state.into_iter(), style::CYAN))
            .unwrap();
    }
}
