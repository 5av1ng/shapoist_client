use nablo::prelude::shape_elements::Rect;
use std::result::Result::Ok;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use nablo_shape::math::Vec2;
#[cfg(not(target_arch = "wasm32"))]
use log::*;
use nablo::prelude::*;
use shapoist_core::system::core_structs::*;
use shapoist_core::system::Error as ShapoistError;
use anyhow::*;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

enum Icon {
	Home,
	Shop,
	Setting,
	Add,
	Search,
	Filter,
	More,
}

impl Icon {
	#[cfg(not(target_arch = "wasm32"))]
	fn init(ui: &mut Ui) -> Result<()> {
		log::info!("inititialing icons...");
		let path_read = fs::read_dir("./assets/icons")?;
		for path in path_read {
			let path = path?.path();
			log::debug!("find: {}", path.display());
			if path.is_file() && path.extension().is_some() {
				let extension = path.extension().unwrap();
				if let Some(t) = extension.to_str() {
					if t == "png" {
						log::debug!("latest finding is a png file, processing...");
						ui.create_texture_from_path(path.clone(), path.file_name().unwrap().to_str().unwrap())?;
					}
				}
			}
		}
		Ok(())
	}

	#[cfg(target_arch = "wasm32")]
	fn init(ui: &mut Ui) -> Result<()> {
		ui.create_texture(include_bytes!("../assets/icons/add.png"), "add.png")?;
		ui.create_texture(include_bytes!("../assets/icons/filter.png"), "filter.png")?;
		ui.create_texture(include_bytes!("../assets/icons/home.png"), "home.png")?;
		ui.create_texture(include_bytes!("../assets/icons/more.png"), "more.png")?;
		ui.create_texture(include_bytes!("../assets/icons/setting.png"), "setting.png")?;
		ui.create_texture(include_bytes!("../assets/icons/search.png"), "search.png")?;
		ui.create_texture(include_bytes!("../assets/icons/shop.png"), "shop.png")?;
		ui.create_texture(include_bytes!("../assets/icons/user.png"), "user.png")?;
		Ok(())
	}

	fn draw(&self, painter: &mut Painter, size: Vec2) {
		match &self {
			Self::Home => painter.image("home.png", size),
			Self::Shop => painter.image("shop.png", size),
			Self::Setting => painter.image("setting.png", size),
			Self::Add => painter.image("add.png", size),
			Self::Search => painter.image("search.png", size),
			Self::Filter => painter.image("filter.png", size),
			Self::More => painter.image("more.png", size),
		};
	}
}

struct Shapoist {
	core: Option<Result<ShapoistCore, ShapoistError>>,
	router: Router,
	is_icon_inititialized: bool,
	test_number: f32
}

enum Router {
	Main(MainRouter),
	Shop(ShopRouter),
	User(UserRouter),
	Setting(SettingRouter)
}

struct MainRouter {
	page: usize,
	total_page: usize
}

struct ShopRouter {}

#[derive(Default)]
struct UserRouter {
	account: String,
	password: String,
}

struct SettingRouter {}

impl Shapoist {
	fn init() -> Self {
		Self {
			core: None,
			router: Router::Main(MainRouter {
				page: 1,
				total_page: 1
			}),
			is_icon_inititialized: false,
			test_number: 100.0
		}
	}
}

