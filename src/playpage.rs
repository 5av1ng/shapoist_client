use time::Duration;
use shapoist_core::system::core_structs::JudgeEvent;
use nablo::event::TouchPhase;
use shapoist_core::system::core_structs::Click;
use shapoist_core::system::core_structs::ClickState;
use shapoist_core::system::core_structs::Diffculty;
use crate::MainRouter;
use crate::Router;
use nablo::prelude::*;
use shapoist_core::system::core_structs::ShapoistCore;

pub fn playpage(router: &mut Router, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore) {
	let (chart, info) = if let Some((t1,t2)) = &core.current_chart {
		(t1, t2)
	}else {
		*router = Router::Main(MainRouter::default());
		msg.message("no chart loaded", ui);
		return;
	};
	let path = format!("{}/back.png", info.path.display());
	let play_info = if let Some(t) = &core.play_info {
		t
	}else {
		ui.delete_texture(path);
		*router = Router::Main(MainRouter::default());
		msg.message("play info failed to load", ui);
		return;
	};
	if play_info.is_finished {
		ui.delete_texture(path);
		*router = Router::ResultPage;
		return;
	}
	#[cfg(not(target_arch = "wasm32"))]
	if let Err(e) = ui.create_texture_from_path(path.clone(), path.clone()) {
		msg.message(format!("{}", e), ui);
	};
	let size = chart.size / chart.size.len();
	let window = ui.window_area();
	let canvas_size = if (window.width() * size).y < window.height() {
		window.width() * size
	}else {
		window.height() * size
	};
	let canvas_position = window.center() - canvas_size / 2.0;
	let scale_factor = canvas_size.x / chart.size.x;
	let time = format!("{:.2}s", (core.timer.read() - Duration::seconds(3)).as_seconds_f32());
	let res = ui.put(Canvas::new(canvas_size, |painter| {
		painter.image(path.clone(), Vec2::new(info.image_size.0 as f32, info.image_size.1 as f32));
		
		for shape in &play_info.render_queue {
			let mut shape = shape.shape.clone();
			shape.pre_scale(scale_factor);
			painter.push(shape);
		}
		let title_area = painter.text_area(info.song_name.clone());
		painter.set_position(Vec2::new(0.0, canvas_size.y) + Vec2::new(16.0, - title_area.height() - 16.0));
		painter.text(info.song_name.clone());

		let score = format!("{:0width$}", play_info.score, width = 7);
		let score_area = painter.text_area(score.clone());
		painter.set_position(Vec2::new(canvas_size.x , 0.0) + Vec2::new(- 16.0 - score_area.width(), 16.0));
		painter.text(score);
		let acc_area = painter.text_area(format!("{:.2}%", play_info.accuracy * 1e2));
		painter.set_position(Vec2::new(canvas_size.x , 0.0) + Vec2::new(- 16.0 - acc_area.width(), 32.0 + score_area.height()));
		painter.text(format!("{:.2}%", play_info.accuracy * 1e2));
		let time_area = painter.text_area(time.clone());
		painter.set_position(Vec2::new(canvas_size.x , 0.0) + Vec2::new(- 16.0 - time_area.width(), 48.0 + score_area.height() + acc_area.height()));
		painter.text(time);

		let diffculty = match &info.diffculty {
			Diffculty::Shapoist(read, play) => format!("({}, {})", read, play),
			Diffculty::Other(inner) => inner.clone(),
		};
		let diffculty_area = painter.text_area(diffculty.clone());
		painter.set_position(canvas_size - diffculty_area.width_and_height() - Vec2::same(16.0));
		painter.text(diffculty);

		let combo_area = painter.text_area(play_info.combo.to_string());
		painter.set_position(Vec2::new((canvas_size.x - combo_area.width()) / 2.0, 16.0));
		painter.text(play_info.combo.to_string());
		let playmode_area = painter.text_area(format!("{:?}", play_info.play_mode));
		painter.set_position(Vec2::new((canvas_size.x - playmode_area.width()) / 2.0, 32.0 + combo_area.height()));
		painter.text(format!("{:?}", play_info.play_mode));

		painter.set_position(Vec2::new(4.0, 0.0));
		painter.rect(Vec2::new(4.0, 16.0), Vec2::ZERO);
		painter.set_position(Vec2::new(12.0, 0.0));
		painter.rect(Vec2::new(4.0, 16.0), Vec2::ZERO);
	}), Area::new(canvas_position, canvas_position + canvas_size));
	let input = ui.input().clone();
	let mut clicks = vec!();
	for touch in input.touches() {
		clicks.push(Click {
			id: touch.id,
			position: touch.position * scale_factor,
			state: match touch.phase {
				TouchPhase::Start => ClickState::Pressed,
				TouchPhase::Hold => ClickState::Pressing,
				TouchPhase::End => ClickState::Released,
			}
		});
	}
	let cursor_position = input.cursor_position().unwrap_or(Vec2::INF) * scale_factor;
	cfg_if::cfg_if! {
		if #[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))] {
			if input.is_any_mouse_pressed() {
				clicks.push(Click {
					id: 999,
					position: cursor_position,
					state: ClickState::Pressed,
				});
			}else if input.is_any_mouse_released() {
				clicks.push(Click {
					id: 999,
					position: cursor_position,
					state: ClickState::Released,
				});
			}else {
				clicks.push(Click {
					id: 999,
					position: cursor_position,
					state: ClickState::Pressing,
				})
			}
		}
	}
	let judge_event = JudgeEvent {
		clicks,
	};
	if let Err(e) = core.judge(judge_event) {
		msg.message(format!("{}", e), ui);
	};
	if res.is_multi_clicked(2) && input.cursor_position().unwrap_or(Vec2::INF).is_inside(Area::new(canvas_position, canvas_position + Vec2::same(32.0))) {
		core.clear_play();
		ui.delete_texture(path);
		*router = Router::Main(MainRouter::default());
	}

}