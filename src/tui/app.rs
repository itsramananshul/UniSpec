use crate::tui::platypus::PLATYPUS_FRAMES;
use crate::tui::state::{AppState, NavState, TopicNode};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum PlatypusState {
    Idle,
    Happy,
    Sad,
    Love,
    Searching,
    Working,
    Celebrating,
}

impl PlatypusState {
    fn to_index(&self) -> usize {
        match self {
            PlatypusState::Idle => 0,
            PlatypusState::Happy => 1,
            PlatypusState::Sad => 2,
            PlatypusState::Love => 3,
            PlatypusState::Searching => 4,
            PlatypusState::Working => 5,
            PlatypusState::Celebrating => 6,
        }
    }
}

pub struct App {
    pub state: AppState,
    pub should_exit: bool,
    pub last_refresh: Instant,
    pub frame: usize,
    pub list_state: ListState,
    pub history: Vec<(NavState, Vec<TopicNode>)>,
    pub current_area: Option<String>,
    pub message: Option<String>,
    pub input_mode: bool,
    pub input_buffer: String,
    pub input_prompt: String,
    pub pending_args: Vec<String>,
    pub message_timer: Option<Instant>,
    pub find_paths: Vec<String>,
    pub saved_topics: Vec<TopicNode>,
    pub saved_nav_state: Option<NavState>,
    pub pending_link_path: Option<String>,
    pub pending_link_topic: Option<String>,
    pub platypus_state: PlatypusState,
    pub animation_step: usize,
    pub expression_timer: Option<Instant>,
    pub platypus_enabled: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(App {
            state: AppState::new()?,
            should_exit: false,
            last_refresh: Instant::now(),
            frame: 0,
            list_state: ListState::default(),
            history: vec![],
            current_area: None,
            message: None,
            input_mode: false,
            input_buffer: String::new(),
            input_prompt: String::new(),
            pending_args: vec![],
            message_timer: None,
            find_paths: vec![],
            saved_topics: vec![],
            saved_nav_state: None,
            pending_link_path: None,
            pending_link_topic: None,
            platypus_state: PlatypusState::Idle,
            animation_step: 0,
            expression_timer: None,
            platypus_enabled: true,
        })
    }

    fn get_platypus_frame(&self, step: usize) -> String {
        let frame_index = step % 4;
        let state_index = self.platypus_state.to_index();
        PLATYPUS_FRAMES[state_index][frame_index].to_string()
    }

    fn trigger_expression(&mut self, state: PlatypusState, duration_secs: u64) {
        self.platypus_state = state;
        self.animation_step = 0;
        self.expression_timer = Some(Instant::now() + Duration::from_secs(duration_secs));
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = self.setup_terminal()?;
        self.list_state.select(Some(0));

        while !self.should_exit {
            if let Some(timer) = self.message_timer {
                if timer.elapsed() >= Duration::from_secs(3) {
                    self.message = None;
                    self.message_timer = None;
                }
            }
            if self.last_refresh.elapsed() >= Duration::from_millis(500) {
                self.frame += 1;
                if let NavState::TopicList(_) = self.state.nav_state {
                    self.state.update_status();
                    if self.expression_timer.is_none() {
                        if let Some(selected_idx) = self.list_state.selected() {
                            if let Some(topic) = self.state.topics.get(selected_idx) {
                                if topic.tasks_in_progress > 0 {
                                    self.platypus_state = PlatypusState::Working;
                                } else if topic.tasks_total > 0
                                    && topic.tasks_completed == topic.tasks_total
                                {
                                    self.platypus_state = PlatypusState::Celebrating;
                                } else {
                                    self.platypus_state = PlatypusState::Idle;
                                }
                            }
                        }
                    }
                }
                self.last_refresh = Instant::now();
            }

            // Check expression timer
            if let Some(timer) = self.expression_timer {
                if Instant::now() >= timer {
                    self.platypus_state = PlatypusState::Idle;
                    self.expression_timer = None;
                    self.animation_step = 0;
                } else {
                    // Animate every 200ms
                    if self.frame % 2 == 0 {
                        self.animation_step += 1;
                    }
                }
            }

            terminal.draw(|f| {
                let chunks = if self.platypus_enabled {
                    Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Length(7),
                            Constraint::Min(1),
                            Constraint::Length(3),
                        ])
                        .split(f.size())
                } else {
                    Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(3),
                        ])
                        .split(f.size())
                };

                let area_name = self.current_area.as_ref().map(|a| a.clone()).unwrap_or_else(|| "None".to_string());
                let version = crate::version::VERSION;
                let mode_name = crate::agent::current_mode().unwrap_or_else(|_| "unknown".to_string());
                let platypus_status = if self.platypus_enabled { "" } else { " | Paddy: OFF" };

                let status_text = match &self.state.nav_state {
                    NavState::FindResults { paths, topic } => {
                        format!("UniSpec v{} | Mode: {} | Topic: {} | Links: {}{}", version, mode_name, topic, paths.len(), platypus_status)
                    }
                    _ => {
                        format!("UniSpec v{} | Mode: {} | Area: {} | Topics: {}{}", version, mode_name, area_name, self.state.topics.len(), platypus_status)
                    }
                };
                let status_widget = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL));
                f.render_widget(status_widget, chunks[0]);

                if self.platypus_enabled {
                    let platypus = self.get_platypus_frame(self.animation_step);
                    let platypus_widget = Paragraph::new(platypus.trim());
                    f.render_widget(platypus_widget, chunks[1]);
                }

                match &self.state.nav_state {
                    NavState::AreaSelection => {
                        let areas = self.state.load_areas().unwrap_or_default();
                        let items: Vec<ListItem> = areas.iter().enumerate().map(|(i, a)| {
                            let style = if Some(i) == self.list_state.selected() { Style::default().fg(Color::Yellow) } else { Style::default() };
                            ListItem::new(format!(">> {}", a.clone())).style(style)
                        }).collect();
                        let list = List::new(items).block(Block::default().title("Select Area").borders(Borders::ALL));
                        f.render_stateful_widget(list, if self.platypus_enabled { chunks[2] } else { chunks[1] }, &mut self.list_state);
                    }
                    NavState::FindResults { topic, paths } => {
                        let items: Vec<ListItem> = if paths.is_empty() {
                            vec![ListItem::new("(no files linked - press n to add)".to_string()).style(Style::default().fg(Color::DarkGray))]
                        } else {
                            paths.iter().enumerate().map(|(i, p)| {
                                let style = if Some(i) == self.list_state.selected() { Style::default().fg(Color::Yellow) } else { Style::default() };
                                ListItem::new(p.clone()).style(style)
                            }).collect()
                        };
                        let title = format!("Files linked to: {}", topic);
                        let list = List::new(items).block(Block::default().title(title.as_str()).borders(Borders::ALL))
                            .highlight_symbol(">> ");
                        f.render_stateful_widget(list, if self.platypus_enabled { chunks[2] } else { chunks[1] }, &mut self.list_state);
                    }
                    _ => {
                        let items: Vec<ListItem> = self.state.topics.iter().enumerate().map(|(i, t)| {
                            let style = if Some(i) == self.list_state.selected() { Style::default().fg(Color::Yellow) } else { Style::default() };
                            let progress = if t.tasks_total > 0 { (t.tasks_completed * 100 / t.tasks_total) as u16 } else { 0 };
                            let bar_width = 20;
                            let filled = (progress as usize * bar_width) / 100;
                            let bar = format!("[{}{}]", "=".repeat(filled), " ".repeat(bar_width - filled));

                            let status_icon = if t.status == "complete" { "✅" } else if t.tasks_in_progress > 0 { "🚧" } else { "⏳" };
                            let animation = if t.tasks_in_progress > 0 {
                                if self.frame % 4 < 1 { "⠋" } else { "⠙" }
                            } else { "-" };

                            let arrow = if !t.children.is_empty() { " >" } else { "" };
                            let topic_display = format!("{:<25}", if t.topic.len() > 25 { format!("{}...", &t.topic[..22]) } else { t.topic.clone() });

                            ListItem::new(format!("{} {} {} {} ({}/{}){}", topic_display, status_icon, bar, animation, t.tasks_completed, t.tasks_total, arrow)).style(style)
                        }).collect();
                        let list = List::new(items).block(Block::default().title("Specs").borders(Borders::ALL))
                            .highlight_symbol(">> ");
                        f.render_stateful_widget(list, if self.platypus_enabled { chunks[2] } else { chunks[1] }, &mut self.list_state);
                    }
                }

                let help_text = if self.input_mode {
                    format!("{}: {}", self.input_prompt, self.input_buffer)
                } else {
                    self.message.clone().unwrap_or_else(|| format!(" 🡙 Move | 🡘  Navigate | ↵ Open | n: New | r: Remove | p: Push | f: Find | q: Quit"))
                };
                let help_widget = Paragraph::new(help_text).block(Block::default().borders(Borders::ALL));
                f.render_widget(help_widget, if self.platypus_enabled { chunks[3] } else { chunks[2] });
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_input(key.code);
                    }
                }
            }
        }

        Ok(())
    }

    fn setup_terminal(&self) -> Result<Terminal<CrosstermBackend<Stdout>>> {
        crossterm::terminal::enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::EnterAlternateScreen
        )?;
        terminal.hide_cursor()?;
        Ok(terminal)
    }

    fn handle_key_input(&mut self, key: KeyCode) {
        if self.input_mode {
            match key {
                KeyCode::Enter => {
                    let input = self.input_buffer.clone();
                    self.pending_args.push(input.clone());
                    self.input_buffer.clear();

                    if self.input_prompt == "Name for new spec?" {
                        if let Some(area) = &self.current_area {
                            let topic_name = self.pending_args[0].clone();

                            let full_path = match &self.state.nav_state {
                                NavState::TopicList(_) => topic_name.clone(),
                                NavState::NestedSpecs(ref path) => {
                                    let spec_dir = crate::fs::spec_dir();
                                    if let Ok(rel) = path.strip_prefix(spec_dir.join(area)) {
                                        let parent = rel
                                            .to_string_lossy()
                                            .trim_start_matches('/')
                                            .to_string();
                                        format!("{}/{}", parent, topic_name)
                                    } else {
                                        topic_name.clone()
                                    }
                                }
                                _ => topic_name.clone(),
                            };

                            match crate::commands::topic::run_new(&full_path, area) {
                                Ok(msg) => {
                                    self.message = Some(msg);
                                    self.trigger_expression(PlatypusState::Happy, 2);
                                }
                                Err(e) => self.message = Some(format!("Error: {}", e)),
                            }
                            self.state.update_status();
                            self.last_refresh = Instant::now();
                        }
                        self.message_timer = Some(Instant::now());
                        self.input_mode = false;
                        self.pending_args.clear();
                    } else if self.input_prompt == "Name for new area?" {
                        match crate::commands::area::run_add(&self.pending_args[0]) {
                            Ok(_) => {
                                self.message = Some("Success: Created area".to_string());
                                self.trigger_expression(PlatypusState::Happy, 2);
                            }
                            Err(e) => self.message = Some(format!("Error: {}", e)),
                        }
                        self.message_timer = Some(Instant::now());
                        self.input_mode = false;
                        self.pending_args.clear();
                    } else if self.input_prompt == "Target Area?" {
                        if let Some(topic) = self
                            .state
                            .topics
                            .get(self.list_state.selected().unwrap_or(0))
                        {
                            let source_area = self.current_area.as_deref();
                            match crate::commands::topic::run_push(
                                &topic.relative_path(),
                                &self.pending_args[0],
                                source_area,
                            ) {
                                Ok(msg) => {
                                    self.message = Some(msg);
                                    self.trigger_expression(PlatypusState::Love, 2);
                                }
                                Err(e) => self.message = Some(format!("Error: {}", e)),
                            }
                        }
                        self.message_timer = Some(Instant::now());
                        self.input_mode = false;
                        self.pending_args.clear();
                    } else if self.input_prompt.starts_with("Confirm remove") {
                        if self
                            .pending_args
                            .get(0)
                            .map(|s| s.to_lowercase() == "y")
                            .unwrap_or(false)
                        {
                            if self.input_prompt.contains("area") {
                                let area = self
                                    .current_area
                                    .as_ref()
                                    .map(|a| a.clone())
                                    .unwrap_or_else(|| {
                                        let areas = self.state.load_areas().unwrap_or_default();
                                        areas
                                            .get(self.list_state.selected().unwrap_or(0))
                                            .cloned()
                                            .unwrap_or_default()
                                    });
                                if !area.is_empty() {
                                    match crate::commands::area::run_remove(&area) {
                                        Ok(msg) => {
                                            self.message = Some(msg);
                                            self.trigger_expression(PlatypusState::Sad, 2);
                                            self.state.nav_state = NavState::AreaSelection;
                                            self.current_area = None;
                                            self.state.topics.clear();
                                            self.list_state.select(Some(0));
                                        }
                                        Err(e) => self.message = Some(format!("Error: {}", e)),
                                    }
                                }
                            } else {
                                if let Some(topic) = self
                                    .state
                                    .topics
                                    .get(self.list_state.selected().unwrap_or(0))
                                {
                                    match crate::commands::topic::run_delete(
                                        &topic.relative_path(),
                                        self.current_area.as_deref().unwrap_or("Working"),
                                        true,
                                    ) {
                                        Ok(msg) => {
                                            self.message = Some(msg);
                                            self.trigger_expression(PlatypusState::Sad, 2);
                                            self.state.update_status();
                                        }
                                        Err(e) => self.message = Some(format!("Error: {}", e)),
                                    }
                                }
                            }
                        }
                        self.message_timer = Some(Instant::now());
                        self.input_mode = false;
                        self.pending_args.clear();
                    } else if self.input_prompt.starts_with("Remove link '")
                        && self.input_prompt.ends_with("'? (y/N)")
                    {
                        let confirm = self
                            .pending_args
                            .first()
                            .map(|s| s.to_lowercase() == "y")
                            .unwrap_or(false);
                        if confirm {
                            let path = self.pending_link_path.clone().unwrap_or_default();
                            let topic = self.pending_link_topic.clone().unwrap_or_default();
                            if !path.is_empty() && !topic.is_empty() {
                                match crate::fs::index::remove_link(&topic, &path) {
                                    Ok(_) => {
                                        self.find_paths.retain(|p| p != &path);
                                        if self.find_paths.is_empty() {
                                            self.state.nav_state = self
                                                .saved_nav_state
                                                .take()
                                                .unwrap_or(NavState::AreaSelection);
                                            self.state.topics = self.saved_topics.clone();
                                            self.message = Some(
                                                "All links removed, returned to topics".to_string(),
                                            );
                                        } else {
                                            self.state.nav_state = NavState::FindResults {
                                                topic: topic.clone(),
                                                paths: self.find_paths.clone(),
                                            };
                                            self.message = Some(format!("Removed: {}", path));
                                        }
                                        self.list_state.select(Some(0));
                                        self.trigger_expression(PlatypusState::Sad, 2);
                                    }
                                    Err(e) => self.message = Some(format!("Error: {}", e)),
                                }
                                self.message_timer = Some(Instant::now());
                            }
                        }
                        self.input_mode = false;
                        self.pending_args.clear();
                        self.pending_link_path = None;
                        self.pending_link_topic = None;
                    } else if self.input_prompt.starts_with("Path to add to") {
                        let topic_name =
                            if let NavState::FindResults { topic, .. } = &self.state.nav_state {
                                topic.clone()
                            } else if let Some(topic) = self
                                .state
                                .topics
                                .get(self.list_state.selected().unwrap_or(0))
                            {
                                topic.relative_path()
                            } else {
                                String::new()
                            };

                        if !topic_name.is_empty() {
                            let path = &self.pending_args[0];
                            let area = self.current_area.as_deref().unwrap_or("Working");
                            let link_type = crate::commands::index::detect_type(path);
                            match crate::fs::index::add_link(&topic_name, area, path, &link_type) {
                                Ok(_) => {
                                    self.message = Some(format!("Linked: {}", path));
                                    self.trigger_expression(PlatypusState::Happy, 2);
                                    if let Ok(links) = crate::fs::index::find_by_topic(&topic_name)
                                    {
                                        let paths: Vec<String> =
                                            links.iter().map(|l| l.path.clone()).collect();
                                        self.find_paths = paths.clone();
                                        self.state.nav_state = NavState::FindResults {
                                            topic: topic_name,
                                            paths,
                                        };
                                        self.list_state.select(Some(0));
                                    }
                                }
                                Err(e) => self.message = Some(format!("Error: {}", e)),
                            }
                            self.message_timer = Some(Instant::now());
                        }
                        self.input_mode = false;
                        self.pending_args.clear();
                    }
                }
                KeyCode::Char(c) => self.input_buffer.push(c),
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.input_buffer.clear();
                }
                _ => {}
            }
            return;
        }

        // Handle Enter key - open directories/files
        if key == KeyCode::Enter {
            match &self.state.nav_state {
                NavState::FindResults { .. } => {
                    if let Some(path) = self
                        .find_paths
                        .get(self.list_state.selected().unwrap_or(0))
                        .cloned()
                    {
                        self.open_file(&path);
                    }
                }
                NavState::AreaSelection => {
                    let areas = self.state.load_areas().unwrap_or_default();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if let Some(area_name) = areas.get(selected) {
                        let area_path = crate::fs::spec_dir().join(area_name);
                        self.open_file(&area_path.to_string_lossy());
                    }
                }
                _ => {
                    if let Some(topic) = self
                        .state
                        .topics
                        .get(self.list_state.selected().unwrap_or(0))
                        .cloned()
                    {
                        self.open_file(&topic.path.to_string_lossy());
                    }
                }
            }
            return;
        }

        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_exit = true;
            }
            KeyCode::Down => {
                let len = match &self.state.nav_state {
                    NavState::AreaSelection => self.state.load_areas().unwrap_or_default().len(),
                    NavState::FindResults { .. } => self.find_paths.len(),
                    _ => self.state.topics.len(),
                };
                let i = self.list_state.selected().unwrap_or(0);
                if i < len.saturating_sub(1) {
                    self.list_state.select(Some(i + 1));
                }
            }
            KeyCode::Up => {
                let i = self.list_state.selected().unwrap_or(0);
                if i > 0 {
                    self.list_state.select(Some(i - 1));
                }
            }
            KeyCode::Left => {
                if let NavState::FindResults { .. } = &self.state.nav_state {
                    self.state.nav_state = self
                        .saved_nav_state
                        .take()
                        .unwrap_or(NavState::AreaSelection);
                    self.state.topics = self.saved_topics.clone();
                    self.find_paths.clear();
                    self.list_state.select(Some(0));
                } else if let Some((prev_state, prev_topics)) = self.history.pop() {
                    self.state.nav_state = prev_state;
                    self.state.topics = prev_topics;
                    self.list_state.select(Some(0));
                } else if let NavState::TopicList(_) | NavState::NestedSpecs(_) =
                    self.state.nav_state
                {
                    self.state.nav_state = NavState::AreaSelection;
                    self.state.topics = vec![];
                    self.current_area = None;
                    self.list_state.select(Some(0));
                }
            }
            KeyCode::Right => match &self.state.nav_state {
                NavState::AreaSelection => {
                    let areas = self.state.load_areas().unwrap_or_default();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if let Some(area_name) = areas.get(selected) {
                        let _ = self.state.load_topics_for_area(area_name);
                        self.current_area = Some(area_name.clone());
                        self.list_state.select(Some(0));
                    }
                }
                NavState::TopicList(_) | NavState::NestedSpecs(_) => {
                    if let Some(topic) = self
                        .state
                        .topics
                        .get(self.list_state.selected().unwrap_or(0))
                        .cloned()
                    {
                        if !topic.children.is_empty() {
                            self.history
                                .push((self.state.nav_state.clone(), self.state.topics.clone()));
                            self.state.nav_state = NavState::NestedSpecs(topic.path.clone());
                            self.state.topics = topic.children;
                            self.list_state.select(Some(0));
                        }
                    }
                }
                _ => {}
            },
            KeyCode::Char('n') => {
                self.input_mode = true;
                match &self.state.nav_state {
                    NavState::AreaSelection => {
                        self.input_prompt = "Name for new area?".to_string();
                    }
                    NavState::FindResults { topic, .. } => {
                        self.input_prompt = format!("Path to add to {}?", topic);
                    }
                    _ => {
                        self.input_prompt = "Name for new spec?".to_string();
                    }
                }
            }
            KeyCode::Char('r') => match &self.state.nav_state {
                NavState::AreaSelection => {
                    self.input_mode = true;
                    self.input_prompt = "Confirm remove area? (y/N)".to_string();
                }
                NavState::FindResults { topic, paths } => {
                    if let Some(path) = paths.get(self.list_state.selected().unwrap_or(0)).cloned()
                    {
                        self.input_mode = true;
                        self.input_prompt = format!("Remove link '{}'? (y/N)", path);
                        self.pending_link_path = Some(path);
                        self.pending_link_topic = Some(topic.clone());
                    }
                }
                _ => {
                    self.input_mode = true;
                    self.input_prompt = "Confirm remove spec? (y/N)".to_string();
                }
            },
            KeyCode::Char('p') => match &self.state.nav_state {
                NavState::AreaSelection => {
                    self.message = Some("Action not allowed: Select a topic first".to_string());
                    self.message_timer = Some(Instant::now());
                }
                NavState::FindResults { .. } => {
                    self.message = Some("Action not allowed in links view".to_string());
                    self.message_timer = Some(Instant::now());
                }
                _ => {
                    self.input_mode = true;
                    self.input_prompt = "Target Area?".to_string();
                }
            },
            KeyCode::Char('f') => {
                if let NavState::AreaSelection = &self.state.nav_state {
                    self.message = Some("Action not allowed: Select a topic first".to_string());
                    self.message_timer = Some(Instant::now());
                } else if let Some(topic) = self
                    .state
                    .topics
                    .get(self.list_state.selected().unwrap_or(0))
                {
                    let topic_name = topic.relative_path();
                    self.trigger_expression(PlatypusState::Searching, 3);
                    match crate::fs::index::find_by_topic(&topic_name) {
                        Ok(links) => {
                            let paths: Vec<String> = links.iter().map(|l| l.path.clone()).collect();
                            self.saved_topics = self.state.topics.clone();
                            self.saved_nav_state = Some(self.state.nav_state.clone());
                            self.find_paths = paths.clone();
                            self.state.nav_state = NavState::FindResults {
                                topic: topic_name,
                                paths,
                            };
                            self.list_state.select(Some(0));
                            if links.is_empty() {
                                self.message =
                                    Some(format!("No files linked. Press n to add a link."));
                            }
                        }
                        Err(e) => {
                            self.message = Some(format!("Error: {}", e));
                            self.message_timer = Some(Instant::now());
                        }
                    }
                }
            }
            KeyCode::Char('\\') => {
                self.platypus_enabled = !self.platypus_enabled;
                if let Err(e) = crate::fs::config::set_paddy_enabled(self.platypus_enabled) {
                    self.message = Some(format!("Failed to save: {}", e));
                } else if self.platypus_enabled {
                    self.message = Some("Platypus enabled!".to_string());
                    self.trigger_expression(crate::tui::app::PlatypusState::Happy, 2);
                } else {
                    self.message = Some("Platypus disabled.".to_string());
                    self.trigger_expression(crate::tui::app::PlatypusState::Sad, 2);
                }
                self.message_timer = Some(Instant::now());
            }
            _ => {}
        }
    }

    fn open_file(&mut self, path: &str) {
        let abs_path = std::path::Path::new(path);
        let final_path = if abs_path.is_absolute() {
            path.to_string()
        } else {
            std::env::current_dir()
                .map(|cwd| cwd.join(path))
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| path.to_string())
        };

        if let Err(e) = open::that(&final_path) {
            self.message = Some(format!("Failed to open: {}", e));
            self.message_timer = Some(Instant::now());
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
