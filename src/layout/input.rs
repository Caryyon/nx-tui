use tui::{
    backend::Backend,
    widgets::{Block, Borders, Paragraph},
    //style::{Color, Style},
    Frame,
    layout::Rect
};

use crate::App;
use unicode_width::UnicodeWidthStr;

pub fn search<B: Backend>(frame: &mut Frame<B>, app: &App, chunks: Vec<Rect>) {
    let input = Paragraph::new(app.input.as_ref())
        .block(Block::default().borders(Borders::ALL).title("Command"));
    frame.render_widget(input, chunks[1]);
    frame.set_cursor(
        // Put cursor past the end of the input text
        chunks[1].x + app.input.width() as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[1].y + 1,
    )
}

