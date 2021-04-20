#![warn(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    missing_docs
)]

//! `piechart` is a library for drawing fancy pie charts like the following in the terminal:
//!
//! ![example image](https://raw.githubusercontent.com/jakobhellermann/piechart/master/examples/config.png)
//!
//! For more information visit the docs for [`Chart`](struct.Chart.html).

mod chart;
mod data;
mod utils;
pub use ansi_term::{Color, Style};
pub use chart::Chart;
pub use data::Data;

#[cfg(test)]
mod tests {
    use crate::{Chart, Data};
    use std::io;

    struct NullWriter;
    impl io::Write for NullWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct ErrWriter;
    impl io::Write for ErrWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::Other))
        }
        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::from(io::ErrorKind::Other))
        }
    }

    #[test]
    #[should_panic]
    fn empty_data() {
        Chart::new().draw_into(NullWriter, &[]).unwrap();
    }

    #[test]
    fn data_len() {
        let chart = Chart::new();
        for len in 1..=16 {
            println!("{}", len);
            let data = vec![Default::default(); len];
            chart.draw_into(NullWriter, &data).unwrap();
        }
    }

    #[test]
    fn very_much_data() {
        let chart = Chart::new();
        let data = vec![Default::default(); 2048];
        chart.draw_into(NullWriter, &data).unwrap();
    }

    #[test]
    fn config_combinations() {
        let mut chart = Chart::new();
        for radius in 0..=5 {
            chart.radius(radius);
            for aspect_ratio in 1..=5 {
                chart.aspect_ratio(aspect_ratio);
                for legend in &[true, false] {
                    chart.legend(*legend);
                    for data_len in 1..=5 {
                        let data = vec![Default::default(); data_len];
                        chart.draw_into(NullWriter, &data).unwrap();
                    }
                }
            }
        }
    }

    #[should_panic]
    #[test]
    fn zero_total() {
        #[rustfmt::skip]
        let data = vec![Data { value: 0.0, ..Default::default() }; 2];
        Chart::new().draw(&data)
    }

    #[test]
    fn writer_err() {
        Chart::new()
            .draw_into(ErrWriter, &[Default::default()])
            .expect_err("expected `draw_into` to propagate io error");
    }
}
