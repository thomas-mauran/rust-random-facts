use tui::{
    backend::{Backend},
    layout::{Alignment, Rect, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, text::{Span, Spans},
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples

    let area = frame.size();

    let main_block = Block::default()
        .title("Random facts")
        .style(Style::default().bg(Color::Black))
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(48),
            Constraint::Percentage(50),
            Constraint::Percentage(2),
            ])
        .split(main_block.inner(area));

                                                                    
    let title_block = Block::default();                                            
    let title_text = vec![
        Spans::from(Span::raw(r"______                _                  ______         _        ")),
        Spans::from(Span::raw(r"| ___ \              | |                 |  ___|       | |       ")),
        Spans::from(Span::raw(r"| |_/ /__ _ _ __   __| | ___  _ __ ___   | |_ __ _  ___| |_ ___  ")),
        Spans::from(Span::raw(r"|    // _` | '_ \ / _` |/ _ \| '_ ` _ \  |  _/ _` |/ __| __/ __| ")),
        Spans::from(Span::raw(r"| |\ \ (_| | | | | (_| | (_) | | | | | | | || (_| | (__| |_\__ \ ")),
        Spans::from(Span::raw(r"\_| \_\__,_|_| |_|\__,_|\___/|_| |_| |_| \_| \__,_|\___|\__|___/"))];

    let title_paragraph = Paragraph::new(title_text)
                    .block(title_block)
                    .alignment(Alignment::Center);
       
    frame.render_widget(title_paragraph, layout_vertical[0]);




    let footer_block = Block::default();

    let footer_text = Paragraph::new(format!("Press 'Esc', 'q' or Ctrl + c to escape the program"))
                            .block(footer_block.clone())
                            .alignment(Alignment::Center);

    frame.render_widget(footer_text, layout_vertical[2]);
 

    // fill_main_block(frame, main_block.inner(area));

    frame.render_widget(main_block, area);
}
