use piechart::{Chart, Data};

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Data { label: "BTC".into(), value: 5977.0, color: None, fill: '•' },
        Data { label: "BCH".into(), value: 3045.0, color: None, fill: '◍' },
        Data { label: "LTC".into(), value: 2030.0, color: None, fill: '.' },
        Data { label: "ETH".into(), value: 2350.0, color: None, fill: '*' },
    ];

    Chart::new().draw(&data);
}
