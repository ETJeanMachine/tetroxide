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
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        Terminal,
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

        pub async fn run(&mut self) -> Result<()> {
            enable_raw_mode()?;
            let mut stdout = stdout();
            execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

            // let stdout = io::stdout();
            // let backend = CrosstermBackend::new(stdout);
            // let mut terminal = Terminal::new(backend)?;
            // terminal.clear()?;

            // self.tetris.soft_drop();
            // self.tetris.soft_drop();
            // println!("{}", self.tetris);
            // let input = format!("{}", self.tetris);
            // backend::write(&mut stdout, input.as_str(), 90, (0, 0), 15)?;

            while !self.tetris.is_game_over {
                let mut stdout_lock = stdout.lock();
                execute!(stdout_lock, terminal::Clear(terminal::ClearType::All))?;
                // trying to write it line by line
                for (r, l) in self.tetris.to_string().lines().enumerate() {
                    execute!(stdout_lock, cursor::MoveTo(0, r as u16))?;
                    writeln!(stdout_lock, "{}", l)?;
                }
                stdout_lock.flush()?;
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

            disable_raw_mode()?;

            Ok(())
        }
    }
}
