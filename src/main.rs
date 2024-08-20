use std::{
    hash::Hash,
    io::{stdout, Result},
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, HighlightSpacing, List, ListDirection},
    Terminal,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Exercice {
    name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Chapitre {
    name: String,
}
impl Exercice {
    fn new(name: String) -> Self {
        Self { name }
    }
}
impl Chapitre {
    fn new(name: String) -> Self {
        Self { name }
    }
}
fn main() -> Result<()> {
    let mut chapitres = vec![
        Chapitre::new("Chapitre 1".to_string()),
        Chapitre::new("Chapitre 2".to_string()),
        Chapitre::new("Chapitre 3".to_string()),
    ];
    let mut exercices = Vec::new();

    for (index, _) in chapitres.iter_mut().enumerate() {
        for i in 1..5 {
            exercices.push(Exercice::new(format!("Exercice {index}{i}")));
        }
    }

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let chapitre_lst = List::new(
        chapitres
            .iter()
            .map(|chapitre| Text::from(chapitre.name.clone())),
    )
    .block(Block::bordered().title("List"))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>")
    .highlight_spacing(HighlightSpacing::Always)
    .repeat_highlight_symbol(true)
    .direction(ListDirection::BottomToTop);

    let exercices_lst = List::new(
        exercices
            .iter()
            .map(|exercice| Text::from(exercice.name.clone())),
    )
    .block(Block::bordered().title("List"))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>")
    .highlight_spacing(HighlightSpacing::Always)
    .repeat_highlight_symbol(true)
    .direction(ListDirection::TopToBottom);
    // TODO main loop
    loop {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());
            frame.render_widget(chapitre_lst.clone(), layout[0]);
            frame.render_widget(exercices_lst.clone(), layout[1]);
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
