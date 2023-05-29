use tui::{
    backend::{Backend, CrosstermBackend},
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

    let center_block_rarea = centered_rect(50, 50, main_block.inner(area), frame);
    center_block_render(frame, main_block.inner(center_block_rarea));

    frame.render_widget(main_block, area);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect<B: Backend>(percent_x: u16, percent_y: u16, r: Rect, f:  &mut Frame<'_, B>) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

        // TOP BLOCK
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

        f.render_widget(title_paragraph, popup_layout[0]);
    

        // BOTTOM BLOCK
        let footer_block = Block::default();

        let footer_text = Paragraph::new(format!(" Press N to get a new random fact
                                                                  \n Press 'Esc', 'q' or Ctrl + c to escape the program"))
                                .block(footer_block.clone())
                                .alignment(Alignment::Center);

        f.render_widget(footer_text, popup_layout[2]);
    

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
}

fn center_block_render<B: Backend>(f: &mut Frame<'_, B>, r: Rect) {
    let layout_vertical = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![Constraint::Percentage(100)])
    .split(r);

    let center_block = Block::default().borders(Borders::all());
    f.render_widget(center_block, layout_vertical[0])

}
