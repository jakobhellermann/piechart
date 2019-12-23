pub use ansi_term::Color;
use std::{
    convert::TryInto,
    io::{self, Write},
};

pub struct Data {
    pub name: String,
    pub value: f32,
    pub color: Option<Color>,
    pub fill: char,
}

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
    pub fn new() -> Self {
        Default::default()
    }
    pub fn radius(&mut self, radius: u16) -> &mut Self {
        self.radius = radius;
        self
    }
    pub fn aspect_ratio(&mut self, aspect_ratio: u16) -> &mut Self {
        assert!(aspect_ratio > 0, "aspect ratio has to be greater than zero");
        self.aspect_ratio = aspect_ratio;
        self
    }
    pub fn legend(&mut self, legend: bool) -> &mut Self {
        self.legend = legend;
        self
    }
}

fn data_angles(total: f32, data: &[Data]) -> Vec<f32> {
    let mut angle = 0.0;
    data.iter()
        .map(|d| d.value / total)
        .map(|pct| {
            let val = pct * 360.0;
            angle += val;
            angle
        })
        .collect()
}

fn calculate_width(radius: i32, y: i32, aspect_ratio: i32) -> i32 {
    let val = radius.pow(2) - y.pow(2);
    let width = ((val * aspect_ratio) as f32).sqrt().round() as i32;

    // without this the circles have an ugly single dot on top and bottom.
    if width == 0 && aspect_ratio > 1 {
        radius / (aspect_ratio * 2)
    } else {
        width
    }
}

impl Chart {
    pub fn draw(&self, data: &[Data]) {
        self.draw_into(io::stdout(), data)
            .expect("failed to write to stdout")
    }

    pub fn draw_into(&self, f: impl io::Write, data: &[Data]) -> io::Result<()> {
        let total: f32 = data.iter().map(|d| d.value).sum();
        let data_angles = data_angles(total, data);

        let radius = self.radius as i32;
        let aspect_ratio = self.aspect_ratio as i32;

        let center_x = (radius as f32 * (aspect_ratio as f32).sqrt()).round() as i32;

        let circle = (-radius..=radius).map(|y| {
            let width = calculate_width(radius, y, aspect_ratio);

            let padding_len = center_x - width;
            let mut output = " ".repeat(padding_len.try_into().unwrap());

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

            output
        });

        let mut buf = io::BufWriter::new(f);
        for line in circle {
            writeln!(buf, "{}", line)?;
        }

        Ok(())
    }
}
