use piechart::{Chart, Data};

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Data { name: "dd1".into(), value: 1.0, color: (), fill: '•' },
        Data { name: "dd2".into(), value: 1.0, color: (), fill: '◍' },
        Data { name: "dd3".into(), value: 1.0, color: (), fill: '.' },
    ];

    Chart::new()
        .radius(8)
        .aspect_ratio(3)
        .legend(true)
        .draw(&data);
}
