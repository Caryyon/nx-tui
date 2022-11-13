use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::App;
use unicode_width::UnicodeWidthStr;

pub fn search<B: Backend>(frame: &mut Frame<B>, app: &App, chunk: Rect, title: &str) {
    let input = Paragraph::new(app.input.as_ref())
        .block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(input, chunk);
    frame.set_cursor(
        // Put cursor past the end of the input text
        chunk.x + app.input.width() as u16 + 1,
        // Move one line down, from the border to the input line
        chunk.y + 1,
    )
}
