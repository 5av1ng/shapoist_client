use crate::detail::detail;
use crate::mainpage::mainpage;
use crate::mainpage::MainRouter;
use crate::resources::Icon;
use std::result::Result::Ok;
#[cfg(not(target_arch = "wasm32"))]
use log::*;
use nablo::prelude::*;
use shapoist_core::system::core_structs::*;
use shapoist_core::system::Error as ShapoistError;
use anyhow::*;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

mod mainpage;
mod resources;
mod detail;

struct Shapoist {
	core: Option<Result<ShapoistCore, ShapoistError>>,
	router: Router,
	is_icon_inititialized: bool,
}

enum Router {
	Main(MainRouter),
	Detail
}

impl Shapoist {
	fn init() -> Self {
		Self {
			core: None,
			router: Router::Main(MainRouter::Main),
			is_icon_inititialized: false,
		}
	}
}

impl App for Shapoist {
	fn app(&mut self, ui: &mut Ui) {
		ui.message_provider("msgprov", |ui, msg| {
			if !self.is_icon_inititialized {
				match Icon::init(ui) {
					Ok(()) => self.is_icon_inititialized = true,
					Err(e) => {
						ui.add(Label::new(format!("{}", e)));
						return
					}
				}
			}
			let core = if let Some(t) = &mut self.core {
				match t {
					Ok(core) => core,
					Err(e) => {
						ui.add(Label::new(format!("{}", e)));
						return;
					}
				}
			}else {
				self.core = Some(ShapoistCore::new("./"));
				return
			};
			if let Err(e) = core.frame() {
				ui.add(Label::new(format!("{}", e)));
				// TODO: change this to dialog
				panic!("{:?}", e);
			};
			match &mut self.router {
				Router::Main(_) => mainpage(&mut self.router, ui, msg, core),
				Router::Detail => detail(&mut self.router, ui, msg, core),
			}
		});
	}
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn run() {
	cfg_if::cfg_if! {
		if #[cfg(target_arch = "wasm32")] {
			std::panic::set_hook(Box::new(console_error_panic_hook::hook));
			console_log::init_with_level(log::Level::Info).expect("az");
		} else {
			env_logger::Builder::new()
			.filter(Some("shapoist_core::system::io_functions"), LevelFilter::Debug)
			.filter(Some("shapoist_core::system::command"), LevelFilter::Debug)
			.filter(Some("shapoist_core::system::core_functions"), LevelFilter::Debug)
			.filter(Some("shapoist_core::system::core_structs"), LevelFilter::Debug)
			.filter(Some("shapoist_core::system::timer"), LevelFilter::Debug)
			.filter(Some("nablo::ui"), LevelFilter::Debug)
			.filter(Some("shapoist_client"), LevelFilter::Warn)
			.init();
		}
	}
	let _ = Manager::new(Shapoist::init()).run();
}

#[allow(dead_code)]
fn main() {
	run();
}