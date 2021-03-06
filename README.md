## piechart

`piechart` is a rust crate for drawing fancy pie charts in the terminal.

# Example usage:

```rust
use piechart::{Chart, Color, Data};

fn main() {
    let data = vec![
        Data { label: "Chocolate".into(), value: 4.0, color: Some(Color::Blue.into()), fill: '•' },
        Data { label: "Strawberry".into(), value: 2.0, color: Some(Color::Red.into()), fill: '▪' },
        Data { label: "Vanilla".into(), value: 2.6, color: Some(Color::Yellow.into()), fill: '▴' },
    ];

    Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&data);
}
```

will result in

![example image](https://raw.githubusercontent.com/jakobhellermann/piechart/main/examples/config.png)

## CLI Usage

You can install a small command line tool using this library by running
```sh
cargo install piechart --features cli
```


```sh
piechart --radius 5 A:2.0 B:3.0 'Red:4.0:italic magenta strike:ω' 'Blue:1.0:bold italic blue ul:u'
```

## Credits

Developer: Jakob Hellermann

The design and code is heaviliy inspired by [piotrmurach's ruby gem](https://github.com/piotrmurach/tty-pie).

## License

The MIT License (MIT)

Copyright (c) 2019 Jakob Hellermann

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
