use std::{io, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    text::{Spans, Span},
    Frame,
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    Result,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use unicode_width::UnicodeWidthStr;

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            messages: Vec::new(),
        }
    }
}
fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // whole app layout wrapper
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(90),
                Constraint::Percentage(10),
            ].as_ref()
        )
        .split(f.size());
   // main block layout for side nav and visual block
   let side_nav = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(chunks[0]);
   let mut tmp_list: Vec<&str> = vec![];
   tmp_list.push("one");
   tmp_list.push("two");
   tmp_list.push("three");
   tmp_list.push("four");
       let items: Vec<ListItem> = tmp_list.iter().map(|i| {
           ListItem::new(Span::from(i.to_string()))
       }).collect();
   // side nav menu block
    let nav = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"));
    f.render_widget(nav, side_nav[0]);

    // Main wrapping block
    let block = Block::default()
         .title("Builder")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    // bottom input block
    let input = Paragraph::new(app.input.as_ref())
        .block(Block::default().borders(Borders::ALL).title("Command"));
    f.render_widget(input, chunks[1]);
    f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
}

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // create app
    let mut app = App::default();
    //TODO: abstract this out some or things will get sloppy
    loop {
    terminal.draw(|f| {
        ui(f, &app)
    })?;
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Enter => {
                        app.messages.push(app.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {
                        // not sure i'll have more yet
                        todo!();
                    }
                }
                // just for making sure i'm not insane when getting inputs
                //println!("{:?}\r", event);
            };
        } else {
            // TODO: make this better it's sloppy
                println!("");
            }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
