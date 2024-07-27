use std::collections::HashSet;
use rayon::slice::ParallelSliceMut;
use shapoist_core::system::core_structs::Chart;
use shapoist_core::system::core_structs::JudgeField;
use shapoist_core::system::core_structs::PlayMode;
use nablo::event::Key;
use shapoist_core::system::core_structs::Diffculty;
use shapoist_core::system::core_structs::Note;
use std::collections::HashMap;
use time::Duration;
use std::f32::consts::PI;
use shapoist_core::system::core_structs::JudgeType;
use nablo::event::MouseButton;
use shapoist_core::system::core_structs::Select;
use shapoist_core::system::core_structs::ShapoistCore;
use crate::Router;
use nablo::prelude::*;

const BEAT_WIDTH: f32 = 50.0;

macro_rules! add_window {
	($inner: expr, $element: ident , $t: expr) => {
		if $inner.$element && !$inner.window_sort.contains(&$t) {
			$inner.window_sort.push($t)
		}
	};
}

pub fn edit(router: &mut Router, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore) {
	if core.play_info.is_some() {
	} else {
		msg.message("no chart loaded", ui);
		*router = Router::Main(Default::default());
		return;
	};
	if core.current_chart.is_some() {} else {
		msg.message("no chart loaded", ui);
		*router = Router::Main(Default::default());
		return;
	};
	let inner = if let Router::Edit(inner) = router {
		inner
	}else {
		unreachable!()
	};

	if ui.input().is_key_released(Key::Space) {
		if core.timer.is_started() {
			if let Err(e) = core.refresh_play_info() {
				if let Err(e) = core.play(PlayMode::Auto) {
					msg.message(format!("{}", e), ui);
				}else {
					if let Err(e) = core.pause() {
						msg.message(format!("{}", e), ui);
					}
					msg.message(format!("{}", e), ui);
				}
			}
			core.timer.set_to(inner.time_pointer + Duration::seconds_f32(3.0));
			if let Err(e) = core.update_render_queue(false) {
				msg.message(format!("{}", e), ui);
			};
			if let Err(e) = core.pause() {
				msg.message(format!("{}", e), ui);
			}
		}else if let Err(e) = core.play_with_time(PlayMode::Auto, inner.current_time) {
			msg.message(format!("{}", e), ui);
		}
	}

	if let Some((_, info)) = &core.current_chart {
		let sustain_time = info.sustain_time; 
		let beats = info.total_beats();
		let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
		let beats = beats / beat_quator;
		let bps = beats / sustain_time.as_seconds_f32();
		let change_time = |input: &mut Duration, ui: &mut Ui| {
			if !inner.is_adsorption {
				let mut sec = input.as_seconds_f32();
				if ui.input().is_key_pressing(Key::AltLeft) {
					if ui.input().is_key_pressing(Key::ArrowLeft) {
						sec -= 0.1
					}
				}else if ui.input().is_key_released(Key::ArrowLeft) {
					sec -= 0.1
				}
				if ui.input().is_key_pressing(Key::AltLeft) {
					if ui.input().is_key_pressing(Key::ArrowRight) {
						sec += 0.1
					}
				}else if ui.input().is_key_released(Key::ArrowRight) {
					sec += 0.1
				}
				*input = Duration::seconds_f32(sec);
			}else {
				let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
				if ui.input().is_key_pressing(Key::AltLeft) {
					if ui.input().is_key_pressing(Key::ArrowLeft) {
						sustain_beats = sustain_beats.saturating_sub(1);
					}
				}else if ui.input().is_key_released(Key::ArrowLeft) {
					sustain_beats = sustain_beats.saturating_sub(1);
				}
				if ui.input().is_key_pressing(Key::AltLeft) {
					if ui.input().is_key_pressing(Key::ArrowRight) {
						sustain_beats += 1;
					}
				}else if ui.input().is_key_released(Key::ArrowRight) {
					sustain_beats += 1;
				}
				*input = Duration::seconds_f32(sustain_beats as f32 / bps);
			}
			if *input > sustain_time {
				*input = sustain_time
			}
			if *input < Duration::ZERO {
				*input = Duration::ZERO
			}
		};
		change_time(&mut inner.time_pointer, ui);

		let current_offset = info.offset;
		// let total_offset = core.settings.offset;
		inner.current_time = if core.timer.is_started() {
			core.current().unwrap_or(inner.time_pointer + current_offset) - current_offset 
		}else {
			core.timer.set_to(inner.time_pointer + Duration::seconds_f32(3.0));
			inner.time_pointer
		};
	}

	if inner.last_time_pointer != inner.time_pointer {
		inner.last_time_pointer = inner.time_pointer;
		if let Err(e) = core.update_render_queue(false) {
			msg.message(format!("{}", e), ui);
		};
		if let Err(e) = core.pause() {
			msg.message(format!("{}", e), ui);
		}
	}

	if let Some((t1, t2)) = ui.show(&mut Card::new("edit_page").set_rounding(Vec2::same(16.0)).set_color(ui.style().background_color.brighter(0.05)), |ui, _| -> (bool, bool) {
		inner.window_sort.retain(|inside| match inside {
			EditInner::Preview => inner.is_preview_on,
			EditInner::Timeline => inner.is_timeline_on,
			EditInner::Detail => inner.is_detail_on,
			EditInner::Arrangement => inner.is_arrangement_on,
			EditInner::ChartInfo => inner.is_chart_info_on,
			EditInner::Linker => inner.is_linker_on,
			EditInner::Macro => inner.is_macro_on,
		});

		add_window!(inner, is_preview_on, EditInner::Preview);
		add_window!(inner, is_timeline_on, EditInner::Timeline);
		add_window!(inner, is_detail_on, EditInner::Detail);
		add_window!(inner, is_arrangement_on, EditInner::Arrangement);
		add_window!(inner, is_chart_info_on, EditInner::ChartInfo);
		add_window!(inner, is_linker_on, EditInner::Linker);
		add_window!(inner, is_macro_on, EditInner::Macro);

		let back = ui.show(&mut Card::new("menu").set_height(64.0).set_position(Vec2::same(16.0)).set_color([0,0,0,0]).set_scrollable([true; 2]), |ui, _| -> (bool, bool) {
			ui.horizental(|ui| -> (bool, bool) {
				let back1 = ui.button("back").is_multi_clicked(2);
				let back2 = ui.button("save").is_clicked();
				ui.switch(&mut inner.is_preview_on, "preview");
				ui.switch(&mut inner.is_timeline_on, "animation editor");
				ui.switch(&mut inner.is_detail_on, "element manager");
				ui.switch(&mut inner.is_arrangement_on, "note arranger");
				ui.switch(&mut inner.is_chart_info_on, "chart info");
				ui.switch(&mut inner.is_linker_on, "linker");
				ui.switch(&mut inner.is_macro_on, "macro");
				(back1, back2)
			})
		}).return_value.unwrap();

		ui.show(&mut Card::new("main editor").set_position(Vec2::new(16.0, 80.0)).set_color([0,0,0,0]), |ui, _| {
			for (i, inside) in inner.window_sort.clone().into_iter().enumerate() {
				if match inside {
					EditInner::Preview => preview(inner, ui, msg, core, Vec2::ZERO),
					EditInner::Timeline => timeline(inner, ui, msg, core, Vec2::ZERO),
					EditInner::Detail => detail(inner, ui, msg, core, Vec2::ZERO),
					EditInner::Arrangement => arrangement(inner, ui, msg, core, Vec2::ZERO),
					EditInner::ChartInfo => chart_info(inner, ui, msg, core, Vec2::ZERO),
					EditInner::Linker => linker(inner, ui, msg, core, Vec2::ZERO),
					EditInner::Macro => macro_window(inner, ui, msg, core, Vec2::ZERO),
				}.response.is_pressed() {
					inner.window_sort.rotate_left(i + 1);
				}
			}
		});

		back
	}).return_value {
		let mut info = MacroInner::from_router(inner);
		for (code, macro_use) in inner.macro_map.values_mut() {
			let info_back_up = info.clone();
			code.backend(&mut info, macro_use, msg, core);
			if info != info_back_up {
				inner.paste_offset = info.paste_offset;
				inner.current_time = info.current_time;
				inner.time_pointer = info.time_pointer;
				inner.is_adsorption = info.is_adsorption;
				inner.round = info.round;
			}
		}

		if inner.is_something_changed || inner.grouper.need_sync {
			for (father_id, child_ids) in &inner.grouper.group {
				if let Some((chart, _)) = &mut core.current_chart {
					let animation = match father_id {
						Select::JudgeField(id) => {
							if let Some(judge_field) = &mut chart.judge_fields.get(id) {
								judge_field.animation.clone()
							}else {
								continue;
							}
						},
						Select::Shape(id) => {
							if let Some(shape) = &mut chart.shapes.get(id) {
								shape.animation.clone()
							}else {
								continue;
							}
						},
						_ => continue,

					};
					for (child_id, offset, attribute) in child_ids {
						let processed_animation = if let Some(animation) = animation.get(attribute) {
							animation.move_by(*offset)
						}else {
							continue
						};
						match child_id {
							Select::JudgeField(id) => {
								if let Some(judge_field) = &mut chart.judge_fields.get_mut(id) {
									judge_field.animation.insert(attribute.to_string(), processed_animation);
								}
							},
							Select::Shape(id) => {
								if let Some(shape) = &mut chart.shapes.get_mut(id) {
									shape.animation.insert(attribute.to_string(), processed_animation);
								}
							},
							_ => continue,
						}
					}
				}
			}
			inner.grouper.need_sync = false;
		}

		if inner.is_something_changed {
			if let Err(e) = core.refresh_play_info() {
				if let Err(e) = core.play(PlayMode::Auto) {
					msg.message(format!("{}", e), ui);
				}else {
					if let Err(e) = core.pause() {
						msg.message(format!("{}", e), ui);
					}
					msg.message(format!("{}", e), ui);
				}
			}
			core.timer.set_to(inner.time_pointer + Duration::seconds_f32(3.0));
			if let Err(e) = core.update_render_queue(false) {
				msg.message(format!("{}", e), ui);
			};
			if let Err(e) = core.pause() {
				msg.message(format!("{}", e), ui);
			}
		}

		inner.is_something_changed = false;
		if t1 {
			core.clear_play();
			core.clear_edit();
			*router = Router::Main(Default::default());
		}
		if t2 {
			if let Err(e) = core.save_current_chart() {
				msg.message(format!("{}", e), ui);
			}else {
				msg.message("saved successfully", ui);
			}
		};
	};
}

pub struct EditRouter {
	is_something_changed: bool,

	filter: Duration,
	paste_offset: Duration,
	need_scroll_to_now: bool,
	window_sort: Vec<EditInner>,
	current_time: Duration,
	time_pointer: Duration,
	last_time_pointer: Duration,

	is_preview_on: bool,
	is_show_id: bool,

	is_timeline_on: bool,
	current_animation_editing: Option<Select>,
	is_changing_to_other_select: bool,
	timeline_scale_factor: f32,
	time_baseline: usize,
	is_adsorption: bool,
	current_animation_attribute: String,
	round: isize,
	current_linker: AnimationLinker,
	is_animation_delete: bool,
	value_offset: f32,
	copied_animation: Option<Animation>,

	is_detail_on: bool,

	is_arrangement_on: bool,
	current_judge_field: Option<String>,
	is_show_note_id: bool,
	default_click_effect_center: Vec2,
	note_type: JudgeType,

	is_chart_info_on: bool,

	is_linker_on: bool,
	range_of_current_note: Duration,
	/// None = not linking, Some(Shape_id)
	is_linking_shape: Option<String>,
	is_linking_note: Option<String>,
	link_id_temp: Option<String>,

	is_macro_on: bool,
	/// this will be user accessible in future, maybe based on lua
	/// for now, just use build-ins.
	macro_map: HashMap<String, (MacroCode, MacroInfo)>,
	note_generator: NoteGenerator,
	grouper: Grouper,
	switcher: Switcher,
}

#[derive(Default)]
struct NoteGenerator {
	tap_shape: Vec<String>,
	slide_shape: Vec<String>,
	flick_shape: Vec<String>,
	time_range: Duration,
}

#[derive(Default)]
struct Grouper {
	group: HashMap<Select, Vec<(Select, f32, String)>>,
	need_sync: bool,
}

#[derive(Default)]
struct Switcher {
	current_attributes: HashSet<String>,
	to_switch: [Option<Select>; 2],
}

#[derive(Clone, PartialEq)]
pub struct MacroInner {
	pub paste_offset: Duration,
	pub current_time: Duration,
	pub time_pointer: Duration,
	pub is_adsorption: bool,
	pub round: isize,
	pub is_something_changed: bool,
}

