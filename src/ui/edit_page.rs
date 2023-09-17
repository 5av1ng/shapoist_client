use crate::ASSETS_PATH;
use crate::ui::shape::style::ShapeAnimate;
use egui::Rect;
use egui::epaint::CubicBezierShape;
use egui::Sense;
use crate::ui::shape::style::StyleAnimate;
use crate::ui::shape::style::StyleAnimation;
use crate::ui::shape::style::Style;
use egui::Stroke;
use std::collections::BTreeMap;
use core::f32::consts::PI;
use crate::ui::shape::bezier_curve::*;
use crate::ui::shape::rectangle::*;
use crate::ui::shape::circle::*;
use egui::Color32;
// use crate::ui::shape::image::*;
// use std::path::PathBuf;
// use crate::system::system_function::create_dir;
// use crate::system::system_function::copy_file;
use crate::play::note::*;
use crate::system::system_function::to_json;
use crate::ui::shapo::Shape;
use crate::ui::ui::*;
use crate::ui::shapo::Shapo;
use egui::DroppedFile;
use egui::Modifiers;
use egui::Key;
use crate::system::system_function::read_file_split;
use crate::setting::setting::read_settings;
use egui::Vec2;
use crate::play::timer::Timer;
use std::collections::HashMap;
use egui::TextureId;
use egui::TextureHandle;
use crate::ui::page::Temp;
use crate::ui::ui::Back;
use crate::ShapoError;
use crate::play::note::EditPages;

