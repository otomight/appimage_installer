use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{Block, Borders};

use crate::{traits::{Screen, ScreenResult}, AppContext};

pub struct MainMenu;


impl Screen<AppContext> for MainMenu {
	fn render(&self, f: &mut ratatui::Frame, ctx: &AppContext) {
		let block = Block::default()
			.title(format!("Hello {}", ctx.name))
			.borders(Borders::ALL);
		f.render_widget(block, f.area());
	}

	fn handle_input(&mut self, key: KeyEvent, ctx: &mut AppContext) -> ScreenResult<AppContext> {
		match key.code {
			KeyCode::Char('q') => ScreenResult::Quit,
			_ => ScreenResult::None,
		}
	}
}
