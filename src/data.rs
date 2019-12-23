use crate::Color;

#[derive(Clone)]
pub struct Data {
    pub label: String,
    pub value: f32,
    pub color: Option<Color>,
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
