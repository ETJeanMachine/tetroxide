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
        time::Duration,
    };
    use tetris::tetris::Tetris;
    use tui::{
        backend::{Backend, CrosstermBackend},
        layout::{Alignment, Constraint, Direction, Layout},
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

    impl Game {
        pub fn new() -> Self {
            Game {
                tetris: Tetris::default(),
            }
        }

        fn draw_game(&self) -> Text {
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
                let spans: Spans = Spans::from(s_vec);
                text.extend(Text::from(spans));
            }
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
            self.tetris.level = 11;
            loop {
                let game = Paragraph::new(self.draw_game())
                    .block(Block::default().title("Welcome").borders(Borders::ALL));
                terminal.draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .margin(0)
                        .constraints([Constraint::Length(40)].as_ref())
                        .split(f.size());
                    f.render_widget(game, chunks[0]);
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
                task::sleep(Duration::from_secs_f32(1.0 / 60.0)).await;
            }

            Ok(())
        }
    }
}
