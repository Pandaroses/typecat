use std::io::{Write, stdout, stdin};
use std::{error::Error, io, time::Instant};
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::screen::AlternateScreen;
use termion::{event::Key, input::MouseTerminal};
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::text::{Span, Spans};
use tui::terminal::Frame;
use tui::style::{Style, Color, Modifier};

enum Mode {
  Start,
  Typing,
  Results,
}

struct App {
  input: String,
  mode: Mode,
  pastwpm: Vec<u16>,
  words: String,
  timestarted: Option<Instant>,
  tempoby: u64,
  cpm: u64,
  lvlslct: usize,
  lvl: Vec<String>,
}

struct score { 
  
}

impl Default for App {
  fn default() -> App {
    App {
      input: String::new(),
      mode: Mode::Start,
      pastwpm: Vec::new(),
      words: String::from("the quick brown fox jumped over the lazy dog"),
      cpm:  0,
      timestarted: None, 
      tempoby: 69,
      lvlslct: 0,
      lvl: vec![String::from("The quick brown fox jumped over the lazy dog"),String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam mauris dolor, interdum sed porttitor in, tempor eget turpis. Phasellus tincidunt tortor ac enim laoreet sollicitudin. Aliquam erat volutpat. Nunc ut est eu diam commodo accumsan. Aenean mattis tortor a quam tincidunt, sagittis dignissim nisi porttitor. Mauris molestie lectus leo, ac euismod tortor maximus in. Nullam efficitur leo id blandit pulvinar. Proin ornare quis erat tincidunt tristique. Aliquam erat volutpat. Donec viverra, eros vel bibendum accumsan, ligula odio sagittis purus, id congue urna mauris et mauris. Vestibulum quam sapien, mattis quis dui sed, imperdiet bibendum sem. Suspendisse dignissim venenatis ultricies. Nulla finibus purus dui.
      "),String::from("Hello this is work in progress but you can use this as a typing test i guess =^._.^= ")]
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
    for c in stdin().keys() {
      match self.mode {
        Mode::Typing => {
          match c? {
            Key::Esc => {
              self.input = "".to_string();
              self.mode = Mode::Start
            }
            Key::Char(c) => self.input.push(c),
            Key::Backspace => {
              self.input.pop();
            }
            _ => {}
          };
          if self.input.len() == self.lvl[self.lvlslct].len() {
            self.tempoby = self.timestarted.unwrap().elapsed().as_secs();
            self.cpm = self.tempoby  / self.lvl[self.lvlslct].len() as u64;
            // this needs the fuck is go here the "" go he
            //can set results later
            self.mode = Mode::Results;
          }
        }
        Mode::Start => match c? {
          Key::Char('q') => break,
          Key::Char('\n') => {
            self.mode = Mode::Typing;
            self.timestarted = Some(Instant::now())
          }
          Key::Left => {
            if self.lvlslct > 0 {
              self.lvlslct -= 1
            }
          }
          Key::Right => {
            if self.lvlslct <= self.lvl.len() {
              self.lvlslct += 1
            }
          }
          
          _ => {}
        },
        Mode::Results => match c? {
          Key::Char('q') => break,
          Key::Char('\n') => self.mode = Mode::Start,
          Key::Char('s') => {self.pastwpm.push(self.cpm as u16)},
          _ => {}
        }
      }

      terminal.draw(|f| self.draw(f))?;
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
        let help = Paragraph::new("Welcome to Typecat, The Terminal Typing test =^._.^= ∫ \n q to quit ||  Enter to switch to typing mode || g to check previous typing speed \n Arrow keys to choose Text level")
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Center)
          .wrap(Wrap { trim: true });

        f.render_widget(help, chunks[0]);

        let mut spans = vec![];
        let levels = vec!["Quick Brown Fox ", "Lorem Ipsum ", "English 1k "];

        for (i, level) in levels.iter().enumerate() {
          if self.lvlslct == i {
            spans.push(Span::styled(
              String::from(levels[i]),
              Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::LightBlue),
            ));
          } else {
            spans.push(Span::raw(String::from(levels[i])));
          }
        }

        let list = Paragraph::new(Spans::from(spans))
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Left)
          .wrap(Wrap { trim: true });
        f.render_widget(list, chunks[1]);
      }

      Mode::Typing => {
        let mut spans = vec![];
        let mut correct = 0;

        for (i, c) in self.lvl[self.lvlslct].chars().enumerate() {
          let style = match self.input.chars().nth(i) {
            Some(a) => {
              if a == c {
                correct += 1;
                Style::default()
                  .fg(Color::Green)
                  .add_modifier(Modifier::BOLD)
              } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
              }
            }
            _ => Style::default().fg(Color::Cyan),
          };
          spans.push(Span::styled(String::from(c), style));
        }
        let typingbox = Paragraph::new(Spans::from(spans))
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Left)
          .wrap(Wrap { trim: true });

        f.render_widget(typingbox, chunks[1]);

        let help = Paragraph::new(format!("Correct: {}", correct))
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Center)
          .wrap(Wrap { trim: true });

        f.render_widget(help, chunks[0]);
        f.set_cursor(chunks[1].x + self.input.len() as u16 + 1, chunks[1].y + 1)
      }
      Mode::Results => {
        let typingbox = Paragraph::new(format!("Your Characters per minute was {} , Well Done", (self.lvl[self.lvlslct].len() as u64 / self.tempoby) * 60  ))
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Left)
          .wrap(Wrap { trim: true });

        f.render_widget(typingbox, chunks[1]);

        let help = Paragraph::new(format!("it took you {} Seconds, Good job", self.tempoby))
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Center)
          .wrap(Wrap { trim: true });

        f.render_widget(help, chunks[0]);
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
