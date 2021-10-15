use std::io::{stdout, stdin};
use std::{error::Error, time::Instant, time::Duration};
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::screen::AlternateScreen;
use termion::event::Key;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
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
  current: Option<Score>,
  level: usize,
  levels: Vec<String>,
}

struct Score {
  correct: u64,
  timestarted: Instant,
  timetaken: Option<Duration>,
}

impl Score {
  fn new() -> Self {
    Self {
      correct: 0,
      timestarted: Instant::now(),
      timetaken: None,
    }
  }
}

impl Default for App {
  fn default() -> App {
    App {
      input: String::new(),
      mode: Mode::Start,
      current: None,
      level: 0,
      levels: vec![String::from("The quick brown fox jumped over the lazy dog"),String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam mauris dolor, interdum sed porttitor in, tempor eget turpis. Phasellus tincidunt tortor ac enim laoreet sollicitudin. Aliquam erat volutpat. Nunc ut est eu diam commodo accumsan. Aenean mattis tortor a quam tincidunt, sagittis dignissim nisi porttitor. Mauris molestie lectus leo, ac euismod tortor maximus in. Nullam efficitur leo id blandit pulvinar. Proin ornare quis erat tincidunt tristique. Aliquam erat volutpat. Donec viverra, eros vel bibendum accumsan, ligula odio sagittis purus, id congue urna mauris et mauris. Vestibulum quam sapien, mattis quis dui sed, imperdiet bibendum sem. Suspendisse dignissim venenatis ultricies. Nulla finibus purus dui.
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
              self.current = None;
              self.mode = Mode::Start
            }
            Key::Char(c) => self.input.push(c),
            Key::Backspace => {
              self.input.pop();
            }
            _ => {}
          };
          if self.input.len() == self.levels[self.level].len() {
            let score = self.current.as_mut().unwrap();
            score.timetaken = Some(score.timestarted.elapsed());
            //save score cool
            self.input = String::from("");
            self.mode = Mode::Results;
          }
        }
        Mode::Start => match c? {
          Key::Char('q') => break,
          Key::Char('\n') => {
            self.mode = Mode::Typing;
            self.current = Some(Score::new())
          }
          Key::Left => {
            if self.level > 0 {
              self.level -= 1
            }
          }
          Key::Right => {
            if self.level <= self.levels.len() {
              self.level += 1
            }
          }

          _ => {}
        },
        Mode::Results => match c? {
          Key::Char('q') => break,
          Key::Char('\n') => {
            self.mode = Mode::Start;
            self.current = None;
          }
          _ => {}
        },
      }

      terminal.draw(|f| self.draw(f))?;
    }
    Ok(())
  }

  fn draw(
    &mut self,
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

        for (i, _) in levels.iter().enumerate() {
          if self.level == i {
            spans.push(Span::styled(
              String::from(levels[i]),
              Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::LightBlue),
            ));
          } else {
            spans.push(Span::raw(String::from(levels[i]))); //alex should fix what am i meant to fix
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
        let score = self.current.as_mut().unwrap();
        score.correct = 0;

        for (i, c) in self.levels[self.level].chars().enumerate() {
          let style = match self.input.chars().nth(i) {
            Some(a) => {
              if a == c {
                score.correct += 1;
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

        f.render_widget(typingbox, chunks[1]); //nofucmjing reytard

        let help = Paragraph::new(format!("Correct: {}", score.correct))
          .block(Block::default().borders(Borders::ALL))
          .style(Style::default().fg(Color::White).bg(Color::DarkGray))
          .alignment(Alignment::Center)
          .wrap(Wrap { trim: true });

        f.render_widget(help, chunks[0]);
        f.set_cursor(chunks[1].x + self.input.len() as u16 + 1, chunks[1].y + 1)
      }
      Mode::Results => {
        let score = self.current.as_mut().unwrap();
        let typingbox = Paragraph::new(format!(
          "Your Characters per minute was {}, Well Done  ",
          (self.levels[self.level].len() as u64 / score.timetaken.unwrap().as_secs() * 60)
        ))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

        f.render_widget(typingbox, chunks[1]);

        let help = Paragraph::new(format!("it took you {} Seconds, Good job \n  Press enter to go back to the start page || press s to save score || press q to quit", score.timetaken.unwrap().as_secs()  ))
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

  app.run(&mut terminal)?;
  Ok(())
}
