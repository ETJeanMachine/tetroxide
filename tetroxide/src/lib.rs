pub mod tetroxide {
    use crossterm::{
        cursor::{Hide, Show},
        event::{read, EnableMouseCapture, Event, KeyCode, KeyEvent},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        Result,
    };
    use spin_sleep::LoopHelper;
    use std::io::{stdout, Write};
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
    impl Game {
        pub fn new() -> Self {
            Game {
                tetris: Tetris::new(),
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
            
            self.tetris.drop();
            self.tetris.drop();
            println!("{}", self.tetris);
            let input = format!("{}", self.tetris);
            backend::write(&mut stdout, input.as_str(), 90, (0, 0), 15)?;
            stdout.flush()?;

            let mut frame_count: i64 = 1;

            while !self.tetris.is_game_over {
                self.looper.loop_start();

                let event = read()?;

                

                match event {
                    Event::Key(KeyEvent { code, .. }) => match code {
                        KeyCode::Esc => break,
                        KeyCode::Left => todo!(),
                        KeyCode::Right => todo!(),
                        KeyCode::Up => todo!(),
                        KeyCode::Down => todo!(),
                        KeyCode::Char(_) => todo!(),
                        KeyCode::F(1) => todo!(),
                        KeyCode::Modifier(_) => todo!(),
                        _ => {},
                    },
                    _ => {}
                }

                if frame_count % 48 == 0 {
                    self.tetris.drop();
                    self.tetris.clean_board();
                }

                self.looper.loop_sleep();
                frame_count += 1;
            }
            stdout.flush()?;

            Ok(())
        }
    }
}