impl MacroInner {
	fn from_router(router: &EditRouter) -> Self {
		Self {
			paste_offset: router.paste_offset,
			current_time: router.current_time,
			time_pointer: router.time_pointer,
			is_adsorption: router.is_adsorption,
			round: router.round,
			is_something_changed: router.is_something_changed,
		}
	}
}

pub struct MacroCode {
	/// lua code
	pub inner: String
}

impl Macro for MacroCode {
	fn ui(&mut self, _: &mut MacroInner, _: &mut MacroInfo, _: &mut Ui, _: &mut MessageProvider, _: &mut ShapoistCore) {}
	fn backend(&mut self, _: &mut MacroInner, _: &mut MacroInfo, _: &mut MessageProvider, _: &mut ShapoistCore) {}
}

pub struct MacroInfo {
	pub select: Vec<Select>,
}

pub trait Macro {
	fn ui(&mut self, inner: &mut MacroInner, info: &mut MacroInfo, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore);
	fn backend(&mut self, inner: &mut MacroInner, info: &mut MacroInfo, msg: &mut MessageProvider, core: &mut ShapoistCore);
}

impl Default for EditRouter {
	fn default() -> Self {
		Self {
			is_something_changed: false,

			filter: Duration::MAX,
			paste_offset: Duration::ZERO,
			need_scroll_to_now: false,
			window_sort: vec!(),
			current_time: Duration::ZERO,
			time_pointer: Duration::ZERO,
			last_time_pointer: Duration::ZERO,

			is_preview_on: false,
			is_show_id: true,

			is_timeline_on: false,
			current_animation_editing: None,
			is_changing_to_other_select: false,
			timeline_scale_factor: 1.0,
			time_baseline: 2,
			is_adsorption: true,
			current_animation_attribute: String::new(),
			round: -2,
			current_linker: AnimationLinker::Bezier(Vec2::new(0.5,0.1), Vec2::new(0.5,0.9)),
			is_animation_delete: false,
			value_offset: 0.0,
			copied_animation: None,

			is_detail_on: false,

			is_arrangement_on: false,
			current_judge_field: None,
			is_show_note_id: false,
			default_click_effect_center: Vec2::ZERO,
			note_type: JudgeType::Tap,

			is_chart_info_on: false,

			is_linker_on: false,
			range_of_current_note: Duration::seconds(1),
			is_linking_shape: None,
			is_linking_note: None,
			link_id_temp: None,

			is_macro_on: false,
			macro_map: HashMap::new(),
			note_generator: Default::default(),
			grouper: Default::default(),
			switcher: Default::default(),
		}
	}
}

#[derive(Clone, PartialEq)]
pub enum EditInner {
	Preview,
	Timeline,
	Detail,
	Arrangement,
	ChartInfo,
	Linker,
	Macro
}

fn preview(inner: &mut EditRouter, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	let mut selects = match core.current_selects() {
		Ok(t) => t,
		Err(e) => {
			msg.message(format!("{:?}", e), ui);
			return ui.card("preview", Vec2::same(1.0), |_, _| {});
		}
	};
	let (chart_size, size) = { 
		let (chart, _) = core.current_chart.as_ref().unwrap();
		(chart.size.x, chart.size / chart.size.len())
	};
	let canvas_size = size * 400.0;
	let fps = format!("{:.2}fps", 1.0 / ui.delay().as_seconds_f32());
	ui.show(&mut Card::new("preview")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(canvas_size.y + 32.0)
		.set_width(canvas_size.x + 32.0)
		.set_resizable(true)
		.set_position(position)
		.set_scrollable([true; 2]), 
		|ui, _| {
			let window = ui.window_area().shrink(Vec2::same(8.0));
			let canvas_size = if (window.width() * size).y < window.height() {
				window.width() * size
			}else {
				window.height() * size
			};
			let scale_factor = canvas_size.x / chart_size;
			let canvas_position = window.center() - canvas_size / 2.0;
			let time = format!("{:.2}s", (core.timer.read() - Duration::seconds(3)).as_seconds_f32());
			let mut areas = HashMap::new();
			let canvas_response = ui.put(Canvas::new(canvas_size, |painter| {
				if let Some(play_info) = &core.play_info {
					painter.set_color(0.0);
					painter.rect(canvas_size, Vec2::ZERO);
					for shape in &play_info.render_queue {
						painter.set_color(1.0);
						let shape_id = &shape.id;
						let mut shape = shape.shape.clone();
						shape.pre_scale(scale_factor);
						let shape_area = shape.get_area();
						let id_position = shape_area.right_bottom();
						if selects.contains(&Select::Shape(shape_id.to_string())) {
							let mut stroke_shape = shape.clone();
							stroke_shape.style.fill = [0,0,0,0].into();
							stroke_shape.style.stroke_width += 3.0;
							stroke_shape.style.stroke_color = [1.0, 0.0, 0.0, 1.0].into();
							painter.push(stroke_shape);
						}
						areas.insert(shape_id.to_string(), shape_area);
						painter.push(shape);
						painter.set_scale(Vec2::same(0.75));
						if inner.is_show_id {
							painter.set_position(id_position);
							painter.text(shape_id.to_string());
						}
					}
				}
				painter.set_color(1.0);
				painter.set_scale(Vec2::same(1.0));
				painter.set_position(Vec2::same(16.0));
				painter.text("preview".to_string());
				painter.set_position(Vec2::new(16.0, 32.0));
				painter.text(time);
				painter.set_position(Vec2::new(16.0, 48.0));
				painter.text(fps);
			}).dragable(true), Area::new(canvas_position, canvas_position + canvas_size));
			if canvas_response.is_clicked() {
				let mut select_to_add = vec!();
				let mouse_position = ui.input().cursor_position().unwrap_or(Vec2::INF) - canvas_position;
				// println!("{:?},  {:?}", mouse_position,  ui.window_area().left_top());
				let mut is_click_outside = true; 
				for (id, area) in areas {
					// println!("{:?}", area.left_top());
					if area.is_point_inside(&mouse_position) {
						// println!("{:?}", id);
						is_click_outside = false;
						select_to_add.push(Select::Shape(id));
					}
				}
				let mut select_to_delete = select_to_add.clone();
				select_to_delete.retain(|inner| selects.contains(inner));
				if ui.input().is_key_pressing(Key::ControlLeft) {
					selects.append(&mut select_to_add);
				}else if !select_to_add.is_empty() {
					selects = select_to_add;
				}
				selects.retain(|inner| !select_to_delete.contains(inner));
				if let Err(e) = core.multi_select(selects.clone()) {
					msg.message(format!("{}", e), ui);
				}
				if is_click_outside && !ui.input().is_key_pressing(Key::ControlLeft) {
					if let Err(e) = core.clear_selects() {
						msg.message(format!("{}", e), ui);
					}
				}
			};
			let drag_delta = canvas_response.drag_delta();
			if drag_delta != Vec2::ZERO {
				// println!("{:?}", drag_delta);
				let x_animation = String::from("----Shape----style----position----x");
				let y_animation = String::from("----Shape----style----position----y");
				for select in &selects {
					if let Select::Shape(id) = select {
						if let Some((chart, _)) = &mut core.current_chart{
							if let Some(shape) = chart.shapes.get_mut(id) {
								// if ui.input().is_key_pressing(Key::ShiftLeft) {
								// 	if drag_delta.x > drag_delta.y {
								// 		drag_delta.y = 0.0;
								// 	}else {
								// 		drag_delta.x = 0.0;
								// 	}
								// }
								if let Some(animation) = shape.animation.get_mut(&x_animation) {
									*animation = animation.move_by(drag_delta.x);
								}else {
									shape.shape.style.position.x += drag_delta.x;
								}
								if let Some(animation) = shape.animation.get_mut(&y_animation) {
									*animation = animation.move_by(drag_delta.y);
								}else {
									shape.shape.style.position.y += drag_delta.y;
								}
							}
						}
					}
				}
				inner.is_something_changed = true;
			}
		}
	)
}

