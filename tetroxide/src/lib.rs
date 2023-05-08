pub mod tetroxide {
    use crossterm::{
        cursor::{Hide, Show},
        event::{
            read, poll, EnableMouseCapture, 
            Event, KeyCode, KeyEventKind, KeyEvent
        },
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, 
            EnterAlternateScreen, LeaveAlternateScreen, Clear
        },
        Result,
    };
    use spin_sleep::LoopHelper;
    use std::{io::{stdout, Write}, time::Duration};
    use tetris::tetris::Tetris;
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
        looper: LoopHelper,
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
                looper: LoopHelper::builder()
                    .report_interval_s(0.5)
                    .build_with_target_rate(60),
            }
        }

        pub fn run(&mut self) -> Result<()> {
            enable_raw_mode()?;
            let stdout = stdout();
            let mut stdout = stdout.lock();

            execute!(stdout, Hide, EnterAlternateScreen, EnableMouseCapture)?;

            // self.tetris.soft_drop();
            // self.tetris.soft_drop();
            // println!("{}", self.tetris);
            // let input = format!("{}", self.tetris);
            // backend::write(&mut stdout, input.as_str(), 90, (0, 0), 15)?;
            stdout.flush()?;

            let mut frame_count: i64 = 1;

            while !self.tetris.is_game_over {
                self.looper.loop_start();

                execute!(stdout, Clear(crossterm::terminal::ClearType::All))?;
                println!("{}", self.tetris);

                let event_waiting = poll(Duration::from_secs(0))?;
                let event = if event_waiting {read()?} else {Event::FocusLost};

                if let Event::Key(KeyEvent { code, kind, .. }) = event {
                    match kind {
                        KeyEventKind::Press => {
                            match code {
                                KeyCode::Esc => break,
                                KeyCode::Char('a') => self.tetris.shift(true),
                                KeyCode::Char('d') => self.tetris.shift(false),
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }

                if frame_count % 48 == 0 {
                    self.tetris.soft_drop();
                    self.tetris.clear_lines();
                }

                self.looper.loop_sleep();
                frame_count += 1;
            }

            disable_raw_mode()?;

            Ok(())
        }
    }
}
