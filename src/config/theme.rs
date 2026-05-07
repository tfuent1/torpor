use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// A complete UI theme. All colors are stored as RGB triples so they round-trip
/// cleanly through TOML without depending on Ratatui's `Color` serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    /// Border of the focused pane.
    pub border_focused: Rgb,
    /// Border of unfocused panes.
    pub border_unfocused: Rgb,
    /// Active tab label.
    pub tab_active: Rgb,
    /// Inactive tab label.
    pub tab_inactive: Rgb,
    /// Status bar text when showing a hint.
    pub status_hint: Rgb,
    /// Status bar text when showing an error or action message.
    pub status_message: Rgb,
    /// JSON key color.
    pub json_key: Rgb,
    /// JSON string value color.
    pub json_string: Rgb,
    /// JSON number color.
    pub json_number: Rgb,
    /// JSON boolean color.
    pub json_bool: Rgb,
    /// JSON null color.
    pub json_null: Rgb,
    /// JSON punctuation (braces, commas).
    pub json_punctuation: Rgb,
    /// HTTP 2xx status code color.
    pub status_2xx: Rgb,
    /// HTTP 3xx status code color.
    pub status_3xx: Rgb,
    /// HTTP 4xx status code color.
    pub status_4xx: Rgb,
    /// HTTP 5xx status code color.
    pub status_5xx: Rgb,
    /// Selected row background in the headers editor.
    pub header_selected_bg: Rgb,
    /// Selected row foreground in the headers editor.
    pub header_selected_fg: Rgb,
    /// Normal row foreground in the headers editor.
    pub header_normal_fg: Rgb,
    /// Header value column foreground.
    pub header_value_fg: Rgb,
    /// Placeholder / empty text color.
    pub placeholder: Rgb,
}

/// RGB color stored as three u8 components. Serializes cleanly to/from TOML.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<Rgb> for Color {
    fn from(c: Rgb) -> Self {
        Color::Rgb(c.r, c.g, c.b)
    }
}

// ---------------------------------------------------------------------------
// Built-in themes
// ---------------------------------------------------------------------------

/// Default dark theme — the original Torpor palette.
pub fn default_dark() -> Theme {
    Theme {
        name: "default-dark".into(),
        border_focused: Rgb::new(255, 215, 0),  // yellow
        border_unfocused: Rgb::new(96, 96, 96), // dim gray
        tab_active: Rgb::new(255, 215, 0),
        tab_inactive: Rgb::new(96, 96, 96),
        status_hint: Rgb::new(96, 96, 96),
        status_message: Rgb::new(255, 215, 0),
        json_key: Rgb::new(0, 255, 255),    // cyan
        json_string: Rgb::new(0, 200, 100), // green
        json_number: Rgb::new(255, 215, 0), // yellow
        json_bool: Rgb::new(200, 100, 200), // magenta
        json_null: Rgb::new(96, 96, 96),
        json_punctuation: Rgb::new(140, 140, 140),
        status_2xx: Rgb::new(0, 200, 100),
        status_3xx: Rgb::new(0, 200, 200),
        status_4xx: Rgb::new(255, 215, 0),
        status_5xx: Rgb::new(220, 50, 50),
        header_selected_bg: Rgb::new(40, 40, 40),
        header_selected_fg: Rgb::new(255, 255, 255),
        header_normal_fg: Rgb::new(180, 180, 180),
        header_value_fg: Rgb::new(140, 140, 140),
        placeholder: Rgb::new(60, 60, 60),
    }
}

/// Nord — cool arctic blues and muted purples.
pub fn nord() -> Theme {
    Theme {
        name: "nord".into(),
        border_focused: Rgb::new(136, 192, 208), // nord8 frost
        border_unfocused: Rgb::new(76, 86, 106), // nord3
        tab_active: Rgb::new(136, 192, 208),
        tab_inactive: Rgb::new(76, 86, 106),
        status_hint: Rgb::new(76, 86, 106),
        status_message: Rgb::new(235, 203, 139), // nord13 yellow
        json_key: Rgb::new(136, 192, 208),       // nord8
        json_string: Rgb::new(163, 190, 140),    // nord14 green
        json_number: Rgb::new(180, 142, 173),    // nord15 purple
        json_bool: Rgb::new(235, 203, 139),      // nord13
        json_null: Rgb::new(76, 86, 106),
        json_punctuation: Rgb::new(67, 76, 94), // nord1
        status_2xx: Rgb::new(163, 190, 140),
        status_3xx: Rgb::new(136, 192, 208),
        status_4xx: Rgb::new(235, 203, 139),
        status_5xx: Rgb::new(191, 97, 106),          // nord11 red
        header_selected_bg: Rgb::new(59, 66, 82),    // nord2
        header_selected_fg: Rgb::new(236, 239, 244), // nord6
        header_normal_fg: Rgb::new(216, 222, 233),   // nord5
        header_value_fg: Rgb::new(129, 161, 193),    // nord9
        placeholder: Rgb::new(59, 66, 82),
    }
}

