pub mod tetroxide {
    use tetris::tetris::Tetris;
    use spin_sleep::LoopHelper;
    use crossterm::{
        cursor::{Hide, Show},
        event::{read, EnableMouseCapture, Event, KeyCode, KeyEvent},
        execute,
        Result,
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    };
    use std::io::{stdout, Write};
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
        looper: LoopHelper
    }
    impl Game {
        pub fn new() -> Self {
            Game {
                tetris: Tetris::new(),
                looper: LoopHelper::builder()
                    .report_interval_s(0.5)
                    .build_with_target_rate(60)
            } 
        }

        pub fn run(&mut self) -> Result<()> {
            enable_raw_mode()?;
            let stdout = stdout();
            let mut stdout = stdout.lock();

            execute!(stdout, Hide, EnterAlternateScreen, EnableMouseCapture)?;

            let mut input: Input = "Hello ".into();
            backend::write(&mut stdout, input.value(), input.cursor(), (0, 0), 15)?;
            stdout.flush()?;

            let mut frame_count: i64 = 1;

            loop {
                self.looper.loop_start();
                
                let event = read()?;

                match event {
                    Event::Key(KeyEvent { code, .. }) => match code {
                        KeyCode::Esc => break,
                        _ => {}
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

            Ok(())
        }
    }
}