use shapoist_core::system::core_structs::PlayMode;
use shapoist_core::system::core_structs::Diffculty;
use crate::Icon;
use crate::MainRouter;
use crate::Router;
use shapoist_core::system::core_structs::ShapoistCore;
use nablo::prelude::*;

pub fn detail(router: &mut Router, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore) {
	let (info, image_size) = if let Some((_, info)) = &core.current_chart {
		(info.clone(), Vec2::new(info.image_size.0 as f32, info.image_size.1 as f32))
	}else {
		*router = Router::Main(MainRouter::default());
		return;
	};
	let path = format!("{}/back.png", info.path.display());
	if let Err(e) = ui.create_texture_from_path(path.clone(), path.clone()) {
		msg.message(format!("{}", e), ui);
	};
	ui.put(Canvas::new(ui.window_area().width_and_height(), |painter| {
		let image_size_after = image_size * Vec2::same(image_size.x / ui.window_area().width());
		if image_size_after.x < ui.window_area().width() {
			painter.set_position(Vec2::new((ui.window_area().width() - image_size_after.x) / 2.0, 0.0));
		}
		if image_size_after.y < ui.window_area().height() {
			painter.set_position(Vec2::new(0.0, (ui.window_area().height() - image_size_after.y) / 2.0));
		}
		painter.set_scale(Vec2::same(image_size.x / ui.window_area().width()));
		painter.image(path.clone(), image_size);
	}), ui.window_area());
	ui.show(&mut Card::new("header").set_position(Vec2::same(16.0)).set_height(64.0).set_color([0,0,0,0]), |ui,_ |{
		ui.horizental(|ui| {
			if ui.button("back").is_clicked() {
				ui.delete_texture(path.clone());
				*router = Router::Main(MainRouter::default());
			}
			ui.horizental_inverse(|ui| {
				if ui.put(Button::new("").icon(Vec2::same(32.0), |painter| {
					Icon::More.draw(painter, Vec2::same(32.0));
				}).set_padding(0.0), Area::new(ui.available_position(), ui.available_position() + Vec2::same(32.0))).is_clicked() {
					msg.message("coming soon", ui);
				};
				if ui.button("edit").is_clicked() {
					if let Err(e) = core.edit() {
						msg.message(format!("{}", e), ui);
					}else {
						ui.delete_texture(path.clone());
						*router = Router::Edit(Default::default());
					};
				}
			});
		});
	});
	let height = 332.0;
	ui.show(&mut Card::new("footer").set_position(Vec2::new(16.0, ui.window_area().height() - height - 16.0)).set_rounding(Vec2::same(10.0)).set_color([0,0,0,0]).set_height(height), |ui, _| {
		ui.horizental(|ui| {
			ui.card("basic info", Vec2::new(300.0, height - 32.0), |ui,_ | {
				ui.label(&info.song_name);
				ui.label(&info.producer);
				ui.label(&info.charter);
				ui.label(&info.artist);
				ui.label(format!("version: {}", info.version));
				match &info.diffculty {
					Diffculty::Shapoist(read, play) => {
						ui.label(format!("diffculty: ({},{})", read, play))
					},
					Diffculty::Other(inner) => ui.label(inner),
				};
				if info.bpm.linkers.len() == 0 {
					ui.label(format!("Bpm:{}", info.bpm.start_bpm));
				}else {
					ui.button(format!("Bpm Detail"));
				}
			});
			ui.card("play info", Vec2::new(ui.window_area().width() - 300.0 - 16.0, height - 32.0), |ui,_ | {
				let play_history = info.history.clone().unwrap_or_default();
				ui.label("high score");
				ui.add(Label::new(play_history.high_score.to_string()).set_scale(Vec2::same(2.0)));
				ui.label("high accurcy");
				ui.add(Label::new(play_history.high_accurcy.to_string()).set_scale(Vec2::same(2.0)));
				ui.horizental_inverse(|ui| {
					if ui.button("start").is_clicked() {
						if let Err(e) = core.play(PlayMode::Normal) {
							msg.message(format!("{}", e), ui);
						}else {
							ui.delete_texture(path);
							*router = Router::PlayPage;
						};
					};
				});
			});
		});
		
	});
}