pub fn edit_page(ui: &mut egui::Ui, _: &Vec2, _: &mut Vec<Timer>, if_enabled: bool, texture: &HashMap<TextureId,TextureHandle>, temp: &mut Temp, _file: &Vec<DroppedFile>) -> Result<Vec<Back>, ShapoError> {
	temp.project.timer()?;
	let uspb = (60.0 * 1e6 / temp.project.chart.bpm) as u64;
	let setting = read_settings()?;
	let language = read_file_split(&format!("{}/assets/language/{}/language.ini",*ASSETS_PATH , setting.language))?;
	let mut vec_back = vec!();
	let mut input = ui.input_mut(|i| i.clone());
	let mut if_shape_delete = false;
	let mut if_note_delete = false;

	if !temp.project.if_playing && temp.project.if_music_play {
		vec_back.push(Back::Pause)
	}

	if input.consume_key(Modifiers::CTRL, Key::Num1) {
		temp.project.now_page = EditPages::Timeline;
	}else if input.consume_key(Modifiers::CTRL, Key::Num2) {
		temp.project.now_page = EditPages::View;
	}else if input.consume_key(Modifiers::CTRL, Key::Num3) {
		temp.project.now_page = EditPages::TimelineAndView;
	}else if input.consume_key(Modifiers::NONE, Key::Q) {
		temp.project.if_info = !temp.project.if_info;
	}else if input.consume_key(Modifiers::CTRL, Key::Z) {
		vec_back.push(Back::Undo);
	}else if input.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::Z) | input.consume_key(Modifiers::CTRL, Key::Y){
		vec_back.push(Back::Redo);
	}else if input.consume_key(Modifiers::CTRL, Key::S) {
		vec_back.push(Back::Save);
	}else if input.consume_key(Modifiers::CTRL, Key::E) {
		vec_back.push(Back::Export);
	}else if input.consume_key(Modifiers::CTRL, Key::W) {
		temp.project.window = HashMap::new();
	}else if input.consume_key(Modifiers::NONE, Key::Space) {
		temp.project.if_playing = !temp.project.if_playing;
		if !temp.project.if_playing {
			vec_back.push(Back::PauseSound)
		}
	}else if input.consume_key(Modifiers::NONE, Key::ArrowLeft) {
		let mut current_beat = temp.project.current_time as f64 / uspb as f64 - 1.0;
		if current_beat < 0.0 {
			current_beat = 0.0
		}
		temp.project.current_time = (current_beat * uspb as f64) as u64
	}else if input.consume_key(Modifiers::CTRL, Key::ArrowLeft) {
		let mut current_beat = temp.project.current_time as f64 / uspb as f64 - 4.0;
		if current_beat < 0.0 {
			current_beat = 0.0
		}
		temp.project.current_time = (current_beat * uspb as f64) as u64
	}else if input.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::ArrowLeft) {
		let mut current_beat = temp.project.current_time as f64 / uspb as f64 - 16.0;
		if current_beat < 0.0 {
			current_beat = 0.0
		}
		temp.project.current_time = (current_beat * uspb as f64) as u64
	}else if input.consume_key(Modifiers::NONE, Key::ArrowRight) {
		let mut current_beat = temp.project.current_time as f64 / uspb as f64 + 1.0;
		if current_beat > 2000.0 {
			current_beat = 2000.0
		}
		temp.project.current_time = (current_beat * uspb as f64) as u64
	}else if input.consume_key(Modifiers::CTRL, Key::ArrowRight) {
		let mut current_beat = temp.project.current_time as f64 / uspb as f64 + 4.0;
		if current_beat > 2000.0 {
			current_beat = 2000.0
		}
		temp.project.current_time = (current_beat * uspb as f64) as u64
	}else if input.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::ArrowRight) {
		let mut current_beat = temp.project.current_time as f64 / uspb as f64 + 16.0;
		if current_beat > 2000.0 {
			current_beat = 2000.0
		}
		temp.project.current_time = (current_beat * uspb as f64) as u64
	}

	let mut current_beat = temp.project.current_time as f64 / uspb as f64;
	if current_beat > 1996.0 {
		current_beat = 1996.0
	}

	// Toolbar
	let back = ui.horizontal(|ui| -> Result<Vec<Back>, ShapoError> {
		let mut vec_back = vec!();
		if let Some(t) = ui.menu_button(language[50].clone(), |ui| -> Result<Back, ShapoError> {
			let mut back = Back::Nothing;

			if let Some(t) = ui.menu_button(language[51].clone(), |ui| -> Result<Back, ShapoError> {
				let mut chart = temp.project.chart.clone();
				if ui.button(language[54].clone()).clicked() {
					chart.shape.push(Shapo{
						shape: Shape::Circle(Circle::default()),
						sustain_time: Some((none_to_zero(&(current_beat as u64).checked_sub(4)) * uspb, (current_beat as u64 + 4) * uspb)),
						label: Some(vec!(temp.project.now_shape_id.to_string())),
						..Default::default()
					});
					temp.project.now_shape_id = temp.project.now_shape_id + 1;
					ui.close_menu();
					return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Add(ShapeAdd::Circle))),to_json(&chart)?))
				};
				if ui.button(language[55].clone()).clicked() {
					chart.shape.push(Shapo{
						shape: Shape::Rectangle(Rectangle::default()),
						sustain_time: Some((none_to_zero(&(current_beat as u64).checked_sub(4)) * uspb, (current_beat as u64 + 4) * uspb)),
						label: Some(vec!(temp.project.now_shape_id.to_string())),
						..Default::default()
					});
					temp.project.now_shape_id = temp.project.now_shape_id + 1;
					ui.close_menu();
					return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Add(ShapeAdd::Rectangle))),to_json(&chart)?))
				};
				if ui.button(language[57].clone()).clicked() {
					chart.shape.push(Shapo{
						shape: Shape::CubicBezier(CubicBezier::default()),
						style: Style {
							stroke: Stroke { width: 3.0, color: Color32::WHITE },
							fill: Color32::TRANSPARENT,
							..Default::default()
						},
						sustain_time: Some((none_to_zero(&(current_beat as u64).checked_sub(4)) * uspb, (current_beat as u64 + 4) * uspb)),
						label: Some(vec!(temp.project.now_shape_id.to_string())),
						..Default::default()
					});
					temp.project.now_shape_id = temp.project.now_shape_id + 1;
					ui.close_menu();
					return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Add(ShapeAdd::CubicBezier))),to_json(&chart)?))
				};
				// if let Some(t) = ui.menu_button(language[58].clone(), |ui| -> Result<Back, ShapoError> {
				// 	for a in file {
				// 		let path = match &a.path {
				// 			Some(t) => t.display().to_string(),
				// 			None => String::new(),
				// 		};
				// 		if !path.is_empty() {
				// 			if ui.button(path.clone()).clicked(){
				// 				let path_buf = PathBuf::from(path.clone());
				// 				let name = match path_buf.file_name().unwrap().to_str() {
				// 					Some(t) => t,
				// 					None => return Err(ShapoError::SystemError(format!("not unicode string"))),
				// 				}.to_string();
				// 				let mut chart = temp.project.chart.clone();
				// 				chart.shape.push(Shapo{
				// 					shape: Shape::Image(Image {
				// 						name: name.clone(),
				// 						first_path: Path::Chart,
				// 						path: format!("{}/image/{}",temp.project.chart.mapname,name),
				// 						bottom_right_point: Vec2{x: 28.0, y: 28.0},
				// 						if_keep: false,
				// 						registered_info: None
				// 					}), 
				// 					label: Some(vec!(temp.project.now_shape_id.to_string())),
				// 					sustain_time: Some((current_beat as u64 * uspb, (current_beat as u64 + 4) * uspb)),
				// 					..Default::default()
				// 				});
				// 				let _ = create_dir(&format!("{}/assets/chart/{}/image",*ASSETS_PATH , temp.project.chart.mapname));
				// 				copy_file(&path, &format!("{}/assets/chart/{}/image/{}",*ASSETS_PATH ,temp.project.chart.mapname,name))?;
				// 				return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Add(Shape::Image))),to_json(&chart)?))
				// 			}
				// 		}
				// 	}
				// 	if ui.button(language[59].clone()).clicked() {
				// 		if let Some(path) = rfd::FileDialog::new().add_filter(&language[59], &["png"]).pick_file() {
				// 			let file_name = match path.file_name().unwrap().to_str() {
				// 				Some(t) => t,
				// 				None => return Err(ShapoError::SystemError(format!("not unicode string"))),
				// 			}.to_string();
				// 			let path = path.display().to_string();
				// 			let mut chart = temp.project.chart.clone();
				// 			chart.shape.push(Shapo{
				// 				shape: Shape::Image(Image {
				// 					name: file_name.clone(),
				// 					first_path: Path::Chart,
				// 					path: format!("{}/image/{}",temp.project.chart.mapname,file_name),
				// 					bottom_right_point: Vec2{x: 28.0, y: 28.0},
				// 					if_keep: false,
				// 					registered_info: None
				// 				}), 
				// 				sustain_time: Some((current_beat as u64 * uspb, (current_beat as u64 + 4) * uspb)),
				// 				label: Some(vec!(temp.project.now_shape_id.to_string())),
				// 				..Default::default()
				// 			});
				// 			let _ = create_dir(&format!("{}/assets/chart/{}/image",*ASSETS_PATH , temp.project.chart.mapname));
				// 			copy_file(&path, &format!("{}/assets/chart/{}/image/{}",*ASSETS_PATH ,temp.project.chart.mapname,file_name))?;
				// 			temp.project.now_shape_id = temp.project.now_shape_id + 1;
				// 			ui.close_menu();
				// 			return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Add(Shape::Image))),to_json(&chart)?))
				// 		}
				// 	}
				// 	Ok(Back::Nothing)
				// }).inner {
				// 	return t;
				// };
				Ok(Back::Nothing)
			}).inner {
				back = t?;
			};

			if let Some(t) = ui.menu_button(language[53].clone(), |ui| -> Result<Back, ShapoError> {
				let mut chart = temp.project.chart.clone();
				if ui.button(language[60].clone()).clicked() {
					match chart.note.get_mut(&temp.project.now_judge_field_id) {
						Some(t) => t,
						None => return Err(ShapoError::SystemError(format!("invailed judge field id"))),
					}.push(Note{
						id: temp.project.now_note_id,
						judge_field_id: temp.project.now_judge_field_id,
						start_time: current_beat as u64 * uspb,
						click_time: current_beat as u64 * uspb,
						judge_type: JudgeType::Tap,
						..Default::default()
					});
					temp.project.now_note_id = temp.project.now_note_id + 1;
					ui.close_menu();
					return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Add(JudgeType::Tap))),to_json(&chart)?))
				}
				if ui.button(language[61].clone()).clicked() {
					match chart.note.get_mut(&temp.project.now_judge_field_id) {
						Some(t) => t,
						None => return Err(ShapoError::SystemError(format!("invailed judge field id")))
					}.push(Note{
						id: temp.project.now_note_id,
						judge_field_id: temp.project.now_judge_field_id,
						start_time: none_to_zero(&(current_beat as u64).checked_sub(4)) * uspb,
						click_time: current_beat as u64 * uspb,
						judge_type: JudgeType::Slide,
						..Default::default()
					});
					temp.project.now_note_id = temp.project.now_note_id + 1;
					ui.close_menu();
					return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Add(JudgeType::Slide))),to_json(&chart)?))
				}
				Ok(Back::Nothing)
			}).inner {
				back = t?;
			};

			if ui.button(language[52].clone()).clicked() {
				let mut chart = temp.project.chart.clone();
				chart.judge_field.push(JudgeField{
					start_time: none_to_zero(&(current_beat as u64).checked_sub(4)) * uspb,
					end_time: (current_beat as u64) * uspb,
					id: temp.project.new_judge_field_id,
					..Default::default()
				});
				chart.note.insert(temp.project.new_judge_field_id, vec!());
				chart.now_judge.insert(temp.project.new_judge_field_id, 0);
				temp.project.new_judge_field_id = temp.project.new_judge_field_id + 1;
				ui.close_menu();
				return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::JudgeField),to_json(&chart)?))
			};
			Ok(back)
		}).inner {
			vec_back.push(t?);
		}

		if let Some(t) = ui.menu_button(language[62].clone(), |ui| -> Result<Back, ShapoError> {
			if ui.button(language[63].clone()).on_hover_text(format!("{}Ctrl + S", language[68].clone())).clicked() {
				ui.close_menu();
				return Ok(Back::Save);
			}
			if ui.button(language[64].clone()).on_hover_text(format!("{}Ctrl + Z", language[68].clone())).clicked() {
				ui.close_menu();
				return Ok(Back::Undo);
			}
			if ui.button(language[65].clone()).on_hover_text(format!("{}Ctrl + Y / Ctrl+ Shift + Z", language[68].clone())).clicked() {
				ui.close_menu();
				return Ok(Back::Redo);
			}
			if ui.button(language[66].clone()).on_hover_text(format!("{}Ctrl + E", language[68].clone())).clicked() {
				ui.close_menu();
				return Ok(Back::Export);
			}
			Ok(Back::Nothing)
		}).inner {
			vec_back.push(t?);
		}

		ui.menu_button(language[88].clone() ,|ui| {
			if ui.button(language[89].clone()).on_hover_text(format!("{}Ctrl + W", language[68].clone())).clicked() {
				temp.project.window = HashMap::new();
				ui.close_menu();
			}
		});

		if let Some(back_mes) = ui.menu_button(language[126].clone(), |ui| -> Vec<Back> {
			let mut vec_out = vec!();
			if temp.project.if_playing {
				if ui.selectable_label(temp.project.if_playing,language[128].clone()).on_hover_text(format!("{}Space", language[68].clone())).clicked() {
					temp.project.if_playing = false;
					vec_out.push(Back::PauseSound);
					ui.close_menu();
				};
			}else {
				if ui.selectable_label(temp.project.if_playing,language[127].clone()).on_hover_text(format!("{}Space", language[68].clone())).clicked() {
					temp.project.if_playing = true;
					ui.close_menu();
				};
			}

			if ui.button(language[69].clone()).clicked() {
				vec_out.push(Back::Save);
				vec_out.push(Back::Router(Router::MainPage));
				ui.close_menu();
			}

			if ui.button(language[70].clone()).double_clicked() {
				vec_out.push(Back::Router(Router::MainPage));
				ui.close_menu();
			}
			vec_out
		}).inner {
			for a in back_mes {
				vec_back.push(a)
			}
		};

		if ui.selectable_label(temp.project.if_info ,language[67].clone()).on_hover_text(format!("{}Q", language[68].clone())).clicked() {
			temp.project.if_info = !temp.project.if_info;
		};

		ui.label(language[132].clone());
		let mut current_beat = temp.project.current_time as f32 / uspb as f32;
		ui.add(egui::Slider::new(&mut current_beat, 0.0..=2000.0));
		if current_beat != temp.project.current_time as f32 / uspb as f32 {
			temp.project.current_time = (current_beat * uspb as f32) as u64;
		}

		Ok(vec_back)
	}).inner;
	for a in back? {
		vec_back.push(a);
	};
	ui.separator();

	// View Mode Bar
	ui.horizontal(|ui| {
		ui.selectable_value(&mut temp.project.now_page, EditPages::Timeline, language[47].clone()).on_hover_text(format!("{}Ctrl + 1", language[68].clone()));
		ui.selectable_value(&mut temp.project.now_page, EditPages::View, language[48].clone()).on_hover_text(format!("{}Ctrl + 2", language[68].clone()));
		ui.selectable_value(&mut temp.project.now_page, EditPages::TimelineAndView, language[49].clone()).on_hover_text(format!("{}Ctrl + 3", language[68].clone()));
	});
	ui.separator();

	// Render
	match temp.project.now_page {
		EditPages::Timeline => {
			ui.label(language[0].clone());
		},
		EditPages::View => {
			let available_size = ui.available_size() + Vec2{ x: 16.0, y: 16.0 };
			let offect = Vec2{x: ui.max_rect().left(), y: ui.cursor().top()};
			let (back, map) = temp.project.render(ui, available_size, if_enabled,texture, Some(offect))?; 
			for a in back{
				match a {
					Back::AnimateDone(_) => {},
					Back::JudgeNote(_) => {},
					Back::Router(_) => {},
					Back::Retry => {},
					Back::Play => {},
					Back::Pause => {},
					Back::MusicPlay(path, bpm, beatnumber, offect) => {
						vec_back.push(Back::MusicPlay(path, bpm, beatnumber, offect));
						temp.project.if_music_play = true;
					},
					_ => vec_back.push(a)
				}
			};
			for (t, r) in map {
				if r.clicked() {
					let key;
					match t.clone() {
						RenderType::Note(u1, u2) => key = RenderLabel::Array(u1.clone(), u2.clone()),
						RenderType::Shape(s) => key = RenderLabel::Text(s.clone())
					};

					match temp.project.window.get(&key) {
						Some(_) => temp.project.window.remove(&key),
						None => temp.project.window.insert(key, t.clone())
					};
				}
				if r.dragged() {
					let normalized = r.drag_delta() / available_size * 100.0;
					match t {
						RenderType::Shape(s) => {
							for a in &mut temp.project.chart.shape {
								if a.label.clone().unwrap()[0] == s {
									a.style.position = a.style.position + normalized;
								}
							}
							vec_back.push(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::Position))), to_json(&temp.project.chart)?))
						},
						RenderType::Note(j, n) => {
							let note = &mut match temp.project.chart.note.get_mut(&j) {
								Some(t) => t,
								None => return Err(ShapoError::SystemError(format!("cant find note to render")))
							}[n];
							note.final_position = note.final_position + normalized;
							note.start_position.x = note.final_position.x;
							note.start_position.y = 0.0;
							vec_back.push(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::FinalPosition)), to_json(&temp.project.chart)?))
						}
					}
				}
			}
		},
		EditPages::TimelineAndView => {
			ui.label(language[0].clone());
		},
	};

	// Project info render
	if let Some(t) = egui::Window::new(language[67].clone()).open(&mut temp.project.if_info).scroll2([true;2]).resizable(true).show(ui.ctx(), |ui| -> Result<Back, ShapoError> {
		let mut bpm = temp.project.chart.bpm.clone().to_string();
		return egui::Grid::new("asashjgauiuiy6381nlzx").show(ui, |ui| -> Result<Back, ShapoError> {
			ui.label(language[32].clone());
			let response = ui.add(egui::TextEdit::singleline(&mut temp.project.chart.songtitle).hint_text(language[32].clone()));
			if response.changed() {
				return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Songtitle), to_json(&temp.project.chart)?));
			}
			ui.end_row();

			ui.label(language[34].clone());
			let response = ui.add(egui::TextEdit::singleline(&mut bpm).hint_text(language[34].clone()));
			if response.changed() {
				let bpm: f32 = match bpm.parse() {
					Ok(t) => t,
					Err(_) => return Err(ShapoError::SystemError(format!("not a number"))),
				};
				temp.project.chart.bpm = bpm;
				return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Bpm), to_json(&temp.project.chart)?));
			}
			ui.end_row();

			ui.label(language[35].clone());
			let response = ui.add(egui::TextEdit::singleline(&mut temp.project.chart.producer).hint_text(language[35].clone()));
			if response.changed() {
				return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Producer), to_json(&temp.project.chart)?));
			}
			ui.end_row();

			ui.label(language[36].clone());
			let response = ui.add(egui::TextEdit::singleline(&mut temp.project.chart.charter).hint_text(language[36].clone()));
			if response.changed() {
				return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Charter), to_json(&temp.project.chart)?));
			}
			ui.end_row();

			ui.label(language[37].clone());
			let response = ui.add(egui::TextEdit::singleline(&mut temp.project.chart.painter).hint_text(language[37].clone()));
			if response.changed() {
				return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Painter), to_json(&temp.project.chart)?));
			}
			ui.end_row();

			let mut offect = temp.project.chart.offect.clone().to_string();
			ui.label(language[131].clone());
			let response = ui.add(egui::TextEdit::singleline(&mut offect).hint_text(language[131].clone()));
			if response.changed() {
				let offect: u64 = match offect.parse() {
					Ok(t) => t,
					Err(_) => return Err(ShapoError::SystemError(format!("not a number"))),
				};
				temp.project.chart.offect = offect;
				return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Offect), to_json(&temp.project.chart)?));
			}
			ui.end_row();

			ui.label(language[132].clone());
			let mut current_beat = temp.project.current_time as f32 / uspb as f32;
			ui.add(egui::Slider::new(&mut current_beat, 0.0..=2000.0));
			if current_beat != temp.project.current_time as f32 / uspb as f32 {
				temp.project.current_time = (current_beat * uspb as f32) as u64;
			}
			ui.end_row();

			ui.label(language[99].clone());
			ui.end_row();

			let id = temp.project.now_judge_field_id;
			if id < temp.project.chart.judge_field.len() { 
				let backup = temp.project.chart.judge_field[id].clone();
				ui.label(language[71].clone());
				let new_judge_field_id = match temp.project.new_judge_field_id.checked_sub(1) {
					Some(t) => t,
					None => 0,
				};
				ui.add(egui::Slider::new(&mut temp.project.now_judge_field_id, 0..=new_judge_field_id));
				ui.end_row();

				ui.label(language[72].clone());
				ui.add(egui::Slider::new(&mut temp.project.chart.judge_field[id].position.x, 0.0..=100.0).step_by(0.0001));
				ui.end_row();

				ui.label(language[73].clone());
				ui.add(egui::Slider::new(&mut temp.project.chart.judge_field[id].position.y, 0.0..=100.0).step_by(0.0001));
				ui.end_row();

				ui.label(language[100].clone());
				ui.add(egui::Slider::new(&mut temp.project.chart.judge_field[id].size.x, 0.0..=100.0).step_by(0.0001));
				ui.end_row();

				ui.label(language[101].clone());
				ui.add(egui::Slider::new(&mut temp.project.chart.judge_field[id].size.y, 0.0..=100.0).step_by(0.0001));
				ui.end_row();

				ui.label(language[77].clone());
				ui.add(egui::Slider::new(&mut temp.project.chart.judge_field[id].rotate, 0.0..=2.0* PI).step_by(0.0001));
				ui.end_row();

				ui.label(language[78].clone());
				ui.add(egui::Slider::new(&mut temp.project.chart.judge_field[id].rotate_center.x, 0.0..=100.0).step_by(0.0001));
				ui.end_row();

				ui.label(language[79].clone());
				ui.add(egui::Slider::new(&mut temp.project.chart.judge_field[id].rotate_center.y, 0.0..=100.0).step_by(0.0001));
				ui.end_row();

				ui.label(language[119].clone());
				let mut start_beat = temp.project.chart.judge_field[id].start_time as f64 / uspb as f64;
				ui.add(egui::Slider::new(&mut start_beat, 0.0..=2000.0).step_by(0.0001));
				if start_beat != temp.project.chart.judge_field[id].start_time as f64 / uspb as f64 {
					temp.project.chart.judge_field[id].start_time = (start_beat * uspb as f64) as u64
				}
				ui.end_row();

				ui.label(language[130].clone());
				let mut end_beat = temp.project.chart.judge_field[id].end_time as f64 / uspb as f64;
				ui.add(egui::Slider::new(&mut end_beat, 0.0..=2000.0).step_by(0.0001));
				if end_beat != temp.project.chart.judge_field[id].end_time as f64 / uspb as f64 {
					temp.project.chart.judge_field[id].end_time = (end_beat * uspb as f64) as u64
				}
				ui.end_row();

				ui.label(language[102].clone());
				egui::ComboBox::from_label(language[102].clone()).selected_text(format!("{}", temp.project.chart.judge_field[id].key.name()))
					.show_ui(ui, |ui| {
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::A, "A");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::B, "B");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::C, "C");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::D, "D");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::E, "E");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::F, "F");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::G, "G");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::H, "H");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::I, "I");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::J, "J");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::K, "K");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::L, "L");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::M, "M");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::N, "N");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::O, "O");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::P, "P");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::Q, "Q");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::R, "R");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::S, "S");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::T, "T");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::U, "U");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::V, "V");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::W, "W");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::X, "X");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::Y, "Y");
						ui.selectable_value(&mut temp.project.chart.judge_field[id].key, Key::Z, "Z");
				    }
				);
				ui.end_row();

				if backup != temp.project.chart.judge_field[id] {
					return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::JudgeField), to_json(&temp.project.chart)?));
				}
			}else {
				ui.label(language[103].clone());
				ui.end_row();
			}

			Ok(Back::Nothing)
		}).inner;
	}) {
		if let Some(e) = t.inner {
			vec_back.push(e?);
		}
	};

	// Edit Window
	let mut window_to_close = None;
	let window_edit = temp.project.window.clone();
	for (id, a) in &window_edit {
		match a {
			RenderType::Note(j,n) => {
				let note = &mut match temp.project.chart.note.get_mut(&j) {
					Some(t) => t,
					None => return Err(ShapoError::SystemError(format!("cant find note to show")))
				};
				if let None = note[*n].shape {
					return Err(ShapoError::SystemError(format!("cant find shape")))
				}
				let backup = note[*n].clone();
				if let Some(t) = egui::Window::new(format!("{} {}", language[53].clone(), note[*n].id)).scroll2([true;2]).resizable(true).show(ui.ctx(), |ui| -> Result<(Back, Note, bool), ShapoError> {
					let mut back_message = Back::Nothing;
					let mut new_note = note[*n].clone();
					let backup = note[*n].clone();
					let mut delete = false;
					if let Some(s) = &mut new_note.shape {
						let mut number = 0;
						for mut c in &mut *s {
							if let Some(t) = ui.collapsing(format!("{} {}", language[51].clone(), number), |ui| -> Result<Back, ShapoError> {
								shape_texture(ui, &mut c, &language, &uspb)
							}).body_returned {
								if c.if_delete {
									delete = true;
								}
								if let Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(change)),_) = t? {
									back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Shape(change))), String::new());
								};
							};
							number = number + 1
						};
						let mut shape = Shapo::default();
						egui::Grid::new("asdyuisdy868689124").show(ui, |ui| {
							if let Some(t) = ui.menu_button(language[92].clone(), |ui| -> Shapo {
								let mut shape = Shapo::default();
								if ui.button(language[54].clone()).clicked() {
									shape = Shapo{
										shape: Shape::Circle(Circle::default()),
										sustain_time: Some((current_beat as u64 * uspb, (current_beat as u64 + 4) * uspb)),
										label: Some(vec!(temp.project.now_shape_id.to_string())),
										..Default::default()
									};
									back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Shape(PossibleShapoChange::Add(ShapeAdd::Circle)))), String::new());
									ui.close_menu();
								};
								if ui.button(language[55].clone()).clicked() {
									shape = Shapo{
										shape: Shape::Rectangle(Rectangle::default()),
										sustain_time: Some((current_beat as u64 * uspb, (current_beat as u64 + 4) * uspb)),
										label: Some(vec!(temp.project.now_shape_id.to_string())),
										..Default::default()
									};
									back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Shape(PossibleShapoChange::Add(ShapeAdd::Rectangle)))), String::new());
									ui.close_menu();
								};
								if ui.button(language[57].clone()).clicked() {
									shape = Shapo{
										shape: Shape::CubicBezier(CubicBezier::default()),
										sustain_time: Some((current_beat as u64 * uspb, (current_beat as u64 + 4) * uspb)),
										label: Some(vec!(temp.project.now_shape_id.to_string())),
										..Default::default()
									};
									back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Shape(PossibleShapoChange::Add(ShapeAdd::CubicBezier)))), String::new());
									ui.close_menu();
								};
								shape
							}).inner {
								shape = t;
							};
							ui.end_row();

							ui.label(format!("{}: {}", language[90].clone(), new_note.id));
							ui.end_row();

							ui.label(language[91].clone());
							let new_judge_field_id = match temp.project.new_judge_field_id.checked_sub(1) {
								Some(t) => t,
								None => 0
							};
							ui.add(egui::Slider::new(&mut new_note.judge_field_id, 0..=new_judge_field_id));
							if backup.judge_field_id != new_note.judge_field_id {
								back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::JudgeFieldId)), String::new());
							}
							ui.end_row();

							ui.label(language[93].clone());
							ui.add(egui::Slider::new(&mut new_note.start_position.x, 0.0..=100.0).step_by(0.0001));
							if backup.start_position.x != new_note.start_position.x {
								back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::StartPosition)), String::new());
							}
							ui.end_row();

							ui.label(language[94].clone());
							ui.add(egui::Slider::new(&mut new_note.start_position.y, 0.0..=100.0).step_by(0.0001));
							if backup.start_position.y != new_note.start_position.y {
								back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::StartPosition)), String::new());
							}
							ui.end_row();

							ui.label(language[95].clone());
							ui.add(egui::Slider::new(&mut new_note.final_position.x, 0.0..=100.0).step_by(0.0001));
							if backup.final_position.x != new_note.final_position.x {
								back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::FinalPosition)), String::new());
							}
							ui.end_row();

							ui.label(language[96].clone());
							ui.add(egui::Slider::new(&mut new_note.final_position.y, 0.0..=100.0).step_by(0.0001));
							if backup.final_position.y != new_note.final_position.y {
								back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::FinalPosition)), String::new());
							}
							ui.end_row();

							let mut start_time_beat = new_note.start_time as f64 / uspb as f64;
							let mut end_time_beat = new_note.click_time as f64 / uspb as f64;
							ui.label(language[119].clone());
							ui.add(egui::Slider::new(&mut start_time_beat, 0.0..=2000.0).step_by(0.0001));
							if start_time_beat != new_note.start_time as f64 / uspb as f64 {
								new_note.start_time = (start_time_beat * uspb as f64) as u64;
								back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::StartTime)), String::new());
							}
							ui.end_row();

							ui.label(language[130].clone());
							ui.add(egui::Slider::new(&mut end_time_beat, 0.0..=2000.0).step_by(0.0001));
							if end_time_beat != new_note.click_time as f64 / uspb as f64 {
								new_note.click_time = (end_time_beat * uspb as f64) as u64;
								back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::ClickTime)), String::new());
							}
							ui.end_row();

							if ui.button(language[97].clone()).clicked() {
								if_note_delete = true;
								new_note.if_delete = true;
								back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Delete)), String::new());
							}
							ui.end_row();

							if ui.button(language[129].clone()).clicked() {
								window_to_close = Some(id)
							}
						});
						if shape != Shapo::default() {
							s.push(shape);
						}
						ui.end_row();
						if ui.button(language[157].clone()).clicked() {
							new_note.shape = None;
							back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Shape(PossibleShapoChange::Add(ShapeAdd::CubicBezier)))), String::new());
						}
						ui.end_row();
						if ui.button("Copy").clicked() {
							new_note.label = Some(vec!("Copy".to_string()));
							back_message = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Shape(PossibleShapoChange::Add(ShapeAdd::CubicBezier)))), String::new());
						}
						ui.end_row();
					};
					Ok((back_message,new_note, delete))
				}) {
					if let Some(p) = t.inner {
						let (back_message, e, delete) = p?;
						let mut back = e;
						if delete {
							let mut new_shape = vec!();
							for a in &back.shape.unwrap() {
								if !a.if_delete {
									new_shape.push(a.clone());
								}
							}
							back.shape = Some(new_shape);
						}
						if let Some(l) = back.label.clone() {
							if l == vec!("Copy".to_string()) {
								back.label = None;
								note.push(back.clone())
							}
						}
						if backup != back {
							note[*n] = back;
							if let Back::Change(change, _) = back_message {
								vec_back.push(Back::Change(change, to_json(&temp.project.chart)?));
							}
						}
					}
				};
			},
			RenderType::Shape(s) => {
				for a in &mut temp.project.chart.shape {
					if a.label.clone().unwrap()[0] == s.clone() {
						if let Some(t) = egui::Window::new(format!("{} {:?}", language[51].clone(), a.label)).scroll2([true;2]).resizable(true).show(ui.ctx(), |ui| -> Result<Back, ShapoError> {
							if let Back::Change(back, u) = shape_texture(ui, a, &language, &uspb)? {
								if u == String::from("Delete") {
									if_shape_delete = true;
								}
								return Ok(Back::Change(back,u))
							}
							if ui.button(language[129].clone()).clicked() {
								window_to_close = Some(id)
							}
							Ok(Back::Nothing)
						}) {
							if let Some(e) = t.inner {
								let e = e?;
								if let Back::Change(_, _) = e {
								}else {
									vec_back.push(e);
								}
							}
						};
					}
				}
			}
		};
	}
	if window_to_close.is_some() {
		temp.project.window.remove(window_to_close.unwrap());
	}

	if if_shape_delete {
		let mut new_shape = vec!();
		for a in &temp.project.chart.shape {
			if !a.if_delete {
				new_shape.push(a.clone());
			}else {
				let label = RenderLabel::Text(a.label.clone().unwrap()[0].clone());
				match temp.project.window.get(&label) {
					Some(_) => {
						temp.project.window.remove(&label);
					},
					None => {}
				};
			}
		}
		temp.project.chart.shape = new_shape;
		vec_back.push(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Delete)), to_json(&temp.project.chart)?));
	} 

	if if_note_delete {
		let mut new_note_map = BTreeMap::new();
		for (id, note) in &temp.project.chart.note {
			let mut new_note = vec!();
			for b in 0..note.len() {
				if !note[b].if_delete {
					new_note.push(note[b].clone());
				}else {
					let label = RenderLabel::Array(id.clone(),b);
					match temp.project.window.get(&label) {
						Some(_) => {
							temp.project.window.remove(&label);
						},
						None => {}
					};
				}
			}
			new_note_map.insert(*id, new_note);
		}
		temp.project.chart.note = new_note_map;
		vec_back.push(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Note(NoteChange::Delete)), to_json(&temp.project.chart)?));
	}

	Ok(vec_back)
}

