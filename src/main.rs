mod config;
mod enums;
mod exc;
mod ext;
mod fmt;
mod models;
mod output;
mod traits;
mod utils;

use log::debug;
use models::Pls;

/// Create a `Pls` instance and immediately delegate to it.
///
/// This is the entry point of the application.
fn main() {
	env_logger::init();
	debug!("Hello!");

	let pls = Pls::default();
	pls.run();

	debug!("Bye!");
}
