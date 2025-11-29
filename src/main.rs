extern crate crossterm;
extern crate ratatui;

use std::{error::Error, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
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
        if let KeyCode::Char('q') = key_event.code {
            self.exit();
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_title = Line::from("Spliminal".bold());
        let input_title = Line::from("input");
        let output_title = Line::from("output");
        let error_title = Line::from("error");

        let outer_block = Block::bordered()
            .title(outer_title.centered())
            .border_set(border::THICK);

        let inner_area = outer_block.inner(area);
        outer_block.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(inner_area);

        let input_block = Block::bordered()
            .title(input_title)
            .border_set(border::ROUNDED);
        let output_block = Block::bordered()
            .title(output_title)
            .border_set(border::ROUNDED);
        let error_block = Block::bordered()
            .title(error_title)
            .border_set(border::ROUNDED);

        input_block.render(chunks[0], buf);
        output_block.render(chunks[1], buf);
        error_block.render(chunks[2], buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
