use crossterm::style::Color;
use std::collections::HashMap;

pub fn make_colors() -> HashMap<&'static str, Color> {
    let mut colors = HashMap::new();
    colors.insert(
        "function",
        Color::Rgb {
            r: 0x7d,
            g: 0xae,
            b: 0xa3,
        },
    );
    colors.insert(
        "function.method",
        Color::Rgb {
            r: 0x82,
            g: 0xaa,
            b: 0xff,
        },
    );
    colors.insert(
        "function.macro",
        Color::Rgb {
            r: 0xff,
            g: 0x9e,
            b: 0x64,
        },
    );
    colors.insert(
        "constant.builtin",
        Color::Rgb {
            r: 0xff,
            g: 0xcc,
            b: 0x66,
        },
    );
    colors.insert(
        "constant",
        Color::Rgb {
            r: 0xd8,
            g: 0xa6,
            b: 0x57,
        },
    );
    colors.insert(
        "type",
        Color::Rgb {
            r: 0x56,
            g: 0x9C,
            b: 0xD6,
        },
    );
    colors.insert(
        "type.builtin",
        Color::Rgb {
            r: 0x4E,
            g: 0xC9,
            b: 0xB0,
        },
    );
    colors.insert(
        "constructor",
        Color::Rgb {
            r: 0xB5,
            g: 0xCE,
            b: 0xA8,
        },
    );
    colors.insert(
        "property",
        Color::Rgb {
            r: 0xCE,
            g: 0x91,
            b: 0x78,
        },
    );
    colors.insert(
        "variable.parameter",
        Color::Rgb {
            r: 0x9C,
            g: 0xDC,
            b: 0xFE,
        },
    );
    colors.insert(
        "variable.builtin",
        Color::Rgb {
            r: 0xC5,
            g: 0x86,
            b: 0xC0,
        },
    );
    colors.insert(
        "label",
        Color::Rgb {
            r: 0xD7,
            g: 0xBA,
            b: 0x7D,
        },
    );
    colors.insert(
        "comment",
        Color::Rgb {
            r: 0x60,
            g: 0x8B,
            b: 0x4E,
        },
    );
    colors.insert(
        "punctuation.bracket",
        Color::Rgb {
            r: 0xD4,
            g: 0xD4,
            b: 0xD4,
        },
    );
    colors.insert(
        "punctuation.delimiter",
        Color::Rgb {
            r: 0xD4,
            g: 0xD4,
            b: 0xD4,
        },
    );
    colors.insert(
        "keyword",
        Color::Rgb {
            r: 0xC5,
            g: 0x86,
            b: 0xC0,
        },
    );
    colors.insert(
        "string",
        Color::Rgb {
            r: 0xCE,
            g: 0x91,
            b: 0x78,
        },
    );
    colors.insert(
        "escape",
        Color::Rgb {
            r: 0xd7,
            g: 0xba,
            b: 0x7d,
        },
    );
    colors.insert(
        "operator",
        Color::Rgb {
            r: 0x56,
            g: 0x9C,
            b: 0xD6,
        },
    );
    colors.insert(
        "attribute",
        Color::Rgb {
            r: 0x4E,
            g: 0xC9,
            b: 0xB0,
        },
    );
    colors
}
