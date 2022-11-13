use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, ListItem, Paragraph},
    Frame, Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

extern crate skullcap;
use skullcap::layout::atom::{div, ul, li};
use skullcap::layout::input::search;
use skullcap::App;

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // whole app layout wrapper
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(10)].as_ref())
        .split(f.size());
    // main block layout for side nav and visual block
    let side_nav = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[0]);
    let side_body = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(side_nav[1]);

    //TODO: figure this out by parsing other lib commands
    let tmp_list: Vec<&str> = vec!["Generate", "Run", "Build", "Serve", "Test"];
    let items: Vec<ListItem> = tmp_list
        .iter()
        .map(|i| li(i.to_string()))
        .collect();

    let body = Paragraph::new("This will be where the builder will be".to_string())
        .block(Block::default().borders(Borders::ALL).title("Generate"));

    f.render_widget(body, side_nav[1]);
    // side nav menu block
    ul(f, app, side_nav[0], "List", items);
    //builder block
    div(f, app, chunks[0], "Builder");
    //search input block
    search(f, app, side_body[0], "Search");
}

fn main() -> Result<()> {
    // trying out the basis of a "react" createComponent macro idea
    // styled!("input", ("className", "error"), vec![0, 1, 2, 3]);
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
        terminal.draw(|f| ui(f, &app)).expect("Failed to draw UI");
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event.code {
                    // takes current character input and collects them
                    KeyCode::Enter => {
                        app.messages.push(app.input.drain(..).collect());
                    }
                    // pushes new characters into the input
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    // pops last character off of input
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {
                        // not sure i'll have more yet
                        break;
                    }
                }
                // just for making sure i'm not insane when getting inputs
                // println!("{:?}\r", event);
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