fn shape_texture(ui: &mut egui::Ui, shape: &mut Shapo, language: &Vec<String>, uspb: &u64) -> Result<Back, ShapoError> {
	let backup = shape.clone();
	let mut back = egui::Grid::new("dafu678936ikjhjzxc").show(ui, |ui| -> Result<Back, ShapoError> {
		let mut back = Back::Nothing;

		ui.label(language[72].clone());
		ui.add(egui::Slider::new(&mut shape.style.position.x, 0.0..=100.0).step_by(0.0001));
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::Position))), String::new());
		}
		ui.end_row();

		ui.label(language[73].clone());
		ui.add(egui::Slider::new(&mut shape.style.position.y, 0.0..=100.0).step_by(0.0001));
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::Position))), String::new());
		}
		ui.end_row();

		if let Some((start_time, end_time)) = shape.sustain_time {
			let mut start_time_beat = start_time as f64 / *uspb as f64;
			let mut end_time_beat = end_time as f64 / *uspb as f64;
			ui.label(language[119].clone());
			ui.add(egui::Slider::new(&mut start_time_beat, 0.0..=2000.0).step_by(0.0001));
			if start_time_beat != start_time as f64 / *uspb as f64 {
				shape.sustain_time = Some(((start_time_beat * *uspb as f64) as u64, end_time));
				back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::SustainTime)), String::new());
			}
			ui.end_row();

			ui.label(language[130].clone());
			ui.add(egui::Slider::new(&mut end_time_beat, 0.0..=2000.0).step_by(0.0001));
			if end_time_beat != end_time as f64 / *uspb as f64 {
				shape.sustain_time = Some((start_time ,(end_time_beat * *uspb as f64) as u64));
				back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::SustainTime)), String::new());
			}
			ui.end_row();
		}

		ui.label(language[75].clone());
		ui.add(egui::Slider::new(&mut shape.style.size.x, 0.0..=10.0).step_by(0.0001));
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::Size))), String::new());
		}
		ui.end_row();

		ui.label(language[76].clone());
		ui.add(egui::Slider::new(&mut shape.style.size.y, 0.0..=10.0).step_by(0.0001));
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::Size))), String::new());
		}
		ui.end_row();

		ui.label(language[77].clone());
		ui.add(egui::Slider::new(&mut shape.style.rotate, 0.0..= 2.0 * PI).step_by(0.0001));
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::Rotate))), String::new());
		}
		ui.end_row();

		ui.label(language[78].clone());
		ui.add(egui::Slider::new(&mut shape.style.rotate_center.x, 0.0..=100.0).step_by(0.0001));
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::RotateCenter))), String::new());
		}
		ui.end_row();

		ui.label(language[79].clone());
		ui.add(egui::Slider::new(&mut shape.style.rotate_center.y, 0.0..=100.0).step_by(0.0001));
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::RotateCenter))), String::new());
		}
		ui.end_row();

		ui.label(language[80].clone());
		egui::widgets::color_picker::color_picker_color32(ui, &mut shape.style.fill, egui::widgets::color_picker::Alpha::OnlyBlend);
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::Fill))), String::new());
		}
		ui.end_row();

		ui.label(language[81].clone());
		ui.add(egui::Slider::new(&mut shape.style.stroke.width, 0.0..=100.0).step_by(0.0001));
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::StrokeWidth))), String::new());
		}
		ui.end_row();

		ui.label(language[82].clone());
		egui::widgets::color_picker::color_picker_color32(ui, &mut shape.style.stroke.color, egui::widgets::color_picker::Alpha::OnlyBlend);
		if shape.clone() != backup {
			back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::StrokeColor))), String::new());
		}
		ui.end_row();

		if let Some(_) = shape.style.layer {
			ui.label(language[83].clone());
			egui::ComboBox::from_label(language[83].clone())
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut shape.style.layer.unwrap().order, egui::layers::Order::Background, language[84].clone());
					ui.selectable_value(&mut shape.style.layer.unwrap().order, egui::layers::Order::PanelResizeLine, language[85].clone());
					ui.selectable_value(&mut shape.style.layer.unwrap().order, egui::layers::Order::Middle, language[86].clone());
					ui.selectable_value(&mut shape.style.layer.unwrap().order, egui::layers::Order::Foreground, language[87].clone());
				}
			);
			if shape.clone() != backup {
				back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Style(PossibleStyleChange::Layer))), String::new());
			}
			ui.end_row();
		}

		match &mut shape.shape {
			Shape::Circle(circle) => {
				let backup = circle.clone();
				ui.label(language[104].clone());
				ui.add(egui::Slider::new(&mut circle.radius, 0.0..=100.0).step_by(0.0001));
				if circle.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::Circle(CircleChange::Radius)))), String::new());
				}
				ui.end_row();
			},
			Shape::Rectangle(rect) => {
				let backup = rect.clone();
				ui.label(language[100].clone());
				ui.add(egui::Slider::new(&mut rect.bottom_right_point.x, 0.0..=100.0).step_by(0.0001));
				if rect.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::Rectangle(RectangleChange::BottomRightPoint)))), String::new());
				}
				ui.end_row();

				let backup = rect.clone();
				ui.label(language[101].clone());
				ui.add(egui::Slider::new(&mut rect.bottom_right_point.y, 0.0..=100.0).step_by(0.0001));
				if rect.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::Rectangle(RectangleChange::BottomRightPoint)))), String::new());
				}
				ui.end_row();
			},
			Shape::CubicBezier(cb) => {
				let backup = cb.clone();
				ui.label(language[105].clone());
				ui.add(egui::Slider::new(&mut cb.points[0].x, 0.0..=100.0).step_by(0.0001));
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::Point1)))), String::new());
				}
				ui.end_row();

				ui.label(language[106].clone());
				ui.add(egui::Slider::new(&mut cb.points[0].y, 0.0..=100.0).step_by(0.0001));
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::Point1)))), String::new());
				}
				ui.end_row();

				ui.label(language[107].clone());
				ui.add(egui::Slider::new(&mut cb.points[1].x, 0.0..=100.0).step_by(0.0001));
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::Point2)))), String::new());
				}
				ui.end_row();

				ui.label(language[108].clone());
				ui.add(egui::Slider::new(&mut cb.points[1].y, 0.0..=100.0).step_by(0.0001));
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::Point2)))), String::new());
				}
				ui.end_row();

				ui.label(language[109].clone());
				ui.add(egui::Slider::new(&mut cb.points[2].x, 0.0..=100.0).step_by(0.0001));
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::Point3)))), String::new());
				}
				ui.end_row();

				ui.label(language[110].clone());
				ui.add(egui::Slider::new(&mut cb.points[2].y, 0.0..=100.0).step_by(0.0001));
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::Point3)))), String::new());
				}
				ui.end_row();

				ui.label(language[111].clone());
				ui.add(egui::Slider::new(&mut cb.points[3].x, 0.0..=100.0).step_by(0.0001));
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::Point4)))), String::new());
				}
				ui.end_row();

				ui.label(language[112].clone());
				ui.add(egui::Slider::new(&mut cb.points[3].y, 0.0..=100.0).step_by(0.0001));
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::Point4)))), String::new());
				}
				ui.end_row();

				ui.label(language[113].clone());
				ui.checkbox(&mut cb.if_close ,language[113].clone());
				if cb.clone() != backup {
					back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Shape(ShapeChange::CubicBezier(CubicBezierChange::IfClose)))), String::new());
				}
				ui.end_row();
			},
			_ => todo!()
		}

		Ok(back)
	}).inner?;

	let mut delete = None;
	if shape.animation.len() != 0{ 
		for a in 0..shape.animation.len() {
			let backup = shape.animation[a].clone();
			ui.collapsing(format!("{} {}",language[115].clone(),a),|ui| {
				egui::Grid::new("asfi65uihfkjxcyv").show(ui, |ui| {
					if ui.button(language[125].clone()).clicked() {
						delete = Some(a);
					}
					ui.end_row();
					let text;
					match &mut shape.animation[a].style {
						StyleAnimate::Position(cb) => {
							bezier_curve_texture(ui,cb,language);
							text = language[121].clone();
						}
						StyleAnimate::Size(cb) => {
							bezier_curve_texture(ui,cb,language);
							text = language[122].clone();
						},
						StyleAnimate::Rotate => text = language[77].clone(),
						StyleAnimate::RoutateCenter(cb) => {
							bezier_curve_texture(ui,cb,language);
							text = language[123].clone();
						},
						StyleAnimate::Alpha => {
							text = language[155].clone();
						},
						StyleAnimate::ShapeAnimate(ShapeAnimate::Rectangle(RectangleAnimate::BottomRightPoint(cb))) => {
							bezier_curve_texture(ui,cb,language);
							text = language[150].clone();
						},
						StyleAnimate::ShapeAnimate(ShapeAnimate::Circle(CircleAnimate::Radius)) => {
							text = language[104].clone();
						},
						StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point1(cb))) => {
							bezier_curve_texture(ui,cb,language);
							text = language[151].clone();
						},
						StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point2(cb))) => {
							bezier_curve_texture(ui,cb,language);
							text = language[152].clone();
						},
						StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point3(cb))) => {
							bezier_curve_texture(ui,cb,language);
							text = language[153].clone();
						},
						StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point4(cb))) => {
							bezier_curve_texture(ui,cb,language);
							text = language[154].clone();
						},
						_ => todo!(),
					}
					ui.label(language[116].clone());
					egui::ComboBox::from_label(language[116].clone()).selected_text(text).show_ui(ui, |ui| {
						ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::Position(CubicBezier::default()), language[121].clone());
						ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::Size(CubicBezier::default()), language[122].clone());
						ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::Rotate, language[77].clone());
						ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::RoutateCenter(CubicBezier::default()), language[123].clone());
						ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::Alpha, language[155].clone());
						match shape.shape {
							Shape::Rectangle(_) => {
								ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::ShapeAnimate(ShapeAnimate::Rectangle(RectangleAnimate::BottomRightPoint(CubicBezier::default()))), language[150].clone());
							},
							Shape::Circle(_) => {
								ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::ShapeAnimate(ShapeAnimate::Circle(CircleAnimate::Radius)), language[104].clone());
							},
							Shape::CubicBezier(_) => {
								ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point1(CubicBezier::default()))), language[151].clone());
								ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point2(CubicBezier::default()))), language[152].clone());
								ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point3(CubicBezier::default()))), language[153].clone());
								ui.selectable_value(&mut shape.animation[a].style, StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point4(CubicBezier::default()))), language[154].clone());
							},
							_=> todo!(),
						}
					});
					ui.end_row();

					ui.label(language[117].clone());
					ui.add(egui::Slider::new(&mut shape.animation[a].start_value, 0.0..=100.0).step_by(0.0001));
					ui.end_row();

					ui.label(language[118].clone());
					ui.add(egui::Slider::new(&mut shape.animation[a].end_value, 0.0..=100.0).step_by(0.0001));
					ui.end_row();

					ui.label(language[119].clone());
					let mut divided = shape.animation[a].start_time.unwrap() as f64 / *uspb as f64;
					ui.add(egui::Slider::new(&mut divided , 0.0..=2000.0).step_by(0.0001));
					if divided != shape.animation[a].start_time.unwrap() as f64 / *uspb as f64 {
						shape.animation[a].start_time = Some((divided * *uspb as f64) as u64)
					}
					ui.end_row();

					ui.label(language[120].clone());
					let mut divided = shape.animation[a].animate_time as f64 / *uspb as f64;
					ui.add(egui::Slider::new(&mut divided , 0.0..=2000.0).step_by(0.0001));
					if divided != shape.animation[a].animate_time as f64 / *uspb as f64 {
						shape.animation[a].animate_time = (divided * *uspb as f64) as u64
					}
					ui.end_row();

					ui.label(language[124].clone());
					ui.end_row();
				});

				ui.separator();
				fn hard_compresser(vec_input: Vec2) -> Vec2 {
					let mut vec = vec_input;
					if vec_input.x < 0.0 {
						vec.x = 0.0
					}else if vec_input.x > 1.0 {
						vec.x = 1.0
					}else {
						vec.x = vec_input.x
					}
					if vec_input.y < 0.0 {
						vec.y = 0.0
					}else if vec_input.y > 1.0 {
						vec.y = 1.0
					}else {
						vec.y = vec_input.y
					}
					vec
				}
				let canvas_size = Vec2::new(ui.available_width(), 150.0);
				let offect = Vec2{x: ui.max_rect().left(), y: ui.cursor().top()};
				let point_0 = offect.to_pos2();
				let mut point_1 = (shape.animation[a].animation.control_point_one * canvas_size + offect).to_pos2();
				let mut point_2 = (shape.animation[a].animation.control_point_two * canvas_size + offect).to_pos2();
				let point_3 = (Vec2::new(1.0, 1.0) * canvas_size + offect).to_pos2();
				ui.allocate_painter(canvas_size, Sense::hover());

				let vol_rect_1 = Rect { min: (point_1.to_vec2() - Vec2::new(5.0,5.0)).to_pos2(), max: (point_1.to_vec2() + Vec2::new(5.0,5.0)).to_pos2() };
				let (_, response_1) = ui.allocate_ui_at_rect(vol_rect_1, |ui| {
					ui.centered_and_justified(|ui| ui.allocate_exact_size(Vec2{x: vol_rect_1.max.x - vol_rect_1.min.x, y: vol_rect_1.max.y - vol_rect_1.min.y}, egui::Sense::drag())).inner
				}).inner;
				let vol_rect_2 = Rect { min: (point_2.to_vec2() - Vec2::new(5.0,5.0)).to_pos2(), max: (point_2.to_vec2() + Vec2::new(5.0,5.0)).to_pos2() };
				let (_, response_2) = ui.allocate_ui_at_rect(vol_rect_2, |ui| {
					ui.centered_and_justified(|ui| ui.allocate_exact_size(Vec2{x: vol_rect_2.max.x - vol_rect_2.min.x, y: vol_rect_2.max.y - vol_rect_2.min.y}, egui::Sense::drag())).inner
				}).inner;

				point_1 = point_1 + response_1.drag_delta();
				point_2 = point_2 + response_2.drag_delta();

				if point_1 != (shape.animation[a].animation.control_point_one * canvas_size + offect).to_pos2() {
					let normalized = hard_compresser((point_1.to_vec2() - offect) / canvas_size);
					shape.animation[a].animation.control_point_one = normalized
				}

				if point_2 != (shape.animation[a].animation.control_point_two * canvas_size + offect).to_pos2() {
					let normalized = hard_compresser((point_2.to_vec2() - offect) / canvas_size);
					shape.animation[a].animation.control_point_two = normalized
				}

				ui.painter().circle(point_1, 10.0, Color32::TRANSPARENT, Stroke::new(3.0, Color32::WHITE));
				ui.painter().circle(point_2, 10.0, Color32::TRANSPARENT, Stroke::new(3.0, Color32::WHITE));
				ui.painter().circle(point_0, 10.0, Color32::TRANSPARENT, Stroke::new(3.0, Color32::WHITE));
				ui.painter().circle(point_3, 10.0, Color32::TRANSPARENT, Stroke::new(3.0, Color32::WHITE));
				ui.painter().line_segment([point_0, point_1], Stroke::new(1.5, Color32::WHITE));
				ui.painter().line_segment([point_3, point_2], Stroke::new(1.5, Color32::WHITE));

				ui.painter().add(CubicBezierShape{
					points: [
						point_0,
						point_1,
						point_2,
						point_3,
					],
					fill: Color32::TRANSPARENT,
					stroke: Stroke::new(3.0, Color32::WHITE),
					closed: false
				});
			});
			ui.separator();
			ui.end_row();

			if backup != shape.animation[a] {
				back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Animation)), String::new());
			}
		}
	}
	let mut new_animation = vec!();
	if delete.is_some() {
		for a in 0..shape.animation.len() {
			if a != delete.unwrap() {
				new_animation.push(shape.animation[a].clone())
			}
		} 
		shape.animation = new_animation.clone();
	}

	if ui.button(language[114].clone()).clicked() {
		shape.animation.push(StyleAnimation{
			start_time: Some(shape.sustain_time.unwrap().0),
			animate_time: shape.sustain_time.unwrap().1 - shape.sustain_time.unwrap().0,
			..Default::default()
		});
		back = Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Animation)), String::new());
	}

	if ui.button(language[98].clone()).clicked() {
		shape.if_delete = true;
		return Ok(Back::Change(ChangeType::ChartTemp(PossibleChartChange::Shape(PossibleShapoChange::Delete)), String::from("Delete")))
	}

	Ok(back)
}

