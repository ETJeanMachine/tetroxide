pub mod tetroxide {
    use async_std::task;
    use crossterm::{
        event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, ModifierKeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
        Result,
    };
    use spin_sleep::LoopHelper;
    use std::io::{self, Stdout};
    use std::time::{Duration, Instant};
    use tetris::tetris::Tetris;
    use tui::{
        backend::{Backend, CrosstermBackend},
        layout::{Alignment, Constraint, Direction, Layout},
        style::{Color, Style},
        text::{Span, Spans, Text},
        widgets::{Block, BorderType, Borders, Paragraph},
        Terminal,
    };

    enum MenuOpts {
        Quit,
        Restart,
        SetLevel,
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
            5 => Color::DarkGray,
            6 => Color::Green,
            7 => Color::Red,
            8 => Color::White,
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

        async fn pause(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {}

        async fn game_loop(
            &mut self,
            terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        ) -> Result<()> {
            let mut loop_helper = LoopHelper::builder().build_with_target_rate(60.0); // limit to 60 FPS if possible
            loop {
                loop_helper.loop_start();
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
                let score_text = if self.tetris.combo_count > 0 {
                    format!("{}\n{}x COMBO", self.tetris.score, self.tetris.combo_count)
                } else {
                    format!("{}", self.tetris.score)
                };
                let score_par = Paragraph::new(Text::from(score_text))
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
                let game_block = Block::default()
                    .border_type(BorderType::Double)
                    .borders(Borders::ALL)
                    .title("TETROXIDE")
                    .title_alignment(Alignment::Center);
                // DRAWING TO THE TERMINAL
                terminal.draw(|f| {
                    let size = f.size();
                    let all = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [
                                Constraint::Length((size.width - 48) / 2),
                                Constraint::Length(48),
                                Constraint::Length((size.width - 48) / 2),
                            ]
                            .as_ref(),
                        )
                        .split(size);
                    let layout = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [
                                Constraint::Length(12),
                                Constraint::Length(24),
                                Constraint::Length(12),
                                Constraint::Percentage(100),
                            ]
                            .as_ref(),
                        ).margin(1)
                        .split(all[1]);
                    let stats_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(4),
                            Constraint::Length(4),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Percentage(100),
                        ])
                        .split(layout[0]);
                    let next_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(13), Constraint::Percentage(100)])
                        .split(layout[2]);
                    // Rendering all of our widgets.
                    f.render_widget(game_block, all[1]);
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
                loop_helper.loop_sleep();
            }
            Ok(())
        }

        pub async fn run(&mut self) -> Result<()> {
            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen)?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;
            // Main game event loop
            self.game_loop(&mut terminal).await?;
            terminal.flush()?;
            disable_raw_mode()?;
            terminal.backend_mut().set_cursor(0, 0)?;
            println!("Exited Game!");
            Ok(())
        }
    }
}
