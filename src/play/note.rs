use std::ops::BitOr;
use std::ops::Sub;
use std::ops::Add;
use crate::ui::shape::text::Text;
use crate::ui::shape::image::Image;
use crate::ui::ui::Component;
use crate::system::system_function::parse_json;
use crate::ASSETS_PATH;
use crate::ui::shapo::rotate;
use crate::play::play_top::Touch;
use crate::language::language::Language;
use crate::ui::shape::circle::CircleChange;
use crate::ui::shape::rectangle::*;
use crate::ui::shape::bezier_curve::*;
use crate::ui::shape::style::ShapeAnimate;
use egui::Rect;
use egui::Response;
use crate::system::system_function::parse_toml;
use crate::play::play_top::PlayTop;
use egui::TextureId;
use std::collections::HashMap;
use egui::TextureHandle;
use crate::ui::shape::bezier_curve::CubicBezier;
use crate::system::system_function::read_file;
use egui::Rounding;
use egui::Color32;
use egui::Stroke;
use crate::ui::shape::rectangle::Rectangle;
use crate::ui::shape::circle::Circle;
use crate::ui::shapo::Shape;
use crate::ui::shape::style::Style;
use crate::setting::setting::read_settings;
use std::collections::BTreeMap;
use crate::ui::shape::style::arc_length;
use crate::ui::shape::style::StyleAnimation;
use crate::play::timer::Timer;
use crate::ui::ui::Back;
use crate::ShapoError;
use crate::ui::ui::Router;
use egui::Vec2;
use egui::Key;
use crate::ui::shape::style::StyleAnimate;
use crate::ui::shapo::Shapo;

