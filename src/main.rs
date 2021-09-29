use std::io::stdout;
use std::{error::Error, io};
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{event::Key, input::MouseTerminal};
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::text::{Span};
use tui::style::{Style, Color};

enum WindowMode {
  Start,
  Typing,
}

struct App {
  input: String,
  window_mode: WindowMode,
  pastwpm: Vec<i32>,
}

impl Default for App {
  fn default() -> App {
    App {
      input: String::new(),
      window_mode: WindowMode::Start,
      pastwpm: Vec::new(),
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let backend = TermionBackend::new(AlternateScreen::from(stdout().into_raw_mode()?));
  let mut terminal = Terminal::new(backend)?;
  let text: &str = "";
  let mut app = App::default();

  let help: &str = match app.window_mode {
    WindowMode::Start =>("Welcome to Typecat, The Terminal Typing test =^._.^= ∫ \n q to quit ||  s to switch to typing mode || g to check previous typing speed"),
    WindowMode::Typing=>("words")
    
  };

  loop {
    terminal.draw(|f| {
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(10)].as_ref())
        .split(f.size());
      let help = Paragraph::new(help)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

      f.render_widget(help, chunks[0]);

      let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

      f.render_widget(para, chunks[1]);

      let para2 = Paragraph::new(text)
      .block(Block::default().borders(Borders::ALL))
      .style(Style::default().fg(Color::Red).bg(Color::DarkGray))
      .alignment(Alignment::Left)
      .wrap(Wrap { trim: true });
      
    })?;
    
  }

  Ok(())
}
