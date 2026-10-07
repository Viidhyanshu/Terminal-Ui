#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardView {
    GitHub,
    LeetCode,
    Spotify,
}

pub struct App {
    pub current_view: DashboardView,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_view: DashboardView::LeetCode,
            should_quit: false,
        }
    }

    pub fn toggle_view(&mut self) {
        self.current_view = match self.current_view {
            DashboardView::GitHub => DashboardView::LeetCode,
            DashboardView::LeetCode => DashboardView::Spotify,
            DashboardView::Spotify => DashboardView::GitHub,
        };
    }

    pub fn set_view(&mut self, view: DashboardView) {
        self.current_view = view;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