fn bezier_curve_texture(ui: &mut egui::Ui, cb: &mut CubicBezier, language: &Vec<String>) {
	ui.label(language[105].clone());
	ui.add(egui::Slider::new(&mut cb.points[0].x, 0.0..=100.0).step_by(0.0001));
	ui.end_row();

	ui.label(language[106].clone());
	ui.add(egui::Slider::new(&mut cb.points[0].y, 0.0..=100.0).step_by(0.0001));
	ui.end_row();

	ui.label(language[107].clone());
	ui.add(egui::Slider::new(&mut cb.points[1].x, 0.0..=100.0).step_by(0.0001));
	ui.end_row();

	ui.label(language[108].clone());
	ui.add(egui::Slider::new(&mut cb.points[1].y, 0.0..=100.0).step_by(0.0001));
	ui.end_row();

	ui.label(language[109].clone());
	ui.add(egui::Slider::new(&mut cb.points[2].x, 0.0..=100.0).step_by(0.0001));
	ui.end_row();

	ui.label(language[110].clone());
	ui.add(egui::Slider::new(&mut cb.points[2].y, 0.0..=100.0).step_by(0.0001));
	ui.end_row();

	ui.label(language[111].clone());
	ui.add(egui::Slider::new(&mut cb.points[3].x, 0.0..=100.0).step_by(0.0001));
	ui.end_row();

	ui.label(language[112].clone());
	ui.add(egui::Slider::new(&mut cb.points[3].y, 0.0..=100.0).step_by(0.0001));
	ui.end_row();
}

fn none_to_zero(input: &Option<u64>) -> u64 {
	match input {
		Some(t) => return *t,
		None => return 0,
	}
}