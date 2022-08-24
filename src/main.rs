use std::{io, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    text::Span,
    style::{Color, Modifier, Style},
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

extern crate skullcap;
use skullcap::{App,styled};
use skullcap::layout::input::search;
use skullcap::layout::atom::div;

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
   //TODO: figure this out by parsing other lib commands
   let mut tmp_list: Vec<&str> = vec![];
   tmp_list.push("Generate");
   tmp_list.push("Run");
   tmp_list.push("Build");
   tmp_list.push("Serve");
   tmp_list.push("Test");
       let items: Vec<ListItem> = tmp_list.iter().map(|i| {
           ListItem::new(Span::from(i.to_string()))
       }).collect();
   // side nav menu block
    let nav = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
    f.render_widget(nav, side_nav[0]);

    let body = Paragraph::new("This will be where the builder will be".to_string())
        .block(Block::default().borders(Borders::ALL).title("Generate"));
    f.render_widget(body, side_nav[1]);
    //builder block
    div(f, app, chunks[0], "Builder");
    //bottom search input block
    search(f, app, chunks);
}



fn main() -> Result<()> {
    // setup terminal
    styled!("input", ("className", "error"), vec![0,1,2,3]);
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
                        break;
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
