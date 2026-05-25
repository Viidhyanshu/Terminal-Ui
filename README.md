# 🖥️ Terminal-Ui: Developer Dashboard

A high-performance, aesthetically pleasing terminal dashboard (TUI) built in Rust. It consolidates your GitHub statistics, LeetCode performance, and Spotify music player directly inside your terminal, using `ratatui` and `crossterm`.

---

## 🚀 Key Modules & Features

### 1. 🐙 GitHub Dashboard
Stay on top of your development workflow with a real-time GitHub integration powered by the **GraphQL API** and **octocrab**.
* **Contribution Graph**: A terminal-rendered grid of your daily contributions.
* **Recent Commits**: Stream of recent commits across your active repositories.
* **Issues & PRs**: Live overview of open issues and pull requests needing your attention.
* **Notifications**: Get alerted to mentions, reviews, and notifications directly in your terminal.
* **Streaks**: Gamify your coding habits with streak tracking metrics.

### 2. 💡 LeetCode Tracker
Monitor your coding prep and competitive programming statistics seamlessly.
* **Solved Count**: Breakdown of problems solved.
* **Contest Rating**: View your contest performance history and rating trajectory.
* **Heatmap**: Visual map of daily submissions.
* **Recent Submissions**: Feed of your latest code submissions and their status (Accepted, WA, TLE, etc.).
* **Difficulty Breakdown**: Color-coded visualization of Easy, Medium, and Hard solved problems.
* **Streak Tracker**: Keep your daily solving streak alive.

### 3. 🎵 Spotify Player
Control your music and visualize your sounds without leaving your workspace.
* **Now Playing**: Gorgeous player showing the active song, artist, album art metadata, and progress bar.
* **Playlists**: Browse and switch between your favorite playlists.
* **Lyrics**: Sync and scroll through lyrics for the currently playing track.
* **Queue**: View and manage upcoming tracks.
* **Volume**: Fine-tune volume levels directly.
* **Visualizer**: Dynamic terminal audio visualizer corresponding to track aesthetics.

---

## 📁 Folder Structure

The project code is modularized as follows:

```text
src/
├── main.rs            # Application entry point and terminal initialization
├── app.rs             # Application event loop, state management, and orchestration
├── ui/                # UI layout, custom rendering, and themes
│   ├── layout.rs      # Dashboard grid layout and grid partitions
│   ├── widgets.rs     # Reusable custom UI components/widgets
│   └── theme.rs       # Colors, borders, and styles matching modern TUI guidelines
├── modules/           # Module-specific logic, fetching, and display
│   ├── github/        # GitHub-specific data models and controllers
│   ├── leetcode/      # LeetCode scraping/API handlers and visualizations
│   └── spotify/       # Spotify Web API player controls and visualizer engine
├── services/          # Core backend infrastructure for the TUI
│   ├── api.rs         # Consolidated API manager and client pool
│   └── cache.rs       # Local caching system for optimal offline & fast render support
├── state/             # Shared state, config storage, and global state machines
└── utils/             # Helper utilities, date/time formatting, and math helper functions
```

---

## 🛠️ Setup & Installation

### Prerequisites
* **Rust**: Ensure you have `rustc` and `cargo` installed. Install from [rustup.rs](https://rustup.rs).
* **API Credentials**:
  * **GitHub**: Personal Access Token (PAT) with `read:user`, `repo`, and `notifications` scopes.
  * **Spotify**: Spotify Developer Credentials (`client_id` and `client_secret`).
  * **LeetCode**: LeetCode session cookies or username for public stats tracking.

### Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Viidhyanshu/Terminal-Ui.git
   cd Terminal-Ui
   ```

2. **Configure environment variables:**
   Create a `.env` file in the root directory:
   ```env
   GITHUB_TOKEN=your_github_token
   SPOTIFY_CLIENT_ID=your_spotify_client_id
   SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
   LEETCODE_USERNAME=your_leetcode_username
   ```

3. **Run the application:**
   ```bash
   cargo run
   ```

---

## 🎨 Design Philosophy

* **Rich Aesthetics**: Utilizing curated HSL-tailored colors, sleek dark modes, and crisp borders.
* **Dynamic Design**: Interactive hover-like states, micro-animations, and fluid transitions.
* **Speed**: Minimal latency UI updates with active local caching for instant layouts on boot.

---

## 📄 License

This project is licensed under the MIT License. See the `LICENSE` file for details.
