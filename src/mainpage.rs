use std::path::PathBuf;
use shapoist_core::system::core_structs::ChartInfo;
use crate::Router;
use shapoist_core::system::core_structs::ShapoistCore;
use crate::Icon;
use shapoist_core::system::core_structs::Settings;
use nablo::prelude::*;
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
use rfd::AsyncFileDialog;

#[macro_export] macro_rules! button_with_msg {
	($n: expr, $t: expr, $ui: expr, $msg: expr) => {
		if $ui.button($n).is_clicked() {
			if let Err(e) = $t {
				$msg.message(format!("{}", e), $ui);
			}else {
				$msg.message("operation success", $ui);
			};
		}
	};
}

pub fn mainpage(router: &mut Router, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore) {
	ui.show(&mut Card::new("sidebar")
	.set_color(ui.style().background_color.brighter(0.1))
	.set_position(Vec2::same(ui.style().space))
	.set_width(96.0)
	.set_rounding(Vec2::same(10.0)), |ui, _| {
		if ui.put(SelectableValue::new(router.is_user(), "").icon(Vec2::same(64.0), |painter| {
			Icon::User.draw(painter, Vec2::same(64.0))
		}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
			*router = MainRouter::User(UserRouter::default()).into();
		};
		if ui.put(SelectableValue::new(router.is_main(), "").icon(Vec2::same(64.0), |painter| {
			Icon::Home.draw(painter, Vec2::same(64.0))
		}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
			*router = MainRouter::default().into();
		};
		if ui.put(SelectableValue::new(router.is_shop(), "").icon(Vec2::same(64.0), |painter| {
			Icon::Shop.draw(painter, Vec2::same(64.0))
		}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
			*router = MainRouter::Shop(ShopRouter {}).into();
		};
		if ui.put(SelectableValue::new(router.is_edit(), "").icon(Vec2::same(64.0), |painter| {
			Icon::Edit.draw(painter, Vec2::same(64.0))
		}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
			*router = MainRouter::Edit(EditRouter::default()).into();
		};
		ui.vertical_inverse(|ui| {
			if ui.put(SelectableValue::new(router.is_setting(), "").icon(Vec2::same(64.0), |painter| {
				Icon::Setting.draw(painter, Vec2::same(64.0))
			}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(64.0))).is_clicked() {
				*router = MainRouter::Setting(SettingRouter {}).into();
			};
		});
	});
	ui.show(&mut Card::new("router_page")
	.set_color(ui.style().background_color.brighter(0.05))
	.set_position(Vec2::new(96.0, 0.0) + Vec2::same(ui.style().space))
	.set_rounding(Vec2::same(10.0)),
	|ui, _| {
		match router {
			Router::Main(MainRouter::Main(_)) => {
				ui.show(&mut Card::new("header").set_height(64.0).set_color([0,0,0,0]), |ui, _| {
					ui.horizental_inverse(|ui| {
						if ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
							Icon::More.draw(painter, Vec2::same(32.0));
						}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0))).is_clicked() {
							msg.message("coming soon", ui);
						};
						if ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
							Icon::Add.draw(painter, Vec2::same(32.0));
						}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0))).is_clicked() {
							*router = MainRouter::Edit(Default::default()).into();
						};
						if ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
							Icon::Search.draw(painter, Vec2::same(32.0));
						}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0))).is_clicked() {
							msg.message("coming soon", ui);
						};
						if ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
							Icon::Filter.draw(painter, Vec2::same(32.0));
						}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0))).is_clicked() {
							msg.message("coming soon", ui);
						};
					});
				});
				if let Some(t) = &mut ui.show(&mut Card::new("content").set_color([0,0,0,0]).set_scrollable([true; 2]), |ui, _| -> Option<ChartInfo> {
					let mut chart_to_change = None;
					for chart in &mut core.chart_list {
						let name = chart.song_name.replace("\n", "");
						let width = ui.window_area().width() - 48.0;
						let height = 90.0;
						if ui.put(Button::new("").icon(Vec2::new(width, height), |painter| {
							let text_area = painter.text_area(name.clone());
							painter.set_position(Vec2::new(16.0, height - text_area.height() - 12.0));
							if text_area.width() > width - 16.0 {
								painter.text(utf8_slice::till(&name, ((utf8_slice::len(&name) as f32 * text_area.width() / (width - 16.0)) as usize).checked_sub(3).unwrap_or(0)).to_owned() + "...");
							}else {
								painter.text(name.clone());
							}
						}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::new(width, height))).is_clicked() {
							chart_to_change = Some(chart.clone())
						};
					}
					chart_to_change
				}).return_value {
					if let Some(t) = t {
						if let Err(e) = core.read_chart(t) {
							msg.message(format!("{}", e), ui);
						}else {
							*router = Router::Detail;
						}
					}
				};
			},
			Router::Main(MainRouter::User(router)) => {
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
			Router::Main(MainRouter::Shop(_)) => {
				ui.show(&mut Card::new("umm").set_color(ui.style().background_color.brighter(0.1)).set_scrollable([true; 2]).set_rounding(Vec2::same(16.0)), |ui, _| {
					ui.label("coming soon");
				});
			},
			Router::Main(MainRouter::Setting(_)) => { 
				ui.show(&mut Card::new("settings").set_color(ui.style().background_color.brighter(0.1)).set_scrollable([true; 2]).set_rounding(Vec2::same(10.0)), |ui, _| {
					if let Err(e) = settings(&mut core.settings, "shapoist settings", ui) {
						ui.label(format!("error: {}", e));
					};
					button_with_msg!("save", core.settings_are_changed(), ui, msg);
					button_with_msg!("reload resource", core.reload_all(), ui, msg);
					button_with_msg!("check", core.chart_list[0].check(), ui, msg);
					if ui.button("reset").is_clicked() {
						core.settings = Settings::default();
					}
				});
			},
			Router::Main(MainRouter::Edit(inner)) => {
				if let Some(t) = ui.show(&mut Card::new("umm").set_color(ui.style().background_color.brighter(0.1)).set_scrollable([true; 2]).set_rounding(Vec2::same(16.0)), |ui, _| -> bool {
					ui.add(SingleTextInput::new(&mut inner.song_name).place_holder("song name").limit(64));
					ui.add(SingleTextInput::new(&mut inner.producer).place_holder("producer").limit(64));
					ui.add(SingleTextInput::new(&mut inner.charter).place_holder("charter").limit(64));
					ui.add(SingleTextInput::new(&mut inner.artist).place_holder("artist").limit(64));
					cfg_if::cfg_if! {
						if #[cfg(target_os = "android")] {
							ui.label("the chart create functions is not available on moblie devices");
						}else if #[cfg(target_arch = "wasm32")] {
							ui.label("the chart create functions is not available on web platfrom");
						}else {
							ui.horizental(|ui| {
								if ui.button("select image").is_clicked() {
									if let Some(t) = pollster::block_on(AsyncFileDialog::new().add_filter("image", &["png"]).set_directory("/").pick_file()) {
										inner.image_path = t.into()
									};
								}
								ui.label(format!("{}", inner.image_path.display()));
							});
							ui.horizental(|ui| {
								if ui.button("select track").is_clicked() {
									if let Some(t) = pollster::block_on(AsyncFileDialog::new().add_filter("track", &["mp3"]).set_directory("/").pick_file()) {
										inner.track_path = t.into()
									};
								}
								ui.label(format!("{}", inner.track_path.display()));
							});
						}
					}
					let back = ui.horizental_inverse(|ui| -> bool {
						let back = if ui.add(Button::new("create").icon(Vec2::same(16.0), |painter| {
							painter.set_color(1.0);
							painter.cir(8.0);
						})).is_clicked() {
							if let Err(e) = core.create_new_chart(
								inner.song_name.clone(), 
								inner.producer.clone(), 
								inner.charter.clone(), 
								inner.artist.clone(), 
								inner.track_path.clone(), 
								inner.image_path.clone()
							){
								msg.message(format!("{}", e), ui);
								false
							}else {
								true
							}
						}else {
							false
						};
						if ui.add(SelectableValue::new(false, "clear").icon(Vec2::same(16.0), |painter| {
							painter.set_color(1.0);
							painter.cir(8.0);
						})).is_clicked() {
							*inner = EditRouter::default();
						};
						back
					});
					ui.label(format!("{}", ui.delay()));
					back
				}).return_value {
					if t {
						if let Err(e) = core.edit() {
							msg.message(format!("{}", e), ui);
						}else {
							*router = Router::Edit(Default::default());
						};
					}
				};
			}
			_ => unreachable!()
		}
	});
}

