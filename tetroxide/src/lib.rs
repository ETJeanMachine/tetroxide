pub mod tetroxide {
    use async_std::task;
    use crossterm::{
        cursor::{self, Hide, Show},
        event::{
            poll, read, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, ModifierKeyCode,
        },
        execute,
        style::Print,
        terminal::{
            self, disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        Result,
    };
    use std::{
        fmt::format,
        io::{self, stdout, Write},
        time::{Duration, Instant},
        vec,
    };
    use tetris::tetris::Tetris;
    use tui::{
        backend::{Backend, CrosstermBackend},
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{Color, Style},
        text::{Span, Spans, Text},
        widgets::{Block, BorderType, Borders, Paragraph},
        Frame, Terminal,
    };
    use tui_input::backend::crossterm as backend;
    use tui_input::backend::crossterm::EventHandler;
    use tui_input::Input;

    enum Inputs {
        RotateCcw,
        RotateCw,
        HardDrop,
        Drop,
        Left,
        Right,
    }

    pub struct Game {
        tetris: Tetris,
    }

    impl Default for Game {
        fn default() -> Self {
            Self::new()
        }
    }

    fn get_style(tet: u8) -> Style {
        let color = match tet {
            1 => Color::Cyan,
            2 => Color::Yellow,
            3 => Color::Magenta,
            4 => Color::Blue,
            5 => Color::White,
            6 => Color::Green,
            7 => Color::Red,
            8 => Color::Gray,
            _ => Color::Reset,
        };
        Style::default().fg(color)
    }

    impl Game {
        pub fn new() -> Self {
            Game {
                tetris: Tetris::default(),
            }
        }

        fn draw_game(&self) -> Text {
            let mut text = Text::default();
            let board = self.tetris.get_state();
            for r in 0..20 {
                let s_vec: Vec<_> = board[r + 20]
                    .into_iter()
                    .map(|x| {
                        if x == 0 {
                            Span::styled(" .", get_style(0))
                        } else {
                            Span::styled("[]", get_style(x))
                        }
                    })
                    .collect();
                let mut spans: Spans = Spans::from(vec![Span::raw("<!")]);
                spans.0.extend(s_vec);
                spans.0.push(Span::raw("!>"));
                text.extend(Text::from(spans));
            }
            text.extend(Text::from(format!("<!{}!>", "=".repeat(20))));
            text.extend(Text::from("\\/".repeat(10)));
            text
        }

        pub async fn run(&mut self) -> Result<()> {
            // let mut stdout = stdout();
            // execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

            // let stdout = io::stdout();
            // let backend = CrosstermBackend::new(stdout);
            // let mut terminal = Terminal::new(backend)?;
            // terminal.clear()?;
            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen)?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;
            // Render the widget
            loop {
                let now = Instant::now();
                let game_par = Paragraph::new(self.draw_game()).alignment(Alignment::Center);
                let (held, h_tet) = self.tetris.get_held();
                let held_par = Paragraph::new(Text::styled(held, get_style(h_tet)))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::all())
                            .title("HELD")
                            .title_alignment(Alignment::Center),
                    );
                let score_par = Paragraph::new(Text::from(format!("{}", self.tetris.score)))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::all())
                            .title("SCORE")
                            .title_alignment(Alignment::Center),
                    );
                let level_par = Paragraph::new(Text::from(format!("{}", self.tetris.level)))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::all())
                            .title("LEVEL")
                            .title_alignment(Alignment::Center),
                    );
                let lines_par = Paragraph::new(Text::from(format!("{}", self.tetris.lines)))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::all())
                            .title("LINES")
                            .title_alignment(Alignment::Center),
                    );
                let mut next_text = Text::default();
                for tet in self.tetris.get_queue() {
                    next_text.extend(Text::styled(tet.to_string(), get_style(tet as u8)));
                    next_text.extend(Text::raw("\n"));
                }
                let queue_par = Paragraph::new(next_text)
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::all())
                            .title("NEXT")
                            .title_alignment(Alignment::Center),
                    );
                // DRAWING TO THE TERMINAL
                terminal.draw(|f| {
                    let layout = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Max(10),
                            Constraint::Max(24),
                            Constraint::Max(10),
                            Constraint::Max(0)
                        ])
                        .split(f.size());
                    let stats_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Max(4),
                            Constraint::Max(3),
                            Constraint::Max(3),
                            Constraint::Max(3),
                            Constraint::Max(0),
                        ])
                        .split(layout[0]);
                    let next_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Max(13), Constraint::Max(0)])
                        .split(layout[2]);
                    f.render_widget(held_par, stats_layout[0]);
                    f.render_widget(score_par, stats_layout[1]);
                    f.render_widget(level_par, stats_layout[2]);
                    f.render_widget(lines_par, stats_layout[3]);
                    f.render_widget(game_par, layout[1]);
                    f.render_widget(queue_par, next_layout[0]);
                })?;
                let event_waiting = poll(Duration::from_secs(0))?;
                let event = if event_waiting {
                    read()?
                } else {
                    Event::FocusLost
                };
                if let Event::Key(KeyEvent { code, kind, .. }) = event {
                    match kind {
                        KeyEventKind::Press => match code {
                            KeyCode::Esc => break,
                            KeyCode::Char('a') | KeyCode::Left => self.tetris.shift(true),
                            KeyCode::Char('d') | KeyCode::Right => self.tetris.shift(false),
                            KeyCode::Char('w') | KeyCode::Up => self.tetris.rotate(true),
                            KeyCode::Char('z')
                            | KeyCode::Modifier(ModifierKeyCode::LeftControl)
                            | KeyCode::Modifier(ModifierKeyCode::RightControl) => {
                                self.tetris.rotate(false)
                            }
                            KeyCode::Char('s') | KeyCode::Down => self.tetris.soft_drop(),
                            KeyCode::Char(' ') => self.tetris.hard_drop(),
                            KeyCode::Char('c')
                            | KeyCode::Modifier(ModifierKeyCode::LeftShift)
                            | KeyCode::Modifier(ModifierKeyCode::RightShift) => self.tetris.hold(),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                self.tetris.frame_advance();
                let elapsed = Instant::now() - now;
                let f_dur = Duration::from_micros(16667);
                if elapsed > f_dur {
                    continue;
                }
                task::sleep(f_dur - elapsed).await;
            }

            Ok(())
        }
    }
}
