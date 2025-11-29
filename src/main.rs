extern crate crossterm;
extern crate ratatui;

use std::{
    io,
    process::{Command, Output},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Debug, Default, PartialEq)]
enum FocusBlock {
    Input,
    Output,
    Error,
    #[default]
    None,
}

impl FocusBlock {
    pub fn next(&self) -> Self {
        match self {
            FocusBlock::Input => FocusBlock::Output,
            FocusBlock::Output => FocusBlock::Error,
            FocusBlock::Error => FocusBlock::None,
            FocusBlock::None => FocusBlock::Input,
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            FocusBlock::Input => FocusBlock::None,
            FocusBlock::Output => FocusBlock::Input,
            FocusBlock::Error => FocusBlock::Output,
            FocusBlock::None => FocusBlock::Error,
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    focus: FocusBlock,
    input_text: Vec<String>,
    output_text: Vec<String>,
    error_text: Vec<String>,
    curr_command: u16,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => {
                if self.focus == FocusBlock::None && c == 'q' {
                    self.exit();
                }

                if self.focus == FocusBlock::Input {
                    if self.input_text.is_empty() {
                        self.input_text.push(String::new());
                    }
                    self.input_text.last_mut().unwrap().push(c);
                }
            }
            KeyCode::Tab => self.next_focus(),
            KeyCode::BackTab => self.prev_focus(),
            KeyCode::Backspace => {
                if self.focus == FocusBlock::Input {
                    if !self.input_text.is_empty() {
                        self.input_text.last_mut().unwrap().pop();
                    }
                }
            }
            KeyCode::Enter => {
                if self.focus == FocusBlock::Input {
                    self.run_command();
                    self.input_text.push(String::new());
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_focus(&mut self) {
        self.focus = self.focus.next();
    }

    fn prev_focus(&mut self) {
        self.focus = self.focus.prev();
    }

    fn run_command(&mut self) {
        let command = self.input_text.last().map(|s| s.trim()).unwrap_or("");

        if command.is_empty() {
            return;
        }

        let result = Command::new("sh").arg("-c").arg(command).output();

        match result {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
                    self.output_text.push(stdout_str);
                }
                if !output.stderr.is_empty() {
                    let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();
                    self.error_text.push(stderr_str);
                }
            }
            _ => {} // TODO: create an alert here, that you command failed to run
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_title = Line::from("Spliminal".bold());
        let input_title = Line::from("input");
        let output_title = Line::from("output").alignment(Alignment::Right);
        let error_title = Line::from("error").alignment(Alignment::Center);

        let input_color = if self.focus == FocusBlock::Input {
            Color::Cyan
        } else {
            Color::Reset
        };
        let output_color = if self.focus == FocusBlock::Output {
            Color::Cyan
        } else {
            Color::Reset
        };
        let error_color = if self.focus == FocusBlock::Error {
            Color::Cyan
        } else {
            Color::Reset
        };

        let outer_block = Block::bordered()
            .title(outer_title.centered())
            .border_set(border::THICK);
        let outer_inner_area = outer_block.inner(area);
        outer_block.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(5, 6), Constraint::Ratio(1, 6)])
            .split(outer_inner_area);

        let main_block = Block::default().borders(Borders::NONE);
        let main_inner_area = main_block.inner(chunks[0]);
        main_block.render(chunks[0], buf);

        let error_block = Block::bordered()
            .title(error_title)
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(error_color));
        let error_area = error_block.inner(chunks[1]);
        error_block.render(chunks[1], buf);
        let error_lines: Vec<Line> = self
            .error_text
            .iter()
            .enumerate()
            .map(|(i, s)| Line::from(format!("[{}]: {}", i + 1, s)))
            .collect();
        let error_paragraph = Paragraph::new(error_lines);
        error_paragraph.render(error_area, buf);

        let inner_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(main_inner_area);

        let input_block = Block::bordered()
            .title(input_title)
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(input_color));
        let input_area = input_block.inner(inner_chunks[0]);
        input_block.render(inner_chunks[0], buf);
        let input_lines: Vec<Line> = self
            .input_text
            .iter()
            .enumerate()
            .map(|(i, s)| Line::from(format!("[{}]: {}", i, s)))
            .collect();
        let input_paragraph = Paragraph::new(input_lines);
        input_paragraph.render(input_area, buf);

        let output_block = Block::bordered()
            .title(output_title)
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(output_color));
        let output_area = output_block.inner(inner_chunks[1]);
        output_block.render(inner_chunks[1], buf);
        let output_lines: Vec<Line> = self
            .output_text
            .iter()
            .enumerate()
            .map(|(i, s)| Line::from(format!("[{}]: {}", i, s)))
            .collect();
        let output_paragraph = Paragraph::new(output_lines);

        output_paragraph.render(output_area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