impl Into<Router> for MainRouter {
	fn into(self) -> Router {
		Router::Main(self)
	}
}

pub enum MainRouter {
	Main(MainInner),
	Shop(ShopRouter),
	User(UserRouter),
	Setting(SettingRouter),
	Edit(EditRouter)
}

impl Default for MainRouter {
	fn default() -> Self {
		Self::Main(MainInner::default())
	}
}

#[derive(Default)]
pub struct MainInner {}

#[derive(Default)]
pub struct ShopRouter {}

#[derive(Default)]
pub struct UserRouter {
	account: String,
	password: String,
}

#[derive(Default)]
pub struct SettingRouter {}

#[derive(Default)]
pub struct EditRouter {
	song_name: String,
	producer: String,
	charter: String,
	artist: String,
	track_path: PathBuf,
	image_path: PathBuf,
}

impl Router {
	fn is_main(&self) -> bool {
		if let Router::Main(MainRouter::Main(_)) = self {
			return true;
		}
		false
	}

	fn is_shop(&self) -> bool {
		if let Router::Main(MainRouter::Shop(_)) = self {
			return true;
		}
		false
	}

	fn is_setting(&self) -> bool {
		if let Router::Main(MainRouter::Setting(_)) = self {
			return true;
		}
		false
	}

	fn is_user(&self) -> bool {
		if let Router::Main(MainRouter::User(_)) = self {
			return true;
		}
		false
	}

	fn is_edit(&self) -> bool {
		if let Router::Main(MainRouter::Edit(_)) = self {
			return true;
		}
		false
	}
}