const IMMACULATE_TIME: i128 = 5 * (1e4 as i128);
const EXTRA_TIME: i128 = 7 * (1e4 as i128);
const NORMAL_TIME: i128 = 12 * (1e4 as i128);
const FADE_TIME: i128 = 15 * (1e4 as i128);

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Note {
	pub id: usize,
	pub judge_field_id: usize,
	pub shape: Option<Vec<Shapo>>,
	pub clicked_time: Option<u64>,
	pub click_time: u64,
	pub start_time: u64,
	pub start_position: Vec2,
	pub final_position: Vec2,
	pub judge_type: JudgeType,
	pub judge: Judge,
	pub label: Vec<String>,
	pub if_delete: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum JudgeType {
	Tap,
	Slide
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Judge {
	Immaculate(f32),
	Extra,
	Normal,
	Fade,
	Miss,
	None
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct JudgeField {
	pub size: Vec2,
	pub position: Vec2,
	pub id: usize,
	pub animation: Vec<StyleAnimation>,
	pub rotate: f32,
	pub end_time: u64,
	pub start_time: u64,
	pub rotate_center: Vec2,
	pub key: Key,
	pub label: Vec<String>,
	pub if_delete: bool
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct Chart {
	pub songtitle: String,
	pub mapname: String,
	pub bpm: f32,
	pub producer: String,
	pub charter: String,
	pub painter: String,
	pub note: BTreeMap<usize,Vec<Note>>,//第一个是判定范围的id
	pub now_judge: BTreeMap<usize,usize>,//第一个同上，第二个是该判定范围下已经判定到的note
	pub judge_field: Vec<JudgeField>,
	pub shape: Vec<Shapo>,
	pub length: u64,
	pub if_playing: bool,
	pub offect: u64
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum PossibleChartSelection {
	JudgeField(usize),
	Note(usize, usize),
	Shape(usize)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct Project {
	pub chart: Chart,
	pub now_page: EditPages,
	pub timer: Option<Timer>,
	pub new_judge_field_id: usize,
	pub now_judge_field_id: usize,
	pub now_note_id: usize,
	pub now_shape_id: usize,
	pub current_time: u64,
	pub if_info: bool,
	pub window: HashMap<RenderLabel, RenderType>,
	pub if_playing: bool,
	pub if_music_play: bool,
	pub now_select: Option<PossibleChartSelection>,
	pub multi_select: Vec<PossibleChartSelection>,
	pub timeline_size: f32,
	pub default: AllDefault
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct AllDefault {
	pub circle: Shapo,
	pub rectangle: Shapo,
	pub bezier_curve: Shapo,
	pub text: Shapo,
	pub image: Shapo,
	pub tap: Note,
	pub slide: Note,
	pub judge_field: JudgeField
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub enum EditPages {
	Timeline,
	View,
	TimelineAndView
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub enum RenderLabel {
	Text(String),
	Array(usize, usize),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub enum RenderType {
	Note(usize, usize),
	Shape(String)
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub enum NoteChange {
	JudgeFieldId,
	Shape(PossibleShapoChange),
	ClickTime,
	StartTime,
	StartPosition,
	FinalPosition,
	Delete,
	Add(JudgeType),
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub enum PossibleChartChange {
	Songtitle,
	Mapname,
	Bpm,
	Producer,
	Charter,
	Note(NoteChange),
	JudgeField,
	Shape(PossibleShapoChange),
	Length,
	Painter,
	Offect
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub enum ShapeAdd {
	Circle,
	Rectangle,
	CubicBezier
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub enum ShapeChange{
	Circle(CircleChange),
	Rectangle(RectangleChange),
	CubicBezier(CubicBezierChange)
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub enum PossibleShapoChange {
	Add(ShapeAdd),
	Delete,
	Shape(ShapeChange),
	Animation,
	Style(PossibleStyleChange),
	SustainTime
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub enum PossibleStyleChange {
	Position,
	IfAbsolute,
	Size,
	Rotate,
	RotateCenter,
	Fill,
	StrokeColor,
	StrokeWidth,
	Layer,
}

impl Default for AllDefault {
	fn default() -> Self {
		Self {
			circle: Shapo {
				shape: Shape::Circle(Circle::default()), 
				..Default::default()
			},
			rectangle: Shapo {
				shape: Shape::Rectangle(Rectangle::default()), 
				..Default::default()
			},
			bezier_curve: Shapo {
				shape: Shape::CubicBezier(CubicBezier::default()), 
				..Default::default()
			},
			text: Shapo {
				shape: Shape::Text(Text::default()), 
				..Default::default()
			},
			image: Shapo {
				shape: Shape::Image(Image::default()), 
				..Default::default()
			},
			tap: Note::default(),
			slide: Note {
				judge_type: JudgeType::Slide,
				..Note::default()
			},
			judge_field: JudgeField::default()
		}
	}
}

impl Default for Project {
	fn default() -> Self {
		Self {
			chart: Chart::default(),
			now_page: EditPages::View,
			timer: None,
			new_judge_field_id: 0,
			now_judge_field_id: 0,
			now_note_id: 0,
			current_time: 0,
			now_shape_id: 0,
			if_info: false,
			window: HashMap::new(),
			if_playing: false,
			if_music_play: false,
			multi_select: vec!(),
			timeline_size: 1.0,
			default: AllDefault::default(),
			now_select: None
		}
	}
}

impl Project {
	pub fn from_chart(chart: Chart) -> Self {
		Self {
			chart,
			..Default::default()
		}
	}

	pub fn timer(&mut self) -> Result<(),ShapoError> {
		if !self.if_playing {
			let mut timer = Timer::new(1);
			timer.start()?;
			timer.set(self.current_time)?;
			self.timer = Some(timer);
			self.if_music_play = false;
		}
		Ok(())
	}

	pub fn render(&mut self, ui: &mut egui::Ui, size: Vec2, if_enabled: bool,texture: &HashMap<TextureId,TextureHandle>, offect: Option<Vec2>) -> Result<(Vec<Back>, HashMap<RenderType, Response>),ShapoError> {
		self.timer()?;
		let mut timer = self.timer.unwrap();
		timer.set(3 * 1e6 as u64)?;
		self.chart.length_normallize();
		self.chart.project_render(ui, &size, &mut timer, if_enabled,texture, offect,&self.clone())
	}
}

impl Default for Chart {
	fn default() -> Self {
		let mut note_vec = Vec::new();
		let space = 5 * 1e5 as u64;
		for a in 1..=20 {
			if rand::random() {
				note_vec.push(Note::new_tap(a as usize - 1, 1, a as u64 * space, (a - 1) as u64 * space, vec!()))
			}else {
				note_vec.push(Note::new_slide(a as usize - 1, 1, a as u64 * space, (a - 1) as u64 * space, vec!()))
			}
		}
		let mut note = BTreeMap::new();
		note.insert(1, note_vec);
		let mut now_judge = BTreeMap::new();
		now_judge.insert(1,0);
		Self {
			songtitle: String::new(),
			mapname: String::new(),
			painter: String::new(),
			bpm: 150.0,
			producer: String::new(),
			charter: String::new(),
			note,
			now_judge,
			length: 1e7 as u64,
			judge_field: vec!(JudgeField::default()),
			shape: vec!(),
			if_playing: false,
			offect: 0
		}
	}
}

impl Chart {
	pub fn is_empty(&self) -> bool {
		self.songtitle.is_empty() ||
		self.mapname.is_empty() ||
		self.painter.is_empty() ||
		self.producer.is_empty() ||
		self.charter.is_empty() ||
		self.bpm == 0.0 
	}

	pub fn length_normallize(&mut self) {
		let mut max_length = 0;
		for (_,b) in &mut self.note {
			let mut new_note = vec!();
			for c in &mut *b {
				c.check();
				if c.click_time > max_length {
					max_length = c.click_time
				}
				if !c.if_delete {
					new_note.push(c.clone())
				}
			};
			*b = new_note;
		}
		let mut new_judge_field = vec!();
		for a in &self.judge_field {
			if a.end_time > max_length {
				max_length = a.end_time
			}
			if !a.if_delete {
				new_judge_field.push(a.clone())
			}
		}
		self.judge_field = new_judge_field;
		let mut new_shape = vec!();
		for a in &self.shape{
			let (_,e) = a.sustain_time.unwrap();
			if e > max_length {
				max_length = e
			}
			if !a.if_delete {
				new_shape.push(a.clone())
			}
		}
		self.shape = new_shape;
		self.length = max_length;

		for (_, b) in &mut self.note {
			b.sort_by(|a, b| a.click_time.cmp(&b.click_time));
			if b.len() > 0 {
				for c in 0..b.len() {
					b[c].id = c;
				}
			}
		}
		self.judge_field.sort_by(|a, b| a.end_time.cmp(&b.end_time));
		for c in 0..self.judge_field.len() {
			self.judge_field[c].id = c
		}
		self.shape.sort_by(|a, b| {
			let (_,b) = b.sustain_time.unwrap();
			let (_,a) = a.sustain_time.unwrap();
			a.cmp(&b)
		});
	}

	pub fn change(&mut self, change: &PossibleChartChange, json: &String) -> Result<Self, ShapoError> {
		match change {
			PossibleChartChange::Songtitle => self.songtitle = parse_json(json)?,
			PossibleChartChange::Mapname => self.mapname = parse_json(json)?,
			PossibleChartChange::Bpm => {
				let text:String = parse_json(json)?;
				let number: f32 = match text.parse() {
					Ok(t) => t,
					Err(_) => 0.0
				};
				self.bpm = number;
			},
			PossibleChartChange::Producer => self.producer = parse_json(json)?,
			PossibleChartChange::Charter => self.charter = parse_json(json)?,
			PossibleChartChange::Note(_) => self.note = parse_json(json)?,
			PossibleChartChange::JudgeField => self.judge_field = parse_json(json)?,
			PossibleChartChange::Shape(_) => self.shape = parse_json(json)?,
			PossibleChartChange::Length => self.length = parse_json(json)?,
			PossibleChartChange::Painter => self.painter = parse_json(json)?,
			PossibleChartChange::Offect => self.offect = parse_json(json)?,
		}
		return Ok(self.clone());
	}

	pub fn max_note(&mut self) -> usize {
		let mut max_note = 0;
		for (_, note) in &self.note {
			max_note = note.len() + max_note;
		}
		max_note
	}

	pub fn project_render(&mut self, ui: &mut egui::Ui, size: &Vec2, input_timer: &mut Timer, if_paused: bool, texture: &HashMap<TextureId,TextureHandle>, offect: Option<Vec2>, project: &Project) -> Result<(Vec<Back>, HashMap<RenderType, Response>), ShapoError> {
		let time_read = match input_timer.read()?.checked_sub(3 * 1e6 as u64){
			Some(t) => t,
			None => return Ok((vec!(Back::Nothing), HashMap::new()))
		};
		let mut map = HashMap::new();
		let mut timer = Timer::new(1);
		let setting = read_settings()?;
		let time_read = match time_read.checked_sub((setting.offect * 1e3) as u64) {
			Some(t) => t,
			None => 0,
		};
		let time_read = match time_read.checked_sub(self.offect) {
			Some(t) => t,
			None => 0,
		};
		let offect_vec = match offect {
			Some(t) => t,
			None => Vec2::new(0.0,0.0),
		};
		timer.start()?;
		timer.set(time_read)?;
		let uspb = (60.0 * 1e6 / project.chart.bpm) as u64;
		ui.label(format!("{} {:.3}",Language::Code(133).get_language()?, timer.read()? as f64 / 1e6));
		ui.label(format!("{} {:.3}",Language::Code(160).get_language()?, (timer.read()? + self.offect + (setting.offect * 1e3) as u64) as f64 / uspb as f64));
		let mut vec_back = Vec::new();
		if !project.if_music_play && project.if_playing {
			vec_back.push(Back::MusicPlay(format!("{}/assets/chart/{}/song.mp3",*ASSETS_PATH , self.mapname), self.bpm,0.0, (-(project.current_time as f32) + self.offect as f32) as f32 / 1e6));
		}
		let mut timer_vec = vec!(timer.clone());
		for i in 0..self.shape.len() {
			self.shape[i].rect_normalize();
			let label = &self.shape[i].label.clone()[0];
			if project.window.get(&RenderLabel::Text(label.to_string())).is_some() {
				for a in &self.shape[i].animation {
					match &a.style {
						StyleAnimate::Position(cb) | 
						StyleAnimate::RoutateCenter(cb) | 
						StyleAnimate::ShapeAnimate(ShapeAnimate::Rectangle(RectangleAnimate::BottomRightPoint(cb))) |
						StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point1(cb))) | 
						StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point2(cb))) |
						StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point3(cb))) |
						StyleAnimate::ShapeAnimate(ShapeAnimate::Bezier(CubicBezierAnimate::Point4(cb)))
						=> {
							Shapo {
								style: Style {
									fill: Color32::TRANSPARENT,
									stroke: Stroke { width: 3.0, color: Color32::from_rgba_premultiplied(200,200,200,200) },
									..Default::default()
								},
								shape: Shape::CubicBezier(cb.clone()),
								..Default::default()
							}.render(ui, size,&mut timer_vec ,offect, if_paused, texture)?;
						}
						_ => {},
					}
				}
			}
			let back = self.shape[i].render(ui, size,&mut timer_vec ,offect, if_paused,texture)?;
			for b in back {
				vec_back.push(b)
			}
			let label = &self.shape[i].label[0];
			if self.shape[i].sustain_time.unwrap().0 < time_read && self.shape[i].sustain_time.unwrap().1 > time_read {
				let vol_rect = self.shape[i].get_rect(size,offect);
				let (_, response) = ui.allocate_ui_at_rect(vol_rect, |ui| {
					ui.centered_and_justified(|ui| ui.allocate_exact_size(Vec2{x: vol_rect.max.x - vol_rect.min.x, y: vol_rect.max.y - vol_rect.min.y}, egui::Sense::click_and_drag())).inner
				}).inner;
				map.insert(RenderType::Shape(label.to_string()), response);
			}
		}
		for (id, a) in &mut self.note {
			for c in 0..a.len() {
				let back = a[c].render(ui, size,&mut timer_vec, if_paused,texture, offect)?;
				for b in back {
					if let Back::Nothing = b {}
					else { 
						vec_back.push(b) 
					}
				}
				let label = RenderLabel::Array(*id, c);
				if project.window.get(&label).is_some() {
					for shape in a[c].shape.clone().unwrap() {
						for anima in shape.animation {
							match &anima.style {
								StyleAnimate::Position(cb) => {
									Shapo {
										style: Style {
											fill: Color32::TRANSPARENT,
											stroke: Stroke { width: 3.0, color: Color32::WHITE },
											..Default::default()
										},
										shape: Shape::CubicBezier(cb.clone()),
										..Default::default()
									}.render(ui, size,&mut timer_vec ,offect, if_paused, texture)?;
								},
								StyleAnimate::RoutateCenter(cb) => {
									Shapo {
										style: Style {
											fill: Color32::TRANSPARENT,
											stroke: Stroke { width: 3.0, color: Color32::WHITE },
											..Default::default()
										},
										shape: Shape::CubicBezier(cb.clone()),
										..Default::default()
									}.render(ui, size,&mut timer_vec ,offect, if_paused, texture)?;
								},
								_ => {},
							}
						}
					}
				}
				let position = a[c].final_position / 100.0 * *size;
				if a[c].click_time > time_read && a[c].start_time < time_read {
					let vol_rect = Rect{
						min: (position - Vec2{x: 30.0, y: 30.0}).to_pos2(),
						max: (position + Vec2{x: 30.0, y: 30.0}).to_pos2()
					};
					let (_, response) = ui.allocate_ui_at_rect(vol_rect, |ui| {
						ui.centered_and_justified(|ui| ui.allocate_exact_size(Vec2{x: vol_rect.max.x - vol_rect.min.x + offect_vec.x, y: vol_rect.max.y - vol_rect.min.y + offect_vec.y}, egui::Sense::click_and_drag())).inner
					}).inner;
					map.insert(RenderType::Note(*id, c), response);
				}
			}
		}
		for a in &mut self.judge_field {
			a.render(ui, size,&mut timer_vec, if_paused, texture, offect)?;
		}
		Ok((vec_back, map))
	}

	pub fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, input_timer: &mut Timer, if_paused: bool, texture: &HashMap<TextureId,TextureHandle>, offect: Option<Vec2>, touch: &mut HashMap<u64, Touch>) -> Result<Vec<Back>, ShapoError> {
		let time_read = match input_timer.read()?.checked_sub(3 * 1e6 as u64) {
			Some(t) => t,
			None => return Ok(vec!(Back::Nothing))
		};
		let setting = read_settings()?;
		let time_read = match time_read.checked_sub((setting.offect * 1e3) as u64) {
			Some(t) => t,
			None => 0,
		};
		let time_read = match time_read.checked_sub(self.offect) {
			Some(t) => t,
			None => 0,
		};
		if time_read > self.length + 1e6 as u64 {
			return Ok(vec!(Back::Router(Router::EndPage(PlayTop::default()))));
		}
		let mut timer = Timer::new(1);
		timer.start()?;
		timer.set(time_read)?;
		let mut vec_back = Vec::new();
		if !self.if_playing {
			vec_back.push(Back::MusicPlay(format!("{}/assets/chart/{}/song.mp3",*ASSETS_PATH , self.mapname), self.bpm, 0.0, self.offect as f32 / 1e6));
			self.if_playing = true;
		}
		let mut timer_vec = vec!(timer.clone());
		let mut judged_note = ui.input_mut(|input| -> Vec<(usize,usize)> {
			let mut judged_note = Vec::new();
			for field in &self.judge_field {
				if input.key_down(field.key) {
					let note = match self.note.get_mut(&field.id) {
						Some(t) => t,
						None => unreachable!(),
					};
					let iter_number = *self.now_judge.get(&field.id).unwrap();
					if iter_number < note.len() {
						let delta = note[iter_number].click_time as i128 - time_read  as i128;
						if let JudgeType::Slide = note[iter_number].judge_type {
							if -IMMACULATE_TIME < delta && delta < IMMACULATE_TIME {
								note[iter_number].clicked_time = Some(time_read);
								note[iter_number].judge = Judge::Immaculate(1.0);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
							} 
						}
					}
				}
				if input.key_released(field.key) {
					let note = match self.note.get_mut(&field.id) {
						Some(t) => t,
						None => unreachable!(),
					};
					let iter_number = *self.now_judge.get(&field.id).unwrap();
					if iter_number < note.len() {
						if let JudgeType::Tap = note[iter_number].judge_type {
							let delta = note[iter_number].click_time as i128 - time_read  as i128;
							if -IMMACULATE_TIME < delta && delta < IMMACULATE_TIME  {
								note[iter_number].judge = Judge::Immaculate((1.0 - (delta.abs() as f32/ IMMACULATE_TIME as f32)) * 0.2 + 0.8);
								note[iter_number].clicked_time = Some(time_read);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
							}else if -EXTRA_TIME < delta && delta < EXTRA_TIME {
								note[iter_number].judge = Judge::Extra;
								note[iter_number].clicked_time = Some(time_read);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
							}else if -NORMAL_TIME < delta && delta < NORMAL_TIME {
								note[iter_number].judge = Judge::Normal;
								note[iter_number].clicked_time = Some(time_read);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
							}else if -FADE_TIME < delta && delta < FADE_TIME {
								note[iter_number].judge = Judge::Fade;
								note[iter_number].clicked_time = Some(time_read);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
							}
						}
					}
				}
			}
			judged_note
		});

		for field in &self.judge_field {
			for (_, t) in &mut *touch {
				let note = match self.note.get_mut(&field.id) {
					Some(note) => note,
					None => return Err(ShapoError::SystemError("???".to_string())),
				};
				let iter_number = *self.now_judge.get(&field.id).unwrap();
				let position = rotate(field.rotate_center ,t.position, field.rotate);
				let judge = field.position.x < position.x && position.x < field.size.x && field.position.y < position.y && position.y < field.size.y;
				if t.if_click && judge {
					if iter_number < note.len() {
						if let JudgeType::Tap = note[iter_number].judge_type {
							let delta = note[iter_number].click_time as i128 - time_read  as i128;
							if -IMMACULATE_TIME < delta && delta < IMMACULATE_TIME  {
								note[iter_number].judge = Judge::Immaculate((1.0 - (delta.abs() as f32/ IMMACULATE_TIME as f32)) * 0.2 + 0.8);
								note[iter_number].clicked_time = Some(time_read);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
								t.if_click = false;
							}else if -EXTRA_TIME < delta && delta < EXTRA_TIME {
								note[iter_number].judge = Judge::Extra;
								note[iter_number].clicked_time = Some(time_read);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
								t.if_click = false;
							}else if -NORMAL_TIME < delta && delta < NORMAL_TIME {
								note[iter_number].judge = Judge::Normal;
								note[iter_number].clicked_time = Some(time_read);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
								t.if_click = false;
							}else if -FADE_TIME < delta && delta < FADE_TIME {
								note[iter_number].judge = Judge::Fade;
								note[iter_number].clicked_time = Some(time_read);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
								t.if_click = false;
							}
						}
					}
				}
				if judge {
					if iter_number < note.len() {
						let delta = note[iter_number].click_time as i128 - time_read  as i128;
						if let JudgeType::Slide = note[iter_number].judge_type {
							if -IMMACULATE_TIME < delta && delta < IMMACULATE_TIME {
								note[iter_number].clicked_time = Some(time_read);
								note[iter_number].judge = Judge::Immaculate(1.0);
								judged_note.push((field.id,iter_number));
								*self.now_judge.get_mut(&field.id).unwrap() = *self.now_judge.get_mut(&field.id).unwrap() + 1;
							} 
						}
					}
				}
			}
		}
		
		let mut shape_vec_to_remove = vec!();
		for i in 0..self.shape.len() {
			let back = self.shape[i].render(ui, size,&mut timer_vec ,offect, if_paused,texture)?;
			for b in back {
				if let Back::AnimateDone(_) = b {
					let mut max_time = 0;
					for c in &self.shape[i].animation {
						let delay = c.start_time;
						if max_time <= delay + c.animate_time {
							max_time = delay + c.animate_time
						}
					}
					if max_time <= time_read {
						shape_vec_to_remove.push(i);
					}
				}
				vec_back.push(b)
			}
		}
		if !shape_vec_to_remove.is_empty() {
			let mut vec_to_push = vec!();
			for i in 0..self.shape.len() {
				let mut should_push = true;
				for j in &shape_vec_to_remove {
					if &i == j {
						should_push = false;
					}
				}
				if should_push {
					vec_to_push.push(self.shape[i].clone())
				}
			}
			self.shape = vec_to_push;
		}
		for (id, a) in &mut self.note {
			let start = match self.now_judge.get(id) {
				Some(t) => t,
				None => return Err(ShapoError::SystemError(format!("cant find note")))
			}.clone();
			for c in start..a.len() {
				if self.now_judge.get(id).unwrap() + setting.search_depth < c {
					break;
				}
				if a[c].click_time + (FADE_TIME as u64) < time_read {
					if let Judge::None = a[c].judge {
						a[c].judge = Judge::Miss;
						judged_note.push((*id,a[c].id));
						*self.now_judge.get_mut(&a[c].judge_field_id).unwrap() = *self.now_judge.get_mut(&a[c].judge_field_id).unwrap() + 1;
					}
				}
				let back = a[c].render(ui, size,&mut timer_vec, if_paused,texture, offect)?;
				for b in back {
					if let Back::Nothing = b {}
					else { 
						vec_back.push(b) 
					}
				}
			}
		}
		for a in &mut self.judge_field {
			a.render(ui, size,&mut timer_vec, if_paused, texture, offect)?;
		}
		vec_back.push(Back::JudgeNote(judged_note));
		Ok(vec_back)
	}
}

impl Note {
	fn check(&mut self) {
		let mut new_shape = vec!();
		if self.shape.is_some() {
			for a in self.shape.clone().unwrap() {
				if !a.if_delete {
					new_shape.push(a);
				}
			}
			self.shape = Some(new_shape);
		}
	}

	fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, if_paused: bool, texture: &HashMap<TextureId,TextureHandle>, offect: Option<Vec2>) -> Result<Vec<Back>, ShapoError>  {
		let mut vec_back = Vec::new();
		let time_read = timer[0].read()?;
		if self.click_time + FADE_TIME as u64 > time_read && self.judge == Judge::None {
			let setting = read_settings()?;
			if let Some(t) = &mut self.shape {
				for a in t {
					let back = a.render(ui, size, timer ,offect, if_paused, texture)?;
					for a in back {
						vec_back.push(a)
					}
				}
			}else {
				let note_read: Component;
				let path: String;
				let curve = CubicBezier {
					points: [
						(self.start_position).to_pos2(),
						(self.start_position).to_pos2(),
						(self.final_position).to_pos2(),
						(self.final_position).to_pos2(),
					],
					if_close: false
				};
				let length = simpsons_rule_integration(1.0, &curve);
				match self.judge_type {
					JudgeType::Tap => {
						path = format!("{}/assets/styles/{}/Note/Tap.toml",*ASSETS_PATH ,setting.ui_theme);
					},
					JudgeType::Slide => {
						path = format!("{}/assets/styles/{}/Note/Slide.toml", *ASSETS_PATH,setting.ui_theme);
					},
				}
				let note_json = read_file(&path)?;
				note_read = parse_toml(&note_json)?;
				if let Component::Shapo(mut nr) = note_read {
					for a in &mut nr {
						a.sustain_time = Some((self.start_time,self.click_time + FADE_TIME as u64));
						let offect = a.style.position;
						a.style.position = Vec2 { x: -100.0, y: -100.0 };
						if let StyleAnimate::Position(t) = &mut a.animation[0].style {
							t.points[0] = (self.start_position + offect).to_pos2();
							t.points[1] = (self.start_position + offect).to_pos2();
							t.points[2] = (self.final_position + offect).to_pos2();
							t.points[3] = (self.final_position + offect).to_pos2();
							a.animation[0].end_value = length;
						}
						if let None = self.click_time.checked_sub((length/setting.drop_velocity) as u64){
							a.animation[0].start_time = 0;
							a.animation[0].animate_time = (length /setting.drop_velocity) as u64 - self.click_time;
						}else {
							a.animation[0].start_time = self.click_time - (length/setting.drop_velocity) as u64;
							a.animation[0].animate_time = (length /setting.drop_velocity) as u64;
						}
					}
					self.shape = Some(nr);
				}
			}
			match self.judge_type {
				JudgeType::Tap => {
					if let Some(t) = setting.tap_prompt_color {
						if time_read > self.start_time {
							Shapo {
								style: Style {
									position: self.final_position,
									fill: t,
									..Style::default()
								},
								shape: Shape::Circle(Circle{radius: setting.note_prompt_radius}),
								..Default::default()
							}.render(ui, size, timer ,offect, if_paused,texture)?;
						}
					}
				},
				JudgeType::Slide => {
					if let Some(t) = setting.slide_prompt_color {
						if time_read > self.start_time {
							Shapo {
								style: Style {
									position: self.final_position,
									fill: t,
									..Style::default()
								},
								shape: Shape::Circle(Circle{radius: setting.note_prompt_radius}),
								..Default::default()
							}.render(ui, size, timer ,offect, if_paused,texture)?;
						}
					}
				}
			}
		}
		Ok(vec_back)
	}
}

impl Default for JudgeField {
	fn default() -> Self {
		Self {
			size: Vec2 { x: 0.0, y: 0.0 },
			position: Vec2 { x: 0.0, y: 0.0 },
			id: 1,
			animation: vec!(),
			rotate: 0.0,
			rotate_center: Vec2{ x: 0.0, y: 0.0 },
			key: Key::A,
			start_time: 0,
			end_time: 0,
			label: vec!(),
			if_delete: false,
		}
	}
}

impl Add for JudgeField {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		let mut animation = vec!();
		if other.animation.len() > self.animation.len() {
			for i in 0..self.animation.len() {
				animation.push(self.animation[i].clone() + other.animation[i].clone());
			}
			for i in self.animation.len()..other.animation.len() {
				animation.push(other.animation[i].clone())
			}
		}else {
			for i in 0..other.animation.len() {
				animation.push(self.animation[i].clone() + other.animation[i].clone());
			}
			for i in other.animation.len()..self.animation.len() {
				animation.push(self.animation[i].clone())
			}
		}

		Self {
			size: self.size + other.size,
			position: self.position + other.position,
			id: self.id,
			animation,
			rotate: self.rotate + other.rotate,
			rotate_center: self.rotate_center + other.rotate_center,
			key: self.key,
			start_time: self.start_time + other.start_time,
			end_time: self.end_time + other.end_time,
			label: self.label,
			if_delete: false,
		}
	}
}

impl Sub for JudgeField {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		let mut animation = vec!();
		if other.animation.len() > self.animation.len() {
			for i in 0..self.animation.len() {
				animation.push(self.animation[i].clone() - other.animation[i].clone());
			}
			for i in self.animation.len()..other.animation.len() {
				animation.push(other.animation[i].clone())
			}
		}else {
			for i in 0..other.animation.len() {
				animation.push(self.animation[i].clone() - other.animation[i].clone());
			}
			for i in other.animation.len()..self.animation.len() {
				animation.push(self.animation[i].clone())
			}
		}

		Self {
			size: self.size - other.size,
			position: self.position - other.position,
			id: self.id,
			animation,
			rotate: self.rotate - other.rotate,
			rotate_center: self.rotate_center - other.rotate_center,
			key: self.key,
			start_time: self.start_time - other.start_time,
			end_time: self.end_time - other.end_time,
			label: self.label,
			if_delete: false,
		}
	}
}

impl BitOr for JudgeField {
	type Output = Self;

	fn bitor(self, other: Self) -> Self::Output {
		let Self {
			mut size,
			mut position,
			mut id,
			mut animation,
			mut rotate,
			mut rotate_center,
			mut key,
			mut start_time,
			mut end_time,
			mut label, 
			mut if_delete } = self;

		if other.size != Self::default().size {
			size = other.size
		}
		if other.position != Self::default().position {
			position = other.position
		}
		if other.id != Self::default().id {
			id = other.id
		}
		if other.animation != Self::default().animation {
			animation = other.animation
		}
		if other.rotate != Self::default().rotate {
			rotate = other.rotate
		}
		if other.rotate_center != Self::default().rotate_center {
			rotate_center = other.rotate_center
		}
		if other.key != Self::default().key {
			key = other.key
		}
		if other.start_time != Self::default().start_time {
			start_time = other.start_time
		}
		if other.end_time != Self::default().end_time {
			end_time = other.end_time
		}
		if other.label != Self::default().label {
			label = other.label
		}
		if other.if_delete != Self::default().if_delete {
			if_delete = other.if_delete
		}

		Self {
			size,
			position,
			id,
			animation,
			rotate,
			rotate_center,
			key,
			start_time,
			end_time,
			label,
			if_delete,
		}
	}
}

/// Warning: Add trait for JudgeField is not exchangeable
impl JudgeField {
	fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, if_paused: bool, texture: &HashMap<TextureId,TextureHandle>, offect: Option<Vec2>) -> Result<(), ShapoError> {
		let time_read = timer[0].read()?;
		if time_read > self.start_time {
			for a in &self.animation {
				let x = time_read as f32 / a.animate_time as f32;
				let length = a.caculate(x)?;
				match &a.style {
					StyleAnimate::Position(t) => self.position = arc_length(length, &t)?,
					StyleAnimate::Size(t) => self.size = arc_length(length, &t)?,
					StyleAnimate::Rotate => self.rotate = length,
					StyleAnimate::RoutateCenter(t) => self.rotate_center = arc_length(length, &t)?,
					_ => unreachable!()
				}
			}
			let setting = read_settings()?;
			if let Some(t) = setting.judge_field_prompt_color {
				if time_read < self.end_time {
					Shapo {
						style: Style {
							position: self.position,
							rotate: self.rotate,
							fill: Color32::from_rgba_premultiplied(0,0,0,0),
							rotate_center: self.rotate_center,
							stroke: Stroke {
								width: setting.judge_field_prompt_size,
								color: t
							}, 
							..Style::default()
						},
						shape: Shape::Rectangle(Rectangle{
							bottom_right_point: self.size,
							rounding: Rounding::same(0.0),
							if_keep: false
						}),
						..Default::default()
					}.render(ui, size, timer ,offect, if_paused,texture)?;
				}
			}
		}
		Ok(())
	}
}

impl Judge {
	pub fn to_accuracy(&self) -> Result<f32,ShapoError> {
		let setting = read_settings()?;
		match self {
			Judge::Immaculate(t) => {
				if setting.if_immaculate {
					return Ok(*t)
				}else {
					return Ok(1.0)
				}
			}
			Judge::Extra => return Ok(0.7),
			Judge::Normal => return Ok(0.5),
			Judge::Fade => return Ok(0.0),
			Judge::Miss => return Ok(0.0),
			Judge::None => unreachable!(),
		}
	}

	pub fn to_color32(&self) -> Result<Color32, ShapoError> {
		let setting = read_settings()?;
		match self {
			Judge::Immaculate(_) => Ok(setting.immaculate_color),
			Judge::Extra => return Ok(setting.extra_color),
			Judge::Normal => return Ok(setting.normal_color),
			Judge::Fade => return Ok(setting.fade_color),
			Judge::Miss => return Ok(setting.miss_color),
			Judge::None => unreachable!(),
		}
	}
}

impl Default for Note {
	fn default() -> Self {
		Self {
			id: 0,
			judge_field_id: 0,
			shape: None,
			clicked_time: None,
			click_time: 0,
			start_time: 0,
			start_position: Vec2 {x: 0.0 , y: 0.0 },
			final_position: Vec2 {x: 0.0 , y: 0.0},
			judge_type: JudgeType::Tap,
			judge: Judge::None,
			label: vec!(),
			if_delete: false
		}
	}
}

impl Note {
	fn new_tap(id: usize, judge_field_id: usize ,click_time: u64, start_time: u64, label: Vec<String>) -> Self {
		Self {
			id,
			judge_field_id,
			shape: None,
			clicked_time: None,
			click_time,
			start_time,
			start_position: Vec2 {x: 50.0 , y: 0.0 },
			final_position: Vec2 {x: 50.0 , y: 80.0},
			judge_type: JudgeType::Tap,
			judge: Judge::None,
			label,
			if_delete: false
		}
	}

	fn new_slide(id: usize,judge_field_id: usize , click_time: u64, start_time: u64, label: Vec<String>) -> Self {
		Self {
			id,
			judge_field_id,
			shape: None,
			clicked_time: None,
			click_time,
			start_time,
			start_position: Vec2 {x: 50.0 , y: 0.0 },
			final_position: Vec2 {x: 50.0 , y: 80.0},
			judge_type: JudgeType::Slide,
			judge: Judge::None,
			label,
			if_delete: false
		}
	}
}

/// Warning: Add trait for Note is not exchangeable
impl Add for Note {

	type Output = Self;

	fn add(self, other: Note) -> Self::Output { 
		let shape;

		if other.shape.is_none() || self.shape.is_none() {
			shape = self.shape;
		}else {
			let shorter_is_self;

			if self.shape.clone().unwrap().len() >= other.shape.clone().unwrap().len() {
				shorter_is_self = false;
			}else {
				shorter_is_self = true;
			}
			if shorter_is_self {
				let mut out_shape = vec!();
				for a in other.shape.unwrap() {
					let mut c = a.clone();
					for b in self.shape.clone().unwrap() {
						if b.label.clone()[0] == a.label.clone()[0] {
							c = b + a.clone();
						}
					}
					out_shape.push(c);
				}
				shape = Some(out_shape);
			}else {
				let mut out_shape = vec!();
				for a in self.shape.unwrap() {
					let mut c = a.clone();
					for b in other.shape.clone().unwrap() {
						if b.label.clone()[0] == a.label.clone()[0] {
							c = a.clone() + b;
						}
					}
					out_shape.push(c);
				}
				shape = Some(out_shape);
			}
		}

		Self {
			id: self.id,
			judge_field_id: self.id,
			shape,
			clicked_time: self.clicked_time,
			click_time: self.click_time + other.click_time,
			start_time: self.start_time + other.start_time,
			start_position: self.start_position + other.start_position,
			final_position: self.final_position + other.final_position,
			judge_type: self.judge_type,
			judge: Judge::None,
			label: self.label,
			if_delete: other.if_delete,
		}
	}
}

impl Sub for Note {

	type Output = Self;

	fn sub(self, other: Note) -> Self::Output { 
		let shape;

		if other.shape.is_none() || self.shape.is_none() {
			shape = self.shape;
		}else {
			let shorter_is_self;

			if self.shape.clone().unwrap().len() >= other.shape.clone().unwrap().len() {
				shorter_is_self = false;
			}else {
				shorter_is_self = true;
			}
			if shorter_is_self {
				let mut out_shape = vec!();
				for a in other.shape.unwrap() {
					let mut c = a.clone();
					for b in self.shape.clone().unwrap() {
						if b.label.clone()[0] == a.label.clone()[0] {
							c = b - a.clone();
						}
					}
					out_shape.push(c);
				}
				shape = Some(out_shape);
			}else {
				let mut out_shape = vec!();
				for a in self.shape.unwrap() {
					let mut c = a.clone();
					for b in other.shape.clone().unwrap() {
						if b.label.clone()[0] == a.label.clone()[0] {
							c = a.clone() - b;
						}
					}
					out_shape.push(c);
				}
				shape = Some(out_shape);
			}
		}

		Self {
			id: self.id,
			judge_field_id: self.id,
			shape,
			clicked_time: self.clicked_time,
			click_time: self.click_time - other.click_time,
			start_time: self.start_time - other.start_time,
			start_position: self.start_position - other.start_position,
			final_position: self.final_position - other.final_position,
			judge_type: self.judge_type,
			judge: Judge::None,
			label: self.label,
			if_delete: self.if_delete,
		}
	}
}

impl BitOr for Note {

	type Output = Self;

	fn bitor(self, other: Note) -> Self::Output { 
		let Self { mut id, mut judge_field_id, mut shape, mut clicked_time, mut click_time,
			mut start_time, mut start_position, mut final_position, mut judge_type,
			mut judge, mut label, mut if_delete } = self;

		if other.id != Note::default().id {
			id = other.id
		}
		if other.judge_field_id != Note::default().judge_field_id {
			judge_field_id = other.judge_field_id
		}
		if other.shape != Note::default().shape {
			shape = other.shape
		}
		if other.click_time != Note::default().click_time {
			click_time = other.click_time
		}
		if other.clicked_time != Note::default().clicked_time {
			clicked_time = other.clicked_time
		}
		if other.start_time != Note::default().start_time {
			start_time = other.start_time
		}
		if other.start_position != Note::default().start_position {
			start_position = other.start_position
		}
		if other.final_position != Note::default().final_position {
			final_position = other.final_position
		}
		if other.judge_type != Note::default().judge_type {
			judge_type = other.judge_type
		}
		if other.judge != Note::default().judge {
			judge = other.judge
		}
		if other.label != Note::default().label {
			label = other.label
		}
		if other.if_delete != Note::default().if_delete {
			if_delete = other.if_delete
		}

		Self {
			id,
			judge_field_id,
			shape,
			clicked_time,
			click_time,
			start_time,
			start_position,
			final_position,
			judge_type,
			judge,
			label,
			if_delete,
		}
	}
}

fn bezier_curve_sqrt(t: f32, bezier_curve: &CubicBezier) -> f32 {
	let a = bezier_curve.points[0].x;
	let b = bezier_curve.points[1].x;
	let c = bezier_curve.points[2].x;
	let d = bezier_curve.points[3].x;
	let e = bezier_curve.points[0].y;
	let f = bezier_curve.points[1].y;
	let g = bezier_curve.points[2].y;
	let h = bezier_curve.points[3].y;
	let back = f32::sqrt(((-3.0 * a - 3.0 * e) * (-1.0 + t) * (-1.0 + t)  + (3.0 * d + 3.0 * h) * t * t + (c + g) * (6.0 * t - 9.0 * t * t) + (b + f) * (3.0 - 12.0 * t + 9.0 * t * t)).abs());
	back
}

fn simpsons_rule_integration(b: f32, bezier_curve: &CubicBezier) -> f32 {
	b / 8.0 * (bezier_curve_sqrt(0.0, &bezier_curve) + 
	3.0 * bezier_curve_sqrt(b / 3.0, &bezier_curve) + 
	3.0 * bezier_curve_sqrt(2.0 * b / 3.0, &bezier_curve) + 
	bezier_curve_sqrt(b, &bezier_curve))
}
