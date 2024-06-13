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
	if let Some(_) = &core.play_info {
	} else {
		msg.message("no chart loaded", ui);
		*router = Router::Main(Default::default());
		return;
	};
	if let Some(_) = &core.current_chart {} else {
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
		if let Err(e) = if core.timer.is_started() {
			core.pause()
		}else {
			core.play_with_time(PlayMode::Auto, inner.current_time)
		} {
			msg.message(format!("{}", e), ui);
		}
	}

	if let Some((_, info)) = &core.current_chart {
		let sustain_time = info.sustain_time.clone(); 
		let beats = info.total_beats();
		let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
		let beats = beats / beat_quator;
		let bps = beats / sustain_time.as_seconds_f32();
		let change_time = |input: &mut Duration, ui: &mut Ui| {
			if !inner.is_adsorption {
				let mut sec = input.as_seconds_f32();
				if ui.input().is_key_pressing(Key::AltLeft) {
					if ui.input().is_key_pressing(Key::ArrowLeft) {
						sec = sec - 0.1
					}
				}else {
					if ui.input().is_key_released(Key::ArrowLeft) {
						sec = sec - 0.1
					}
				}
				if ui.input().is_key_pressing(Key::AltLeft) {
					if ui.input().is_key_pressing(Key::ArrowRight) {
						sec = sec + 0.1
					}
				}else {
					if ui.input().is_key_released(Key::ArrowRight) {
						sec = sec + 0.1
					}
				}
				*input = Duration::seconds_f32(sec);
			}else {
				let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
				if ui.input().is_key_pressing(Key::AltLeft) {
					if ui.input().is_key_pressing(Key::ArrowLeft) {
						sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
					}
				}else {
					if ui.input().is_key_released(Key::ArrowLeft) {
						sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
					}
				}
				if ui.input().is_key_pressing(Key::AltLeft) {
					if ui.input().is_key_pressing(Key::ArrowRight) {
						sustain_beats = sustain_beats + 1;
					}
				}else {
					if ui.input().is_key_released(Key::ArrowRight) {
						sustain_beats = sustain_beats + 1;
					}
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
	timeline_scale_factor: f32,
	time_baseline: usize,
	is_adsorption: bool,
	current_animation_id: String,
	round: isize,
	current_linker: AnimationLinker,
	is_animation_delete: bool,
	value_offset: f32,
	copied_animation: Option<Animation>,

	is_detail_on: bool,

	is_arrangement_on: bool,
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
	note_generator: NoteGenerator
}

#[derive(Default)]
struct NoteGenerator {
	tap_shape: Vec<String>,
	slide_shape: Vec<String>,
	flick_shape: Vec<String>,
	time_range: Duration,
}

#[derive(Clone, PartialEq)]
pub struct MacroInner {
	pub paste_offset: Duration,
	pub current_time: Duration,
	pub time_pointer: Duration,
	pub is_adsorption: bool,
	pub round: isize,
}

impl MacroInner {
	fn from_router(router: &EditRouter) -> Self {
		Self {
			paste_offset: router.paste_offset.clone(),
			current_time: router.current_time.clone(),
			time_pointer: router.time_pointer.clone(),
			is_adsorption: router.is_adsorption.clone(),
			round: router.round.clone(),
		}
	}
}

pub struct MacroCode {
	/// lua code
	pub inner: String
}

impl Macro for MacroCode {
	fn ui(&mut self, _: &mut MacroInner, _: &mut MacroInfo, _: &mut Ui, _: &mut MessageProvider, _: &mut ShapoistCore) {}
}

pub struct MacroInfo {
	pub select: Vec<Select>,
}

pub trait Macro {
	fn ui(&mut self, inner: &mut MacroInner, info: &mut MacroInfo, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore);
}

impl Default for EditRouter {
	fn default() -> Self {
		Self {
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
			timeline_scale_factor: 1.0,
			time_baseline: 2,
			is_adsorption: true,
			current_animation_id: String::new(),
			round: -2,
			current_linker: AnimationLinker::Bezier(Vec2::new(0.5,0.1), Vec2::new(0.5,0.9)),
			is_animation_delete: false,
			value_offset: 0.0,
			copied_animation: None,

			is_detail_on: false,

			is_arrangement_on: false,
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

fn preview(inner: &mut EditRouter, ui: &mut Ui, _: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	let play_info = core.play_info.as_ref().unwrap();
	let (chart, _) = &core.current_chart.as_ref().unwrap();
	let size = chart.size / chart.size.len();
	let canvas_size = size * 400.0;
	let fps = format!("{:.2}fps", 1.0 / ui.delay().as_seconds_f32());
	ui.show(&mut Card::new("preview")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(canvas_size.y + 32.0)
		.set_width(canvas_size.x + 32.0)
		.set_resizable(true)
		.set_stroke_width(2.0)
		.set_stroke_color(1.0)
		.set_position(position)
		.set_scrollable([true; 2]), 
		|ui, _| {
			let window = ui.window_area().shrink(Vec2::same(8.0));
			let canvas_size = if (window.width() * size).y < window.height() {
				window.width() * size
			}else {
				window.height() * size
			};
			let scale_factor = canvas_size.x / chart.size.x;
			let canvas_position = window.center() - canvas_size / 2.0;
			let time = format!("{:.2}s", (core.timer.read() - Duration::seconds(3)).as_seconds_f32());
			ui.put(Canvas::new(canvas_size, |painter| {
				painter.set_color(0.0);
				painter.rect(canvas_size, Vec2::ZERO);
				for shape in &play_info.render_queue {
					painter.set_color(1.0);
					let shape_id = &shape.id;
					let mut shape = shape.shape.clone();
					shape.pre_scale(scale_factor);
					let id_position = shape.get_area().right_bottom();
					painter.push(shape);
					painter.set_scale(Vec2::same(0.75));
					if inner.is_show_id {
						painter.set_position(id_position);
						painter.text(shape_id.to_string());
					}
				}
				painter.set_scale(Vec2::same(1.0));
				painter.set_position(Vec2::same(16.0));
				painter.text("preview".to_string());
				painter.set_position(Vec2::new(16.0, 32.0));
				painter.text(time);
				painter.set_position(Vec2::new(16.0, 48.0));
				painter.text(fps);
			}), Area::new(canvas_position, canvas_position + canvas_size));
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
		.set_stroke_width(2.0)
		.set_stroke_color(1.0)
		.set_position(position)
		.set_scrollable([false; 2]), 
		|ui, _| {
			let area = ui.window_area();
			let selects = match core.current_selects() {
				Ok(t) => t,
				Err(e) => {
					msg.message(format!("{}", e), ui);
					return;
				}	
			};
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

					if selects.is_empty() {
						ui.label("N/A");
					}else {
						match &selects[0] {
							Select::JudgeField(id) => ui.label(id),
							_ => {
								ui.label("not a judge field");
								return
							}
						};
					}

					if let Some(play_info) = &core.play_info {
						ui.label(format!("total notes: {}", play_info.total_notes));
					}

					if !selects.is_empty() {
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
											sec = sec - 0.1
										}
										ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
										if ui.button("+").is_clicked() {
											sec = sec + 0.1
										}
										*input = Duration::seconds_f32(sec);
									}else {
										let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
										if ui.button("-").is_clicked() {
											sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
										}
										ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
										if ui.button("+").is_clicked() {
											sustain_beats = sustain_beats + 1;
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

							let (start_time, sustain_time) = match &selects[0] {
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
					}

					if ui.button("clear selects").is_clicked() {
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
					if ui.switch(&mut if let JudgeType::Hold(_) = inner.note_type {
						true
					}else {
						false
					}, "hold").is_clicked() {
						inner.note_type = JudgeType::Hold(Duration::ZERO);
					};
					if ui.switch(&mut if let JudgeType::AngledFilck(_) = inner.note_type {
						true
					}else {
						false
					}, "angled flick").is_clicked() {
						inner.note_type = JudgeType::AngledFilck(0.0);
					};
					if ui.switch(&mut if let JudgeType::AngledTapFilck(_) = inner.note_type {
						true
					}else {
						false
					}, "angled tap flick").is_clicked() {
						inner.note_type = JudgeType::AngledTapFilck(0.0);
					};
					
					match &mut inner.note_type {
						JudgeType::Hold(t) => {
							if !inner.is_adsorption {
								let mut sec = t.as_seconds_f32();
								ui.add(Slider::new(0.01..=10.0, &mut sec, "hold length").step(0.01).suffix("s"));
								*t = Duration::seconds_f32(sec);
							}else {
								if let Some((_, info)) = &mut core.current_chart {
									let sustain_time = info.sustain_time.as_seconds_f32(); 
									let beats = info.total_beats();
									let beat_quator = (inner.time_baseline as f32).powf(((1.0 / inner.timeline_scale_factor).log(inner.time_baseline as f32)).floor());
									let beats = beats / beat_quator;
									let bps = beats / sustain_time;
									let mut sustain_beats = (t.as_seconds_f32() * bps).floor() as usize;
									ui.add(Slider::new(1..=100, &mut sustain_beats, "hold length").step(1.0).suffix(format!("*{} beat", beat_quator)));
									*t = Duration::seconds_f32(sustain_beats as f32 / bps);
								}
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
						if selects.is_empty() {
							ui.label("select a judge field in element manager first");
						}else {
							let judge_field_id = match &selects[0] {
								Select::JudgeField(id) => id.clone(),
								_ => {
									ui.label("not a judge field");
									return
								}
							};
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
							if let Some(judge_field) = chart.judge_fields.get_mut(&judge_field_id) {
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
										if note.judge_field_id == judge_field_id {
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
										let x;
										if inner.is_adsorption {
											x = ((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats;
										}else {
											x = cursor_position.x - 16.0 + current_scroll;
										}
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
									let x;
									if inner.is_adsorption {
										x = ((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats;
									}else {
										x = cursor_position.x - 16.0 + current_scroll;
									}
									inner.time_pointer = Duration::seconds_f32(x / width * sustain_time);
								}else if res.is_multi_clicked(2) && ui.input().is_mouse_released(MouseButton::Left) {
									let x;
									if inner.is_adsorption {
										x = ((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats;
									}else {
										x = cursor_position.x - 16.0 + current_scroll;
									}
									let judge_time = x / width * sustain_time;
									let mut index = chart.notes.len();
									loop {
										let id = format!("{} {}",judge_field_id ,index);
										if let None = chart.notes.get(&id) {
											chart.notes.insert(id.clone(), Note {
												judge_type: inner.note_type.clone(),
												judge_time: Duration::seconds_f32(judge_time),
												judge_field_id,
												note_id: id,
												click_effect_position: inner.default_click_effect_center,
												..Default::default()
											});
											break;
										}
										index = index + 1
									}
								}
								if res.is_clicked() && ui.input().is_mouse_released(MouseButton::Right) {
									let x;
									if inner.is_adsorption {
										x = ((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats;
									}else {
										x = cursor_position.x - 16.0 + current_scroll;
									}
									let y = cursor_position.y - 16.0;
									let key = (x * 0.1).round() as u64;
									let index = (y / 32.0).abs().floor() as usize;
									let index = index.checked_sub(1).unwrap_or(0);
									if let Some(inner) = back.get(&key) {
										if index < inner.len() {
											chart.notes.remove(&inner[index]);
										}
									}
								}
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
	ui.show(&mut Card::new("animation editor")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(320.0)
		.set_width(540.0)
		.set_resizable(true)
		.set_stroke_width(2.0)
		.set_stroke_color(1.0)
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
					if inner.current_animation_id.is_empty() {
						ui.label("N/A");
					}else {
						ui.label(inner.current_animation_id.replace("----", " ").replace("Shape ", "").replace("style ", "").replace("JudgeFieldInner ", "").trim());
					};

					if !selects.is_empty() {
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
											sec = sec - 0.1
										}
										ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
										if ui.button("+").is_clicked() {
											sec = sec + 0.1
										}
										*input = Duration::seconds_f32(sec);
									}else {
										let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
										if ui.button("-").is_clicked() {
											sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
										}
										ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
										if ui.button("+").is_clicked() {
											sustain_beats = sustain_beats + 1;
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

							let (start_time, sustain_time) = match &selects[0] {
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
					}

					if ui.button("scroll to current").is_clicked() {
						inner.need_scroll_to_now = true;
					}
					if ui.button("copy").is_clicked() {
						need_copy = true;
					}
					if ui.button("paste").is_clicked() {
						need_paste = true;
					}
					if ui.button("delete animation").is_multi_clicked(2) && !inner.current_animation_id.is_empty() {
						inner.is_animation_delete = true;
					}
					if ui.button("clear attribute").is_clicked() {
						inner.current_animation_id.clear();
					}
					if ui.button("clear selects").is_clicked() {
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
							inner.round = inner.round - 1;
						}
						ui.dragable_value(&mut inner.round);
						if ui.button("+").is_clicked() {
							inner.round = inner.round + 1;
						}
						if inner.round > 4 {
							inner.round = 4;
						}else if inner.round < -4 {
							inner.round = -4;
						}
					});
					ui.label("value_offset");
					ui.horizental(|ui| {
						if ui.button("-").is_clicked() {
							inner.value_offset = inner.value_offset - 10_f32.powf(inner.round as f32);
						}
						ui.add(Slider::new(-500.0..=500.0, &mut inner.value_offset, "").step(0.01));
						if ui.button("+").is_clicked() {
							inner.value_offset = inner.value_offset + 10_f32.powf(inner.round as f32);
						}
						if ui.button("round").is_clicked() {
							inner.value_offset = (inner.value_offset / 10_f32.powf(inner.round as f32)).round() * 10_f32.powf(inner.round as f32);
						}
						if inner.value_offset > 500.0 {
							inner.value_offset = 500.0;
						}else if inner.value_offset < -500.0 {
							inner.value_offset = -500.0;
						}
					});

					ui.label("linker type");
					if ui.switch(&mut if let AnimationLinker::Bezier(_, _) = inner.current_linker {
						true
					}else {
						false
					}, "bezier").is_clicked() {
						inner.current_linker = AnimationLinker::Bezier(Vec2::new(0.5,0.1), Vec2::new(0.5,0.9));
					};
					if ui.switch(&mut if let AnimationLinker::Linear = inner.current_linker {
						true
					}else {
						false
					}, "linear").is_clicked() {
						inner.current_linker = AnimationLinker::Linear;
					};
					if ui.switch(&mut if let AnimationLinker::Mutation = inner.current_linker {
						true
					}else {
						false
					}, "mutation").is_clicked() {
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
								if *input > 1.0 {
									*input = 1.0
								}else if *input < 0.0 {
									*input = 0.0
								}
							}
							compress(&mut point1.x);
							compress(&mut point1.y);
							compress(&mut point2.x);
							compress(&mut point2.y);
							ui.memory_save(&res.id, &(is_point1_draging, is_point2_draging));
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
						inner.current_animation_id.clear();
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
							if inner.current_animation_id.is_empty() {
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
											if input.name != String::from("time") {
												if ui.button(id.replace("----", " ").replace("Shape ", "").replace("style ", "").replace("JudgeFieldInner ", "").trim()).is_clicked() {
													out = Some(id)
												}
											}
										}
										_ => {}
									}

									out
								}

								match &selects[0] {
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
												inner.current_animation_id = t;
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
												inner.current_animation_id = t;
											}
										}
									},
									_ => {
										ui.label("unsupported");
									}
								};
								return;
							}
							let (shape_start_time, shape_sustain_time, animation) = match &selects[0] {
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
								animation.remove(&inner.current_animation_id);
								inner.current_animation_id.clear();
								inner.is_animation_delete = false;
							}
							let animation = if let Some(t) = animation.get_mut(&inner.current_animation_id) {
								t
							}else {
								animation.insert(inner.current_animation_id.clone(), Default::default());
								return;
							};

							if need_copy {
								let mut copied_animation = animation.clone();
								copied_animation.start_time = copied_animation.start_time - inner.time_pointer;
								inner.copied_animation = Some(copied_animation);
							}
							if need_paste {
								if let Some(mut copied_animation) = inner.copied_animation.clone() {
									copied_animation.start_time = copied_animation.start_time + inner.time_pointer + inner.paste_offset;
									copied_animation.start_value = copied_animation.start_value + inner.value_offset;
									for linker in &mut copied_animation.linkers {
										linker.end_value = linker.end_value + inner.value_offset;
									}
									animation.combine(&mut copied_animation, inner.current_linker.clone());
								}else {
									msg.message("no animation has cpoied", ui);
								}
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
									x = x + width * (linker.sustain_time / sustain_time) as f32;
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
									let x;
									if inner.is_adsorption {
										x = ((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats;
									}else {
										x = cursor_position.x - 16.0 + current_scroll;
									}
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
								let x;
								if inner.is_adsorption {
									x = ((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats;
								}else {
									x = cursor_position.x - 16.0 + current_scroll;
								}
								inner.time_pointer = x / width * sustain_time;
							}else if res.is_multi_clicked(2) && ui.input().is_mouse_released(MouseButton::Left) {
								let x;
								if inner.is_adsorption {
									x = ((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats;
								}else {
									x = cursor_position.x - 16.0 + current_scroll;
								}
								let y = cursor_position.y - 16.0;
								let value = ((height - y) - 0.1 * height) / (height * 0.8) * delta + min_value;
								let time = x / width * sustain_time;
								let round = (value / 10.0_f32.powf(inner.round as f32)).round() * 10.0_f32.powf(inner.round as f32) + inner.value_offset;
								animation.insert_point(time, round, inner.current_linker.clone());
							}
							if res.is_clicked() && ui.input().is_mouse_released(MouseButton::Right) {
								let x;
								if inner.is_adsorption {
									x = ((cursor_position.x - 16.0 + current_scroll) * beats / width).round() * width / beats;
								}else {
									x = cursor_position.x - 16.0 + current_scroll;
								}
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
		.set_stroke_width(2.0)
		.set_stroke_color(1.0)
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
										sec = sec - 0.1
									}
									ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
									if ui.button("+").is_clicked() {
										sec = sec + 0.1
									}
									*input = Duration::seconds_f32(sec);
								}else {
									let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
									if ui.button("-").is_clicked() {
										sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
									}
									ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
									if ui.button("+").is_clicked() {
										sustain_beats = sustain_beats + 1;
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
							for (id, note) in &chart.notes {
								if note.judge_time < inner.time_pointer - inner.filter || note.judge_time > inner.time_pointer + inner.filter {
									continue;
								}
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
							for (id, filed) in &chart.judge_fields {
								if filed.start_time < inner.time_pointer - inner.filter || filed.start_time > inner.time_pointer + inner.filter {
									continue;
								}
								if ui.switch(&mut selects.contains(&Select::JudgeField(id.clone())), id).is_clicked() {
									if selects.contains(&Select::JudgeField(id.clone())) {
										selects.retain(|inner| inner != &Select::JudgeField(id.clone()));
									}else {
										selects.push(Select::JudgeField(id.clone()));
									}
								};
							}
							if ui.button("add").is_clicked() {
								let mut index = chart.judge_fields.len();
								loop {
									if let None = chart.judge_fields.get(&index.to_string()) {
										chart.judge_fields.insert(index.to_string(), JudgeField {
											start_time: inner.current_time,
											sustain_time: Duration::seconds(1),
											..Default::default()
										});
										break;
									}else {
										index = index + 1
									}
								}
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
							for (id, shape) in &chart.shapes {
								if !(shape.start_time - inner.filter < inner.time_pointer && shape.start_time + shape.sustain_time + inner.filter > inner.time_pointer) {
									continue;
								}
								if ui.switch(&mut selects.contains(&Select::Shape(id.clone())), id).is_clicked() {
									if selects.contains(&Select::Shape(id.clone())) {
										selects.retain(|inner| inner != &Select::Shape(id.clone()));
									}else {
										selects.push(Select::Shape(id.clone()));
									}
								};
							}
							ui.collapsing("add" ,|ui, _| {
								let mut add_shape = |ui: &mut Ui, text: String, add: ShapeElement| {
									if ui.button(text).is_clicked() {
										let mut index = chart.shapes.len();
										loop {
											if let None = chart.shapes.get(&index.to_string()) {
												chart.shapes.insert(index.to_string(), shapoist_core::system::core_structs::Shape { 
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
											}else {
												index = index + 1
											}
										}
									}
								};
								add_shape(ui, "Circle".to_string(), ShapeElement::Circle(Default::default()));
								add_shape(ui, "Rect".to_string(), ShapeElement::Rect(Default::default()));
								add_shape(ui, "Text".to_string(), ShapeElement::Text(Default::default()));
								add_shape(ui, "CubicBezier".to_string(), ShapeElement::CubicBezier(Default::default()));
								add_shape(ui, "Line".to_string(), ShapeElement::Line(Default::default()));
							})
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
										sec = sec - 0.1
									}
									ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
									if ui.button("+").is_clicked() {
										sec = sec + 0.1
									}
									*input = Duration::seconds_f32(sec);
								}else {
									let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
									if ui.button("-").is_clicked() {
										sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
									}
									ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
									if ui.button("+").is_clicked() {
										sustain_beats = sustain_beats + 1;
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
		.set_stroke_width(2.0)
		.set_stroke_color(1.0)
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
							*value = *value - 10_f32.powf(inner.round as f32);
						}
						ui.add(Slider::new(range, value, "").step(step));
						if ui.button("+").is_clicked() {
							*value = *value + 10_f32.powf(inner.round as f32);
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
								sec = sec - 0.1
							}
							let range = if allow_neg {
								-sustain_time.as_seconds_f32()..=sustain_time.as_seconds_f32()
							}else {
								0.01..=sustain_time.as_seconds_f32()
							};
							ui.add(Slider::new(range, &mut sec, text).step(0.01).suffix("s"));
							if ui.button("+").is_clicked() {
								sec = sec + 0.1
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
								sustain_beats = sustain_beats + 1;
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
					}else {
						if *input < Duration::ZERO {
							*input = Duration::ZERO
						}
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
					if let Err(e) = core.clear_selects() {
						msg.message(format!("{}", e), ui);
					}
				}
				ui.switch(&mut inner.is_adsorption, "adsorption");
				ui.add(Slider::new(0.06..=16.0, &mut inner.timeline_scale_factor, "scale").step(0.01).logarithmic(true));
				ui.label("round");
				ui.horizental(|ui| {
					if ui.button("-").is_clicked() {
						inner.round = inner.round - 1;
					}
					ui.dragable_value(&mut inner.round);
					if ui.button("+").is_clicked() {
						inner.round = inner.round + 1;
					}
					if inner.round > 4 {
						inner.round = 4;
					}else if inner.round < -4 {
						inner.round = -4;
					}
				});

				if is_need_play {
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
		.set_stroke_width(2.0)
		.set_stroke_color(1.0)
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
					for (_, shape) in &chart.shapes {
						if shape.start_time <= inner.time_pointer && shape.start_time + shape.sustain_time >= inner.time_pointer {
							if ui.button(&shape.id).is_clicked() {
								inner.link_id_temp = Some(shape.id.clone());
							};
						}
					}
				}else {
					for (_, shape) in &chart.shapes {
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
								}else {
									if ui.button("link").is_clicked() {
										inner.is_linking_shape = Some(shape.id.clone())
									}
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
								sec = sec - 0.1
							}
							ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
							if ui.button("+").is_clicked() {
								sec = sec + 0.1
							}
							*input = Duration::seconds_f32(sec);
						}else {
							let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
							if ui.button("-").is_clicked() {
								sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
							}
							ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
							if ui.button("+").is_clicked() {
								sustain_beats = sustain_beats + 1;
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
					for (_, note) in &chart.notes {
						if note.judge_time > inner.time_pointer - inner.range_of_current_note && note.judge_time < inner.time_pointer + inner.range_of_current_note {
							if ui.button(&note.note_id).is_clicked() {
								inner.link_id_temp = Some(note.note_id.clone());
							};
						}
					}
				}else {
					for (_, note) in &mut chart.notes {
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
								}else {
									if ui.button("link").is_clicked() {
										inner.is_linking_note = Some(note.note_id.clone())
									}
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

fn macro_window(inner: &mut EditRouter, ui: &mut Ui, msg: &mut MessageProvider, core: &mut ShapoistCore, position: Vec2) -> InnerResponse<()> {
	ui.show(&mut Card::new("macro")
		.set_rounding(Vec2::same(16.0))
		.set_color(ui.style().background_color.brighter(0.1))
		.set_dragable(true)
		.set_height(320.0)
		.set_width(480.0)
		.set_resizable(true)
		.set_stroke_width(2.0)
		.set_stroke_color(1.0)
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
								sec = sec - 0.1
							}
							ui.add(Slider::new(0.01..=sustain_time.as_seconds_f32(), &mut sec, text).step(0.01).suffix("s"));
							if ui.button("+").is_clicked() {
								sec = sec + 0.1
							}
							*input = Duration::seconds_f32(sec);
						}else {
							let mut sustain_beats = (input.as_seconds_f32() * bps).floor() as usize;
							if ui.button("-").is_clicked() {
								sustain_beats = sustain_beats.checked_sub(1).unwrap_or(0);
							}
							ui.add(Slider::new(0..=beats.ceil() as usize, &mut sustain_beats, text).step(1.0).suffix(format!("*{} beat", beat_quator)));
							if ui.button("+").is_clicked() {
								sustain_beats = sustain_beats + 1;
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
						if id_inner.len() == 2 {
							if id_inner[0] == "" {
								if shape.start_time >= inner.time_pointer && shape.start_time <= inner.time_pointer + inner.note_generator.time_range {
									id_to_delete.push(id.clone());
								}
							}
						}
					}
					for id in id_to_delete {
						chart.shapes.remove(&id);
					}
				};

				if ui.button("clear").is_clicked() {
					clear(inner, chart);
					msg.message("cleard successfully", ui);
				}

				if ui.button("generate").is_clicked() {
					clear(&inner, chart);
					let offset = inner.paste_offset;
					let time_pointer = inner.time_pointer;
					let get_shape = |inner: &Vec<String>, chart: &mut Chart| -> Vec<shapoist_core::system::core_structs::Shape> { 
						let mut output_shapes = vec!();
						for id in inner {
							if let Some(shape_inner) = chart.shapes.get(id) {
								let mut output = shape_inner.clone();
								output.start_time = output.start_time - time_pointer + offset;
								for (_, animation) in &mut output.animation {
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
							shape.start_time = shape.start_time + time;
							for (_, animation) in &mut shape.animation {
								animation.start_time = animation.start_time + time;
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