/// Catppuccin Mocha — warm, pastel dark theme.
pub fn catppuccin_mocha() -> Theme {
    Theme {
        name: "catppuccin-mocha".into(),
        border_focused: Rgb::new(137, 180, 250), // blue
        border_unfocused: Rgb::new(88, 91, 112), // surface2
        tab_active: Rgb::new(137, 180, 250),
        tab_inactive: Rgb::new(88, 91, 112),
        status_hint: Rgb::new(88, 91, 112),
        status_message: Rgb::new(249, 226, 175), // yellow
        json_key: Rgb::new(137, 180, 250),       // blue
        json_string: Rgb::new(166, 227, 161),    // green
        json_number: Rgb::new(250, 179, 135),    // peach
        json_bool: Rgb::new(203, 166, 247),      // mauve
        json_null: Rgb::new(88, 91, 112),
        json_punctuation: Rgb::new(108, 112, 134), // overlay0
        status_2xx: Rgb::new(166, 227, 161),
        status_3xx: Rgb::new(137, 220, 235), // sky
        status_4xx: Rgb::new(249, 226, 175),
        status_5xx: Rgb::new(243, 139, 168),         // red
        header_selected_bg: Rgb::new(49, 50, 68),    // surface0
        header_selected_fg: Rgb::new(205, 214, 244), // text
        header_normal_fg: Rgb::new(166, 173, 200),   // subtext1
        header_value_fg: Rgb::new(108, 112, 134),    // overlay0
        placeholder: Rgb::new(49, 50, 68),
    }
}

/// Dracula — the classic dark purple theme.
pub fn dracula() -> Theme {
    Theme {
        name: "dracula".into(),
        border_focused: Rgb::new(189, 147, 249), // purple
        border_unfocused: Rgb::new(68, 71, 90),  // current line
        tab_active: Rgb::new(189, 147, 249),
        tab_inactive: Rgb::new(68, 71, 90),
        status_hint: Rgb::new(98, 114, 164),     // comment
        status_message: Rgb::new(241, 250, 140), // yellow
        json_key: Rgb::new(139, 233, 253),       // cyan
        json_string: Rgb::new(80, 250, 123),     // green
        json_number: Rgb::new(255, 184, 108),    // orange
        json_bool: Rgb::new(189, 147, 249),      // purple
        json_null: Rgb::new(98, 114, 164),
        json_punctuation: Rgb::new(98, 114, 164),
        status_2xx: Rgb::new(80, 250, 123),
        status_3xx: Rgb::new(139, 233, 253),
        status_4xx: Rgb::new(241, 250, 140),
        status_5xx: Rgb::new(255, 85, 85), // red
        header_selected_bg: Rgb::new(68, 71, 90),
        header_selected_fg: Rgb::new(248, 248, 242), // foreground
        header_normal_fg: Rgb::new(200, 200, 210),
        header_value_fg: Rgb::new(98, 114, 164),
        placeholder: Rgb::new(68, 71, 90),
    }
}

/// Solarized Dark — Ethan Schoonover's precision colour scheme.
pub fn solarized_dark() -> Theme {
    // Base tones — only the ones actually mapped to TUI slots
    let base02 = Rgb::new(7, 54, 66); // background highlights / selected row bg
    let base01 = Rgb::new(88, 110, 117); // comments / secondary content / dim text
    let base0 = Rgb::new(131, 148, 150); // body text / normal foreground

    // Accent tones
    let yellow = Rgb::new(181, 137, 0);
    let red = Rgb::new(220, 50, 47);
    let magenta = Rgb::new(211, 54, 130);
    let blue = Rgb::new(38, 139, 210);
    let cyan = Rgb::new(42, 161, 152);
    let green = Rgb::new(133, 153, 0);

    Theme {
        name: "solarized-dark".into(),
        border_focused: blue,
        border_unfocused: base02,
        tab_active: blue,
        tab_inactive: base01,
        status_hint: base01,
        status_message: yellow,
        json_key: blue,
        json_string: green,
        json_number: cyan,
        json_bool: magenta,
        json_null: base01,
        json_punctuation: base01,
        status_2xx: green,
        status_3xx: cyan,
        status_4xx: yellow,
        status_5xx: red,
        header_selected_bg: base02,
        header_selected_fg: base0,
        header_normal_fg: base0,
        header_value_fg: base01,
        placeholder: base02,
    }
}

/// Returns all built-in themes.
pub fn all_builtin() -> Vec<Theme> {
    vec![
        default_dark(),
        nord(),
        catppuccin_mocha(),
        dracula(),
        solarized_dark(),
    ]
}

/// Looks up a built-in theme by name (case-insensitive). Falls back to `default_dark`.
pub fn by_name(name: &str) -> Theme {
    all_builtin()
        .into_iter()
        .find(|t| t.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(default_dark)
}
