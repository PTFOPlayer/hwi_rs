use std::ops::{Sub, SubAssign};

use plotters::{series::LineSeries, style};
use plotters_iced::{Chart, ChartBuilder, DrawingBackend};

use crate::Message;

#[derive(Debug, Clone)]
pub struct Graph<T> {
    state: [(u32, T); 50],
    max_value: T,
}

impl<T> Graph<T>
where
    T: SubAssign + Sized + Copy + Sub<Output = T>,
{
    pub fn new(max_value: T) -> Graph<T> {
        let zeroed = max_value - max_value;
        let mut state: [(u32, T); 50] = [(0u32, zeroed); 50];
        for i in 0..50 {
            state[i].0 = i as u32;
        }
        Graph { state, max_value }
    }
    pub fn modify_graph(&mut self, entry: T) {
        for i in 0..49 {
            self.state[i] = self.state[i + 1];
            self.state[i].0 -= 1;
        }
        self.state[49] = (50, entry);
    }
}

impl Chart<Message> for Graph<f64> {
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

impl Chart<Message> for Graph<f32> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .margin(5)
            .build_cartesian_2d(0u32..50u32, 0f32..self.max_value)
            .unwrap();
        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(self.state.into_iter(), style::CYAN))
            .unwrap();
    }
}
