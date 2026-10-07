use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{App, DashboardView};
use crate::ui::theme::Theme;

pub fn render_dashboard(frame: &mut Frame, app: &App) {
    let theme = Theme::default();
    let area = frame.area();

    // Divide screen into Header, Main Body, Footer
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(20),   // Main Dashboard Area
            Constraint::Length(1), // Footer
        ])
        .split(area);

    render_header(frame, app, &theme, vertical_chunks[0]);

    match app.current_view {
        DashboardView::GitHub => render_github_layout(frame, &theme, vertical_chunks[1]),
        DashboardView::LeetCode => render_leetcode_layout(frame, &theme, vertical_chunks[1]),
        DashboardView::Spotify => render_spotify_layout(frame, &theme, vertical_chunks[1]),
    }

    render_footer(frame, app, &theme, vertical_chunks[2]);
}

fn render_header(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let title_str = match app.current_view {
        DashboardView::GitHub => " GitHub Dashboard ",
        DashboardView::LeetCode => " LeetCode Dashboard ",
        DashboardView::Spotify => " Spotify Player ",
    };

    let title_line = Line::from(vec![
        Span::styled(title_str, theme.title),
        Span::styled("v0.1.0", theme.text_muted),
    ]);

    let title_widget = Paragraph::new(title_line)
        .block(header_block)
        .alignment(Alignment::Left);

    let status_line = Line::from(vec![
        Span::styled(
            if app.current_view == DashboardView::GitHub { "[1] GitHub" } else { " 1  GitHub" },
            if app.current_view == DashboardView::GitHub { theme.accent.add_modifier(Modifier::BOLD) } else { theme.text_muted },
        ),
        Span::styled("  │  ", theme.text_muted),
        Span::styled(
            if app.current_view == DashboardView::LeetCode { "[2] LeetCode" } else { " 2  LeetCode" },
            if app.current_view == DashboardView::LeetCode { theme.accent.add_modifier(Modifier::BOLD) } else { theme.text_muted },
        ),
        Span::styled("  │  ", theme.text_muted),
        Span::styled(
            if app.current_view == DashboardView::Spotify { "[3] Spotify" } else { " 3  Spotify" },
            if app.current_view == DashboardView::Spotify { theme.accent.add_modifier(Modifier::BOLD) } else { theme.text_muted },
        ),
        Span::styled("  │ Connected ", Style::default().fg(Color::Green)),
    ]);

    let status_widget = Paragraph::new(status_line).alignment(Alignment::Right);

    frame.render_widget(title_widget, area);

    // Overlay right-aligned view indicators
    let inner_header = Rect {
        x: area.x + area.width.saturating_sub(60),
        y: area.y + 1,
        width: 58.min(area.width),
        height: 1,
    };
    frame.render_widget(status_widget, inner_header);
}

/* ========================================================================= */
/*                          LEETCODE DASHBOARD LAYOUT                        */
/* ========================================================================= */

fn render_leetcode_layout(frame: &mut Frame, theme: &Theme, area: Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Top: Profile/Rating + Solved Breakdown
            Constraint::Length(8), // Middle: Submissions Heatmap
            Constraint::Min(9),    // Bottom: Recent Submissions & Topic Mastery
        ])
        .split(area);

    render_leetcode_top_section(frame, theme, sections[0]);
    render_leetcode_heatmap_section(frame, theme, sections[1]);
    render_leetcode_bottom_section(frame, theme, sections[2]);
}

fn render_leetcode_top_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(36), // Profile & Rating
            Constraint::Min(40),    // 4 Problems Solved Cards
        ])
        .split(area);

    // Profile & Ranking Block
    let profile_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Profile & Ranking ", theme.title));

    let profile_lines = vec![
        Line::from(vec![
            Span::styled("Username: ", theme.text_muted),
            Span::styled("--", theme.text_secondary),
        ]),
        Line::from(vec![
            Span::styled("Ranking:  ", theme.text_muted),
            Span::styled("--", theme.text_secondary),
        ]),
        Line::from(vec![
            Span::styled("Rating:   ", theme.text_muted),
            Span::styled("--", theme.text_secondary),
        ]),
        Line::from(vec![
            Span::styled("Badges:   ", theme.text_muted),
            Span::styled("--", theme.text_secondary),
        ]),
    ];

    let profile_widget = Paragraph::new(profile_lines).block(profile_block);
    frame.render_widget(profile_widget, chunks[0]);

    // 4 Solved Cards: Total, Easy, Medium, Hard
    let stat_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(chunks[1]);

    let stats_data = [
        ("Total Solved", "0", theme.text_primary),
        ("Easy", "0", Style::default().fg(Color::Green)),
        ("Medium", "0", Style::default().fg(Color::Yellow)),
        ("Hard", "0", Style::default().fg(Color::Red)),
    ];

    for (i, (title, value, style)) in stats_data.iter().enumerate() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme.border)
            .title(Span::styled(format!(" {} ", title), theme.title));

        let content = vec![
            Line::from(""),
            Line::from(Span::styled(*value, style.add_modifier(Modifier::BOLD))),
        ];

        let widget = Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(widget, stat_chunks[i]);
    }
}

