use std::{io, time::Duration};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{event, execute, terminal};

use crate::traits::{Screen, ScreenResult};

pub struct App<C> {
	context: C,
	current_screen: Box<dyn Screen<C>>,
	should_quit: bool,
}

impl<C> App<C> {
	pub fn new(context: C, first_screen: Box<dyn Screen<C>>) -> Self {
		Self {
			context,
			current_screen: first_screen,
			should_quit: false,
		}
	}

	pub fn run(&mut self) -> io::Result<()> {
		let mut stdout = io::stdout();
		execute!(stdout, terminal::EnterAlternateScreen)?;
		terminal::enable_raw_mode()?;
		let backend = CrosstermBackend::new(stdout);
		let mut terminal = Terminal::new(backend)?;

		while !self.should_quit {
			terminal.draw(|f| self.current_screen.render(f, &self.context))?;

			if event::poll(Duration::from_millis(200))? {
				if let event::Event::Key(key) = event::read()? {
					match self.current_screen.handle_input(key, &mut self.context) {
						ScreenResult::None => {}
						ScreenResult::Change(next) => self.current_screen = next,
						ScreenResult::Quit => self.should_quit = true,
					}
				}
			}
		}

		terminal::disable_raw_mode()?;
		execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
		terminal.show_cursor()?;
		Ok(())
	}

	// ─── Public API available to screens ─────────────────────────────

	pub fn context(&self) -> &C {
		&self.context
	}

	pub fn context_mut(&mut self) -> &mut C {
		&mut self.context
	}

	pub fn set_screen(&mut self, screen: Box<dyn Screen<C>>) {
		self.current_screen = screen;
	}

	pub fn quit(&mut self) {
		self.should_quit = true;
	}
}