impl App for Shapoist {
	fn app(&mut self, ui: &mut Ui) {
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
		ui.message_provider("msgprov", |ui, msg| {
			ui.show(&mut Card::new("sidebar")
			.set_color(ui.style().background_color.brighter(0.1))
			.set_position(Vec2::same(ui.style().space))
			.set_width(96.0)
			.set_rounding(Vec2::same(10.0)), |ui, _| {
				if ui.put(SelectableValue::new(self.router.is_user(), "").icon(Vec2::same(64.0), |painter| {
					painter.image_mask("user.png", Vec2::same(64.0), ShapeMask::Rect(Rect { width_and_height: Vec2::same(64.0), rounding: Vec2::same(32.0) }));
				}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
					self.router = Router::User(UserRouter::default());
				};
				if ui.put(SelectableValue::new(self.router.is_main(), "").icon(Vec2::same(64.0), |painter| {
					Icon::Home.draw(painter, Vec2::same(64.0))
				}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
					self.router = Router::Main(MainRouter {
						page: 1,
						total_page: 1
					});
				};
				if ui.put(SelectableValue::new(self.router.is_shop(), "").icon(Vec2::same(64.0), |painter| {
					Icon::Shop.draw(painter, Vec2::same(64.0))
				}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
					self.router = Router::Shop(ShopRouter {});
				};
				ui.vertical_inverse(|ui| {
					if ui.put(SelectableValue::new(self.router.is_setting(), "").icon(Vec2::same(64.0), |painter| {
						Icon::Setting.draw(painter, Vec2::same(64.0))
					}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
						self.router = Router::Setting(SettingRouter {});
					};
				});
			});
			ui.show(&mut Card::new("router_page")
			.set_color(ui.style().background_color.brighter(0.05))
			.set_position(Vec2::new(96.0, 0.0) + Vec2::same(ui.style().space))
			.set_rounding(Vec2::same(10.0)),
			|ui, _| {
				match &mut self.router {
					Router::Main(router) => {
						ui.show(&mut Card::new("header").set_height(64.0).set_color([0,0,0,0]), |ui, _| {
							ui.horizental_inverse(|ui| {
								ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
									Icon::More.draw(painter, Vec2::same(32.0));
								}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0)));
								ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
									Icon::Add.draw(painter, Vec2::same(32.0));
								}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0)));
								ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
									Icon::Search.draw(painter, Vec2::same(32.0));
								}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0)));
								ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
									Icon::Filter.draw(painter, Vec2::same(32.0));
								}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0)));
							});
						});
						ui.show(&mut Card::new("content").set_height(ui.window_area().height() - 192.0).set_color([0,0,0,0]), |ui, _| {
							ui.add(Label::new(format!("{}", 1.0 / ui.delay().as_seconds_f32())));
						});
						ui.show(&mut Card::new("footer").set_color([0,0,0,0]), |ui, _| {
							ui.horizental(|ui| {
								for index in 1..=router.total_page {
									if ui.add(Canvas::new(Vec2::same(32.0), |painter| {
										painter.set_color(1.0);
										painter.rect(Vec2::same(32.0), Vec2::same(5.0));
									})).is_clicked() {
										router.page = index
									};
								}
							})
						});
					},
					Router::User(router) => {
						ui.show(&mut Card::new("login")
						.set_color(ui.style().background_color.brighter(0.1))
						.set_rounding(Vec2::same(10.0))
						.set_height(160.0)
						.set_width(400.0)
						.set_position(Vec2::new((ui.window_area().width() - 400.0) / 2.0, (ui.window_area().height() - 176.0) / 2.0)),
						|ui, _| {
							ui.add(SingleTextInput::new(&mut router.account).place_holder("username").limit(64));
							ui.add(SingleTextInput::new(&mut router.password).place_holder("password").limit(16).password(true));
							ui.horizental_inverse(|ui| {
								ui.add(Button::new("sign in").icon(Vec2::same(16.0), |painter| {
									painter.set_color(1.0);
									painter.cir(8.0);
								}));
								ui.add(SelectableValue::new(false, "sign up").icon(Vec2::same(16.0), |painter| {
									painter.set_color(1.0);
									painter.cir(8.0);
								}));
							})
						});
					},
					Router::Shop(_) => {
						ui.show(&mut Card::new("umm").set_color(ui.style().background_color.brighter(0.1)).set_scrollable([true; 2]), |ui, card| {
							for _ in 0..20 {
								if ui.add(Button::new("")).is_clicked() {
									card.scroll_to(Vec2::same(200.0), ui);
								};
							}
							ui.show(&mut Collapsing::new("214214214"), |ui, _| {
								ui.show(&mut Collapsing::new("asdasdasd"), |ui, _| {
									ui.add(Slider::new(0.0..=500.0, &mut self.test_number).set_text("umm").prefix("test: ").suffix("deg"));
									ui.add(DragableValue::new(&mut self.test_number));
								});
								ui.add(Slider::new(0.0..=500.0, &mut self.test_number).set_text("umm").prefix("test: ").suffix("deg"));
								ui.add(DragableValue::new(&mut self.test_number));
							});
							settings(&mut core.settings, "shapoist settings".into(), ui).unwrap();
							ui.horizental(|ui| {
								for i in 0..20 {
									if ui.add(Button::new("")).is_clicked() {
										msg.message(format!("{}", i), ui);
									};
								}
							});
						});
					},
					Router::Setting(_) => { 
						ui.show(&mut Card::new("settingssssssss").set_color(ui.style().background_color.brighter(0.1)).set_scrollable([true, true]).set_dragable(true).set_height(200.0).set_width(200.0), |ui, _| {
							for i in 0..20 {
								if ui.add(Button::new("")).is_clicked() {
									msg.message(format!("{}", i), ui);
								};
							}
						});
					}
				}
			});
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


impl Router {
	fn is_main(&self) -> bool {
		if let Router::Main(_) = self {
			return true;
		}
		false
	}

	fn is_shop(&self) -> bool {
		if let Router::Shop(_) = self {
			return true;
		}
		false
	}

	fn is_setting(&self) -> bool {
		if let Router::Setting(_) = self {
			return true;
		}
		false
	}

	fn is_user(&self) -> bool {
		if let Router::User(_) = self {
			return true;
		}
		false
	}
}
