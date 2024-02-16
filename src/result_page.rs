use shapoist_core::system::core_structs::Judge;
use crate::Router;
use shapoist_core::system::core_structs::ShapoistCore;
use crate::MainRouter;
use nablo::prelude::*;

pub fn result_page(router: &mut Router, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore) {
	ui.show(&mut Card::new("result_page").set_rounding(Vec2::same(16.0)).set_color(ui.style().background_color.brighter(0.1)), |ui, _| {
		let play_info = if let Some(t) = &core.play_info {
			t
		}else {
			*router = Router::Main(MainRouter::default());
			msg.message("play info failed to load", ui);
			return;
		};
		let info = if let Some((_,t2)) = &core.current_chart {
			t2
		}else {
			*router = Router::Main(MainRouter::default());
			msg.message("no chart loaded", ui);
			return;
		};
		let mut judge_info = vec!(0,0,0,0,0);
		for judge in &play_info.judge_vec {
			match judge {
				Judge::Immaculate(_) => judge_info[0] = judge_info[0] + 1,
				Judge::Extra => judge_info[1] = judge_info[1] + 1,
				Judge::Normal => judge_info[2] = judge_info[2] + 1,
				Judge::Fade => judge_info[3] = judge_info[3] + 1,
				Judge::Miss => judge_info[4] = judge_info[4] + 1,
			}
		}
		ui.label("Score");
		ui.add(Label::new(format!("{:0width$}", play_info.score, width = 7)).set_scale(Vec2::same(2.0)));
		ui.label("Accuracy");
		ui.add(Label::new(format!("{:.2}%", play_info.accuracy)).set_scale(Vec2::same(2.0)));
		ui.label(format!("Max Combo: {}", play_info.max_combo));
		ui.label(format!("Immaculate: {}, Extra: {}, Normal: {}, Fade: {}, Miss: {}", judge_info[0], judge_info[1], judge_info[2], judge_info[3], judge_info[4]));
		ui.label(&info.song_name);
		ui.label(&info.producer);
		ui.label(format!("play mode: {:?}", play_info.play_mode));
		ui.label("ui设计不存在的,直接列出来就完了.jpg");
		if ui.button("back").is_clicked() {
			*router = Router::Main(MainRouter::default())
		}
		if ui.button("retry").is_clicked() {
			if let Err(e) = core.play(play_info.play_mode.clone()) {
				msg.message(format!("{}", e), ui);
			}else {
				*router = Router::PlayPage;
			};
		}
	});
}