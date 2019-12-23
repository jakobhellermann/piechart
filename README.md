## piechart

`piechart` is a rust crate for drawing fancy pie charts in the terminal.

# Example usage:

```rust
use piechart::{Chart, Color, Data};

fn main() {
    let data = vec![
       Data { label: "dd1".into(), value: 4.0, color: Some(Color::Red), fill: '•' },
       Data { label: "dd2".into(), value: 2.0, color: Some(Color::Green), fill: '•' },
       Data { label: "dd3".into(), value: 2.6, color: Some(Color::Blue), fill: '•' },
    ];

    Chart::new()
        .radius(9)
        .aspect_ratio(2)
        .legend(true)
        .draw(&data);
}
```

will result in

![example image](https://raw.githubusercontent.com/jakobhellermann/piechart/master/examples/config.png)

## Credits

Developer: Jakob Hellermann @jakobhellermann

The design and code is heaviliy inspired by [piotrmurach's](https://github.com/piotrmurach/tty-pie) ruby gem.

## License

The MIT License (MIT)

Copyright (c) 2019 Jakob Hellermann

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
