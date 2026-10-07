use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub border: Style,
    pub title: Style,
    pub text_primary: Style,
    pub text_secondary: Style,
    pub text_muted: Style,
    pub accent: Style,
    pub spotify_green: Style,
    pub footer_key: Style,
    pub footer_text: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            border: Style::default().fg(Color::DarkGray),
            title: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            text_primary: Style::default().fg(Color::White),
            text_secondary: Style::default().fg(Color::Gray),
            text_muted: Style::default().fg(Color::DarkGray),
            accent: Style::default().fg(Color::Cyan),
            spotify_green: Style::default().fg(Color::Rgb(30, 215, 96)),
            footer_key: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            footer_text: Style::default().fg(Color::DarkGray),
        }
    }
}
