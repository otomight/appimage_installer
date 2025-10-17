use ratatui::Frame;
use crossterm::event::KeyEvent;


pub enum ScreenResult<C> {
	None,
	Change(Box<dyn Screen<C>>),
	Quit,
}

pub trait Screen<C> {
	fn render(&self, f: &mut Frame, ctx: &C);
	fn handle_input(&mut self, key: KeyEvent, ctx: &mut C) -> ScreenResult<C>;
}
