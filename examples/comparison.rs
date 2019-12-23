use piechart::{Chart, Color, Data};

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Data { label: "dd1".into(), value: 4.0, color: Some(Color::Red), fill: '•' },
        Data { label: "dd2".into(), value: 2.0, color: Some(Color::Green), fill: '•' },
        Data { label: "dd3".into(), value: 2.6, color: Some(Color::Blue), fill: '•' },
    ];

    let mut chart = Chart::new();

    for a in 0..=12 {
        chart.radius(a);
        chart.draw(&data);
    }
}
