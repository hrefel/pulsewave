use ratatui::style::Color;

pub const BG: Color = Color::Rgb(10, 10, 15);
pub const BORDER: Color = Color::Rgb(189, 147, 249);
pub const TITLE: Color = Color::Rgb(0, 255, 255);
pub const UPLOAD: Color = Color::Rgb(255, 106, 193);
pub const DOWNLOAD: Color = Color::Rgb(80, 250, 123);
pub const TOTAL: Color = Color::Rgb(241, 250, 140);
pub const GOOD: Color = Color::Rgb(80, 250, 123);
pub const WARN: Color = Color::Rgb(241, 250, 140);
pub const BAD: Color = Color::Rgb(255, 85, 85);
pub const FOOTER_KEY: Color = Color::Rgb(98, 114, 164);
pub const AXIS: Color = Color::Rgb(68, 71, 90);

pub const HOST_COLORS: &[Color] = &[
    Color::Rgb(0, 255, 255),
    Color::Rgb(255, 121, 198),
    Color::Rgb(241, 250, 140),
    Color::Rgb(80, 250, 123),
    Color::Rgb(255, 184, 108),
    Color::Rgb(189, 147, 249),
];

pub fn latency_color(ms: f64) -> Color {
    if ms < 50.0 {
        GOOD
    } else if ms < 150.0 {
        WARN
    } else {
        BAD
    }
}

pub fn host_color(index: usize) -> Color {
    HOST_COLORS[index % HOST_COLORS.len()]
}
