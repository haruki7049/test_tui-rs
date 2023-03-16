use std::io;
use tui::{
    style::{Style, Color},
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
    Frame,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(ui)?;

    while let Event::Key(KeyEvent {code, .. }) = event::read()? {
        match code {
            KeyCode::Enter => break,
            _ => {}
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn ui<B: Backend> (f: &mut Frame<B>) {
    let size = f.size();
    let block = Block::default()
        .title("please enter any key")
        .borders(Borders::ALL);
    let brdr_style = Style::default().bg(Color::Cyan);
    f.render_widget(block, size);
}
