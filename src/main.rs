use std::io;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::Text,
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

mod life;
mod tui;

pub struct App {
    board: life::Board,
    exit: bool,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Tutorial App ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);
        let contents: Text = self.board.to_string().into();
        Paragraph::new(contents)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let event::Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event)
            }
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('a') => self.board.get_next_state(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn default() -> App {
        let default_starting_board = life::Board::new(20, 10);
        App {
            board: default_starting_board.unwrap(),
            exit: false,
        }
    }
}

fn main() -> io::Result<()> {
    let mut starting_board = life::Board::new(38, 22).unwrap();
    starting_board.set(7, 7, life::Cell::Alive);
    starting_board.set(7, 8, life::Cell::Alive);
    starting_board.set(7, 10, life::Cell::Alive);
    starting_board.set(7, 11, life::Cell::Alive);
    starting_board.set(7, 12, life::Cell::Alive);
    starting_board.set(7, 13, life::Cell::Alive);
    starting_board.set(7, 15, life::Cell::Alive);
    starting_board.set(7, 16, life::Cell::Alive);
    starting_board.set(6, 9, life::Cell::Alive);
    starting_board.set(8, 9, life::Cell::Alive);
    starting_board.set(6, 14, life::Cell::Alive);
    starting_board.set(8, 14, life::Cell::Alive);
    let mut terminal = tui::init()?;
    let app_result = App {
        board: starting_board,
        exit: false,
    }
    .run(&mut terminal);
    tui::restore()?;
    app_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.exit, true);

        Ok(())
    }
}
