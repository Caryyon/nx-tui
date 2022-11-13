use tui::{
    backend::Backend,
    widgets::{Block, Borders, List, ListItem},
    text::Span,
    style::{Color, Modifier, Style},
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


pub fn li(item: String) -> ListItem<'static> {
   ListItem::new(Span::from(item))
}

pub fn ul<B: Backend>(frame: &mut Frame<B>, _app: &App, chunk: Rect, title: &str, items: Vec<ListItem>) {
    let nav = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
    frame.render_widget(nav, chunk);
}

