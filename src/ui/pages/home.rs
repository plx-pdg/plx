use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_home(frame: &mut Frame) {
    let title_text = r#"
████████  ██       ██     ██ 
██     ██ ██        ██   ██  
██     ██ ██         ██ ██   
████████  ██          ███    
██        ██         ██ ██   
██        ██        ██   ██  
██        ████████ ██     ██
                    "#;

    // how can i change this ???
    let info_text_one = "Press 'r' to resume progress";
    let info_text_two = "Press 'l' to list all exercices";
    let info_text_three = "Press '?' to display help";

    // Ratatui needs RGB => need convert hex colors to RGB
    // starting color = #fc1100 => RGB = 252, 17, 0
    // ending color = #ffb000 => RGB = 255, 176, 0

    let first_color = (252, 17, 0);
    let second_color = (255, 176, 0);

    // Function to mix colors and create the gradient result
    // factor = float qui indique la pos entre la couleur de first et second.
    // on fait les diff entre les r, g, b des 2 couleurs en utilisant le factor.
    fn mixed_color(start: (u8, u8, u8), end: (u8, u8, u8), factor: f32) -> (u8, u8, u8) {
        let r = start.0 as f32 + factor * (end.0 as f32 - start.0 as f32);
        let g = start.1 as f32 + factor * (end.1 as f32 - start.1 as f32);
        let b = start.2 as f32 + factor * (end.2 as f32 - start.2 as f32);
        (r as u8, g as u8, b as u8)
    }

    // Create gradient style on title_text
    // ?????????
    // split le text en ligne pour pouvoir "repartir" la couleur
    // vecteur de rendu pour recup les lines modifiee du style voulu (gradient)
    // calcul factor pour mixed color pour connaitre la pos de la ligne actuel dans le gradient final
    let lines_from_text: Vec<&str> = title_text.lines().collect();
    let mut spans = Vec::new();
    for (i, line) in lines_from_text.iter().enumerate() {
        let factor = i as f32 / (lines_from_text.len() - 1) as f32;
        let color = mixed_color(first_color, second_color, factor);
        let span = Span::styled(
            *line,
            Style::default().fg(Color::Rgb(color.0, color.1, color.2)),
        );
        spans.push(span);
    }

    // prend spans et traduit en It puis map = traduit de type spans a line, collect => rend en vec de line
    // tout ca car Text:: peut pas lire des Vec de spans mais de Line.
    let mut lines: Vec<Line> = spans.into_iter().map(Line::from).collect();

    lines.push(Line::from(info_text_one));
    lines.push(Line::from(info_text_two));
    lines.push(Line::from(info_text_three));

    let title = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::NONE))
        .alignment(ratatui::layout::Alignment::Center);

    // let content = Paragraph::new("").centered().block(Block::default().borders(Borders::ALL).padding(Padding::new(0, 0, 10, 10)).style(Style::default().bold()));

    frame.render_widget(title, frame.area());

    // frame.render_widget(content, display[1]);
}
