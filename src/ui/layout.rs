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

    let (title_str, title_style) = match app.current_view {
        DashboardView::GitHub => (" GitHub Dashboard ", theme.title),
        DashboardView::LeetCode => (" LeetCode Dashboard ", theme.title),
        DashboardView::Spotify => (" Spotify Player ", theme.spotify_green.add_modifier(Modifier::BOLD)),
    };

    let title_line = Line::from(vec![
        Span::styled(title_str, title_style),
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
            if app.current_view == DashboardView::Spotify { theme.spotify_green.add_modifier(Modifier::BOLD) } else { theme.text_muted },
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
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Top: Now Playing & Playback Controls Bar
            Constraint::Min(10),   // Middle: Playlists & Queue
            Constraint::Length(8), // Bottom: Visualizer & Devices
        ])
        .split(area);

    render_spotify_top_section(frame, theme, sections[0]);

    // Middle section: Left (Library & Playlists) and Right (Track Queue)
    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(36), // Left: Your Library & Playlists (Step 3)
            Constraint::Min(45),    // Right: Current Queue & Tracklist (Step 4)
        ])
        .split(sections[1]);

    render_spotify_playlists_section(frame, theme, mid_chunks[0]);
    render_spotify_queue_section(frame, theme, mid_chunks[1]);

    render_spotify_bottom_section(frame, theme, sections[2]);
}

fn render_spotify_top_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let spotify_green = theme.spotify_green;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(42), // Track Info & Metadata
            Constraint::Min(40),    // Playback Controls & Progress Bar
        ])
        .split(area);

    // Track Info Block
    let track_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Now Playing ", spotify_green.add_modifier(Modifier::BOLD)));

    let track_lines = vec![
        Line::from(vec![
            Span::styled("Track:   ", theme.text_muted),
            Span::styled("Starboy ", theme.text_primary.add_modifier(Modifier::BOLD)),
            Span::styled("• Explicit", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled("Artist:  ", theme.text_muted),
            Span::styled("The Weeknd, Daft Punk", theme.text_secondary),
        ]),
        Line::from(vec![
            Span::styled("Album:   ", theme.text_muted),
            Span::styled("Starboy (Deluxe)", theme.text_secondary),
        ]),
        Line::from(vec![
            Span::styled("From:    ", theme.text_muted),
            Span::styled("Coding Focus Beats", theme.accent),
        ]),
    ];

    let track_widget = Paragraph::new(track_lines).block(track_block);
    frame.render_widget(track_widget, chunks[0]);

    // Playback Controls Block
    let controls_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Playback Controls ", theme.title));

    let controls_line = Line::from(vec![
        Span::styled("[🔀 Shuffle] ", spotify_green),
        Span::styled("  ⏮ Prev  ", theme.text_secondary),
        Span::styled("  ▶ PLAYING  ", spotify_green.add_modifier(Modifier::BOLD)),
        Span::styled("  ⏭ Next  ", theme.text_secondary),
        Span::styled("  [🔁 Repeat: All] ", spotify_green),
    ]);

    let progress_bar_line = Line::from(vec![
        Span::styled("01:48 ", theme.text_secondary),
        Span::styled("━━━━━━━━━━━━━━━━━━━●", spotify_green),
        Span::styled("────────────────────", theme.text_muted),
        Span::styled(" 03:50", theme.text_muted),
    ]);

    let meta_line = Line::from(vec![
        Span::styled("Volume: ", theme.text_muted),
        Span::styled("75% ", theme.text_primary),
        Span::styled("[■■■■■■■□□□]", spotify_green),
        Span::styled("  │  Device: ", theme.text_muted),
        Span::styled("MacBook Pro", theme.text_secondary),
        Span::styled("  │  Audio: ", theme.text_muted),
        Span::styled("320 kbps (Lossless)", spotify_green),
    ]);

    let controls_widget = Paragraph::new(vec![
        controls_line,
        Line::from(""),
        progress_bar_line,
        meta_line,
    ])
    .alignment(Alignment::Center)
    .block(controls_block);

    frame.render_widget(controls_widget, chunks[1]);
}

