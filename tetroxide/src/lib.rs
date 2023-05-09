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
        style::{Color, Modifier, Style},
        text::{Span, Spans, Text},
        widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
        Terminal,
    };

    #[derive(Debug, Clone, Copy)]
    enum MenuOpts {
        Restart,
        Quit,
        SetLevel,
    }

    #[derive(Debug, Clone, Copy)]
    enum MenuState {
        Pause,
        Level,
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

        async fn level_select(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {}

        async fn pause(
            &self,
            terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        ) -> Result<Option<MenuOpts>> {
            let mut loop_helper = LoopHelper::builder().build_with_target_rate(60.0);
            let mut menu_opt = MenuOpts::Restart;
            loop {
                loop_helper.loop_start();
                self.render(terminal, Some((MenuState::Pause, menu_opt)))?;
                let event_waiting = poll(Duration::from_secs(0))?;
                let event = if event_waiting {
                    read()?
                } else {
                    Event::FocusLost
                };
                if let Event::Key(KeyEvent { code, kind, .. }) = event {
                    menu_opt = match kind {
                        KeyEventKind::Press => match code {
                            KeyCode::Esc => {
                                if !self.tetris.is_game_over {
                                    break;
                                } else {
                                    continue;
                                }
                            }
                            KeyCode::Up => match menu_opt {
                                MenuOpts::Quit => MenuOpts::SetLevel,
                                MenuOpts::Restart => MenuOpts::Quit,
                                MenuOpts::SetLevel => MenuOpts::Restart,
                                _ => menu_opt,
                            },
                            KeyCode::Down => match menu_opt {
                                MenuOpts::Quit => MenuOpts::Restart,
                                MenuOpts::Restart => MenuOpts::SetLevel,
                                MenuOpts::SetLevel => MenuOpts::Quit,
                                _ => menu_opt,
                            },
                            KeyCode::Enter => return Ok(Some(menu_opt)),
                            _ => menu_opt,
                        },
                        _ => menu_opt,
                    }
                }
                loop_helper.loop_sleep();
            }
            Ok(None)
        }

        fn render(
            &self,
            terminal: &mut Terminal<CrosstermBackend<Stdout>>,
            menu_data: Option<(MenuState, MenuOpts)>,
        ) -> Result<()> {
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
                    )
                    .margin(1)
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
                let pause_vert = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(5),
                        Constraint::Length(5),
                        Constraint::Percentage(100),
                    ])
                    .split(layout[1]);
                let pause_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(26),
                        Constraint::Length(12),
                        Constraint::Percentage(100),
                    ])
                    .split(pause_vert[1]);
                // Rendering all of our widgets.
                f.render_widget(game_block, all[1]);
                f.render_widget(held_par, stats_layout[0]);
                f.render_widget(score_par, stats_layout[1]);
                f.render_widget(level_par, stats_layout[2]);
                f.render_widget(lines_par, stats_layout[3]);
                f.render_widget(game_par, layout[1]);
                f.render_widget(queue_par, next_layout[0]);
                if let Some((menu_state, menu_opt)) = menu_data {
                    match menu_state {
                        MenuState::Pause => {
                            let items = [
                                ListItem::new("Restart   "),
                                ListItem::new("Set Level "),
                                ListItem::new("Quit      "),
                            ];
                            let title = if self.tetris.is_game_over {
                                "GAME OVER"
                            } else {
                                "PAUSE"
                            };
                            let pause_list = List::new(items)
                                .block(
                                    Block::default()
                                        .border_type(BorderType::Thick)
                                        .borders(Borders::ALL)
                                        .title(title),
                                )
                                .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                                .style(Style::default().fg(Color::White).bg(Color::Black));
                            let mut state = ListState::default();
                            let idx = match menu_opt {
                                MenuOpts::Restart => 0,
                                MenuOpts::SetLevel => 1,
                                _ => 2,
                            };
                            state.select(Some(idx));
                            f.render_stateful_widget(
                                pause_list,
                                pause_layout[1],
                                &mut state,
                            );
                        }
                        MenuState::Level => todo!(),
                    }
                }
            })?;
            Ok(())
        }

        async fn game_loop(
            &mut self,
            terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        ) -> Result<()> {
            let mut loop_helper = LoopHelper::builder().build_with_target_rate(60.0); // limit to 60 FPS if possible
            loop {
                loop_helper.loop_start();
                self.render(terminal, None)?;
                if self.tetris.is_game_over {
                    match self.pause(terminal).await? {
                        Some(MenuOpts::Restart) => {
                            self.tetris = Tetris::default();
                            continue;
                        }
                        Some(MenuOpts::Quit) => break,
                        Some(MenuOpts::SetLevel) => todo!(),
                        None => {}
                    }
                }
                let event_waiting = poll(Duration::from_secs(0))?;
                let event = if event_waiting {
                    read()?
                } else {
                    Event::FocusLost
                };
                if let Event::Key(KeyEvent { code, kind, .. }) = event {
                    match kind {
                        KeyEventKind::Press => match code {
                            KeyCode::Esc => match self.pause(terminal).await? {
                                Some(MenuOpts::Restart) => {
                                    self.tetris = Tetris::default();
                                    continue;
                                }
                                Some(MenuOpts::Quit) => break,
                                Some(MenuOpts::SetLevel) => todo!(),
                                None => {}
                            },
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
            disable_raw_mode()?;
            Ok(())
        }
    }
}
