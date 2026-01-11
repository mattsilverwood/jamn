use std::{io};

use ratatui::{
    DefaultTerminal, Frame, crossterm::{
        self,
        event::{KeyCode, KeyEvent, KeyEventKind},
    }, layout::Layout, style::{Color, Stylize}, text::Line, widgets::Widget
};

enum RunningState {
    Running,
    Exiting,
}

pub struct JamnApp {
    state: RunningState,
    progress_bar_color: Color,
}

impl JamnApp {
    pub fn init_and_run() -> io::Result<()> {
        let mut term = ratatui::init();
        let mut app = Self::default();

        let res = app.run(&mut term);

        ratatui::restore();
        res
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        // Syntax can be confusing here. If NOT Exiting.
        while !matches!(self.state, RunningState::Exiting) {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => self.handle_key_event(key)?,
                _ => {}
            };
            term.draw(|f| self.draw(f))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => self.state = RunningState::Exiting,
                _ => {}
            }
        }
        Ok(())
    }
}

impl Widget for &JamnApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vert_layout = Layout::vert
        Line::from("Jamn").bold().render(area, buf);
    }
}

impl Default for JamnApp {
    fn default() -> Self {
        Self {
            state: RunningState::Running,
        }
    }
}
