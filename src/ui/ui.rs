use std::{
    io::Stdout,
    sync::{Arc, Mutex},
};

use crate::{
    core::core::PlxCore,
    models::ui_state::UiState,
};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Text},
    widgets::{block::title, Block, Borders, Padding, Paragraph},
    Frame, Terminal,
};
use std::io::{self, stdout};

pub struct Ui<'a> {
    core: Arc<Mutex<PlxCore<'a>>>,
}
impl Ui<'_> {
    pub fn new(core: Arc<Mutex<PlxCore>>) -> Ui<'_> {
        Ui { core }
    }
    fn setup(&mut self) -> io::Result<()> {
        println!("Ui Setup...");
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Ok(())
    }
    fn teardown(&mut self) -> io::Result<()> {
        println!("Ui Teardown...");
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn loop_forever(&mut self) -> io::Result<()> {
        self.setup()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            match self.core.lock() {
                Ok(core) => {
                    // self.run(&mut terminal, core.get_state());
                    // // if !self.run(&mut terminal, core.get_state()) {
                    // //     break;
                    // // }
                    if !self.run(&mut terminal, core.get_state())? {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        self.teardown()?;
        Ok(())
    }

    pub fn run(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        state: &UiState,
    ) -> Result<bool, io::Error> {
        terminal.draw(|frame| {
            if !self.render_frame(frame, state) {
                // return Ok(false);
                self.render_frame(frame, state);
            }
        })?;
        if !self.handle_events()? {
            return Ok(false);
        };
        Ok(true)
    }

    fn render_frame(&self, frame: &mut Frame, state: &UiState) -> bool {
        let display = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(frame.area());

        match state {
            UiState::Home => {
                let title_text = r#"
████████  ██       ██     ██ 
██     ██ ██        ██   ██  
██     ██ ██         ██ ██   
████████  ██          ███    
██        ██         ██ ██   
██        ██        ██   ██  
██        ████████ ██     ██
                    "#;

                // Ratatui needs RGB => need convert hex colors to RGB
                // starting color = #fc1100 => RGB = 252, 17, 0
                // ending color = #ffb000 => RGB = 255, 176, 0

                let first_color = (252, 17, 0);
                let second_color = (255, 176, 0);

                // Function to mix colors and create the gradient result
                // factor = float qui indique la pos entre la couleur de first et second.
                // on fait les diff entre les r, g, b des 2 couleurs en utilisant le factor.
                fn mixed_color(
                    start: (u8, u8, u8),
                    end: (u8, u8, u8),
                    factor: f32,
                ) -> (u8, u8, u8) {
                    let r = start.0 as f32 + factor * (end.0 as f32 - start.0 as f32);
                    let g = start.1 as f32 + factor * (end.1 as f32 - start.1 as f32);
                    let b = start.2 as f32 + factor * (end.2 as f32 - start.2 as f32);
                    (r as u8, g as u8, b as u8)
                }

                // Create gradient style on title_text
                // split le text en ligne pour pouvoir "repartir" la couleur
                let lines_from_text: Vec<&str> = title_text.lines().collect();
                // vecteur de rendu pour recup les lines modifiee du style voulu (gradient)
                let mut spans = Vec::new();
                for (i, line) in lines_from_text.iter().enumerate() {
                    let factor = i as f32 / (lines_from_text.len() - 1) as f32; // calcul factor pour mixed color pour connaitre la pos de la ligne actuel dans le gradient final
                    let color = mixed_color(first_color, second_color, factor);
                    let span = Span::styled(
                        *line,
                        Style::default().fg(Color::Rgb(color.0, color.1, color.2)),
                    );
                    spans.push(span);
                }

                // prend spans et traduit en It puis map = traduit de type spans a line, collect => rend en vec de line
                // tout ca car Text:: peut pas lire des Vec de spans mais de Line.
                let lines: Vec<Line> = spans.into_iter().map(Line::from).collect();
                let title = Paragraph::new(Text::from(lines))
                    .block(
                        Block::default()
                            .borders(Borders::NONE)
                            .padding(Padding::new(0, 0, 10, 10)),
                    )
                    .alignment(ratatui::layout::Alignment::Center);

                let content = Paragraph::new("Press 'r' to resume progress\nPress 'l' to list all exercices\nPress '?' to display help\n").centered().block(Block::default().borders(Borders::NONE).padding(Padding::new(0, 0, 10, 10)).style(Style::default().bold()));

                frame.render_widget(title, display[0]);
                frame.render_widget(content, display[1]);
            }
            _ => {}
        }
        return true;
    }

    fn handle_events(&self /*, ui_state aussi pour render les pages*/) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(false),


                    _ => {}
                }
            }
        }
        Ok(true)
    }

    // fn handle_events(app_state: &mut AppState) -> io::Result<bool> {
    //     if event::poll(std::time::Duration::from_millis(50))? {
    //         if let Event::Key(key) = event::read()? {
    //             //TODO send event to the core
    //         }
    //     }
    //     Ok(false)
    // }
}
