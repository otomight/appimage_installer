use ratatui::widgets::ListState;

use crate::{components::MainMenu, traits::App};

mod traits;
mod components;

// Application state
struct AppContext {
	name: String
}

fn main() {
	let main_menu = MainMenu;
	let app_context = AppContext{
		name: String::from("Youpi")
	};
	let mut app = App::new(app_context, Box::new(main_menu));
	app.run();
}
