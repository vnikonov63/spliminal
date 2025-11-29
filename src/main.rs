extern crate crossterm;
extern crate ratatui;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Widget},
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Tab => self.next_focus(),
            KeyCode::BackTab => self.prev_focus(),
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

        let error_block = Block::bordered()
            .title(error_title)
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(error_color));
        let main_block = Block::default().borders(Borders::NONE);
        let main_inner_area = main_block.inner(chunks[0]);

        main_block.render(chunks[0], buf);
        error_block.render(chunks[1], buf);

        let inner_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(main_inner_area);

        let input_block = Block::bordered()
            .title(input_title)
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(input_color));
        let output_block = Block::bordered()
            .title(output_title)
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(output_color));

        input_block.render(inner_chunks[0], buf);
        output_block.render(inner_chunks[1], buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
