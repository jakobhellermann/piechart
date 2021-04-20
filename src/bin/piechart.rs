use clap::{App, Arg};
use piechart::{Chart, Color, Data, Style};

fn parse_value_error(name: &str, err: impl std::error::Error) -> ! {
    clap::Error::with_description(
        format!("failed to parse {}: {}\n", name, err),
        clap::ErrorKind::InvalidValue,
    )
    .exit()
}
fn clap_err(msg: &str) -> ! {
    clap::Error::with_description(format!("{}\n", msg), clap::ErrorKind::InvalidValue).exit()
}

fn main() {
    let matches = App::new("piechart")
        .about("displays fancy pie charts in the terminal")
        .arg(
            Arg::new("radius")
                .long("radius")
                .takes_value(true)
                .default_value("7"),
        )
        .arg(
            Arg::new("aspect ratio")
                .long("aspect")
                .takes_value(true)
                .default_value("3"),
        )
        .arg(Arg::new("no legend").long("no-legend"))
        .arg(
            Arg::new("values")
                .multiple(true)
                .about("e.g. `Label:2.0` or `Other:4.2:italic strike magenta:▴`"),
        )
        .setting(clap::AppSettings::ColoredHelp)
        .get_matches();

    let radius = matches
        .value_of("radius")
        .unwrap()
        .parse()
        .unwrap_or_else(|err| parse_value_error("radius", err));
    let aspect_ratio = matches
        .value_of("aspect ratio")
        .unwrap()
        .parse()
        .unwrap_or_else(|err| parse_value_error("aspect ratio", err));
    if aspect_ratio == 0 {
        clap_err("the aspect ratio should be greater than 0");
    }
    let legend = !matches.is_present("no legend");

    let mut color_fallback = std::array::IntoIter::new([
        Color::Blue,
        Color::Red,
        Color::Purple,
        Color::Cyan,
        Color::Yellow,
        Color::Green,
    ])
    .cycle()
    .map(Style::from);

    let mut fill_fallback = std::array::IntoIter::new(['•', '▪', '▴']).cycle();

    let mut data: Vec<Data> = matches
        .values_of("values")
        .unwrap_or_else(|| clap_err("expected values: `piechart A:4.0 'B:2.1:bold red:*'`"))
        .map(|str| parse_data(str, &mut color_fallback, &mut fill_fallback))
        .collect::<Result<_, _>>()
        .unwrap_or_else(|err| parse_value_error("values", err));

    let n_without_color = data.iter().filter(|data| data.color.is_none()).count();

    for data in data.iter_mut().filter(|data| data.color.is_none()) {
        data.color = Some(Style::new().fg(piechart::Color::Blue));
    }
    if n_without_color > 1 {}

    let mut chart = Chart::new();

    chart.radius(radius);
    chart.aspect_ratio(aspect_ratio);
    chart.legend(legend);

    chart.draw(&data);
}

#[derive(Debug)]
enum ParseError {
    IncorrectAmountOfFields,
    InvalidValue(std::num::ParseFloatError),
    InvalidColor(colorparse::Error),
    InvalidFill(String),
}

use std::fmt;
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IncorrectAmountOfFields => {
                write!(f, "expected 2-4 fields: `Red:4.0:bold red:•`")
            }
            ParseError::InvalidValue(e) => write!(f, "cannot parse value: {}", e),
            ParseError::InvalidColor(e) => write!(f, "{}", e),
            ParseError::InvalidFill(fill) => {
                write!(f, "invalid fill '{}', should be a single char", fill)
            }
        }
    }
}
impl std::error::Error for ParseError {}

fn parse_data(
    input: &str,
    color_fallback: &mut impl Iterator<Item = Style>,
    fill_fallback: &mut impl Iterator<Item = char>,
) -> Result<Data, ParseError> {
    let mut input = input.split(':');
    let label = input
        .next()
        .ok_or(ParseError::IncorrectAmountOfFields)?
        .to_string();
    let value = input
        .next()
        .ok_or(ParseError::IncorrectAmountOfFields)?
        .parse()
        .map_err(ParseError::InvalidValue)?;

    let color = input
        .next()
        .map(colorparse::parse)
        .transpose()
        .map_err(ParseError::InvalidColor)?
        .or_else(|| color_fallback.next());

    let fill = input.next();

    let fill = fill
        .map(|fill| {
            let mut chars = fill.chars();
            let char = chars
                .next()
                .ok_or(ParseError::InvalidFill(fill.to_string()))?;
            match chars.next() {
                Some(_) => Err(ParseError::InvalidFill(fill.to_string())),
                None => Ok(char),
            }
        })
        .transpose()?
        .or_else(|| fill_fallback.next())
        .unwrap_or('•');

    Ok(Data {
        label,
        value,
        color,
        fill,
    })
}

