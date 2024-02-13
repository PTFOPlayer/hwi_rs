use std::rc::Rc;

use iced::{
    widget::{text, Column},
    Renderer, Theme,
};
use plotters::{
    series::LineSeries,
    style::{self, IntoTextStyle},
};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::Message;

#[derive(Clone)]
pub struct Graph {
    state: [(u32, f32); 50],
    max_value: f32,
    name: Rc<str>,
}

impl Graph {
    pub fn new(max_value: f32, name: &str) -> Graph {
        let mut state: [(u32, f32); 50] = [(0u32, 0f32); 50];
        for i in 0..50 {
            state[i].0 = i as u32;
        }
        Graph {
            state,
            max_value,
            name: name.into(),
        }
    }

    pub fn modify_graph(&mut self, entry: f32) {
        for i in 0..49 {
            self.state[i] = self.state[i + 1];
            self.state[i].0 -= 1;
        }
        self.state[49] = (49, entry);
    }

    pub fn into_view<'a>(&self) -> Column<'a, Message, Renderer<Theme>> {
        let chart = ChartWidget::new(self.clone()).height(iced::Length::Fixed(250.)).width(750.into());
        Column::new()
            .spacing(10)
            .push(text(self.name.clone()))
            .push(chart)
    }
}

impl Chart<Message> for Graph {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .margin(5)
            .set_left_and_bottom_label_area_size(60)
            .build_cartesian_2d(0u32..50u32, 0f32..self.max_value)
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
            .draw_series(LineSeries::new(self.state.into_iter(), style::CYAN))
            .unwrap()
            .label("lorem ipsum");
    }
}
