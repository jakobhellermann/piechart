use piechart::{Chart, Data};

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Data { name: "BTC".into(), value: 5977.0, color: (), fill: '•' },
        Data { name: "BCH".into(), value: 3045.0, color: (), fill: '◍' },
        Data { name: "LTC".into(), value: 2030.0, color: (), fill: '.' },
        Data { name: "ETH".into(), value: 2350.0, color: (), fill: '*' },
    ];

    Chart::new().draw(&data);
}
