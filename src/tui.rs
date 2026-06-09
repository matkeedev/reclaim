//! The interactive terminal UI: browse hits, toggle them, clean.

use std::io::{self, Stdout};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, ExecutableCommand};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::format;
use crate::scanner::Hit;

/// What the user decided when they left the UI.
pub enum Exit {
    /// Clean these (already filtered to picked).
    Clean(Vec<Hit>),
    /// Leave without touching anything.
    Quit,
}

/// In-memory state for the running UI.
struct App {
    hits: Vec<Hit>,
    state: ListState,
}

impl App {
    fn new(hits: Vec<Hit>) -> Self {
        let mut state = ListState::default();
        if !hits.is_empty() {
            state.select(Some(0));
        }
        Self { hits, state }
    }

    fn cursor(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    fn move_by(&mut self, delta: isize) {
        if self.hits.is_empty() {
            return;
        }
        let len = self.hits.len() as isize;
        let next = (self.cursor() as isize + delta).rem_euclid(len);
        self.state.select(Some(next as usize));
    }

    fn toggle(&mut self) {
        let i = self.cursor();
        if let Some(hit) = self.hits.get_mut(i) {
            hit.picked = !hit.picked;
        }
    }

    fn toggle_all(&mut self) {
        let any = self.hits.iter().any(|h| !h.picked);
        for hit in &mut self.hits {
            hit.picked = any;
        }
    }

    fn picked_size(&self) -> u64 {
        self.hits.iter().filter(|h| h.picked).map(|h| h.size).sum()
    }

    fn total_size(&self) -> u64 {
        self.hits.iter().map(|h| h.size).sum()
    }

    fn picked_count(&self) -> usize {
        self.hits.iter().filter(|h| h.picked).count()
    }
}

/// Run the UI to completion, restoring the terminal afterwards.
pub fn run(hits: Vec<Hit>) -> Result<Exit> {
    let mut term = setup()?;
    let mut app = App::new(hits);

    let result = event_loop(&mut term, &mut app);

    restore(&mut term)?;
    result
}

fn event_loop(term: &mut Term, app: &mut App) -> Result<Exit> {
    loop {
        term.draw(|f| draw(f, app))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(Exit::Quit),
            KeyCode::Down | KeyCode::Char('j') => app.move_by(1),
            KeyCode::Up | KeyCode::Char('k') => app.move_by(-1),
            KeyCode::Char(' ') => app.toggle(),
            KeyCode::Char('a') => app.toggle_all(),
            KeyCode::Enter => {
                let picked = app.hits.iter().filter(|h| h.picked).cloned().collect();
                return Ok(Exit::Clean(picked));
            }
            _ => {}
        }
    }
}

fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_header(f, app, chunks[0]);
    draw_list(f, app, chunks[1]);
    draw_footer(f, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(vec![
        Span::styled("  reclaim  ", Style::new().fg(Color::Black).bg(Color::Cyan)),
        Span::raw("  "),
        Span::raw(format!("{} dirs found", app.hits.len())),
        Span::raw("   "),
        Span::styled(
            format!("{} selected", format::bytes(app.picked_size())),
            Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("  ({} dirs)", app.picked_count())),
        Span::raw(format!("  /  {} total", format::bytes(app.total_size()))),
    ]);

    let block = Block::default().borders(Borders::ALL);
    f.render_widget(Paragraph::new(line).block(block), area);
}

fn draw_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.hits.iter().map(row).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::new().bg(Color::DarkGray))
        .highlight_symbol("> ");

    let mut state = app.state.clone();
    f.render_stateful_widget(list, area, &mut state);
}

fn row(hit: &Hit) -> ListItem<'_> {
    let mark = if hit.picked { "[x]" } else { "[ ]" };
    let mark_style = if hit.picked {
        Style::new().fg(Color::Green)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let line = Line::from(vec![
        Span::styled(format!(" {mark} "), mark_style),
        Span::styled(
            format!("{:>9}  ", format::bytes(hit.size)),
            Style::new().fg(Color::Yellow),
        ),
        Span::styled(format!("{:<8}", hit.kind), Style::new().fg(Color::Cyan)),
        Span::raw(hit.path.display().to_string()),
    ]);

    ListItem::new(line)
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let keys = [
        ("up/dn", "move"),
        ("space", "toggle"),
        ("a", "all/none"),
        ("enter", "clean"),
        ("q", "quit"),
    ];

    let mut spans = vec![Span::raw(" ")];
    for (key, label) in keys {
        spans.push(Span::styled(
            format!(" {key} "),
            Style::new().fg(Color::Black).bg(Color::Gray),
        ));
        spans.push(Span::raw(format!(" {label}   ")));
    }

    let block = Block::default().borders(Borders::ALL);
    f.render_widget(Paragraph::new(Line::from(spans)).block(block), area);
}

// --- terminal plumbing ---------------------------------------------------

type Term = Terminal<CrosstermBackend<Stdout>>;

fn setup() -> Result<Term> {
    enable_raw_mode()?;
    let mut out = io::stdout();
    out.execute(EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(out))?)
}

fn restore(term: &mut Term) -> Result<()> {
    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen)?;
    term.show_cursor()?;
    Ok(())
}
