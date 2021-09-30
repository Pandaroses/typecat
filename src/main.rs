use std::io::{Write, stdout, stdin};
use std::{error::Error, io};
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::screen::AlternateScreen;
use termion::{event::Key, input::MouseTerminal};
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::text::{Span};
use tui::terminal::Frame;
use tui::style::{Style, Color};

enum Mode {
  Start,
  Typing,
}

struct App {
  input: String,
  mode: Mode,
  pastwpm: Vec<u16>,
  words: String,
}

impl Default for App {
  fn default() -> App {
    App {
      input: String::new(),
      mode: Mode::Start,
      pastwpm: Vec::new(),
      words: String::from("Welcome to Typecat, The Terminal Typing test =^._.^= ∫ \n q to quit ||  Enter to switch to typing mode || g to check previous typing speed \n Arrow keys to choose Text level")
    }
  }
}

impl App {
  fn run(
    &mut self,
    terminal: &mut Terminal<
      tui::backend::TermionBackend<
        termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
      >,
    >,
  ) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| self.draw(f))?;
    match self.mode {
      Mode::Start => {
        for c in stdin().keys() {
          terminal.draw(|f| self.draw(f))?;

          match c? {
            Key::Char('q') => break,
            Key::Char('\n') => self.mode = Mode::Typing,
            _ => {}
          }
        }
      }
      Mode::Typing => {
        for c in stdin().keys() {
          terminal.draw(|f| self.draw(f))?;

          match c? {
            Key::Esc => self.mode = Mode::Start,
            _ => {}
          }
        }
      }
    }
    Ok(())
  }

  fn draw(
    &self,
    f: &mut Frame<
      tui::backend::TermionBackend<
        termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
      >,
    >,
  ) {
    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Percentage(20), Constraint::Percentage(10)].as_ref())
      .split(f.size());

    match self.mode {
      Mode::Start => {
        let help = Paragraph::new(self.words.clone())
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Center)
          .wrap(Wrap { trim: true });

        f.render_widget(help, chunks[0]);

        
        let list = Paragraph::new("Quick Brown fox \n Lorem Ipsum \n English 1k ")
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Left)
          .wrap(Wrap { trim: true });
        f.render_widget(list, chunks[1]);
      }

      Mode::Typing => {
        let help = Paragraph::new(self.words.clone())
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Center)
          .wrap(Wrap { trim: true });

        f.render_widget(help, chunks[0]);

        
        let list = Paragraph::new("Typing Typing Typing ")
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Left)
          .wrap(Wrap { trim: true });
        f.render_widget(list, chunks[1]);
      }
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let backend = TermionBackend::new(AlternateScreen::from(stdout().into_raw_mode()?));
  let mut terminal = Terminal::new(backend)?;
  let mut app = App::default();

  app.run(&mut terminal);
  Ok(())
}