fn arrangement(inner: &mut EditRouter, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	ui.show(&mut Card::new("note arranger")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(320.0)
		.set_width(540.0)
		.set_resizable(true)
		.set_position(position)
		.set_scrollable([false; 2]), 
		|ui, _| {
			let area = ui.window_area();
			// let selects = match core.current_selects() {
			// 	Ok(t) => t,
			// 	Err(e) => {
			// 		msg.message(format!("{}", e), ui);
			// 		return;
			// 	}	
			// };
			let toolbar_width = 270.0;
			ui.show(&mut Card::new("note arranger toolbar")
				.set_position(Vec2::same(16.0))
				.set_width(toolbar_width)
				.set_height(area.height() - 32.0)
				.set_rounding(Vec2::same(16.0))
				.set_color(ui.style().background_color.brighter(0.05))
				.set_scrollable([true; 2]), |ui, _| {
					ui.label("note arranger");
					if ui.button("close").is_clicked() {
						inner.is_arrangement_on = false;
					}

					if let Some(id) = &inner.current_judge_field {
						ui.label(id);
					}else {
						ui.label("N/A");
						return;
					}

					// if inner.current_judge_field.is_none() {
					// 	ui.label("N/A");
					// }else {
					// 	match &selects[0] {
					// 		Select::JudgeField(id) => ui.label(id),
					// 		_ => {
					// 			ui.label("not a judge field");
					// 			return
					// 		}
					// 	};
					// }

					if let Some(play_info) = &core.play_info {
						ui.label(format!("total notes: {}", play_info.total_notes));
					}

					if let Some(id) = &inner.current_judge_field {
						if let Some((chart, info)) = &mut core.current_chart {
							let sustain_time = info.sustain_time; 
							let beats = info.total_beats();
							let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
							let beats = beats / beat_quator;
							let bps = beats / sustain_time.as_seconds_f32();
							let change_time = |input: &mut Duration, text: String, ui: &mut Ui| {
								ui.horizental(|ui| {
									if !inner.is_adsorption {
										let mut sec = input.as_seconds_f32();
										if ui.button("-").is_clicked() {
											sec -= 0.1
										}
										ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
										if ui.button("+").is_clicked() {
											sec += 0.1
										}
										*input = Duration::seconds_f32(sec);
									}else {
										let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
										if ui.button("-").is_clicked() {
											sustain_beats = sustain_beats.saturating_sub(1);
										}
										ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
										if ui.button("+").is_clicked() {
											sustain_beats += 1;
										}
										*input = Duration::seconds_f32(sustain_beats as f32 / bps);
									}
								});
								if *input > sustain_time {
									*input = sustain_time
								}
								if *input < Duration::ZERO {
									*input = Duration::ZERO
								}
							};

							let (start_time, sustain_time) = if let Some(t) = chart.judge_fields.get_mut(id) {
								(&mut t.start_time, &mut t.sustain_time)
							}else {
								msg.message("cant find select resorce", ui);
								return;
							};

							change_time(start_time, "start time".to_string(), ui);
							change_time(sustain_time, "sustain time".to_string(), ui);
						}
					}

					if ui.button("clear current judge field").is_clicked() {
						inner.current_judge_field = None;
					}
					if ui.button("clear selects").is_clicked() {
						inner.current_animation_editing = None;
						if let Err(e) = core.clear_selects() {
							msg.message(format!("{}", e), ui);
						}
					}
					if ui.button("scroll to current").is_clicked() {
						inner.need_scroll_to_now = true;
					}
					ui.switch(&mut inner.is_adsorption, "adsorption");
					ui.switch(&mut inner.is_show_note_id, "note id");
					ui.add(Slider::new(0.06..=16.0, &mut inner.timeline_scale_factor, "scale").step(0.01).logarithmic(true));
					ui.label("note type");
					ui.selectable_value(&mut inner.note_type, JudgeType::Tap, "tap");
					ui.selectable_value(&mut inner.note_type, JudgeType::Slide, "slide");
					ui.selectable_value(&mut inner.note_type, JudgeType::Flick, "flick");
					ui.selectable_value(&mut inner.note_type, JudgeType::TapAndFlick, "tap and flick");
					if ui.switch(&mut matches!(inner.note_type, JudgeType::Hold(_)), "hold").is_clicked() {
						inner.note_type = JudgeType::Hold(Duration::ZERO);
					};
					if ui.switch(&mut matches!(inner.note_type, JudgeType::AngledFilck(_)), "angled flick").is_clicked() {
						inner.note_type = JudgeType::AngledFilck(0.0);
					};
					if ui.switch(&mut matches!(inner.note_type, JudgeType::AngledTapFilck(_)), "angled tap flick").is_clicked() {
						inner.note_type = JudgeType::AngledTapFilck(0.0);
					};
					
					match &mut inner.note_type {
						JudgeType::Hold(t) => {
							if !inner.is_adsorption {
								let mut sec = t.as_seconds_f32();
								ui.add(Slider::new(0.01..=10.0, &mut sec, "hold length").step(0.01).suffix("s"));
								*t = Duration::seconds_f32(sec);
							}else if let Some((_, info)) = &mut core.current_chart {
								let sustain_time = info.sustain_time.as_seconds_f32(); 
								let beats = info.total_beats();
								let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
								let beats = beats / beat_quator;
								let bps = beats / sustain_time;
								let mut sustain_beats = (t.as_seconds_f32() * bps).floor() as usize;
								ui.add(Slider::new(1..=100, &mut sustain_beats, "hold length").step(1.0).suffix(format!("*{} beat", beat_quator)));
								*t = Duration::seconds_f32(sustain_beats as f32 / bps);
							}
						},
						JudgeType::AngledFilck(t) | JudgeType::AngledTapFilck(t) => {
							let mut angle_degree = *t / PI * 180.0;
							ui.add(Slider::new(0.0..=360.0, &mut angle_degree, "angle").step(1.0).suffix("deg"));
							*t = angle_degree / 180.0 * PI;
						},
						_ => {}
					}
				}
			);

			ui.show(&mut Card::new("note arranger timeline")
				.set_rounding(Vec2::same(16.0))
				.set_color(ui.style().background_color.brighter(0.05))
				.set_width(area.width() - toolbar_width - 16.0 * 3.0)
				.set_height(area.height() - 32.0)
				.set_scrollable([true; 2])
				.set_position(Vec2::new(toolbar_width + 16.0 * 2.0, 16.0)), |ui, card| {
					let current_time = inner.current_time;
					if let Some((chart, info)) = &mut core.current_chart {
						inner.default_click_effect_center = chart.size / 2.0;
						if let Some(judge_field_id) = &inner.current_judge_field {
							let sustain_time = info.sustain_time.as_seconds_f32(); 
							let beats = info.total_beats();
							let width = beats * BEAT_WIDTH * inner.timeline_scale_factor;
							let height = ui.window_area().height() - 32.0;
							let current_scroll = - card.scroll(ui).x;
							let card_area = ui.window_area();
							let cursor_position = ui.input().cursor_position().unwrap_or(Vec2::INF) - card_area.left_top();
							let inner_position = ui.input().cursor_position().unwrap_or(Vec2::INF);

							let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
							let beats = beats.floor() / beat_quator;
							if let Some(judge_field) = chart.judge_fields.get_mut(judge_field_id) {
								let (canvas, back) = Canvas::new_with_return(Vec2::new(width, height), |painter| -> HashMap<u64, Vec<String>> {
									let start = (current_scroll * beats / width).floor();
									for i in (start as usize)..(beats as usize) {
										let position = width / beats * i as f32;
										painter.set_position(Vec2::x(position));
										painter.set_color(1.0);
										painter.set_scale(Vec2::same(0.8));
										if beat_quator <= 2.0 {
											let divide = (4.0 / beat_quator).floor() as usize;
											let text = format!("{}.{}", i / divide, i % divide + 1);
											painter.text(text);
										}else {
											let divide = 4.0 / beat_quator;
											let text = format!("{:.0}", i as f32 / divide);
											painter.text(text);
										}
										painter.set_scale(Vec2::NOT_TO_SCALE);
										
										painter.set_position(Vec2::new(position, 16.0));
										painter.set_color([1.0,1.0,1.0, 0.3]);
										painter.rect(Vec2::new(4.0, height - 16.0), Vec2::same(2.0));
										if position - current_scroll > card_area.width() {
											break;
										}
									}

									let x = judge_field.start_time.as_seconds_f32() / sustain_time * width;
									painter.set_position(Vec2::x(x));
									painter.set_color([1.0,1.0,1.0, 0.5]);
									painter.rect(Vec2::new(4.0, height), Vec2::same(2.0));
									painter.set_color(1.0);
									painter.set_position(Vec2::x(x) + Vec2::y(16.0));
									painter.text("start".to_string());

									let x = judge_field.sustain_time.as_seconds_f32() / sustain_time * width + x;
									painter.set_position(Vec2::x(x));
									painter.set_color([1.0,1.0,1.0, 0.5]);
									painter.rect(Vec2::new(4.0, height), Vec2::same(2.0));
									painter.set_color(1.0);
									painter.set_position(Vec2::x(x) + Vec2::y(16.0));
									painter.text("end".to_string());

									let mut map: HashMap<u64, Vec<String>> = HashMap::new();
									let mut notes = vec!();
									for (id, note) in &chart.notes {
										if &note.judge_field_id == judge_field_id {
											notes.push((id, note));
										}
									}
									notes.sort_by(|(_, note1), (_, note2)| {
										if let JudgeType::Hold(_) = note1.judge_type {
											if let JudgeType::Hold(_) = note2.judge_type {
												note1.judge_time.cmp(&note2.judge_time)
											}else {
												std::cmp::Ordering::Less
											}
										}else {
											std::cmp::Ordering::Equal
										}
									});
									for (id, note) in notes {
										let x = note.judge_time.as_seconds_f32() / sustain_time * width;
										let y = if let JudgeType::Hold(sustain) = note.judge_type {
											let sustain = sustain.as_seconds_f32();
											let key = (x * 0.1).round() as u64;
											let y = if let Some(t) = map.get_mut(&key) {
												t.len() + 1
											}else {
												1
											};
											for i in 0..(sustain / sustain_time * beats) as usize {
												let key = ((x + i as f32 * width / beats) * 0.1).round() as u64;
												if let Some(t) = map.get_mut(&key) {
													t.push(id.clone());
												}else {
													map.insert(key, vec!(id.clone()));
												};
											} 
											y
										}else {
											let key = (x * 0.1).round() as u64;
											if let Some(t) = map.get_mut(&key) {
												t.push(id.clone());
												t.len()
											}else {
												map.insert(key, vec!(id.clone()));
												1
											}
										} as f32 * 32.0;
										
										if x - current_scroll > card_area.width() || x - current_scroll < 0.0 {
											continue;
										}
										let coner = Vec2::new(x - 6.0, y);
										match note.judge_type {
											JudgeType::Tap => {
												painter.set_position(coner);
												painter.cir(8.0);
											},
											JudgeType::Slide => {
												painter.set_position(coner);
												painter.set_stroke_width(2.0);
												painter.set_color([0,0,0,0]);
												painter.set_stroke_color(1.0);
												painter.cir(8.0);
												painter.set_stroke_width(0.0);
												painter.set_color(1.0);
											},
											JudgeType::Flick => {
												painter.set_position(coner);
												painter.set_stroke_width(2.0);
												painter.set_color([0,0,0,0]);
												painter.set_stroke_color(1.0);
												painter.rect(Vec2::same(16.0), Vec2::same(4.0));
												painter.set_stroke_width(0.0);
												painter.set_color(1.0);
											},
											JudgeType::Hold(sustain) => {
												let length = sustain.as_seconds_f32() / sustain_time * width;
												painter.set_position(coner + Vec2::x(8.0));
												painter.set_color([1.0,1.0,1.0,1.0]);
												painter.rect(Vec2::new(length, 16.0), Vec2::same(8.0));
												painter.set_color(1.0);
											},
											JudgeType::TapAndFlick => {
												painter.set_position(coner);
												painter.rect(Vec2::same(16.0), Vec2::same(4.0));
											},
											JudgeType::AngledFilck(_) => {
												painter.set_position(coner);
												painter.set_stroke_width(2.0);
												painter.set_color([0,0,0,0]);
												painter.set_stroke_color(1.0);
												painter.polygon(vec!(Vec2::x(8.0), Vec2::new(16.0, 8.0), Vec2::new(8.0, 16.0), Vec2::y(8.0)));
												painter.set_stroke_width(0.0);
												painter.set_color(1.0);
											},
											JudgeType::AngledTapFilck(_) => {
												painter.set_position(coner);
												painter.polygon(vec!(Vec2::x(8.0), Vec2::new(16.0, 8.0), Vec2::new(8.0, 16.0), Vec2::y(8.0)));
											},
											_ => {// TODO
											},
										}
										painter.set_position(coner + Vec2::y(16.0));
										if inner.is_show_note_id {
											painter.set_scale(Vec2::same(0.6));
											painter.text(id.clone());
											painter.set_scale(Vec2::same(1.0));
										}
									}

									let current = (current_time.as_seconds_f32() / sustain_time) * width;
									painter.set_position(Vec2::x(current));
									painter.set_color(0.5);
									painter.rect(Vec2::new(4.0, height), Vec2::same(2.0)); 

									if card_area.is_point_inside(&(inner_position)) {
										let x = if inner.is_adsorption {
											((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats
										}else {
											cursor_position.x - 16.0 + current_scroll
										};
										let y = cursor_position.y - 16.0;
										painter.set_color([1.0,1.0,1.0, 0.3]);
										painter.set_position(Vec2::new(current_scroll, y));
										painter.rect(Vec2::new(card_area.width(), 4.0), Vec2::same(2.0));
										painter.set_position(Vec2::x(x));
										painter.rect(Vec2::new(4.0, height), Vec2::same(2.0)); 
										painter.set_color(1.0);
										painter.set_position(Vec2::new(x, y));
										painter.set_position(Vec2::new(x - 2.0, y - 2.0));
										painter.cir(4.0);
									};

									map
								});

								if inner.need_scroll_to_now {
									inner.need_scroll_to_now = false;
									let current = (current_time.as_seconds_f32() / sustain_time) * width - ui.window_area().width() / 2.0;
									card.scroll_to_x(current, ui);
								}
	
								let res = ui.add(canvas);
								if res.is_clicked() && 
								(ui.input().cursor_position().unwrap_or(Vec2::INF).is_inside(Area::new(res.area.left_top(), res.area.right_top() + Vec2::y(16.0))) ||
								ui.input().is_key_pressing(Key::AltLeft) ||
								ui.input().is_mouse_released(MouseButton::Middle) ) {
									inner.is_something_changed = true;
									let x = if inner.is_adsorption {
										((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats
									}else {
										cursor_position.x - 16.0 + current_scroll
									};
									inner.time_pointer = Duration::seconds_f32(x / width * sustain_time);
								}else if res.is_multi_clicked(2) && ui.input().is_mouse_released(MouseButton::Left) {
									let x = if inner.is_adsorption {
										((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats
									}else {
										cursor_position.x - 16.0 + current_scroll
									};
									let judge_time = x / width * sustain_time;
									let mut index = chart.notes.len();
									loop {
										let id = format!("{} {}",judge_field_id ,index);
										if !chart.notes.contains_key(&id) {
											chart.notes.insert(id.clone(), Note {
												judge_type: inner.note_type.clone(),
												judge_time: Duration::seconds_f32(judge_time),
												judge_field_id: judge_field_id.to_string(),
												note_id: id,
												click_effect_position: inner.default_click_effect_center,
												..Default::default()
											});
											break;
										}
										index += 1
									}
								}
								if res.is_clicked() && ui.input().is_mouse_released(MouseButton::Right) {
									inner.is_something_changed = true;
									let x = if inner.is_adsorption {
										((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats
									}else {
										cursor_position.x - 16.0 + current_scroll
									};
									let y = cursor_position.y - 16.0;
									let key = (x * 0.1).round() as u64;
									let index = (y / 32.0).abs().floor() as usize;
									let index = index.saturating_sub(1);
									if let Some(inner) = back.get(&key) {
										if index < inner.len() {
											chart.notes.remove(&inner[index]);
										}
									}
								}
							}
						}else if let Some((chart, _)) = &core.current_chart {
							for id in chart.judge_fields.keys() {
								// if field.start_time + field.sustain_time < inner.time_pointer - inner.filter || field.start_time > inner.time_pointer + inner.filter {
								// 	continue;
								// }
								if ui.button(id).is_clicked() {
									inner.current_judge_field = Some(id.to_string());
								};
							}
						}
					}
				}
			);
		}
	)
}

fn timeline(inner: &mut EditRouter, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	if inner.timeline_scale_factor <= 0.0 {
		inner.timeline_scale_factor = 0.1;
	};
	let mut need_copy = false;
	let mut need_paste = false;
	let mut need_paste_to_all = false;
	ui.show(&mut Card::new("animation editor")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(320.0)
		.set_width(540.0)
		.set_resizable(true)
		.set_position(position)
		.set_scrollable([false; 2]), |ui, _| {
			let selects = match core.current_selects() {
				Ok(t) => t,
				Err(e) => {
					msg.message(format!("{}", e), ui);
					return;
				}	
			};
			let area = ui.window_area();
			let toolbar_width = 270.0;
			ui.show(&mut Card::new("animation editor toolbar")
				.set_position(Vec2::same(16.0))
				.set_width(toolbar_width)
				.set_height(area.height() - 32.0)
				.set_rounding(Vec2::same(16.0))
				.set_color(ui.style().background_color.brighter(0.05))
				.set_scrollable([true; 2]), |ui, _| {
					ui.label("animation editor");
					if ui.button("close").is_clicked() {
						inner.is_timeline_on = false;
					}
					if inner.current_animation_attribute.is_empty() || inner.current_animation_editing.is_none() {
						ui.label("N/A");
					}else {
						let attribute = inner.current_animation_attribute.replace("----", " ").replace("Shape ", "").replace("style ", "").replace("JudgeFieldInner ", "");
						ui.label(format!("{:?} - {}", inner.current_animation_editing.clone().unwrap(), attribute.trim()));
					};

					if ui.button("change to").is_clicked() {
						inner.is_changing_to_other_select = true;
					}

					if !selects.is_empty() {
						if inner.current_animation_editing.is_none() {
							inner.current_animation_editing = Some(selects[0].clone());
						}
						if let (Some((chart, info)), Some(current_editing)) = (&mut core.current_chart, &inner.current_animation_editing) {
							let sustain_time = info.sustain_time; 
							let beats = info.total_beats();
							let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
							let beats = beats / beat_quator;
							let bps = beats / sustain_time.as_seconds_f32();
							let change_time = |input: &mut Duration, text: String, ui: &mut Ui| {
								ui.horizental(|ui| {
									if !inner.is_adsorption {
										let mut sec = input.as_seconds_f32();
										if ui.button("-").is_clicked() {
											sec -= 0.1
										}
										ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
										if ui.button("+").is_clicked() {
											sec += 0.1
										}
										*input = Duration::seconds_f32(sec);
									}else {
										let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
										if ui.button("-").is_clicked() {
											sustain_beats = sustain_beats.saturating_sub(1);
										}
										ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
										if ui.button("+").is_clicked() {
											sustain_beats += 1;
										}
										*input = Duration::seconds_f32(sustain_beats as f32 / bps);
									}
								});
								if *input > sustain_time {
									*input = sustain_time
								}
								if *input < Duration::ZERO {
									*input = Duration::ZERO
								}
							};

							let (start_time, sustain_time) = match &current_editing {
								Select::Shape(id) => {
									if let Some(t) = chart.shapes.get_mut(id) {
										(&mut t.start_time, &mut t.sustain_time)
									}else {
										msg.message("cant find select resorce", ui);
										return;
									}
								},
								Select::JudgeField(id) => {
									if let Some(t) = chart.judge_fields.get_mut(id) {
										(&mut t.start_time, &mut t.sustain_time)
									}else {
										msg.message("cant find select resorce", ui);
										return;
									}
								},
								_ => {
									ui.label("unsupported");
									return;
								}
							};

							change_time(start_time, "start time".to_string(), ui);
							change_time(sustain_time, "sustain time".to_string(), ui);
						}
					}else {
						inner.current_animation_editing = None;
					}

					if ui.button("scroll to current").is_clicked() {
						inner.need_scroll_to_now = true;
					}
					if ui.button("copy").is_clicked() {
						need_copy = true;
					}
					if ui.button("paste").is_clicked() {
						inner.is_something_changed = true;
						need_paste = true;
					}
					if ui.button("paste to all").is_clicked() {
						inner.is_something_changed = true;
						need_paste_to_all = true;
					}
					if ui.button("delete animation").is_multi_clicked(2) && !inner.current_animation_attribute.is_empty() {
						inner.is_something_changed = true;
						inner.is_animation_delete = true;
					}
					if ui.button("clear attribute").is_clicked() {
						inner.current_animation_attribute.clear();
					}
					if ui.button("clear selects").is_clicked() {
						inner.current_animation_editing = None;
						if let Err(e) = core.clear_selects() {
							msg.message(format!("{}", e), ui);
						}
					}
					ui.switch(&mut inner.is_adsorption, "adsorption");
					ui.add(Slider::new(0.06..=16.0, &mut inner.timeline_scale_factor, "scale").step(0.01).logarithmic(true));
					// ui.label("time baseline");
					// ui.horizental(|ui| {
					// 	if ui.button("-").is_clicked() {
					// 		inner.time_baseline = inner.time_baseline - 1;
					// 	}
					// 	ui.dragable_value(&mut inner.time_baseline);
					// 	if ui.button("+").is_clicked() {
					// 		inner.time_baseline = inner.time_baseline + 1;
					// 	}
					// 	if inner.time_baseline > 16 {
					// 		inner.time_baseline = 16;
					// 	}else if inner.time_baseline < 2 {
					// 		inner.time_baseline = 2;
					// 	}
					// });
					ui.label("round");
					ui.horizental(|ui| {
						if ui.button("-").is_clicked() {
							inner.round -= 1;
						}
						ui.dragable_value(&mut inner.round);
						if ui.button("+").is_clicked() {
							inner.round += 1;
						}
						inner.round = inner.round.clamp(-4, 4);
					});
					ui.label("value_offset");
					ui.horizental(|ui| {
						if ui.button("-").is_clicked() {
							inner.value_offset -= 10_f32.powf(inner.round as f32);
						}
						ui.add(Slider::new(-500.0..=500.0, &mut inner.value_offset, "").step(0.01));
						if ui.button("+").is_clicked() {
							inner.value_offset += 10_f32.powf(inner.round as f32);
						}
						if ui.button("round").is_clicked() {
							inner.value_offset = (inner.value_offset / 10_f32.powf(inner.round as f32)).round() * 10_f32.powf(inner.round as f32);
						}
						inner.value_offset = inner.value_offset.clamp(-500.0, 500.0);
					});
					if ui.button("reset offset").is_clicked() {
						inner.value_offset = 0.0;
					}
					let mut move_by_animation = |id: &Select, ui: &mut Ui| {
						let chart = if let Some((chart, _)) = &mut core.current_chart {
							chart
						}else {
							return;
						};
						let animation = match id {
							Select::Shape(id) => {
								if let Some(t) = chart.shapes.get_mut(id) {
									&mut t.animation
								}else {
									msg.message("cant find select resorce", ui);
									return;
								}
							},
							Select::JudgeField(id) => {
								if let Some(t) = chart.judge_fields.get_mut(id) {
									&mut t.animation
								}else {
									msg.message("cant find select resorce", ui);
									return;
								}
							},
							_ => {
								return;
							}
						};
						if let Some(animation) = animation.get_mut(&inner.current_animation_attribute) {
							*animation = animation.move_by(inner.value_offset);
						}
					};
					if ui.button("move by offset").is_clicked() {
						inner.is_something_changed = true;
						if let Some(current_editing) = &inner.current_animation_editing {
							move_by_animation(current_editing, ui);
						}
					}

					if ui.button("move by offset for all").is_clicked() {
						inner.is_something_changed = true;
						for id in &selects {
							move_by_animation(id, ui);
						}
					}

					ui.label("linker type");
					if ui.switch(&mut matches!(inner.current_linker, AnimationLinker::Bezier(_, _)), "bezier").is_clicked() {
						inner.current_linker = AnimationLinker::Bezier(Vec2::new(0.5,0.1), Vec2::new(0.5,0.9));
					};
					if ui.switch(&mut matches!(inner.current_linker, AnimationLinker::Linear), "linear").is_clicked() {
						inner.current_linker = AnimationLinker::Linear;
					};
					if ui.switch(&mut matches!(inner.current_linker, AnimationLinker::Mutation), "mutation").is_clicked() {
						inner.current_linker = AnimationLinker::Mutation;
					};

					match &mut inner.current_linker {
						AnimationLinker::Bezier(point1, point2) => {
							let width = ui.window_area().width() - 32.0;
							let res = ui.add(Canvas::new(Vec2::same(width), |painter| {
								painter.set_stroke_width(4.0);
								painter.set_stroke_color(1.0);
								painter.set_color([0,0,0,0]);
								let points = [Vec2::ZERO + Vec2::same(4.0), 
									*point1 * Vec2::same(width), 
									*point2 * Vec2::same(width), 
									Vec2::same(width)
								];
								painter.bezier(points);
								let radius = 16.0;
								painter.set_position(points[1] - Vec2::same(radius));
								painter.cir(radius);
								painter.set_position(points[2] - Vec2::same(radius));
								painter.cir(radius);
								painter.set_stroke_width(0.0);
								painter.set_color([1.0,1.0,1.0,0.3]);
								painter.set_position(Vec2::ZERO + Vec2::same(4.0));
								painter.line(points[1]);
								painter.set_position(points[3]);
								painter.line(points[2] - points[3]);
							}).dragable(true));
							let cursor_position = ui.input().cursor_position().unwrap_or(Vec2::INF);
							let (mut is_point1_draging, mut is_point2_draging) = if let Some((t1, t2)) = ui.memory_read(&res.id) {
								(t1, t2)
							}else {
								(false, false)
							};
							if res.is_pressing() {
								let relative = (cursor_position - res.area.left_top()) / Vec2::same(width);
								if ((relative - *point1).len() < 32.0 / width || is_point1_draging) && !is_point2_draging {
									*point1 = *point1 + res.drag_delta() / Vec2::same(width);
									is_point1_draging = true;
								}
								if ((relative - *point2).len() < 32.0 / width || is_point2_draging) && !is_point1_draging {
									*point2 = *point2 + res.drag_delta() / Vec2::same(width);
									is_point2_draging = true
								}
							}else {
								is_point1_draging = false;
								is_point2_draging = false;
							}
							fn compress(input: &mut f32) {
								*input = (*input).clamp(0.0, 1.0);
							}
							compress(&mut point1.x);
							compress(&mut point1.y);
							compress(&mut point2.x);
							compress(&mut point2.y);
							ui.memory_save(&res.id, (is_point1_draging, is_point2_draging));
						},
						AnimationLinker::Power(n) => {
							ui.add(Slider::new(0.25..=4.0, n, "n").step(0.01).logarithmic(true));
						},
						_ => {},
					}

					if ui.button("reset").is_clicked() {
						inner.is_adsorption = true;
						inner.timeline_scale_factor = 1.0;
						inner.time_baseline = 2;
						inner.current_animation_attribute.clear();
					}
				}
			);

			ui.show(&mut Card::new("animation editor timeline")
				.set_rounding(Vec2::same(16.0))
				.set_color(ui.style().background_color.brighter(0.05))
				.set_width(area.width() - toolbar_width - 16.0 * 3.0)
				.set_height(area.height() - 32.0)
				.set_scrollable([true; 2])
				.set_position(Vec2::new(toolbar_width + 16.0 * 2.0, 16.0)), |ui, card| {
					let current_time = inner.current_time;
					if let Some((chart, info)) = &mut core.current_chart {
						if selects.is_empty() {
							ui.label("select a element in element manager first");
						}else {
							if inner.is_changing_to_other_select {
								for id in selects {
									if ui.button(format!("{:?}", id)).is_clicked() {
										inner.current_animation_editing = Some(id.clone());
										inner.is_changing_to_other_select = false;
									}
								}
								return;
							}
							let current_editing = if let Some(t) = &inner.current_animation_editing {
								t.clone()
							}else {
								return;
							};
							if inner.current_animation_attribute.is_empty() {
								fn selector(input: &mut ParsedData, ui: &mut Ui, id: &String) -> Option<String> {
									let id = format!("{}----{}", id, input.name);
									let mut out = None;
									match &mut input.data {
										DataEnum::Node(inner) => {
											for inside in inner {
												if let Some(t) = selector(inside, ui, &id) {
													out = Some(t);
												};
											}
										},
										DataEnum::Enum(_, inner) => {
											for inside in inner {
												if let Some(t) = selector(inside, ui, &id) {
													out = Some(t);
												};
											}
										},
										DataEnum::Int(_, _) | DataEnum::Float(_) => {
											if input.name != *"time" && ui.button(id.replace("----", " ").replace("Shape ", "").replace("style ", "").replace("JudgeFieldInner ", "").trim()).is_clicked() {
												out = Some(id)
											}
										}
										_ => {}
									}

									out
								}

								match &current_editing {
									Select::Shape(id) => {
										ui.label("select a attribute to animate");
										if let Some(t) = chart.shapes.get(id) {
											let mut input = match to_data(&t.shape) {
												Ok(t) => t,
												Err(e) => {
													msg.message(format!("{}", e), ui);
													return;
												}
											};
											if let Some(t) = selector(&mut input, ui, &String::new()) {
												inner.current_animation_attribute = t;
											}
										}
									},
									Select::JudgeField(id) => {
										if let Some(t) = chart.judge_fields.get(id) {
											let mut input = match to_data(&t.inner) {
												Ok(t) => t,
												Err(e) => {
													msg.message(format!("{}", e), ui);
													return;
												}
											};
											if let Some(t) = selector(&mut input, ui, &String::new()) {
												inner.current_animation_attribute = t;
											}
										}
									},
									_ => {
										ui.label("unsupported");
									}
								};
								return;
							};
							let mut paste_to = |animation: &mut Animation, copied_animation: Option<Animation>| {
								if let Some(mut copied_animation) = copied_animation {
									copied_animation.start_time = copied_animation.start_time + inner.time_pointer + inner.paste_offset;
									copied_animation.start_value += inner.value_offset;
									for linker in &mut copied_animation.linkers {
										linker.end_value += inner.value_offset;
									}
									animation.combine(&mut copied_animation, inner.current_linker.clone());
								}else {
									msg.message("no animation has pasted", ui);
								};
							};
							if need_paste_to_all {
								for id in selects {
									let animation = match &id {
										Select::Shape(id) => {
											if let Some(t) = chart.shapes.get_mut(id) {
												t.animation.get_mut(&inner.current_animation_attribute)
											}else {
												msg.message("cant find select resorce", ui);
												return;
											}
										},
										Select::JudgeField(id) => {
											if let Some(t) = chart.judge_fields.get_mut(id) {
												t.animation.get_mut(&inner.current_animation_attribute)
											}else {
												msg.message("cant find select resorce", ui);
												return;
											}
										},
										_ => {
											ui.label("unsupported");
											return;
										}
									};
									if let Some(animation) = animation {
										paste_to(animation, inner.copied_animation.clone());
									}
								}
							}
							let (shape_start_time, shape_sustain_time, animation) = match &current_editing {
								Select::Shape(id) => {
									if let Some(t) = chart.shapes.get_mut(id) {
										(t.start_time, t.sustain_time, t.get_animation_map())
									}else {
										msg.message("cant find select resorce", ui);
										return;
									}
								},
								Select::JudgeField(id) => {
									if let Some(t) = chart.judge_fields.get_mut(id) {
										(t.start_time, t.sustain_time, t.get_animation_map())
									}else {
										msg.message("cant find select resorce", ui);
										return;
									}
								},
								_ => {
									ui.label("unsupported");
									return;
								}
							};
							if inner.is_animation_delete {
								animation.remove(&inner.current_animation_attribute);
								inner.current_animation_attribute.clear();
								inner.is_animation_delete = false;
							}
							let animation = if let Some(t) = animation.get_mut(&inner.current_animation_attribute) {
								t
							}else {
								animation.insert(inner.current_animation_attribute.clone(), Default::default());
								return;
							};

							if need_copy {
								let mut copied_animation = animation.clone();
								copied_animation.start_time -= inner.time_pointer;
								inner.copied_animation = Some(copied_animation);
								need_copy = false;
							}
							if need_paste {
								paste_to(animation, inner.copied_animation.clone());
								need_paste = false;
							}
							let sustain_time = info.sustain_time; 
							let beats = info.total_beats();
							let height = ui.window_area().height() - 32.0;
							let width = beats * BEAT_WIDTH * inner.timeline_scale_factor;
							let current_scroll = - card.scroll(ui).x;
							let card_area = ui.window_area();
							let cursor_position = ui.input().cursor_position().unwrap_or(Vec2::INF) - card_area.left_top();
							let inner_position = ui.input().cursor_position().unwrap_or(Vec2::INF);

							let min_value = animation.min_value();
							let max_value = animation.max_value();
							let mut delta = max_value - min_value;
							let animation_y_start = height * 0.1;
							let animation_y_end = height * 0.9;

							if delta == 0.0 {
								delta = 0.5
							}

							let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
							let beats = beats.floor() / beat_quator;
							let res = ui.canvas(Vec2::new(width, height), |painter| {
								let start = (current_scroll * beats / width).floor();
								for i in (start as usize)..(beats as usize) {
									let position = width / beats * i as f32;
									painter.set_position(Vec2::x(position));
									painter.set_color(1.0);
									painter.set_scale(Vec2::same(0.8));
									if beat_quator <= 2.0 {
										let divide = (4.0 / beat_quator).floor() as usize;
										let text = format!("{}.{}", i / divide, i % divide + 1);
										painter.text(text);
									}else {
										let divide = 4.0 / beat_quator;
										let text = format!("{:.0}", i as f32 / divide);
										painter.text(text);
									}
									painter.set_scale(Vec2::NOT_TO_SCALE);
									
									painter.set_position(Vec2::new(position, 16.0));
									painter.set_color([1.0,1.0,1.0, 0.3]);
									painter.rect(Vec2::new(4.0, height - 16.0), Vec2::same(2.0));
									if position - current_scroll > card_area.width() {
										break;
									}
								}

								let offset = (animation_y_start - animation_y_end) * (inner.value_offset / delta);
								painter.set_color(1.0);
								let mut x = width * (animation.start_time / sustain_time) as f32;
								let y = animation_y_end + (animation_y_start - animation_y_end) * (animation.start_value - min_value) / delta - offset;
								let mut end_position = Vec2::new(x, y);
								painter.set_position(end_position - Vec2::same(4.0));
								painter.cir(4.0);
								painter.set_position(end_position);
								painter.text(format!("{:.4}", animation.start_value));

								for linker in &animation.linkers {
									x += width * (linker.sustain_time / sustain_time) as f32;
									let y = (animation_y_end + (animation_y_start - animation_y_end) * (linker.end_value - min_value) / delta) - offset;
									end_position = Vec2::new(x, y);
									match linker.linker {
										AnimationLinker::Bezier(pt1, pt2) => {
											let start_position = painter.style().position;
											let ctrl1 = (end_position - start_position) * pt1;
											let ctrl2 = (end_position - start_position) * pt2;
											painter.set_stroke_width(4.0);
											painter.set_stroke_color(1.0);
											painter.set_color([0,0,0,0]);
											painter.bezier([Vec2::ZERO, ctrl1, ctrl2, end_position - start_position]);
											painter.set_stroke_width(0.0);
											painter.set_position(ctrl1 - Vec2::same(4.0) + start_position);
											painter.set_color(1.0);
											painter.cir(4.0);
											painter.set_position(start_position);
											painter.set_color([255,255,255,100]);
											painter.line(ctrl1);
											painter.set_position(ctrl2 - Vec2::same(4.0) + start_position);
											painter.set_color(1.0);
											painter.cir(4.0);
											painter.set_position(end_position);
											painter.set_color([255,255,255,100]);
											painter.line(ctrl2 + start_position - end_position);
											painter.set_color(1.0);
										},
										AnimationLinker::Power(_) => {
											// TODO
										},
										AnimationLinker::Linear => {
											let start_position = painter.style().position;
											painter.set_stroke_width(4.0);
											painter.set_stroke_color(1.0);
											painter.set_color([0,0,0,0]);
											painter.line(end_position - start_position);
											painter.set_stroke_width(0.0);
											painter.set_color(1.0);
										},
										AnimationLinker::Mutation => {
											let start_position = painter.style().position;
											painter.set_stroke_width(4.0);
											painter.set_stroke_color(1.0);
											painter.set_color([0,0,0,0]);
											painter.line(Vec2::x((end_position - start_position).x));
											painter.set_position(end_position);
											painter.line(Vec2::y((start_position - end_position).y));
											painter.set_stroke_width(0.0);
											painter.set_color(1.0);
										},
									}
									painter.set_position(end_position - Vec2::same(4.0));
									painter.cir(4.0);
									painter.set_position(end_position);
									painter.text(format!("{:.4}", linker.end_value));
								}

								let x = shape_start_time.as_seconds_f32() / sustain_time.as_seconds_f32() * width;
								painter.set_position(Vec2::x(x));
								painter.set_color([1.0,1.0,1.0, 0.5]);
								painter.rect(Vec2::new(4.0, height), Vec2::same(2.0));
								painter.set_color(1.0);
								painter.set_position(Vec2::x(x) + Vec2::y(16.0));
								painter.text("start".to_string());

								let x = shape_sustain_time.as_seconds_f32() / sustain_time.as_seconds_f32() * width + x;
								painter.set_position(Vec2::x(x));
								painter.set_color([1.0,1.0,1.0, 0.5]);
								painter.rect(Vec2::new(4.0, height), Vec2::same(2.0));
								painter.set_color(1.0);
								painter.set_position(Vec2::x(x) + Vec2::y(16.0));
								painter.text("end".to_string());

								let current = (current_time.as_seconds_f32() / sustain_time.as_seconds_f32()) * width;
								painter.set_position(Vec2::x(current));
								painter.set_color(0.5);
								painter.rect(Vec2::new(4.0, height), Vec2::same(2.0)); 

								if card_area.is_point_inside(&(inner_position)) {
									let x = if inner.is_adsorption {
										((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats
									}else {
										cursor_position.x - 16.0 + current_scroll
									};
									let y = cursor_position.y - 16.0;
									let value = ((height - y) - 0.1 * height) / (height * 0.8) * delta + min_value;
									painter.set_color([1.0,1.0,1.0, 0.3]);
									painter.set_position(Vec2::new(current_scroll, y));
									painter.rect(Vec2::new(card_area.width(), 4.0), Vec2::same(2.0));
									painter.set_position(Vec2::x(x));
									painter.rect(Vec2::new(4.0, height), Vec2::same(2.0)); 
									painter.set_color(1.0);
									painter.set_position(Vec2::new(x, y));
									painter.text(format!("{}", (value / 10.0_f32.powf(inner.round as f32)).round() * 10.0_f32.powf(inner.round as f32) + inner.value_offset));
									painter.set_position(Vec2::new(x - 2.0, y - 2.0));
									painter.cir(4.0);
								};
							});

							if inner.need_scroll_to_now {
								inner.need_scroll_to_now = false;
								let current = (current_time.as_seconds_f32() / sustain_time.as_seconds_f32()) * width - ui.window_area().width() / 2.0;
								card.scroll_to_x(current, ui);
							}

							if res.is_clicked() && 
							(ui.input().cursor_position().unwrap_or(Vec2::INF).is_inside(Area::new(res.area.left_top(), res.area.right_top() + Vec2::y(16.0))) ||
							ui.input().is_key_pressing(Key::AltLeft) ||
							ui.input().is_mouse_released(MouseButton::Middle) ) {
								inner.is_something_changed = true;
								let x = if inner.is_adsorption {
									((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats
								}else {
									cursor_position.x - 16.0 + current_scroll
								};
								inner.time_pointer = x / width * sustain_time;
							}else if res.is_multi_clicked(2) && ui.input().is_mouse_released(MouseButton::Left) {
								let x = if inner.is_adsorption {
									((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats
								}else {
									cursor_position.x - 16.0 + current_scroll
								};
								let y = cursor_position.y - 16.0;
								let value = ((height - y) - 0.1 * height) / (height * 0.8) * delta + min_value;
								let time = x / width * sustain_time;
								let round = (value / 10.0_f32.powf(inner.round as f32)).round() * 10.0_f32.powf(inner.round as f32) + inner.value_offset;
								animation.insert_point(time, round, inner.current_linker.clone());
							}
							if res.is_clicked() && ui.input().is_mouse_released(MouseButton::Right) {
								inner.is_something_changed = true;
								let x = if inner.is_adsorption {
									((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats
								}else {
									cursor_position.x - 16.0 + current_scroll
								};
								let time = x / width * sustain_time;
								animation.remove_point(time);
							}
						};
					}
				}
			);
		}
	)
}

fn detail(inner: &mut EditRouter, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	ui.show(&mut Card::new("element manager")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(320.0)
		.set_width(480.0)
		.set_resizable(true)
		.set_position(position)
		.set_scrollable([false; 2]), |ui, _| {
			let mut selects = match core.current_selects() {
				Ok(t) => t,
				Err(e) => {
					msg.message(format!("{}", e), ui);
					return;
				}	
			};
			// let card_height = ui.window_area().height();
			// let card_window_y = ui.window_area().left_top().y;
			let toolbar_width = 270.0;
			ui.show(&mut Card::new("selector")
				.set_rounding(Vec2::same(16.0))
				.set_color(ui.style().background_color.brighter(0.05))
				.set_width(toolbar_width)
				.set_scrollable([true; 2]), |ui,_| {
					ui.label("element manager");
					if ui.button("close").is_clicked() {
						inner.is_detail_on = false;
					}
					if ui.button("clear selects").is_clicked() {
						inner.current_animation_editing = None;
						if let Err(e) = core.clear_selects() {
							msg.message(format!("{}", e), ui);
						}
					}
					if ui.button("delete selects").is_multi_clicked(2) {
						if let Some((chart, _)) = &mut core.current_chart {
							for select in &selects {
								match select {
									Select::Note(id) => {
										chart.notes.remove(id);
									},
									Select::Shape(id) => {
										chart.shapes.remove(id);
									},
									Select::JudgeField(id) => {
										chart.judge_fields.remove(id);
									},
									Select::ClickEffect(_) => {},
									Select::Script(()) => {},
								}
							}
						}

						if let Err(e) = core.clear_selects() {
							msg.message(format!("{}", e), ui);
						}
					}
					ui.switch(&mut inner.is_adsorption, "adsorption");
					if ui.button("copy").is_clicked() {
						if let Err(e) = core.copy_select(inner.time_pointer, |_| true) {
							msg.message(format!("{}", e), ui);
						}else {
							msg.message("copied successfully", ui);
						}
					}
					if ui.button("paste").is_clicked() {
						if let Err(e) = core.paste_select(inner.time_pointer + inner.paste_offset) {
							msg.message(format!("{}", e), ui);
						}else {
							msg.message("pasted successfully", ui);
						}
					}
					ui.add(Slider::new(0.06..=16.0, &mut inner.timeline_scale_factor, "scale").step(0.01).logarithmic(true));
					if let Some((chart, info)) = &mut core.current_chart {
						let sustain_time = info.sustain_time; 
						let beats = info.total_beats();
						let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
						let beats = beats / beat_quator;
						let bps = beats / sustain_time.as_seconds_f32();
						let change_time = |input: &mut Duration, text: String, ui: &mut Ui| {
							ui.horizental(|ui| {
								if !inner.is_adsorption {
									let mut sec = input.as_seconds_f32();
									if ui.button("-").is_clicked() {
										sec -= 0.1
									}
									ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
									if ui.button("+").is_clicked() {
										sec += 0.1
									}
									*input = Duration::seconds_f32(sec);
								}else {
									let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
									if ui.button("-").is_clicked() {
										sustain_beats = sustain_beats.saturating_sub(1);
									}
									ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
									if ui.button("+").is_clicked() {
										sustain_beats += 1;
									}
									*input = Duration::seconds_f32(sustain_beats as f32 / bps);
								}
							});
							if *input > sustain_time {
								*input = sustain_time
							}
							if *input < Duration::ZERO {
								*input = Duration::ZERO
							}
						};
						change_time(&mut inner.filter, "filter(±)".to_string(), ui);
						ui.collapsing("notes", |ui, _| {
							let mut note_display_vec = vec!();
							for (id, note) in &chart.notes {
								if note.judge_time < inner.time_pointer - inner.filter || note.judge_time > inner.time_pointer + inner.filter {
									continue;
								}
								note_display_vec.push(id);
							}
							note_display_vec.par_sort();
							for id in note_display_vec {
								if ui.switch(&mut selects.contains(&Select::Note(id.clone())), id).is_clicked() {
									if selects.contains(&Select::Note(id.clone())) {
										selects.retain(|inner| inner != &Select::Note(id.clone()));
									}else {
										selects.push(Select::Note(id.clone()));
									}
								};
							}
						});

						ui.collapsing("judge_fields", |ui, _| {
							if ui.button("add").is_clicked() {
								let mut index = chart.judge_fields.len();
								loop {
									if let std::collections::hash_map::Entry::Vacant(e) = chart.judge_fields.entry(index.to_string()) {
										e.insert(JudgeField {
											start_time: inner.current_time,
											sustain_time: Duration::seconds(1),
											..Default::default()
										});
										break;
									} else {
										index += 1
									}
								}
							}
							let mut field_display_vec = vec!();
							for (id, field) in &chart.judge_fields {
								if field.start_time + field.sustain_time < inner.time_pointer - inner.filter || field.start_time > inner.time_pointer + inner.filter {
									continue;
								}
								field_display_vec.push(id);
							}
							field_display_vec.par_sort();
							for id in field_display_vec {
								if ui.switch(&mut selects.contains(&Select::JudgeField(id.clone())), id).is_clicked() {
									if selects.contains(&Select::JudgeField(id.clone())) {
										selects.retain(|inner| inner != &Select::JudgeField(id.clone()));
									}else {
										selects.push(Select::JudgeField(id.clone()));
									}
								};
							}
						});

						// ui.collapsing("click effects", |ui, _| {
						// 	for (id, _) in &chart.click_effects {
						// 		if ui.switch(&mut selects.contains(&Select::ClickEffect(id.clone())), id).is_clicked() {
						// 			if selects.contains(&Select::ClickEffect(id.clone())) {
						// 				selects.retain(|inner| inner != &Select::ClickEffect(id.clone()));
						// 			}else {
						// 				selects.push(Select::ClickEffect(id.clone()));
						// 			}
						// 		};
						// 	}
						// });

						ui.collapsing("shapes", |ui, _| {
							ui.collapsing("add" ,|ui, _| {
								let mut add_shape = |ui: &mut Ui, text: String, add: ShapeElement| {
									if ui.button(text).is_clicked() {
										let mut index = chart.shapes.len();
										loop {
											if let std::collections::hash_map::Entry::Vacant(e) = chart.shapes.entry(index.to_string()) {
												e.insert(shapoist_core::system::core_structs::Shape { 
													id: index.to_string(),
													shape: Shape {
														shape: add,
														..Default::default()
													},
													start_time: inner.current_time,
													sustain_time: Duration::seconds(1),
													..Default::default()
												});
												break;
											} else {
												index += 1
											}
										}
									}
								};
								add_shape(ui, "Circle".to_string(), ShapeElement::Circle(Default::default()));
								add_shape(ui, "Rect".to_string(), ShapeElement::Rect(Default::default()));
								add_shape(ui, "Text".to_string(), ShapeElement::Text(Default::default()));
								add_shape(ui, "CubicBezier".to_string(), ShapeElement::CubicBezier(Default::default()));
								add_shape(ui, "Line".to_string(), ShapeElement::Line(Default::default()));
							});
							let mut shape_display_vec = vec!();
							for (id, shape) in &chart.shapes {
								if !(shape.start_time - inner.filter < inner.time_pointer && shape.start_time + shape.sustain_time + inner.filter > inner.time_pointer) {
									continue;
								}
								shape_display_vec.push(id);
							}
							shape_display_vec.par_sort();
							for id in shape_display_vec {
								if ui.switch(&mut selects.contains(&Select::Shape(id.clone())), id).is_clicked() {
									if selects.contains(&Select::Shape(id.clone())) {
										selects.retain(|inner| inner != &Select::Shape(id.clone()));
									}else {
										selects.push(Select::Shape(id.clone()));
									}
								};
							}
						});
					}
				}
			);

			let area = ui.window_area();
			let mut delta: HashMap<String, f64> = HashMap::new();

			ui.show(&mut Card::new("inner")
				.set_rounding(Vec2::same(16.0))
				.set_color(ui.style().background_color.brighter(0.05))
				.set_width(area.width() - toolbar_width - 16.0 * 3.0)
				.set_scrollable([true; 2])
				.set_position(Vec2::new(toolbar_width + 16.0 * 2.0, 16.0)),
				|ui, _| {
					if let Some((chart, info)) = &mut core.current_chart {
						let sustain_time = info.sustain_time; 
						let beats = info.total_beats();
						let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
						let beats = beats / beat_quator;
						let bps = beats / sustain_time.as_seconds_f32();
						let change_time = |input: &mut Duration, text: String, ui: &mut Ui| {
							ui.horizental(|ui| {
								if !inner.is_adsorption {
									let mut sec = input.as_seconds_f32();
									if ui.button("-").is_clicked() {
										sec -= 0.1
									}
									ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
									if ui.button("+").is_clicked() {
										sec += 0.1
									}
									*input = Duration::seconds_f32(sec);
								}else {
									let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
									if ui.button("-").is_clicked() {
										sustain_beats = sustain_beats.saturating_sub(1);
									}
									ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
									if ui.button("+").is_clicked() {
										sustain_beats += 1;
									}
									*input = Duration::seconds_f32(sustain_beats as f32 / bps);
								}
							});
							if *input > sustain_time {
								*input = sustain_time
							}
							if *input < Duration::ZERO {
								*input = Duration::ZERO
							}
						};
						for select in &selects {
							match &select {
								Select::Note(inner) => {
									let mut is_delete = false;
									if let Some(t) = chart.notes.get_mut(inner) {
										ui.collapsing(format!("{:?}", select), |ui, _| {
											match settings_with_delta(t, inner.clone(), ui) {
												Ok(t) => {
													if !t.is_empty() {
														delta = t
													}
												},
												Err(e) => msg.message(format!("{}", e), ui),
											}
											change_time(&mut t.judge_time, "judge_time".into(), ui);
											is_delete = ui.button("delete").is_clicked();
										});
									};
									if is_delete {
										chart.notes.remove(inner);
									}
								},
								Select::JudgeField(inner) => {
									let mut is_delete = false;
									if let Some(t) = chart.judge_fields.get_mut(inner) {
										ui.collapsing(format!("{:?}", select), |ui, _| {
											match settings_with_delta(&mut t.inner, inner.clone(), ui) {
												Ok(t) => { 
													if !t.is_empty() {
														delta = t
													}
												},
												Err(e) => msg.message(format!("{}", e), ui),
											}
											change_time(&mut t.start_time, "start_time".into(), ui);
											change_time(&mut t.sustain_time, "sustain_time".into(), ui);
											is_delete = ui.button("delete").is_clicked();
										});
									};
									if is_delete {
										chart.judge_fields.remove(inner);
									}
								},
								Select::Shape(inner) => {
									let mut is_delete = false;
									if let Some(t) = chart.shapes.get_mut(inner) {
										ui.collapsing(format!("{:?}", select), |ui, _| {
											match settings_with_delta(&mut t.shape, inner.clone(), ui) {
												Ok(t) => { 
													if !t.is_empty() {
														delta = t
													}
												},
												Err(e) => msg.message(format!("{}", e), ui),
											}
											change_time(&mut t.start_time, "start_time".into(), ui);
											change_time(&mut t.sustain_time, "sustain_time".into(), ui);
											is_delete = ui.button("delete").is_clicked();
										});
									};
									if is_delete {
										chart.shapes.remove(inner);
									}
								},
								Select::ClickEffect(inner) => {
									let mut is_delete = false;
									if let Some(t) = chart.click_effects.get_mut(inner) {
										ui.collapsing(format!("{:?}", select), |ui, _| {
											match settings_with_delta(t, inner.clone(), ui) {
												Ok(t) => { 
													if !t.is_empty() {
														delta = t
													}
												},
												Err(e) => msg.message(format!("{}", e), ui),
											}
											is_delete = ui.button("delete").is_clicked();
										});
									}
									if is_delete {
										chart.click_effects.remove(inner);
									}
								},
								Select::Script(_) => {},
							}
						}
					};
				}
			);
			if !delta.is_empty() {
				if !core.timer.is_started() {
					inner.is_something_changed = true;
				}
				if let Some((chart, _)) = &mut core.current_chart {
					for select in &selects {
						match &select {
							Select::Note(inner) => {
								if let Some(t) = chart.notes.get_mut(inner) {
									if let Err(e) = apply_delta(t, &delta) {
										msg.message(format!("{}", e), ui);
									}
								};
							},
							Select::JudgeField(inner) => {
								if let Some(t) = chart.judge_fields.get_mut(inner) {
									if let Err(e) = apply_delta(&mut t.inner, &delta) {
										msg.message(format!("{}", e), ui);
									}
								};
							},
							Select::Shape(inner) => {
								if let Some(t) = chart.shapes.get_mut(inner) {
									if let Err(e) = apply_delta(&mut t.shape, &delta) {
										msg.message(format!("{}", e), ui);
									}
								};
							},
							Select::ClickEffect(_) => {},
							Select::Script(_) => {},
						}
					}
				}
			}
			if let Err(e) = core.multi_select(selects) {
				msg.message(format!("{}", e), ui);
			};
		}
	)
}

fn chart_info(inner: &mut EditRouter, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	ui.show(&mut Card::new("chart info")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(180.0 - 32.0)
		.set_width(480.0)
		.set_resizable(true)
		.set_position(position)
		.set_scrollable([true; 2]), |ui, _| {
			ui.label("chart info");
			if ui.button("close").is_clicked() {
				inner.is_chart_info_on = false;
			}
			if let Some((_, info)) = &mut core.current_chart {
				let value = |ui: &mut Ui, value: &mut f32, range: std::ops::RangeInclusive<f32>, step: f64| {
					ui.horizental(|ui| {
						if ui.button("round").is_clicked() {
							*value = (*value / 10_f32.powf(inner.round as f32)).round() * 10_f32.powf(inner.round as f32);
						}
						if ui.button("-").is_clicked() {
							*value -= 10_f32.powf(inner.round as f32);
						}
						ui.add(Slider::new(range, value, "").step(step));
						if ui.button("+").is_clicked() {
							*value += 10_f32.powf(inner.round as f32);
						}
					});
				};

				let sustain_time = info.sustain_time; 
				let beats = info.total_beats();
				let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
				let beats = beats / beat_quator;
				let bps = beats / sustain_time.as_seconds_f32();
				let change_time = |input: &mut Duration, text: String, ui: &mut Ui, allow_neg: bool| {
					ui.horizental(|ui| {
						if !inner.is_adsorption {
							let mut sec = input.as_seconds_f32();
							if ui.button("-").is_clicked() {
								sec -= 0.1
							}
							let range = if allow_neg {
								-sustain_time.as_seconds_f32()..=sustain_time.as_seconds_f32()
							}else {
								0.01..=sustain_time.as_seconds_f32()
							};
							ui.add(Slider::new(range, &mut sec, text).step(0.01).suffix("s"));
							if ui.button("+").is_clicked() {
								sec += 0.1
							}
							*input = Duration::seconds_f32(sec);
						}else {
							let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as isize;
							if ui.button("-").is_clicked() {
								sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
							}
							let range = if allow_neg {
								-beats.ceil() as isize..=beats.ceil() as isize
							}else {
								0..=beats.ceil() as isize
							};
							ui.add(Slider::new(range, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
							if ui.button("+").is_clicked() {
								sustain_beats += 1;
							}
							*input = Duration::seconds_f32(sustain_beats as f32 / bps);
						}
					});
					if *input > sustain_time {
						*input = sustain_time
					}
					if allow_neg {
						if *input < -sustain_time {
							*input = sustain_time
						}
					}else if *input < Duration::ZERO {
						*input = Duration::ZERO
					}
					
				};
				change_time(&mut inner.time_pointer, "time pointer".to_string(), ui, false);
				change_time(&mut inner.paste_offset, "paste offset".to_string(), ui, true);
				change_time(&mut inner.filter, "filter(±)".to_string(), ui, false);
				if ui.button("reset offset").is_clicked() {
					inner.paste_offset = Duration::ZERO;
				}
				let is_need_play = ui.switch(&mut core.timer.is_started(), "play").is_clicked();
				if let Some(editor) = &mut core.chart_editor {
					ui.switch(&mut editor.show_click_effect, "show click effect");
				}

				ui.switch(&mut inner.is_show_id, "show shape id");

				ui.label("song_name");
				ui.single_input(&mut info.song_name);
				ui.label("producer");
				ui.single_input(&mut info.producer);
				ui.label("charter");
				ui.single_input(&mut info.charter);
				ui.label("artist");
				ui.single_input(&mut info.artist);
				ui.label("version");
				ui.horizental(|ui| {
					ui.add(DragableValue::new(&mut info.version.major).set_text("major").speed(0.1));
					ui.add(DragableValue::new(&mut info.version.minor).set_text("minor").speed(0.1));
					ui.add(DragableValue::new(&mut info.version.patch).set_text("patch").speed(0.1));
				});

				ui.label("bpm");
				value(ui, &mut info.bpm.start_bpm, 50.0..=500.0, 0.01);

				ui.label("offset(ms)");
				let mut ms = info.offset.as_seconds_f32() * 1e3;
				value(ui, &mut ms, 0.0..=1000.0, 1.0);
				info.offset = Duration::seconds_f32(ms / 1e3);

				ui.label("diffculty");
				match &mut info.diffculty {
					Diffculty::Shapoist(t1, t2) => {
						ui.label("read diffculty");
						value(ui, t1, 0.0..=50.0, 0.01);
						ui.label("play diffculty");
						value(ui, t2, 0.0..=50.0, 0.01);
						if ui.button("change").is_clicked() {
							info.diffculty = Diffculty::Other(String::new())
						}
					},
					Diffculty::Other(inner) => {
						ui.single_input(inner);
						if ui.button("change").is_clicked() {
							info.diffculty = Diffculty::Shapoist(2.0,2.0)
						}
					}
				};

				if ui.button("clear selects").is_clicked() {
					inner.current_animation_editing = None;
					if let Err(e) = core.clear_selects() {
						msg.message(format!("{}", e), ui);
					}
				}
				ui.switch(&mut inner.is_adsorption, "adsorption");
				ui.add(Slider::new(0.06..=16.0, &mut inner.timeline_scale_factor, "scale").step(0.01).logarithmic(true));
				ui.label("round");
				ui.horizental(|ui| {
					if ui.button("-").is_clicked() {
						inner.round -= 1;
					}
					ui.dragable_value(&mut inner.round);
					if ui.button("+").is_clicked() {
						inner.round += 1;
					}
					inner.round = inner.round.clamp(-4, 4);
				});

				if is_need_play {
					inner.is_something_changed = true;
					if let Err(e) = if core.timer.is_started() {
						core.pause()
					}else {
						core.play_with_time(PlayMode::Auto, inner.current_time)
					} {
						msg.message(format!("{}", e), ui);
					}
				}
			}
		}
	)
}

fn linker(inner: &mut EditRouter, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	ui.show(&mut Card::new("linker")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(320.0)
		.set_width(480.0)
		.set_resizable(true)
		.set_position(position)
		.set_scrollable([false; 2]), 
	|ui, _| {
		let area = ui.window_area();
		let toolbar_width = (area.width() - 48.0) / 2.0;
		ui.label("linker");
		if ui.button("close").is_clicked() {
			inner.is_linker_on = false;
		}

		ui.show(&mut Card::new("current shape")
			.set_rounding(Vec2::same(16.0))
			.set_color(ui.style().background_color.brighter(0.05))
			.set_width(toolbar_width)
			.set_position(Vec2::new(16.0, 96.0))
			.set_scrollable([true; 2]), 
		|ui,_| {
			ui.label("current shapes");
			if let Some((chart, _)) = &mut core.current_chart {
				let mut need_clear = None;
				if inner.is_linking_note.is_some() {
					for shape in chart.shapes.values() {
						if shape.start_time <= inner.time_pointer && shape.start_time + shape.sustain_time >= inner.time_pointer && ui.button(&shape.id).is_clicked() {
							inner.link_id_temp = Some(shape.id.clone());
						}
					}
				}else {
					for shape in chart.shapes.values() {
						if shape.start_time <= inner.time_pointer && shape.start_time + shape.sustain_time >= inner.time_pointer {
							ui.collapsing(format!("Shape: {}", shape.id), |ui, _| {
								if let Some(id) = &shape.linked_note_id {
									ui.label("linked to");
									for id in id {
										ui.label(id);
									}
									if ui.button("change").is_clicked() {
										inner.is_linking_shape = Some(shape.id.clone())
									}
									if ui.button("clear").is_clicked() {
										need_clear = Some(shape.id.clone());
									}
								}else if ui.button("link").is_clicked() {
									inner.is_linking_shape = Some(shape.id.clone())
								}
							});
						}
					}
				}
				if let Some(id) = need_clear {
					if let Some(shape) = chart.shapes.get_mut(&id) {
						shape.linked_note_id = None;
					}
					if let Err(e) = core.refresh_play_info() {
						msg.message(format!("{}", e), ui);
					};
				}
			}
		});

		ui.show(&mut Card::new("current note")
			.set_rounding(Vec2::same(16.0))
			.set_color(ui.style().background_color.brighter(0.05))
			.set_width(toolbar_width)
			.set_position(Vec2::new(toolbar_width + 32.0, 16.0))
			.set_scrollable([true; 2]), 
		|ui,_| {
			ui.label("current notes");
			if let Some((chart, info)) = &mut core.current_chart {
				let sustain_time = info.sustain_time; 
				let beats = info.total_beats();
				let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
				let beats = beats / beat_quator;
				let bps = beats / sustain_time.as_seconds_f32();
				let change_time = |input: &mut Duration, text: String, ui: &mut Ui| {
					ui.horizental(|ui| {
						if !inner.is_adsorption {
							let mut sec = input.as_seconds_f32();
							if ui.button("-").is_clicked() {
								sec -= 0.1
							}
							ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
							if ui.button("+").is_clicked() {
								sec += 0.1
							}
							*input = Duration::seconds_f32(sec);
						}else {
							let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
							if ui.button("-").is_clicked() {
								sustain_beats = sustain_beats.saturating_sub(1);
							}
							ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
							if ui.button("+").is_clicked() {
								sustain_beats += 1;
							}
							*input = Duration::seconds_f32(sustain_beats as f32 / bps);
						}
					});
					if *input > sustain_time {
						*input = sustain_time
					}
					if *input < Duration::ZERO {
						*input = Duration::ZERO
					}
				};
				change_time(&mut inner.range_of_current_note, "search range(±): ".to_string(), ui);
				if inner.is_linking_shape.is_some() {
					for note in chart.notes.values() {
						if note.judge_time > inner.time_pointer - inner.range_of_current_note && note.judge_time < inner.time_pointer + inner.range_of_current_note && ui.button(&note.note_id).is_clicked() {
							inner.link_id_temp = Some(note.note_id.clone());
						}
					}
				}else {
					for note in chart.notes.values_mut() {
						if note.judge_time > inner.time_pointer - inner.range_of_current_note && note.judge_time < inner.time_pointer + inner.range_of_current_note {
							ui.collapsing(format!("Note: {}", note.note_id), |ui, _| {
								if let Some(id) = &note.linked_shape {
									ui.label("linked to");
									for id in id {
										ui.label(id);
									}
									if ui.button("change").is_clicked() {
										inner.is_linking_note = Some(note.note_id.clone())
									}
									if ui.button("clear").is_clicked() {
										note.linked_shape = None;
									}
								}else if ui.button("link").is_clicked() {
									inner.is_linking_note = Some(note.note_id.clone())
								}
							});
						}
					}
				}
			}
		});

		if if let Some(id) = &inner.link_id_temp {
			if let Some((chart, _)) = &mut core.current_chart {
				if let Some(note_id) = &inner.is_linking_note {
					if let Some(note) = chart.notes.get_mut(note_id) {
						note.linked_shape = Some(vec!(id.clone()))
					}
				}

				if let Some(shape_id) = &inner.is_linking_shape {
					if let Some(shape) = chart.shapes.get_mut(shape_id) {
						shape.linked_note_id = Some(vec!(id.clone()))
					}
				}
			}
			inner.is_linking_note = None;
			inner.is_linking_shape = None;
			if let Err(e) = core.refresh_play_info() {
				if let Err(e) = core.play(PlayMode::Auto) {
					msg.message(format!("{}", e), ui);
				}else {
					if let Err(e) = core.pause() {
						msg.message(format!("{}", e), ui);
					}
					msg.message(format!("{}", e), ui);
				}
			}
			true
		}else {
			false
		} {
			inner.link_id_temp = None;
		}
	})
}

macro_rules! switch_left_right {
	($type_to_get: tt, $to_animation: tt, $chart: ident, $inner: ident, $left: ident, $right: ident, $msg: ident, $ui: ident) => {
		let mut left_get = $chart.$type_to_get.remove($left);
		let mut right_get = $chart.$type_to_get.remove($right);
		if let (Some(left), Some(right)) = (&mut left_get, &mut right_get) {
			switch_animation(&mut left.animation, &mut right.animation, &$inner.switcher.current_attributes);
			let mut delta_left = match caculate_delta(&left.$to_animation, &right.$to_animation) {
				Ok(t) => t,
				Err(e) => {
					$msg.message(format!("{}", e), $ui);
					return;
				}
			};
			delta_left.retain(|key, _| {
				$inner.switcher.current_attributes.contains(key)
			});
			let mut delta_right = match caculate_delta(&right.$to_animation, &left.$to_animation) {
				Ok(t) => t,
				Err(e) => {
					$msg.message(format!("{}", e), $ui);
					return;
				}
			};
			delta_right.retain(|key, _| {
				$inner.switcher.current_attributes.contains(key)
			});
			let _ = apply_delta(left, &delta_left);
			let _ = apply_delta(right, &delta_right);
		}
		if let Some(left_get) = &left_get {
			$chart.$type_to_get.insert($left.to_string(), left_get.clone());
		}
		if let Some(right_get) = &left_get {
			$chart.$type_to_get.insert($right.to_string(), right_get.clone());
		}
	}
}

fn macro_window(inner: &mut EditRouter, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	ui.show(&mut Card::new("macro")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(320.0)
		.set_width(480.0)
		.set_resizable(true)
		.set_position(position)
		.set_scrollable([true; 2]), 
	|ui, _| {
		ui.collapsing("note-shape gengerator", |ui, _| {
			ui.label("a simple macro to generate shape for notes automaticly");
			ui.label("Warn: you need put time pointer exactly at your linked shapes to let this macro function correctly");
			ui.label("Note: linkings are dynamically, whick means they will change as long as orignal shapes changes until finally generated");
			ui.label("Note: shape offset follows paste offset");
			ui.label("Note: start form where timer pointer points");
			ui.label("Note: when generating, note-shape gengerator will automaticly remove last generate result in curren range(based on start_time)");
			let show = |ui: &mut Ui, core: &mut ShapoistCore, shape: &mut Vec<String>| {
				for id in shape.iter() {
					ui.label(id);
				}
				if ui.button("change to selected").is_clicked() {
					let selects = match core.current_selects() {
						Ok(t) => t,
						Err(e) => {
							msg.message(format!("{}", e), ui);
							return;
						}	
					};
					let mut new_shape = vec!();
					for select in selects {
						if let Select::Shape(id) = select {
							new_shape.push(id.clone())
						}
					}
					*shape = new_shape;
				}
			};
			ui.collapsing("linked tap shape", |ui, _| {
				show(ui, core, &mut inner.note_generator.tap_shape);
			});
			ui.collapsing("linked slide shape", |ui, _| {
				show(ui, core, &mut inner.note_generator.slide_shape);
			});
			ui.collapsing("linked flick shape", |ui, _| {
				show(ui, core, &mut inner.note_generator.flick_shape);
			});
			if let Some((chart, info)) = &mut core.current_chart {
				let sustain_time = info.sustain_time; 
				let beats = info.total_beats();
				let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
				let beats = beats / beat_quator;
				let bps = beats / sustain_time.as_seconds_f32();
				let change_time = |input: &mut Duration, text: String, ui: &mut Ui| {
					ui.horizental(|ui| {
						if !inner.is_adsorption {
							let mut sec = input.as_seconds_f32();
							if ui.button("-").is_clicked() {
								sec -= 0.1
							}
							ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
							if ui.button("+").is_clicked() {
								sec += 0.1
							}
							*input = Duration::seconds_f32(sec);
						}else {
							let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
							if ui.button("-").is_clicked() {
								sustain_beats = sustain_beats.saturating_sub(1);
							}
							ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
							if ui.button("+").is_clicked() {
								sustain_beats += 1;
							}
							*input = Duration::seconds_f32(sustain_beats as f32 / bps);
						}
					});
					if *input > sustain_time {
						*input = sustain_time
					}
					if *input < Duration::ZERO {
						*input = Duration::ZERO
					}
				};
				change_time(&mut inner.note_generator.time_range, "generate range".to_string(), ui);

				let clear = |inner: &EditRouter, chart: &mut Chart| {
					let mut id_to_delete = vec!();
					for (id, shape)in &mut chart.shapes {
						let id_inner: Vec<&str> = id.split("generated shape").collect();
						if id_inner.len() == 2 && id_inner[0].is_empty() && shape.start_time >= inner.time_pointer && shape.start_time <= inner.time_pointer + inner.note_generator.time_range {
							id_to_delete.push(id.clone());
						}
					}
					for id in id_to_delete {
						chart.shapes.remove(&id);
					}
				};

				if ui.button("clear").is_clicked() {
					inner.is_something_changed = true;
					clear(inner, chart);
					msg.message("cleard successfully", ui);
				}

				if ui.button("generate").is_clicked() {
					inner.is_something_changed = true;
					clear(inner, chart);
					let offset = inner.paste_offset;
					let time_pointer = inner.time_pointer;
					let get_shape = |inner: &Vec<String>, chart: &mut Chart| -> Vec<shapoist_core::system::core_structs::Shape> { 
						let mut output_shapes = vec!();
						for id in inner {
							if let Some(shape_inner) = chart.shapes.get(id) {
								let mut output = shape_inner.clone();
								output.start_time = output.start_time - time_pointer + offset;
								for animation in output.animation.values_mut() {
									animation.start_time = animation.start_time - time_pointer + offset;
								}
								output_shapes.push(output);
							}
						}
						output_shapes
					};
					let process_shape = |input: &Vec<shapoist_core::system::core_structs::Shape>, note_id: &String, time: Duration| -> Vec<shapoist_core::system::core_structs::Shape> {
						let mut output = input.clone();
						for (i, shape) in output.iter_mut().enumerate() {
							shape.start_time += time;
							for animation in shape.animation.values_mut() {
								animation.start_time += time;
							}
							shape.id = format!("generated shape {} {}", note_id, i);
							shape.linked_note_id = Some(vec!(format!("{}", note_id)));
						}
						
						output
					};
					let tap_shape = get_shape(&inner.note_generator.tap_shape, chart);
					let slide_shape = get_shape(&inner.note_generator.slide_shape, chart);
					let flick_shape = get_shape(&inner.note_generator.flick_shape, chart);
					for (id, note) in &mut chart.notes {
						if note.judge_time >= inner.time_pointer && note.judge_time <= inner.time_pointer + inner.note_generator.time_range {
							if let JudgeType::Tap = note.judge_type {
								let shapes = process_shape(&tap_shape, id, note.judge_time);
								let mut ids = vec!();
								for shape in shapes {
									let shape_id = shape.id.clone();
									ids.push(shape_id.clone());
									chart.shapes.insert(shape_id, shape);
								}
								note.linked_shape = Some(ids);
							}else if let JudgeType::Slide = note.judge_type {
								let shapes = process_shape(&slide_shape, id, note.judge_time);
								let mut ids = vec!();
								for shape in shapes {
									let shape_id = shape.id.clone();
									ids.push(shape_id.clone());
									chart.shapes.insert(shape_id, shape);
								}
								note.linked_shape = Some(ids);
							}else if let JudgeType::Flick = note.judge_type {
								let shapes = process_shape(&flick_shape, id, note.judge_time);
								let mut ids = vec!();
								for shape in shapes {
									let shape_id = shape.id.clone();
									ids.push(shape_id.clone());
									chart.shapes.insert(shape_id, shape);
								}
								note.linked_shape = Some(ids);
							}
						}
					}

					msg.message("generated successfully", ui);
				}
			}	
		});
		ui.collapsing("temporary group", |ui, _| {
			ui.label("a temporary group that will sync animation to each child element");
			ui.label("only support judge_field and shape");
			ui.label("offset is same as animation editor's");
			let selects = match core.current_selects() {
				Ok(t) => t,
				Err(e) => {
					msg.message(format!("{}", e), ui);
					return;
				}	
			};
			let mut father_id_to_remove = vec!();
			let mut father_id_to_insert = None;
			for (father_id, child_ids) in &mut inner.grouper.group {
				ui.collapsing(format!("{:?}", father_id), |ui, _| {
					let mut child_id_to_remove = vec!();
					for (child_id, offset, attribute) in child_ids.iter_mut() {
						ui.collapsing(format!("{:?}", child_id), |ui, _| {
							ui.label(format!("current offset: {}", offset));
							if ui.button("change to current offset").is_clicked() {
								*offset = inner.value_offset;
							}
							ui.label(format!("current attribute: {}", attribute.replace("----", " ").replace("Shape ", "").replace("style ", "").replace("JudgeFieldInner ", "").trim()));
							if ui.button("change to current attribute").is_clicked() {
								attribute.clone_from(&inner.current_animation_attribute);
							}
							if ui.button("remove").is_clicked() {
								child_id_to_remove.push(child_id.clone());
							}
						});
					}
					child_ids.retain(|(id, _, _)| {
						!child_id_to_remove.contains(id)
					});
					if ui.button("add current select to child").is_clicked() {
						father_id_to_insert = Some(father_id.clone());
					}
					if ui.button("delete").is_clicked() {
						father_id_to_remove.push(father_id.clone());
					}
				});
			}
			for id in father_id_to_remove {
				inner.grouper.group.remove(&id);
			}
			if let Some(father_id) = father_id_to_insert {
				let mut childs = vec!();
				for select in &selects {
					match select {
						Select::JudgeField(_) | Select::Shape(_) => childs.push((select.clone(), inner.value_offset, inner.current_animation_attribute.clone())),
						_ => continue
					}
				}
				inner.grouper.group.insert(father_id.clone(), childs);
			}
			if ui.button("add current select to father").is_clicked() {
				for select in &selects {
					match select {
						Select::JudgeField(_) | Select::Shape(_) => inner.grouper.group.insert(select.clone(), vec!()),
						_ => continue
					};
				}
			}
			if ui.button("sync now").is_clicked() {
				inner.is_something_changed = true;
				inner.grouper.need_sync = true;
			}
			ui.label("value_offset");
			ui.horizental(|ui| {
				if ui.button("-").is_clicked() {
					inner.value_offset -= 10_f32.powf(inner.round as f32);
				}
				ui.add(Slider::new(-500.0..=500.0, &mut inner.value_offset, "").step(0.01));
				if ui.button("+").is_clicked() {
					inner.value_offset += 10_f32.powf(inner.round as f32);
				}
				if ui.button("round").is_clicked() {
					inner.value_offset = (inner.value_offset / 10_f32.powf(inner.round as f32)).round() * 10_f32.powf(inner.round as f32);
				}
				inner.value_offset = inner.value_offset.clamp(-500.0, 500.0);
			});
			if ui.button("reset offset").is_clicked() {
				inner.value_offset = 0.0;
			}
		});
		ui.collapsing("switch attributes", |ui, _| {
			ui.label("switch two object as long as they're in the same type");
			ui.label("switch operation includes animation");
			ui.collapsing("current attributes", |ui, _| {
				ui.label("click attribute to delete it");
				let mut id_to_delete = None;
				for attribute in &inner.switcher.current_attributes {
					if ui.button(attribute.replace("----", " ").replace("Shape ", "").replace("Note ", "").replace("style ", "").replace("JudgeFieldInner ", "").trim().to_string()).is_clicked() {
						id_to_delete = Some(attribute.clone());
					}
				}
				if let Some(id) = id_to_delete {
					inner.switcher.current_attributes.remove(&id);
				}
				if ui.button("add current attribute").is_clicked() && !inner.current_animation_attribute.is_empty() {
					inner.switcher.current_attributes.insert(inner.current_animation_attribute.clone());
				}
			});
			ui.collapsing("current select", |ui, _| {
				ui.label("click id to delete it");
				let mut current_selects = 0;
				let mut id_to_delete = None;
				for i in 0..2 {
					if let Some(select) = &inner.switcher.to_switch[i] {
						if ui.button(format!("{:?}", select)).is_clicked() {
							id_to_delete = Some(i);
						}
						current_selects += 1;
					}
				}
				if let Some(i) = id_to_delete {
					inner.switcher.to_switch[i] = None;
				}
				if current_selects < 2 {
					if let Some(current_animation_editing) = &inner.current_animation_editing {
						ui.label(format!("current editing: {:?}", current_animation_editing));
					}
					if ui.button("add current editing").is_clicked() {
						if let Some(current_animation_editing) = &inner.current_animation_editing {
							inner.switcher.to_switch[current_selects] = Some(current_animation_editing.clone());
						}
					}
				}
			});
			if ui.button("switch").is_clicked() {
				inner.is_something_changed = true;
				fn switch_animation(left: &mut HashMap<String, Animation>, right: &mut HashMap<String, Animation>, attributes: &HashSet<String>) {
					let mut middle_left = HashMap::new();
					for attribute in attributes {
						if let Some(animation) = left.get(attribute) {
							middle_left.insert(attribute.clone(), animation.clone());
						}
					}
					let mut middle_right = HashMap::new();
					for attribute in attributes {
						if let Some(animation) = right.get(attribute) {
							middle_right.insert(attribute.clone(), animation.clone());
						}
					}
					for (attribute_right, animation_right) in middle_right {
						left.insert(attribute_right, animation_right);
					}
					for (attribute_left, animation_left) in middle_left {
						right.insert(attribute_left, animation_left);
					}
				}
				if let Some((chart, _)) = &mut core.current_chart {
					if let (Some(left), Some(right)) = (&inner.switcher.to_switch[0] ,&inner.switcher.to_switch[1]) {
						match (left, right)  {
							(Select::Shape(left), Select::Shape(right)) => {
								switch_left_right!(shapes, shape, chart, inner, left, right, msg, ui);
							},
							(Select::JudgeField(left), Select::JudgeField(right)) => {
								switch_left_right!(judge_fields, inner, chart, inner, left, right, msg, ui);
							},
							_ => {
								msg.message("Error: not the same type or unsupported", ui);
								return;
							},
						}
						msg.message("switched successfully", ui);
					}else {
						msg.message("Error: less than 2 to switch", ui);
					}
				}
			}
		});
		let mut info = MacroInner::from_router(inner);
		for (id, (code, macro_use)) in &mut inner.macro_map {
			ui.collapsing(id, |ui, _| {
				let info_back_up = info.clone();
				code.ui(&mut info, macro_use, ui, msg, core);
				if info != info_back_up {
					inner.paste_offset = info.paste_offset;
					inner.current_time = info.current_time;
					inner.time_pointer = info.time_pointer;
					inner.is_adsorption = info.is_adsorption;
					inner.round = info.round;
				}
			});
		}
	})
}