#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::blocks_in_conditions)]

#[cfg(not(target_arch = "wasm32"))]
use log::*;
use nablo::prelude::shape_elements::Layer;
use crate::edit::EditRouter;
use crate::edit::edit;
use crate::result_page::result_page;
use crate::playpage::playpage;
use crate::detail::detail;
use crate::mainpage::mainpage;
use crate::mainpage::MainRouter;
use crate::resources::Icon;
use std::result::Result::Ok;
use nablo::prelude::*;
use shapoist_core::system::core_structs::*;
use shapoist_core::system::Error as ShapoistError;
use anyhow::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

mod mainpage;
mod resources;
mod detail;
mod playpage;
mod result_page;
mod edit;

pub struct Shapoist {
	core: Option<Result<ShapoistCore, ShapoistError>>,
	router: Router,
	is_icon_inititialized: bool,
	#[cfg(target_os = "android")]
	android_app: Option<AndroidApp>
}

pub enum Router {
	Main(MainRouter),
	Detail {
		is_auto: bool,
	},
	PlayPage,
	ResultPage,
	Edit(Box<EditRouter>)
}

impl Shapoist {
	pub fn init() -> Self {
		Self {
			core: None,
			router: Router::Main(MainRouter::default()),
			is_icon_inititialized: false,
			#[cfg(target_os = "android")]
			android_app: None,
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
			if self.core.is_none() {
				cfg_if::cfg_if! {
					if #[cfg(target_os = "android")] {
						use std::path::PathBuf;
						let android_app = self.android_app.clone().unwrap();
						let path = format!("{}", android_app.external_data_path()
							.unwrap_or(android_app.internal_data_path()
								.unwrap_or(PathBuf::from("data/data/com.saving.shapoist")))
							.display()
						);
						self.core = Some(ShapoistCore::new(&path));
					}else if #[cfg(target_arch = "wasm32")] {
						self.core = Some(Ok(ShapoistCore::minimal()));
					}else {
						self.core = Some(ShapoistCore::new("./"));
					}
				}
				
			}
			let core = if let Some(t) = &mut self.core {
				match t {
					Ok(core) => core,
					Err(e) => {
						msg.message(format!("{}", e), ui);
						return;
					}
				}
			}else {
				unreachable!()
			};
			if let Err(e) = core.frame() {
				msg.message(format!("{}", e), ui)
			};
			match &mut self.router {
				Router::Main(_) => mainpage(&mut self.router, ui, msg, core),
				Router::Detail{ .. } => detail(&mut self.router, ui, msg, core),
				Router::PlayPage => playpage(&mut self.router, ui, msg, core),
				Router::ResultPage => result_page(&mut self.router, ui, msg, core),
				Router::Edit(_) => edit(&mut self.router, ui, msg, core),
			}
			ui.paint_style_mut().layer = Layer::Bottom;
			ui.put(Label::new(format!("{:.2}fps", 1.0 / ui.delay().as_seconds_f32())), Area::new(ui.window_area().right_bottom() - Vec2::new(128.0, 48.0), ui.window_area().right_bottom()));
		});
	}
	#[cfg(target_os = "android")]
	fn android_app(&mut self, app: nablo::prelude::AndroidApp) {
		self.android_app = Some(app);
	}
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
#[cfg(not(target_os = "android"))]
fn run() {
	let _ = Manager::new_with_settings(Shapoist::init(), nablo::Settings {
		title: "Shapoist".into(),
		..Default::default()
	}).run();
}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
	log();
	run();
}

#[allow(dead_code)]
#[cfg(target_os = "android")]
fn main() {}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
	log();
	let _ = Manager::new_with_settings(Shapoist::init(), nablo::Settings {
		title: "Shapoist".into(),
		..Default::default()
	}, app).run();
}

fn log() {
	cfg_if::cfg_if! {
		if #[cfg(target_arch = "wasm32")] {
			std::panic::set_hook(Box::new(console_error_panic_hook::hook));
			console_log::init_with_level(log::Level::Info).expect("az");
		}else if #[cfg(target_os = "android")] {
			android_logger::init_once(
				android_logger::Config::default()
					.with_max_level(LevelFilter::Info)
					.with_filter(android_logger::FilterBuilder::new()
						.filter(Some("shapoist_core::system::io_functions"), LevelFilter::Debug)
						.filter(Some("shapoist_core::system::command"), LevelFilter::Debug)
						.filter(Some("shapoist_core::system::core_functions"), LevelFilter::Info)
						.filter(Some("shapoist_core::system::core_structs"), LevelFilter::Debug)
						.filter(Some("shapoist_core::system::timer"), LevelFilter::Debug)
						.filter(Some("nablo::ui"), LevelFilter::Debug)
						.filter(Some("shapoist_client"), LevelFilter::Warn)
					.build()
				),
			);
		}else {
			env_logger::Builder::new()
				.filter(Some("shapoist_core::system::io_functions"), LevelFilter::Debug)
				.filter(Some("shapoist_core::system::command"), LevelFilter::Debug)
				.filter(Some("shapoist_core::system::core_functions"), LevelFilter::Info)
				.filter(Some("shapoist_core::system::core_structs"), LevelFilter::Debug)
				.filter(Some("shapoist_core::system::timer"), LevelFilter::Debug)
				.filter(Some("nablo::ui"), LevelFilter::Debug)
				.filter(Some("shapoist_client"), LevelFilter::Warn)
				.init();
		}
	}
}