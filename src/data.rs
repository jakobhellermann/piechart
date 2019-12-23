use crate::Color;

/// A data item is used to describe a datapoint that will be rendered in the [`Chart::draw`](struct.Chart.html#method.draw)-method.
#[derive(Debug, Clone)]
pub struct Data {
    /// The label will be rendered if the [`legend`](struct.Chart.html#method.legend) is set to `true`.
    pub label: String,
    /// The value is an arbitrary datapoint and will be interpreted relative to the sum of values of the datapoints.
    pub value: f32,
    /// If `color` is specified each character will be prefixed with the ANSI escape code for its color.
    pub color: Option<Color>,
    /// The `fill` char controls how the area in the pie chart will be rendered.
    pub fill: char,
}
impl Default for Data {
    fn default() -> Self {
        Data {
            label: "".into(),
            value: 1.0,
            color: None,
            fill: '•',
        }
    }
}

impl Data {
    pub(crate) fn format_label(&self, total: f32) -> String {
        let fill = match self.color {
            Some(c) => c.paint(self.fill.to_string()).to_string(),
            None => self.fill.to_string(),
        };
        format!("{} {} {:.2}%", fill, self.label, self.value / total * 100.0)
    }
}
