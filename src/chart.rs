use crate::{utils, Data};
use std::{convert::TryInto, io};

/// The `Chart` struct contains the configuration for displaying some data.
///
/// By default, a chart has a radius of `9`, an aspect ratio of `2` and doesn't show its legend.
/// # Example usage:
/// ```rust
/// use piechart::{Chart, Color, Data};
///
/// fn main() {
///     let data = vec![
///         Data { label: "dd1".into(), value: 4.0, color: Some(Color::Red), fill: '•' },
///         Data { label: "dd2".into(), value: 2.0, color: Some(Color::Green), fill: '•' },
///         Data { label: "dd3".into(), value: 2.6, color: Some(Color::Blue), fill: '•' },
///     ];
///
///     Chart::new()
///         .radius(9)
///         .aspect_ratio(2)
///         .legend(true)
///         .draw(&data);
/// }
/// ```
/// will result in
///
/// ![example image](https://raw.githubusercontent.com/jakobhellermann/piechart/master/examples/config.png)
#[derive(Debug)]
pub struct Chart {
    radius: u16,
    aspect_ratio: u16,
    legend: bool,
}
impl Default for Chart {
    fn default() -> Self {
        Self {
            radius: 8,
            aspect_ratio: 2,
            legend: false,
        }
    }
}

impl Chart {
    /// Contructs a new chart initialized with its default values
    pub fn new() -> Self {
        Default::default()
    }
    /// Sets the radius of the pie chart.
    /// To choose which size fits the best, you can run a code snippet like this:
    /// ```rust
    /// # use piechart::Chart;
    /// # let data = vec![Default::default()];
    /// let mut chart = Chart::new();
    /// for radius in 0..=12 {
    ///     chart.radius(radius);
    ///     chart.draw(&data);
    /// }
    /// ```
    pub fn radius(&mut self, radius: u16) -> &mut Self {
        self.radius = radius;
        self
    }
    /// The aspect ratio controls how stretched or squished the circle is.
    /// Since terminal columns are more tall than wide a ration of 2 or 3 is the best in most cases.
    pub fn aspect_ratio(&mut self, aspect_ratio: u16) -> &mut Self {
        assert!(aspect_ratio > 0, "aspect ratio has to be greater than zero");
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Specifies whether the chart should render a legend with the labels and their percentages.
    pub fn legend(&mut self, legend: bool) -> &mut Self {
        self.legend = legend;
        self
    }
}

impl Chart {
    /// Renders the chart and outputs it onto `stdout`.
    /// The method panics in case of an error. If you want more fine-grained control about error recovery
    /// and how the buffer the chart is rendered into the buffer, use [`Chart::draw_into`](struct.Chart.html#method.draw_into).
    pub fn draw(&self, data: &[Data]) {
        self.draw_into(io::stdout(), data)
            .expect("failed to write to stdout")
    }

    /// Same as [`Chart::draw`](struct.Chart.html#method.draw), but you can supply your own `impl Write`
    /// and you can handle errors gracefully.
    pub fn draw_into(&self, mut f: impl io::Write, data: &[Data]) -> io::Result<()> {
        let total: f32 = data.iter().map(|d| d.value).sum();
        assert!(!data.is_empty(), "chart data cannot be empty");
        assert!(
            total > 0.0,
            "total of data values has to be greater than zero"
        );
        let data_angles = utils::data_angles(total, data);

        let radius = self.radius as i32;
        let aspect_ratio = self.aspect_ratio as i32;

        let center_x = (radius as f32 * (aspect_ratio as f32).sqrt()).round() as i32;

        let circle = (-radius..=radius).map(|y| {
            let width = utils::calculate_width(radius, y, aspect_ratio);

            let mut output = " ".repeat((center_x - width).try_into().unwrap());

            (-width..=width).for_each(|x| {
                let (x, y) = (x as f32, y as f32);
                let angle = x.atan2(y).to_degrees();

                let idx = data_angles
                    .iter()
                    .position(|a| 360.0 / 2.0 - angle <= *a)
                    .unwrap();
                let item = &data[idx];

                match item.color {
                    None => output.push(item.fill),
                    Some(c) => output.push_str(&c.paint(item.fill.to_string()).to_string()),
                }
            });

            if self.legend {
                let label_padding = 2;

                let padding = " ".repeat((center_x - width + label_padding).try_into().unwrap());
                output.push_str(&padding);

                let max_label_idx = (data.len() - 1) as i32;

                let mut iter = (0..=max_label_idx)
                    .map(|x| x * 2)
                    .map(|x| x - max_label_idx);

                if let Some(idx) = iter.position(|i| i == y) {
                    let item = &data[idx];
                    output.push_str(&item.format_label(total));
                }
            }

            output
        });

        for line in circle {
            writeln!(&mut f, "{}", line)?;
        }

        Ok(())
    }
}