fn render_leetcode_heatmap_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Submission Activity ", theme.title));

    frame.render_widget(Paragraph::new("").block(block), area);
}

fn render_leetcode_bottom_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55), // Recent Submissions
            Constraint::Percentage(45), // Topic Mastery
        ])
        .split(area);

    let sub_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Recent Submissions ", theme.title));

    frame.render_widget(Paragraph::new("").block(sub_block), chunks[0]);

    let skills_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Topic Mastery ", theme.title));

    frame.render_widget(Paragraph::new("").block(skills_block), chunks[1]);
}

/* ========================================================================= */
/*                          GITHUB DASHBOARD LAYOUT                          */
/* ========================================================================= */

fn render_github_layout(frame: &mut Frame, theme: &Theme, area: Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Top: Profile + 4 Stat Cards
            Constraint::Length(8), // Middle: Contribution Graph
            Constraint::Min(9),    // Bottom: Repositories & Recent Activity
        ])
        .split(area);

    render_github_top_section(frame, theme, sections[0]);
    render_github_heatmap_section(frame, theme, sections[1]);
    render_github_bottom_section(frame, theme, sections[2]);
}

fn render_github_top_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(36), // Profile Card
            Constraint::Min(40),    // 4 Stat Cards
        ])
        .split(area);

    // Profile Card
    let profile_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Profile ", theme.title));

    let profile_lines = vec![
        Line::from(vec![
            Span::styled("Name:     ", theme.text_muted),
            Span::styled("Viidhyanshu", theme.text_primary.add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Username: ", theme.text_muted),
            Span::styled("@Viidhyanshu", theme.accent),
        ]),
        Line::from(vec![
            Span::styled("Bio:      ", theme.text_muted),
            Span::styled("Rust & Systems Developer", theme.text_secondary),
        ]),
        Line::from(vec![
            Span::styled("Location: ", theme.text_muted),
            Span::styled("India", theme.text_secondary),
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

    let stats_data = [
        ("Repositories", "28", "+3 this month"),
        ("Total Stars", "142", "Top 5% dev"),
        ("Followers", "95", "+8 this week"),
        ("Following", "64", "Active network"),
    ];

    for (i, (title, value, sub)) in stats_data.iter().enumerate() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme.border)
            .title(Span::styled(format!(" {} ", title), theme.title));

        let content = vec![
            Line::from(""),
            Line::from(Span::styled(
                *value,
                theme.text_primary.add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(*sub, theme.text_muted)),
        ];

        let widget = Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(widget, stat_chunks[i]);
    }
}

fn render_github_heatmap_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Contributions (2026) ", theme.title));

    let l0 = Style::default().fg(Color::Rgb(35, 40, 48));
    let l1 = Style::default().fg(Color::Rgb(20, 75, 45));
    let l2 = Style::default().fg(Color::Rgb(35, 120, 65));
    let l3 = Style::default().fg(Color::Rgb(45, 160, 80));
    let l4 = Style::default().fg(Color::Rgb(60, 200, 100));

    let mut heatmap_lines = Vec::new();
    let day_labels = ["Mon ", "    ", "Wed ", "    ", "Fri ", "    ", "Sun "];

    let pattern: [[u8; 24]; 7] = [
        [0, 1, 2, 4, 3, 2, 1, 0, 3, 4, 2, 1, 4, 3, 2, 0, 1, 2, 4, 3, 2, 1, 4, 3],
        [1, 0, 3, 2, 1, 4, 2, 1, 0, 2, 4, 3, 1, 0, 3, 2, 4, 1, 0, 2, 3, 4, 2, 1],
        [2, 3, 4, 1, 0, 2, 3, 4, 1, 0, 3, 2, 4, 3, 1, 2, 0, 3, 4, 1, 2, 3, 4, 2],
        [0, 1, 2, 3, 4, 1, 0, 2, 3, 4, 1, 0, 2, 4, 3, 1, 2, 4, 3, 2, 1, 0, 3, 4],
        [3, 4, 1, 0, 2, 3, 4, 2, 1, 4, 3, 2, 0, 1, 4, 3, 2, 1, 0, 3, 4, 2, 1, 4],
        [1, 2, 0, 1, 3, 2, 0, 1, 2, 0, 1, 4, 2, 1, 0, 2, 3, 1, 0, 2, 1, 3, 2, 0],
        [0, 1, 2, 0, 1, 0, 2, 1, 0, 1, 2, 3, 1, 0, 1, 2, 0, 1, 2, 1, 0, 1, 2, 1],
    ];

    for row_idx in 0..7 {
        let mut spans = vec![
            Span::styled(day_labels[row_idx], theme.text_muted),
        ];

        for &val in pattern[row_idx].iter() {
            let style = match val {
                1 => l1,
                2 => l2,
                3 => l3,
                4 => l4,
                _ => l0,
            };
            spans.push(Span::styled("■ ", style));
        }

        heatmap_lines.push(Line::from(spans));
    }

    heatmap_lines.push(Line::from(vec![
        Span::styled("  Total: ", theme.text_muted),
        Span::styled("842 contributions in the last year", theme.text_secondary),
        Span::styled("  │  Streak: ", theme.text_muted),
        Span::styled("14 days", theme.accent),
        Span::styled("  │  Less ", theme.text_muted),
        Span::styled("■ ", l0),
        Span::styled("■ ", l1),
        Span::styled("■ ", l2),
        Span::styled("■ ", l3),
        Span::styled("■ ", l4),
        Span::styled("More", theme.text_muted),
    ]));

    let widget = Paragraph::new(heatmap_lines).block(block);
    frame.render_widget(widget, area);
}

fn render_github_bottom_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    // Top Repositories Panel
    let repos_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Top Repositories ", theme.title));

    let repos_lines = vec![
        Line::from(vec![
            Span::styled("Terminal-Ui", theme.text_primary.add_modifier(Modifier::BOLD)),
            Span::styled("  [Rust]  ", theme.accent),
            Span::styled("Stars: 142  ", theme.text_secondary),
            Span::styled("Forks: 24", theme.text_muted),
        ]),
        Line::from(Span::styled("  Developer dashboard built in Rust Ratatui", theme.text_muted)),
        Line::from(""),
        Line::from(vec![
            Span::styled("rust-algos", theme.text_primary.add_modifier(Modifier::BOLD)),
            Span::styled("   [Rust]  ", theme.accent),
            Span::styled("Stars: 89   ", theme.text_secondary),
            Span::styled("Forks: 12", theme.text_muted),
        ]),
        Line::from(Span::styled("  Competitive programming algorithms and data structures", theme.text_muted)),
        Line::from(""),
        Line::from(vec![
            Span::styled("dotfiles", theme.text_primary.add_modifier(Modifier::BOLD)),
            Span::styled("     [Lua]   ", theme.accent),
            Span::styled("Stars: 46   ", theme.text_secondary),
            Span::styled("Forks: 5", theme.text_muted),
        ]),
        Line::from(Span::styled("  Neovim, tmux and shell configuration", theme.text_muted)),
    ];

    let repos_widget = Paragraph::new(repos_lines).block(repos_block);
    frame.render_widget(repos_widget, chunks[0]);

    // Recent Activity Panel
    let activity_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Recent Activity ", theme.title));

    let activity_lines = vec![
        Line::from(vec![
            Span::styled("PUSH  ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("main -> Terminal-Ui ", theme.text_primary),
            Span::styled("(12m ago)", theme.text_muted),
        ]),
        Line::from(Span::styled("  refactor: clean layout styling and typography", theme.text_muted)),
        Line::from(""),
        Line::from(vec![
            Span::styled("STAR  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("starred ratatui/ratatui ", theme.text_primary),
            Span::styled("(2h ago)", theme.text_muted),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("PR    ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("#4 Add Spotify module ", theme.text_primary),
            Span::styled("(5h ago)", theme.text_muted),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("TAG   ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::styled("released v0.1.0 ", theme.text_primary),
            Span::styled("(Yesterday)", theme.text_muted),
        ]),
    ];

    let activity_widget = Paragraph::new(activity_lines).block(activity_block);
    frame.render_widget(activity_widget, chunks[1]);
}

fn render_footer(frame: &mut Frame, _app: &App, theme: &Theme, area: Rect) {
    let shortcuts = Line::from(vec![
        Span::styled("[1] ", theme.footer_key),
        Span::styled("GitHub  ", theme.footer_text),
        Span::styled("│  ", theme.text_muted),
        Span::styled("[2] ", theme.footer_key),
        Span::styled("LeetCode  ", theme.footer_text),
        Span::styled("│  ", theme.text_muted),
        Span::styled("[3] ", theme.footer_key),
        Span::styled("Spotify  ", theme.footer_text),
        Span::styled("│  ", theme.text_muted),
        Span::styled("[Tab] ", theme.footer_key),
        Span::styled("Switch View  ", theme.footer_text),
        Span::styled("│  ", theme.text_muted),
        Span::styled("[q] ", theme.footer_key),
        Span::styled("Quit", theme.footer_text),
    ]);

    let footer = Paragraph::new(shortcuts);
    frame.render_widget(footer, area);
}

/* ========================================================================= */
/*                          SPOTIFY DASHBOARD LAYOUT                         */
/* ========================================================================= */

fn render_spotify_layout(frame: &mut Frame, theme: &Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Spotify Music Player ", theme.title));

    let placeholder = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled("♫ Spotify Layout Initialized", theme.text_primary.add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(Span::styled("Preparing layout sections: Now Playing bar, Library & Playlists, Queue, and Audio Visualizer...", theme.text_muted)),
    ])
    .alignment(Alignment::Center)
    .block(block);

    frame.render_widget(placeholder, area);
}