// taken from https://github.com/joshtriplett/colorparse/ (the ascii_term version does not match)
mod colorparse {
    use piechart::{Color, Style};

    /// Type for errors returned by the parser.
    #[derive(Debug, PartialEq)]
    pub enum Error {
        /// An extra color appeared after the foreground and background colors.
        ExtraColor(String, String),
        /// An unknown word appeared.
        UnknownWord(String, String),
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Self::ExtraColor(s, word) => {
                    write!(
                        fmt,
                        "Error parsing style \"{}\": extra color \"{}\"",
                        s, word
                    )
                }
                Self::UnknownWord(s, word) => {
                    write!(
                        fmt,
                        "Error parsing style \"{}\": unknown word: \"{}\"",
                        s, word
                    )
                }
            }
        }
    }

    impl std::error::Error for Error {}

    fn parse_color(word: &str) -> Result<Option<Color>, ()> {
        let color = match word {
            "normal" => None,
            "-1" => None,
            "black" => Some(Color::Black),
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "yellow" => Some(Color::Yellow),
            "blue" => Some(Color::Blue),
            "magenta" => Some(Color::Purple),
            "cyan" => Some(Color::Cyan),
            "white" => Some(Color::White),
            _ => {
                if word.starts_with('#') && word.len() == 7 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&word[1..3], 16),
                        u8::from_str_radix(&word[3..5], 16),
                        u8::from_str_radix(&word[5..7], 16),
                    ) {
                        return Ok(Some(Color::RGB(r, g, b)));
                    }
                } else if let Ok(n) = u8::from_str_radix(word, 10) {
                    return Ok(Some(Color::Fixed(n)));
                }
                return Err(());
            }
        };
        Ok(color)
    }

    /// Parse a string in Git's color configuration syntax into an
    /// `ansi_term::Style`.
    pub fn parse(s: &str) -> Result<Style, Error> {
        let mut style = Style::new();
        let mut colors = 0;
        let mut bold = false;
        let mut dim = false;
        let mut ul = false;
        let mut blink = false;
        let mut reverse = false;
        let mut italic = false;
        let mut strike = false;
        for word in s.split_whitespace() {
            match word.to_lowercase().as_ref() {
                "nobold" | "no-bold" => {
                    bold = false;
                }
                "bold" => {
                    bold = true;
                }
                "nodim" | "no-dim" => {
                    dim = false;
                }
                "dim" => {
                    dim = true;
                }
                "noul" | "no-ul" => {
                    ul = false;
                }
                "ul" => {
                    ul = true;
                }
                "noblink" | "no-blink" => {
                    blink = false;
                }
                "blink" => {
                    blink = true;
                }
                "noreverse" | "no-reverse" => {
                    reverse = false;
                }
                "reverse" => {
                    reverse = true;
                }
                "noitalic" | "no-italic" => {
                    italic = false;
                }
                "italic" => {
                    italic = true;
                }
                "nostrike" | "no-strike" => {
                    strike = false;
                }
                "strike" => {
                    strike = true;
                }
                w => {
                    if let Ok(color) = parse_color(w) {
                        if colors == 2 {
                            return Err(Error::ExtraColor(s.to_string(), word.to_string()));
                        } else if let Some(color) = color {
                            if colors == 0 {
                                style = style.fg(color);
                            } else if colors == 1 {
                                style = style.on(color);
                            }
                        }
                        colors += 1;
                    } else {
                        return Err(Error::UnknownWord(s.to_string(), word.to_string()));
                    }
                }
            }
        }
        if bold {
            style = style.bold();
        }
        if dim {
            style = style.dimmed();
        }
        if ul {
            style = style.underline();
        }
        if blink {
            style = style.blink();
        }
        if reverse {
            style = style.reverse();
        }
        if italic {
            style = style.italic();
        }
        if strike {
            style = style.strikethrough();
        }
        Ok(style)
    }
}
