use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub border: Style,
    pub border_active: Style,
    pub title: Style,
    pub tab_normal: Style,
    pub tab_selected: Style,
    pub footer_key: Style,
    pub footer_text: Style,
    pub accent: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            border: Style::default().fg(Color::DarkGray),
            border_active: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            title: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            tab_normal: Style::default().fg(Color::Gray),
            tab_selected: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            footer_key: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            footer_text: Style::default().fg(Color::DarkGray),
            accent: Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        }
    }
}