fn render_spotify_playlists_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let spotify_green = theme.spotify_green;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Your Library ", spotify_green.add_modifier(Modifier::BOLD)));

    let playlists = vec![
        Line::from(vec![
            Span::styled(" ♥ ", spotify_green.add_modifier(Modifier::BOLD)),
            Span::styled("Liked Songs", theme.text_primary.add_modifier(Modifier::BOLD)),
            Span::styled("       342 tracks", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled(" 📻 ", theme.accent),
            Span::styled("Episodes & Shows", theme.text_secondary),
            Span::styled("   14 saved", theme.text_muted),
        ]),
        Line::from(Span::styled(" ─────────────────────────────", theme.border)),
        Line::from(vec![
            Span::styled(" ▶ ", spotify_green),
            Span::styled("Coding Focus Beats", spotify_green.add_modifier(Modifier::BOLD)),
            Span::styled("   128", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled("   ", theme.text_muted),
            Span::styled("Synthwave & Night Drive", theme.text_primary),
            Span::styled("  84", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled("   ", theme.text_muted),
            Span::styled("Lo-Fi Chillhop Beats", theme.text_primary),
            Span::styled("     210", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled("   ", theme.text_muted),
            Span::styled("Deep Work & Rust Focus", theme.text_primary),
            Span::styled("   65", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled("   ", theme.text_muted),
            Span::styled("Discover Weekly", theme.accent),
            Span::styled("          30", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled("   ", theme.text_muted),
            Span::styled("Release Radar", theme.accent),
            Span::styled("            30", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled("   ", theme.text_muted),
            Span::styled("Ambient Electronic", theme.text_primary),
            Span::styled("       92", theme.text_muted),
        ]),
        Line::from(""),
        Line::from(Span::styled(" 📁 8 Playlists  •  961 Tracks", theme.text_muted)),
    ];

    let widget = Paragraph::new(playlists).block(block);
    frame.render_widget(widget, area);
}

fn render_spotify_queue_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let spotify_green = theme.spotify_green;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Current Queue & Tracklist ", spotify_green.add_modifier(Modifier::BOLD)));

    let show_album = area.width >= 70;

    let mut queue_lines = Vec::new();

    if show_album {
        queue_lines.push(Line::from(vec![
            Span::styled(" #   ", theme.text_muted),
            Span::styled("TITLE                         ", theme.text_muted),
            Span::styled("ARTIST              ", theme.text_muted),
            Span::styled("ALBUM                 ", theme.text_muted),
            Span::styled("TIME", theme.text_muted),
        ]));
        queue_lines.push(Line::from(Span::styled(
            " ────────────────────────────────────────────────────────────────────────",
            theme.border,
        )));
    } else {
        queue_lines.push(Line::from(vec![
            Span::styled(" #   ", theme.text_muted),
            Span::styled("TITLE                   ", theme.text_muted),
            Span::styled("ARTIST              ", theme.text_muted),
            Span::styled("TIME", theme.text_muted),
        ]));
        queue_lines.push(Line::from(Span::styled(
            " ────────────────────────────────────────────────────────",
            theme.border,
        )));
    }

    struct TrackEntry<'a> {
        num: &'a str,
        title: &'a str,
        artist: &'a str,
        album: &'a str,
        duration: &'a str,
        is_playing: bool,
    }

    let tracks = [
        TrackEntry { num: "1", title: "Starboy", artist: "The Weeknd, Daft Punk", album: "Starboy (Deluxe)", duration: "3:50", is_playing: true },
        TrackEntry { num: "2", title: "Midnight City", artist: "M83", album: "Hurry Up, We're...", duration: "4:03", is_playing: false },
        TrackEntry { num: "3", title: "Resonance", artist: "HOME", album: "Odyssey", duration: "3:32", is_playing: false },
        TrackEntry { num: "4", title: "After Dark", artist: "Mr.Kitty", album: "Time", duration: "4:17", is_playing: false },
        TrackEntry { num: "5", title: "Nightcall", artist: "Kavinsky", album: "OutRun", duration: "4:19", is_playing: false },
        TrackEntry { num: "6", title: "Tech Noir", artist: "GUNSHIP", album: "GUNSHIP", duration: "4:57", is_playing: false },
        TrackEntry { num: "7", title: "Genesis", artist: "Justice", album: "Cross", duration: "3:54", is_playing: false },
        TrackEntry { num: "8", title: "Days of Thunder", artist: "The Midnight", album: "Days of Thunder", duration: "5:24", is_playing: false },
    ];

    for track in tracks.iter() {
        let (num_style, title_style, artist_style, duration_style) = if track.is_playing {
            (
                spotify_green.add_modifier(Modifier::BOLD),
                spotify_green.add_modifier(Modifier::BOLD),
                spotify_green,
                spotify_green,
            )
        } else {
            (
                theme.text_muted,
                theme.text_primary,
                theme.text_secondary,
                theme.text_muted,
            )
        };

        let icon = if track.is_playing { "▶ " } else { "  " };

        if show_album {
            queue_lines.push(Line::from(vec![
                Span::styled(format!("{}{:<2} ", icon, track.num), num_style),
                Span::styled(format!("{:<30}", track.title), title_style),
                Span::styled(format!("{:<20}", track.artist), artist_style),
                Span::styled(format!("{:<22}", track.album), theme.text_muted),
                Span::styled(track.duration, duration_style),
            ]));
        } else {
            queue_lines.push(Line::from(vec![
                Span::styled(format!("{}{:<2} ", icon, track.num), num_style),
                Span::styled(format!("{:<24}", track.title), title_style),
                Span::styled(format!("{:<20}", track.artist), artist_style),
                Span::styled(track.duration, duration_style),
            ]));
        }
    }

    queue_lines.push(Line::from(""));
    queue_lines.push(Line::from(vec![
        Span::styled(" Queue Status: ", theme.text_muted),
        Span::styled("8 songs queued", theme.text_primary),
        Span::styled("  │  Total Duration: ", theme.text_muted),
        Span::styled("34 min 16 sec", theme.text_secondary),
        Span::styled("  │  Autoplay: ", theme.text_muted),
        Span::styled("ON", spotify_green),
    ]));

    let widget = Paragraph::new(queue_lines).block(block);
    frame.render_widget(widget, area);
}

fn render_spotify_bottom_section(frame: &mut Frame, theme: &Theme, area: Rect) {
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55), // Audio Spectrum Visualizer
            Constraint::Percentage(45), // Spotify Connect & Output Devices
        ])
        .split(area);

    // 1. Audio Spectrum Visualizer Block
    let spec_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Audio Spectrum Visualizer ", theme.spotify_green.add_modifier(Modifier::BOLD)));

    let g1 = theme.spotify_green;
    let g2 = Style::default().fg(Color::Rgb(60, 230, 120));
    let g3 = Style::default().fg(Color::Rgb(40, 160, 80));

    let spec_lines = vec![
        Line::from(vec![
            Span::styled(" Peak: ", theme.text_muted),
            Span::styled(" ▂ ▄ ▅ ▆ ▇ █ ▇ ▆ ▅ ▄ ▃ ▅ ▆ ▇ █ ▇ ▆ ▅ ▄ ▃ ▂   ▂ ▃ ▄ ▅ ▆ ▇ █ ▇ ▆ ▅", g1),
        ]),
        Line::from(vec![
            Span::styled(" Live: ", theme.text_muted),
            Span::styled(" ▃ ▅ ▇ █ ▇ ▅ ▃ ▄ ▆ ▇ █ ▇ ▅ ▃ ▂ ▃ ▅ ▇ █ ▇ ▅ ▃ ▄ ▆ ▇ █ ▇ ▅ ▃ ▂ ▄ ▆", g2),
        ]),
        Line::from(vec![
            Span::styled(" RMS:  ", theme.text_muted),
            Span::styled(" ▂ ▃ ▄ ▅ ▆ ▅ ▄ ▃ ▄ ▅ ▆ ▅ ▄ ▃ ▂ ▂ ▃ ▄ ▅ ▆ ▅ ▄ ▃ ▄ ▅ ▆ ▅ ▄ ▃ ▂ ▂ ▃ ▄", g3),
        ]),
        Line::from(vec![
            Span::styled(" Freq: ", theme.text_muted),
            Span::styled(" [20Hz]       [100Hz]       [500Hz]       [2.5kHz]       [10kHz]     [20kHz]", theme.text_muted),
        ]),
        Line::from(vec![
            Span::styled(" Mode: ", theme.text_muted),
            Span::styled("Realtime FFT (44.1 kHz • 16-bit)  │  Dynamic Range: 96 dB", theme.text_secondary),
        ]),
    ];

    let spec_widget = Paragraph::new(spec_lines).block(spec_block);
    frame.render_widget(spec_widget, bottom_chunks[0]);

    // 2. Spotify Connect & Devices Block
    let dev_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Spotify Connect & Devices ", theme.spotify_green.add_modifier(Modifier::BOLD)));

    let dev_lines = vec![
        Line::from(vec![
            Span::styled(" ● Active:  ", theme.spotify_green),
            Span::styled("MacBook Pro Speakers", theme.text_primary.add_modifier(Modifier::BOLD)),
            Span::styled("  [Vol: 75%]", theme.text_secondary),
        ]),
        Line::from(vec![
            Span::styled(" ○ Device:  ", theme.text_muted),
            Span::styled("Living Room Echo (Spotify Connect)", theme.text_secondary),
        ]),
        Line::from(vec![
            Span::styled(" ○ Device:  ", theme.text_muted),
            Span::styled("Sony WH-1000XM5 (Bluetooth)", theme.text_secondary),
        ]),
        Line::from(Span::styled(" ──────────────────────────────────────────", theme.border)),
        Line::from(vec![
            Span::styled(" Quality: ", theme.text_muted),
            Span::styled("Very High (320 kbps)", theme.spotify_green),
            Span::styled(" │ Normalize: ", theme.text_muted),
            Span::styled("-14 LUFS", theme.text_secondary),
        ]),
    ];

    let dev_widget = Paragraph::new(dev_lines).block(dev_block);
    frame.render_widget(dev_widget, bottom_chunks[1]);
}




