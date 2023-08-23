use crate::ASSETS_PATH;
use crate::language::language::Language;
use crate::create_file;
use crate::system::system_function::remove_file;
use crate::system::system_function::to_json;
use crate::system::system_function::write_file;
use crate::system::system_function::prase_json_form_path;
use crate::system::system_function::prase_json;
use crate::ui::ui::Router;
use std::collections::HashMap;
use egui::TextureId;
use egui::TextureHandle;
use crate::ui::shapo::Shapo;
use crate::setting::setting::read_settings;
use egui::Vec2;
use crate::ui::ui::Back;
use crate::ShapoError;
use crate::system::system_function::read_file;
use crate::play::note::*;
use crate::play::timer::Timer;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Replay {
	clicks: Vec<Click>
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Click {
	pub click_time: Option<u128>,
	pub note_position: Vec2
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlayInfo {
	pub score: usize,
	pub accuracy: f32,
	pub immaculate_number: usize,
	pub extra_number: usize,
	pub normal_number: usize,
	pub fade_number: usize,
	pub miss_number: usize,
	pub max_combo: usize,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlayTop {
	pub timer: Timer,
	pub chart: Chart,
	pub score: usize,
	pub max_combo: usize,
	pub combo: usize,
	pub accuracy: f32,
	pub judge_vec: Vec<Judge>,
	pub replay: Replay,
	pub if_paused: bool,
	pub path: String,
	pub current_time: u128,
	pub touch: HashMap<u64, Touch>
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Touch {
	pub time: usize,
	pub position: Vec2,
	pub if_click: bool
}

impl Touch {
	fn update(&mut self) {
		self.time = self.time + 1;
		if self.time > 10 {
			self.if_click = false;
		}
	}
}

impl Default for PlayInfo {
	fn default() -> Self {
		Self {
			score: 0,
			accuracy: 0.0,
			immaculate_number: 0,
			extra_number: 0,
			normal_number: 0,
			fade_number: 0,
			miss_number: 0,
			max_combo: 0,
		}
	}
}

impl PlayInfo {
	pub fn read(mapname: String) -> Self {
		match prase_json_form_path(&format!("{}/assets/chart/{}/play.info",*ASSETS_PATH , mapname)) {
			Ok(t) => return t,
			Err(_) => return Self::default(),
		}
	}

	pub fn write(&self, mapname: String) -> Result<i32,ShapoError> {
		let mut read = Self::read(mapname.clone());
		let delta = self.score as i32 - read.score as i32;
		if self.score > read.score {
			read.score = self.score
		}else if self.accuracy > read.accuracy {
			read.accuracy = self.accuracy
		}

		let _ = remove_file(&format!("{}/assets/chart/{}/play.info",*ASSETS_PATH , mapname));
		create_file(&format!("{}/assets/chart/{}/play.info",*ASSETS_PATH , mapname))?;
		write_file(&format!("{}/assets/chart/{}/play.info",*ASSETS_PATH , mapname), &to_json(&read)?)?;
		Ok(delta)
	}
}

impl PlayTop {
	pub fn default() -> Result<Self,ShapoError> {
		let mut timer = Timer::new(1);
		timer.start()?;
		Ok(Self {
			timer,
			chart: Chart::default(),
			score: 0,
			max_combo: 0,
			combo: 0,
			accuracy: 1.00,
			judge_vec: vec!(),
			replay: Replay::new(),
			if_paused: false,
			path: String::new(),
			current_time: 0,
			touch: HashMap::new()
		})
	}

	pub fn get_info(&self) -> PlayInfo {
		let mut immaculate_number = 0;
		let mut extra_number = 0;
		let mut normal_number = 0;
		let mut fade_number = 0;
		let mut miss_number= 0;
		for judge in &self.judge_vec {
			match judge {
				Judge::Immaculate(_) => immaculate_number = immaculate_number + 1,
				Judge::Extra => extra_number = extra_number + 1,
				Judge::Normal => normal_number = normal_number + 1,
				Judge::Fade => fade_number = fade_number + 1,
				Judge::Miss => miss_number = miss_number + 1,
				Judge::None => unreachable!(),
			}
		}
		PlayInfo {
			score: self.score,
			accuracy: self.accuracy,
			immaculate_number,
			extra_number,
			normal_number,
			fade_number,
			miss_number,
			max_combo: self.max_combo
		}
	}

	pub fn timer(&mut self) -> Result<(),ShapoError> {
		if self.if_paused {
			let mut timer = Timer::new(1);
			timer.start()?;
			timer.set(self.current_time)?;
			self.timer = timer;
		}else {
			self.current_time = self.timer.read()?;
		}
		Ok(())
	}

	pub fn read(path: &String) -> Result<Self, ShapoError> {
		let chart:Chart = prase_json_form_path(&path)?;
		let mut timer = Timer::new(1);
		timer.start()?;
		Ok(Self {
			timer,
			chart,
			score: 0,
			max_combo: 0,
			combo: 0,
			replay: Replay::new(),
			accuracy: 1.00,
			judge_vec: vec!(),
			if_paused: false,
			path: path.to_string(),
			current_time: 0,
			touch: HashMap::new()
		})
	}

	pub fn play(&mut self) -> Result<Back,ShapoError> {
		// self.timer.start()?;
		let delta = match self.current_time.checked_sub(3 * 1e6 as u128) {
			Some(t) => t,
			None => 0,
		};
		self.current_time = delta;
		self.timer()?;
		// self.timer.set(3 * 1e6 as u128)?;
		self.if_paused = false;
		Ok(Back::Play)
	}

	pub fn pause(&mut self) -> Result<Back,ShapoError> {
		// self.timer.pause()?;
		self.if_paused = true;
		Ok(Back::Pause)
	}

	pub fn retry(&mut self) -> Result<Back,ShapoError> {
		let path = self.path.clone();
		*self = Self::read(&path)?;
		Ok(Back::Retry)
	}

	pub fn update(&mut self, judge: Judge) -> Result<(),ShapoError> {
		let setting = read_settings()?;
		let max_note = self.chart.max_note();
		if let Judge::Miss = judge {
			self.combo = 0;
		}else if let Judge::Fade = judge {
			self.combo = 0;
		}else {
			self.combo = self.combo + 1;
		}
		self.judge_vec.push(judge);
		self.caculate_accuracy()?;
		if self.combo > self.max_combo {
			self.max_combo = self.combo;
		}
		if setting.if_immaculate {
			self.score = ((((self.judge_vec.len() as f32) * self.accuracy / (max_note as f32)) * 0.95 + self.max_combo as f32 / max_note as f32 * 0.05) * 2.0 * 1e6) as usize
		}else {
			self.score = ((((self.judge_vec.len() as f32) * self.accuracy / (max_note as f32)) * 0.95 + self.max_combo as f32 / max_note as f32 * 0.05) * 1e6) as usize
		}
		Ok(())
	}

	pub fn caculate_accuracy(&mut self) -> Result<(),ShapoError> {
		let mut back = 0.0;
		for a in &self.judge_vec {
			back = back + a.to_accuracy()?;
		}
		back = back / self.judge_vec.len() as f32;
		self.accuracy = back;
		Ok(())
	}

	pub fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, texture: &HashMap<TextureId,TextureHandle>, if_normal: bool) -> Result<Vec<Back>,ShapoError> {
		self.timer()?;
		let mut back_vec = Vec::new();
		if !if_normal {
			return Ok(back_vec)
		}
		let mut input = ui.input_mut(|input| input.clone());

		for (_, touch) in &mut self.touch {
			touch.update();
		}

		for event in &input.events {
			if let egui::Event::Touch { device_id: _, id, phase, pos, force: _ } = event {
				let mut pos = pos.to_vec2();
				pos.x = pos.x / size.x * 100.0;
				pos.y = pos.y / size.y * 100.0;
				match phase {
					egui::TouchPhase::Start => {
						self.touch.insert(id.0, Touch{time: 0,position: pos,if_click: true});
					},
					egui::TouchPhase::Move => {
						if let Some(t) = self.touch.get_mut(&id.0){
							t.position = pos;
						};
					},
					_ => {
						self.touch.remove(&id.0);
					}
				};
			}
		}
		
		if input.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
			if self.if_paused {
				self.play()?;
				back_vec.push(Back::Play);
			}else {
				self.pause()?;
				back_vec.push(Back::Pause);
			}	
		}
		for a in self.chart.render(ui, size,&mut self.timer, !self.if_paused, texture, None, &mut self.touch)? {
			match a {
				Back::AnimateDone(_) => {},
				Back::JudgeNote(t) => {
					for (judge_id, note_id) in t {
						let setting = read_settings()?;
						let read = &self.chart.note.get(&judge_id).unwrap()[note_id];
						let click_effect_json = read_file(&format!("{}/assets/styles/{}/Note/ClickEffect.json",*ASSETS_PATH ,setting.ui_theme))?;
						let mut click_effect_read: Vec<Shapo>;
						click_effect_read = prase_json(&click_effect_json)?;
						for a in &mut click_effect_read {
							a.style.position = match read.shape.clone() {
								Some(t) => t,
								None => return Err(ShapoError::SystemError(format!("????????????"))),
							}[0].style.position;
							a.style.stroke.color = read.judge.to_color32()?;
							let time = match read.clicked_time {
								Some(t) => t,
								None => 0
							};
							a.animation[1].start_time = Some(time + a.animation[0].animate_time);
							a.animation[0].start_time = read.clicked_time;
						}
						for a in click_effect_read {
							self.chart.shape.push(a);
						}
						if let Judge::Miss = read.judge.clone(){}
						else {
							match read.clone().judge_type {
								JudgeType::Tap => back_vec.push(Back::PlaySound(format!("{}/assets/styles/{}/Sound/TapClickSound.mp3",*ASSETS_PATH ,setting.ui_theme))),
								JudgeType::Slide=> back_vec.push(Back::PlaySound(format!("{}/assets/styles/{}/Sound/SlideClickSound.mp3",*ASSETS_PATH ,setting.ui_theme))),
							}
						}
						self.replay.clicks.push(Click{click_time: read.clicked_time, note_position: read.shape.clone().unwrap()[0].style.position});
						self.update(read.judge.clone())?;
					}
				},
				Back::Router(r) => {
					if let Router::EndPage(_) = r {
						back_vec.push(Back::Router(Router::EndPage(self.clone())))
					}else {
						back_vec.push(Back::Router(r))
					}
				},
				Back::Retry => {
					back_vec.push(self.retry()?);
				},
				Back::Play => {
					self.play()?;
					back_vec.push(Back::Play)
				},
				Back::Pause => {
					self.pause()?;
					back_vec.push(Back::Pause)
				},
				_ => back_vec.push(a)
			}
		}

		if ui.add(egui::Label::new(egui::RichText::new(format!("{}",self.score)).size(20.0)).sense(egui::Sense::click())).double_clicked() {
			self.pause()?;
			back_vec.push(Back::Pause);
		};
		ui.label(format!("{}{:.2}%", Language::Code(143).get_language()?,self.accuracy * 100.0));
		ui.label(format!("{}{}",Language::Code(148).get_language()?,self.combo));
		ui.label(format!("{}{:.2}s",Language::Code(149).get_language()?,self.timer.read()? as f64 / 1e6));
		Ok(back_vec)
	}
}

impl Replay {
	fn new() -> Self {
		Self {
			clicks: vec!()
		}
	}
}