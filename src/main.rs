extern crate crossterm;
extern crate ratatui;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Widget},
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
        let output_title = Line::from("output").alignment(Alignment::Right);
        let error_title = Line::from("error").alignment(Alignment::Center);

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
            .border_set(border::ROUNDED);
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
            .border_set(border::ROUNDED);
        let output_block = Block::bordered()
            .title(output_title)
            .border_set(border::ROUNDED);

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
