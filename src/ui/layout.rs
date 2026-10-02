use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::theme::Theme;

pub fn render_dashboard(frame: &mut Frame, _app: &App) {
    let theme = Theme::default();
    let area = frame.area();

    // Divide screen into Header, Main Body, Footer
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(16),   // Main GitHub Content Area
            Constraint::Length(1), // Footer
        ])
        .split(area);

    render_header(frame, &theme, vertical_chunks[0]);
    render_github_layout(frame, &theme, vertical_chunks[1]);
    render_footer(frame, &theme, vertical_chunks[2]);
}

fn render_header(frame: &mut Frame, theme: &Theme, area: Rect) {
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let title_line = Line::from(vec![
        Span::styled(" 🐙 GitHub Dashboard ", theme.title),
    ]);

    let header_widget = Paragraph::new(title_line)
        .block(header_block)
        .alignment(Alignment::Left);

    frame.render_widget(header_widget, area);
}

fn render_github_layout(frame: &mut Frame, theme: &Theme, area: Rect) {
    // Divide into 3 vertical sections: Top (Profile & Stats), Middle (Heatmap), Bottom (Repos & Activity)
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Profile & Stat Cards
            Constraint::Length(7), // Contribution Heatmap
            Constraint::Min(8),    // Repositories & Recent Activity
        ])
        .split(area);

    render_top_section(frame, theme, sections[0]);
    render_heatmap_section(frame, theme, sections[1]);
    render_bottom_section(frame, theme, sections[2]);
}

fn render_top_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    // Split into Left Profile Card (30%) and Right Stats Cards (70%)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

    // Profile Card
    let profile_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(" Profile ");

    let profile_lines = vec![
        Line::from(vec![
            Span::styled("User: ", Style::default().fg(Color::DarkGray)),
            Span::styled("--", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Bio:  ", Style::default().fg(Color::DarkGray)),
            Span::raw("--"),
        ]),
        Line::from(vec![
            Span::styled("Loc:  ", Style::default().fg(Color::DarkGray)),
            Span::raw("--"),
        ]),
    ];

    let profile_widget = Paragraph::new(profile_lines).block(profile_block);
    frame.render_widget(profile_widget, chunks[0]);

    // 4 Stat Cards
    let stat_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(chunks[1]);

    let stats = [
        ("Repositories", "0", Color::Green),
        ("Total Stars", "0", Color::Yellow),
        ("Followers", "0", Color::Cyan),
        ("Following", "0", Color::Magenta),
    ];

    for (i, (title, value, color)) in stats.iter().enumerate() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme.border)
            .title(*title);

        let content = Paragraph::new(Span::styled(
            *value,
            Style::default().fg(*color).add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center)
        .block(block);

        frame.render_widget(content, stat_chunks[i]);
    }
}

fn render_heatmap_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(" Contribution Heatmap ");

    frame.render_widget(Paragraph::new("").block(block), area);
}

fn render_bottom_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let repos_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(" Top Repositories ");
    frame.render_widget(Paragraph::new("").block(repos_block), chunks[0]);

    let activity_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(" Recent Activity ");
    frame.render_widget(Paragraph::new("").block(activity_block), chunks[1]);
}

fn render_footer(frame: &mut Frame, theme: &Theme, area: Rect) {
    let shortcuts = Line::from(vec![
        Span::styled(" [q / Esc] ", theme.footer_key),
        Span::styled("Quit", theme.footer_text),
    ]);

    let footer = Paragraph::new(shortcuts);
    frame.render_widget(footer, area);
}
