use piechart::{Chart, Color, Data};

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Data { label: "Chocolate".into(), value: 4.0, color: Some(Color::Blue), fill: '•' },
        Data { label: "Strawberry".into(), value: 2.0, color: Some(Color::Red), fill: '▪' },
        Data { label: "Vanilla".into(), value: 2.6, color: Some(Color::Yellow), fill: '▴' },
    ];

    Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&data);
}
