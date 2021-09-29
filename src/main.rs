use std::io::stdout;
use std::error::Error;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Span};
use tui::style::{Style, Color};

fn main() -> Result<(), Box<dyn Error>> {
  let backend = TermionBackend::new(AlternateScreen::from(stdout().into_raw_mode()?));
  let mut terminal = Terminal::new(backend)?;
  loop {
    terminal.draw(|f| {
      let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(f.size());
      
      f.render_widget(block, chunk[0]);
      

      
    })?;
  }

  Ok(())
}
