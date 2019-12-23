mod chart;
mod data;
mod utils;
pub use ansi_term::Color;
pub use chart::Chart;
pub use data::Data;

#[cfg(test)]
mod tests {
    use crate::Chart;
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
}
