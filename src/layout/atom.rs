use tui::{
    backend::Backend,
    widgets::{Block, Borders},
    //style::{Color, Style},
    Frame,
    layout::Rect
};
use crate::App;
pub fn div<B: Backend>(frame: &mut Frame<B>, _app: &App, chunk: Rect, title: &str) {
    let block = Block::default()
         .title(title)
         .borders(Borders::ALL);
    frame.render_widget(block, chunk);
}
