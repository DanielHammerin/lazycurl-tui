use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = app(&mut terminal);
    ratatui::restore();
    result?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello", frame.area());
}
