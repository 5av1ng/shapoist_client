use crate::ASSETS_PATH;
use kira::clock::ClockSpeed;
use kira::clock::ClockHandle;
use std::collections::BTreeMap;
use crate::system::system_function::prase_json_form_path;
use crate::system::system_function::remove_path;
use crate::system::system_function::copy_file;
use crate::system::system_function::to_json;
use crate::system::system_function::create_dir;
use crate::play::note::Project;
use crate::play::note::PossibleChartChange;
use crate::setting::setting::read_settings;
use crate::system::system_function::prase_json;
use crate::play::note::Chart;
use egui::DroppedFile;
use crate::create_file;
use crate::system::system_function::remove_file;
use crate::system::system_function::write_file;
use kira::tween::Tween;
use std::time::Duration;
use std::collections::HashMap;
use egui::TextureId;
use egui::TextureHandle;
use crate::system::system_function::load_sound;
use kira::manager::backend::DefaultBackend;
use kira::manager::AudioManagerSettings;
use kira::manager::AudioManager;
use crate::play::play_top::PlayTop;
use crate::ShapoError;
use crate::ui::component::window::Window;
use egui::Label;
use egui::Rect;
use crate::log_export::log_export::*;
use egui::Vec2;
use egui::FontDefinitions;
use egui::FontData;
use egui::FontFamily;
use crate::play::timer::Timer;
use crate::ui::ui::*;
use egui::Visuals;
use eframe::App;
use egui::CentralPanel;
use egui::Key;

pub struct Page {
	router: Router,
	display: Display,
	condition: Condition,
	music_manager: AudioManager,
	texture: HashMap<TextureId,TextureHandle>,
	file: Vec<DroppedFile>,
	if_mute: bool,
	temp: Temp,
	clock_handle: Vec<ClockHandle>
}

pub struct Temp {
	pub chart: Chart,
	pub chart_undo: Vec<(PossibleChartChange,Chart)>,
	pub project: Project,
	pub music_path: String,
	pub image_path: String,
	pub now_project_path: String,
	pub undo_times: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
enum Condition{
	Normal,
	Debug(Status)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Status {
	if_inspection: bool,
	if_ui_test_window: bool
}

impl Default for Status {
	fn default() -> Self {
		Self {
			if_inspection: false,
			if_ui_test_window: false,
		}
	}
}

impl Default for Temp {
	fn default() -> Self {
		let mut chart = Chart::default();
		chart.note = BTreeMap::new();
		chart.now_judge = BTreeMap::new();
		chart.shape = vec!();
		chart.judge_field = vec!();
		Self {
			chart,
			chart_undo: vec!(),
			music_path: String::new(),
			image_path: String::new(),
			project: Project::default(),
			now_project_path: String::new(),
			undo_times: 0,
		}
	}
}

impl Page {
	pub fn default() -> Result<Self, ShapoError> {
		let mut music_manager = match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
			Ok(t) => t,
			Err(e) => return Err(ShapoError::SystemError(e.to_string()))
		};
		let clock_handle = vec!(match music_manager.add_clock(ClockSpeed::TicksPerMinute(150.0)) {
			Ok(t) => t,
			Err(e) => return Err(ShapoError::SystemError(e.to_string()))
		});
		Ok(Self {
			router: Router::MainPage,
			display: Display::default(),
			condition: Condition::Normal,
			texture: HashMap::new(),
			if_mute: false,
			file: vec!(),
			music_manager,
			temp: Temp::default(),
			clock_handle,
		})
	}

	pub fn new(cc: &eframe::CreationContext<'_>, args: Vec<String>) -> Result<Self, ShapoError> {
		let mut fonts = FontDefinitions::default();
		fonts.font_data.insert("NeoXiHei".to_owned(),
			FontData::from_static(include_bytes!("../../font.ttf")));
		fonts.families.get_mut(&FontFamily::Proportional).unwrap()
			.insert(0, "NeoXiHei".to_owned());
		fonts.families.get_mut(&FontFamily::Monospace).unwrap()
			.insert(0, "NeoXiHei".to_owned());
		cc.egui_ctx.set_fonts(fonts);
		let mut return_self: Self = Self::default()?;
		for a in args {
			if a == "debug".to_string() {
				print_log("Running in Debug Mod.");
				return_self.condition = Condition::Debug(Status::default());
			}else if a == "mute".to_string() {
				print_log("Running with Mute Mod");
				return_self.if_mute = true;
			}
		}
		Ok(return_self)
	}

	fn push(&mut self, router:Router) {
		self.router = router;
		self.display = Display::default();
		self.texture.clear();
	}

	fn close_window(&mut self, id: usize) {
		let mut window_after = Vec::new();
		for a in &self.display.window {
			if a.id != id {
				window_after.push(a.clone());
			} 
		}
		self.display.window = window_after;
	}

	fn debug_functions(&mut self, ui: &egui::Ui, frame: &mut eframe::Frame) {
		ui.input_mut(|input| {
			let consume = egui::Modifiers::CTRL | egui::Modifiers::ALT;
			if input.consume_shortcut(&egui::KeyboardShortcut::new(consume, Key::Z)) {
				self.display.if_normal = true;
				self.push(Router::MainPage);
			}else if input.consume_shortcut(&egui::KeyboardShortcut::new(consume, Key::V)) {
				self.display.if_normal = true;
				self.router = Router::PlayPage(String::from("TEST"));
				self.display.component = Some(vec!());
				self.display.window = vec!();
				self.display.play_top = Some(PlayTop::default().unwrap());
				self.stop_sound().unwrap();
			}else if input.consume_shortcut(&egui::KeyboardShortcut::new(consume, Key::T)) {
				self.display.if_normal = true;
				self.push(Router::TestPage);
			}else if input.consume_shortcut(&egui::KeyboardShortcut::new(consume, Key::P)) {
				self.display.if_normal = true;
				self.push(Router::TestPage);
			}else if input.consume_shortcut(&egui::KeyboardShortcut::new(consume, Key::L)) {
				self.display.window = vec!();
			}else if input.consume_shortcut(&egui::KeyboardShortcut::new(consume, Key::I)) {
				if let Condition::Debug(t) = &mut self.condition {
					t.if_inspection = !t.if_inspection;
				}
			}else if input.consume_shortcut(&egui::KeyboardShortcut::new(consume, Key::K)) {
				if let Condition::Debug(t) = &mut self.condition {
					if !t.if_ui_test_window {
						self.handle_back(&Back::OpenWindow(Window::test()), frame).unwrap();
					}else {
						self.handle_back(&Back::CloseWindow(999999), frame).unwrap();
					}
				}
				if let Condition::Debug(t) = &mut self.condition {
					t.if_ui_test_window = !t.if_ui_test_window;
				}
			}
		})
	}

	fn play_music(&mut self, path: String, bpm: f32, beatnumber: f32, offect: f32) -> Result<(),ShapoError> {
		if !self.if_mute {
			let music_manager = match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
				Ok(t) => t,
				Err(e) => return Err(ShapoError::SystemError(e.to_string()))
			};
			self.music_manager = music_manager;
			self.clock_handle = vec!(load_sound(&path, Some(0.0..), bpm, beatnumber, offect, &mut self.music_manager)?);
		}
		Ok(())
	}

	fn play_sound(&mut self, path: String) -> Result<(),ShapoError> {
		if !self.if_mute {
			if self.clock_handle.len() > 1 {
				self.clock_handle[1] = load_sound(&path, None, -1.0, 0.0, 0.0, &mut self.music_manager)?;
			}else {
				self.clock_handle.push(load_sound(&path, None, -1.0, 0.0, 0.0, &mut self.music_manager)?);
			}	
		}
		Ok(())
	}

	fn stop_sound(&mut self) -> Result<(),ShapoError> {
		if !self.if_mute {
			match self.music_manager.pause(Tween {
				duration: Duration::from_secs(2),
				..Default::default()
			}) {
				Ok(_) => return Ok(()),
				Err(e) => {
					print_log(&format!("[ERROR] failed to stop sound. info: {}",e.to_string()));
					return Err(ShapoError::SystemError(e.to_string()))
				}
			}
		}
		Ok(())
	}

	fn handle_back(&mut self, back: &Back, frame: &mut eframe::Frame) -> Result<(), ShapoError> {
		if let Back::Nothing | Back::Error(_) | Back::Router(Router::EndPage(_)) | Back::Change(_,_) = back {}
		else { print_log(&format!("got back: {}", back.to_string())) };
		match back {
			Back::Nothing => {},
			Back::Router(t) => {
				self.push(t.clone())
			},
			Back::Animation(t) => {
				let mut timer = Timer::new(*t);
				let mut if_timer_exist = false;
				if let Err(_) = timer.start() {
					unreachable!()
				}
				for a in &mut self.display.timer {
					if a.id == *t {
						*a = timer;
						if_timer_exist = true
					}
				}
				if !if_timer_exist {
					self.display.timer.push(timer);
				}
			},
			Back::AnimateDone(t) => {
				let mut back_vec = Vec::new();
				for a in &self.display.timer {
					if a.id != *t {
						back_vec.push(*a)
					}
				}
				self.display.timer = back_vec;
			},
			Back::OpenWindow(t) => {
				let mut judge:bool = true;
				for a in 0..self.display.window.len() {
					if self.display.window[a].id == t.id {
						self.display.window[a] = t.clone();
						judge = false
					}
				}
				if judge {
					self.display.window.push(t.clone());
				}
			},
			Back::CloseWindow(t) => {
				self.close_window(*t);
			},
			Back::Close => {
				if self.display.if_normal {
					print_log("user closed.");
				}else {
					print_log("user closed due to previous error.");
				}
				frame.close();
			},
			Back::Error(e) => {
				print_log(&format!("[ERROR] {:#?}", e));
				self.display.if_normal = false;
			},
			Back::Ignore => {
				self.display.if_normal = true;
				for a in &mut self.display.window {
					a.if_enabled = true;
				}
				self.close_window(0);
			},
			Back::Pause => {
				let t = Window::pause_window()?; 
				self.stop_sound()?;
				let mut judge:bool = true;
				for a in 0..self.display.window.len() {
					if self.display.window[a].id == t.id {
						self.display.window[a] = t.clone();
						judge = false
					}
				}
				if judge {
					self.display.window.push(t.clone());
				}
			}
			Back::Play => {
				if let Some(t) = &mut self.display.play_top {
					t.play()?;
					if !(t.current_time.checked_sub(3 * 1e6 as u128)).is_some() {
						t.chart.if_playing = false;
					}
				}
				if let Some(t) = &self.display.play_top {
					if (t.current_time.checked_sub(3 * 1e6 as u128)).is_some() {
						self.play_music(format!("{}/assets/chart/{}/song.mp3",*ASSETS_PATH , t.chart.mapname), t.chart.bpm, 0.0, (t.chart.offect as f32 - t.current_time as f32 + 3.0 * 1e6) as f32 / 1e6)?;
					}
				}
				self.close_window(1004);
			},
			Back::JudgeNote(_) => unreachable!(),
			Back::MusicPlay(t, bpm, beatnumber, offect) => {
				self.play_music(t.to_string(), *bpm, *beatnumber, *offect)?;
			},
			Back::LoadedTexture((id, handle)) => {
				self.texture.insert(*id, handle.clone());
			},
			Back::PlaySound(t) => {
				self.play_sound(t.to_string())?;
			},
			Back::Retry => {
				if let Some(t) = &mut self.display.play_top {
					t.retry()?;
				}
				self.close_window(1004);
			},
			Back::PauseSound => self.stop_sound()?,
			Back::Change(type_to_change,json) => {
				match type_to_change {
					ChangeType::Setting(_) => {
						remove_file(&format!("{}/assets/setting.json", *ASSETS_PATH))?;
						create_file(&format!("{}/assets/setting.json", *ASSETS_PATH))?;
						write_file(&format!("{}/assets/setting.json", *ASSETS_PATH),json)?;
					},
					ChangeType::ChartTemp(t) => {
						let setting = read_settings()?;
						if self.temp.undo_times != 0{
							let mut new_chart_undo = vec!();
							for a in 1..self.temp.chart_undo.len() - self.temp.undo_times {
								new_chart_undo.push(self.temp.chart_undo[a].clone())
							}
							self.temp.chart_undo = new_chart_undo;
						}
						if self.temp.chart_undo.len() >= setting.undo_steps {
							let mut new_chart_undo = vec!();
							for a in 1..self.temp.chart_undo.len() {
								new_chart_undo.push(self.temp.chart_undo[a].clone())
							}
							self.temp.chart_undo = new_chart_undo;
						}
						let length = self.temp.chart_undo.len();
						if length < 1 {
							self.temp.chart_undo.push((t.clone(),self.temp.chart.clone()));
						}else {
							let (change_type,_) = &self.temp.chart_undo[length - 1];
							if change_type == &t.clone() {
								self.temp.chart_undo[length - 1] = (t.clone(),self.temp.chart.clone());
							}else {
								self.temp.chart_undo.push((t.clone(),self.temp.chart.clone()));
							}
						}
						self.temp.chart = prase_json(&json)?;
						self.temp.chart.length_normallize();
						self.temp.undo_times = 0;
						self.temp.project.chart = self.temp.chart.clone();
					},
					ChangeType::MuiscPath => {
						self.temp.music_path = prase_json(&json)?;
					},
					ChangeType::ImagePath => {
						self.temp.image_path = prase_json(&json)?;
					},
					ChangeType::ProjectPath => {
						self.temp.now_project_path = prase_json(&json)?;
					},
				}
			},
			Back::NewProject => {
				if self.temp.chart.is_empty() || self.temp.music_path.is_empty() || self.temp.image_path.is_empty() {
					return Err(ShapoError::SystemError(format!("invailed chart info input")));
				}
				self.temp.project = Project::from_chart(self.temp.chart.clone());
				create_dir(&format!("{}/assets/chart/{}",*ASSETS_PATH ,self.temp.chart.mapname))?;
				fn error(temp: &Temp) -> Result<(),ShapoError> {
					let split:Vec<&str> = temp.music_path.split("\\\\").collect(); 
					let mut actual_music_path = String::new();
					for a in split {
						actual_music_path = actual_music_path + "/" + a
					}
					actual_music_path = actual_music_path[2..actual_music_path.len()-1].to_string();
					let split:Vec<&str> = temp.image_path.split("\\\\").collect(); 
					let mut actual_image_path = String::new();
					for a in split {
						actual_image_path = actual_image_path + "/" + a
					} 
					actual_image_path = actual_image_path[2..actual_image_path.len()-1].to_string();
					create_file(&format!("{}/assets/chart/{}/map.shapoistmap",*ASSETS_PATH ,temp.chart.mapname))?;
					write_file(&format!("{}/assets/chart/{}/map.shapoistmap", *ASSETS_PATH,temp.chart.mapname), &to_json(&temp.chart)?)?;
					create_file(&format!("{}/assets/chart/{}/map.shapoistproject",*ASSETS_PATH ,temp.chart.mapname))?;
					write_file(&format!("{}/assets/chart/{}/map.shapoistproject",*ASSETS_PATH,temp.chart.mapname), &to_json(&temp.project)?)?;
					copy_file(&actual_music_path, &format!("{}/assets/chart/{}/song.mp3",*ASSETS_PATH,temp.chart.mapname))?;
					copy_file(&actual_image_path, &format!("{}/assets/chart/{}/image.png", *ASSETS_PATH,temp.chart.mapname))?;
					Ok(())
				}
				match error(&self.temp) {
					Ok(_) => {},
					Err(e) => {
						remove_path(&format!("{}/assets/chart/{}",*ASSETS_PATH,self.temp.chart.mapname))?;
						return Err(e);
					}
				}
				self.temp.now_project_path = format!("{}/assets/chart/{}",*ASSETS_PATH,self.temp.chart.mapname);
				self.stop_sound()?;
				self.push(Router::EditPage);
			},
			Back::OpenProject => {
				if self.temp.now_project_path.is_empty() {
					return Err(ShapoError::SystemError(format!("invailed chart info input")));
				}
				let split:Vec<&str> = self.temp.now_project_path.split("\\\\").collect(); 
				let mut actual_project_path = String::new();
				for a in split {
					actual_project_path = actual_project_path + "/" + a
				}
				actual_project_path = actual_project_path[2..actual_project_path.len()-1].to_string();
				self.temp.chart = prase_json_form_path(&format!("{}/map.shapoistmap", actual_project_path))?;
				self.temp.project = match prase_json_form_path(&format!("{}/map.shapoistproject", actual_project_path)) {
					Ok(t) => t,
					Err(_) => Project::from_chart(self.temp.chart.clone()),
				};
				self.stop_sound()?;
				self.push(Router::EditPage);
			},
			Back::Save => {
				if self.temp.now_project_path.is_empty() {
					return Err(ShapoError::SystemError(format!("invailed chart info input")));
				}
				let split:Vec<&str> = self.temp.now_project_path.split("\\\\").collect(); 
				let mut actual_project_path = String::new();
				for a in split {
					actual_project_path = actual_project_path + "/" + a
				}
				actual_project_path = actual_project_path[2..actual_project_path.len()-1].to_string();
				remove_file(&format!("{}/map.shapoistmap",actual_project_path))?;
				remove_file(&format!("{}/map.shapoistproject",actual_project_path))?;
				create_file(&format!("{}/map.shapoistmap",actual_project_path))?;
				create_file(&format!("{}/map.shapoistproject",actual_project_path))?;
				write_file(&format!("{}/map.shapoistmap",actual_project_path), &to_json(&self.temp.chart)?)?;
				write_file(&format!("{}/map.shapoistproject",actual_project_path), &to_json(&self.temp.project)?)?;
			},
			Back::Undo => {
				let length = self.temp.chart_undo.len();
				if self.temp.undo_times >= length {
					return Err(ShapoError::SystemError(format!("nothing undo")));
				}
				let (_, chart) = &self.temp.chart_undo[self.temp.undo_times];
				self.temp.chart = chart.clone();
				self.temp.project.chart = chart.clone();
				self.temp.undo_times = self.temp.undo_times + 1;
				self.temp.project.window = HashMap::new();
			},
			Back::Redo => {
				if self.temp.undo_times == 0 {
					return Err(ShapoError::SystemError(format!("nothing redo")));
				}
				let (_, chart) = &self.temp.chart_undo[self.temp.undo_times - 1];
				self.temp.chart = chart.clone();
				self.temp.project.chart = chart.clone();
				self.temp.undo_times = self.temp.undo_times - 1;
			},
			Back::Export => {
				export(&self.temp)?;
			},
		}
		Ok(())
	}
}

impl App for Page {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
		ctx.set_visuals(Visuals::dark());
		CentralPanel::default().show(ctx, |ui|{
			ui.input(|input| {
				let backup = self.file.clone();
				if !input.raw.dropped_files.is_empty() {
					for a in &input.raw.dropped_files {
						self.file.push(a.clone());
					}
				}
				if backup != self.file {
					self.display.if_normal = true;
					self.display = Display::default();
				}
			});
			let router = &self.router;
			let available_size = ui.available_size() + Vec2{ x: 16.0, y: 16.0 };
			let back = match self.display.render(ui, router, &available_size, ctx, &self.texture, &self.file, &mut self.temp){
				Ok(t) => t,
				Err(e) => {
					let mut if_model_exist = false;
					for a in &mut self.display.window {
						if a.id == 0 {
							*a = Window::error_model(format!("{:#?}",e), ui);
							if_model_exist = true;
						}else {
							a.if_enabled = false;
						}
					}
					if !if_model_exist {
						self.display.window.push(Window::error_model(format!("{:#?}",e),ui));
					}
					vec!(Back::Error(e))
				}
			};
			for a in back { 
				if let Err(e) = self.handle_back(&a, frame) {
					print_log(&format!("[ERROR] {:#?}", e));
					let mut if_model_exist = false;
					for a in &mut self.display.window {
						if a.id == 0 {
							*a = Window::error_model(format!("{:#?}",e), ui);
							if_model_exist = true;
						}else {
							a.if_enabled = false;
						}
					}
					if !if_model_exist {
						self.display.window.push(Window::error_model(format!("{:#?}",e),ui));
					}
				}
			}
			ui.input_mut(|input|{
				let consume = egui::Modifiers::CTRL;
				if input.consume_shortcut(&egui::KeyboardShortcut::new(consume, Key::R)) {
					self.display.if_normal = true;
					self.texture = HashMap::new();
					self.display = Display::default();
				}
			});
			if let Condition::Debug(t) = &self.condition {
				let latecy = match frame.info().cpu_usage {
					None => 0.0,
					Some(t) => t
				};
				ui.put(Rect{min: (available_size - Vec2{x: 120.0, y: 30.0}).to_pos2(), max: available_size.to_pos2()}, Label::new(format!("latecy: {:.2}ms",latecy * 1000.0)));
				ui.put(Rect{min: (available_size - Vec2{x: 120.0, y: 60.0}).to_pos2(), max: available_size.to_pos2()}, Label::new(format!("fps: {:.0}",1000.0 / (latecy * 1000.0))));
				if t.if_inspection {
					egui::Window::new("Inspection").scroll2([true,true]).default_size(Vec2{x: 300.0, y: 500.0}).show(ctx, |ui| {
						ctx.inspection_ui(ui);
					});
				}
				self.debug_functions(ui, frame);
			}
		});
		ctx.request_repaint();
	}
}

#[cfg(not(target_os = "android"))]
fn export(temp: &Temp) -> Result<(), ShapoError> {
	use std::fs::File;
	use flate2::Compression;
	use flate2::write::GzEncoder;
	use rfd::FileDialog;

	let _ = create_dir("./temp");
	fn error(temp: &Temp) -> Result<(),ShapoError> {
		if temp.now_project_path.is_empty() {
			return Err(ShapoError::SystemError(format!("invailed chart info input")));
		}
		let split:Vec<&str> = temp.now_project_path.split("\\\\").collect(); 
		let mut actual_project_path = String::new();
		for a in split {
			actual_project_path = actual_project_path + "/" + a
		}
		actual_project_path = actual_project_path[2..actual_project_path.len()-1].to_string();
		let tar_gz = match File::create(&format!("./temp/{}.shapoistcompress", temp.chart.mapname)) {
			Ok(t) => t,
			Err(e) => return Err(ShapoError::SystemError(format!("failed to create export package. info: {}", e.to_string()))),
		};
		let enc = GzEncoder::new(tar_gz, Compression::default());
		let mut tar = tar::Builder::new(enc);
		match tar.append_dir_all("./temp", actual_project_path) {
			Ok(_) => {},
			Err(e) => return Err(ShapoError::SystemError(format!("failed to compress export package. info: {}", e.to_string()))),
		};
		if let Some(path) = FileDialog::new().pick_folder() {
			copy_file(&format!("./temp/{}.shapoistcompress", temp.chart.mapname), &format!("{}/{}.shapoistcompress",path.display().to_string(),temp.chart.mapname))?;
		}
		Ok(())
	}
	match error(temp) {
		Ok(_) => remove_path("./temp")?,
		Err(e) => {
			remove_path("./temp")?;
			return Err(e);
		}
	}

	Ok(())
}

#[cfg(target_os = "android")]
fn export(_: &Temp) -> Result<(), ShapoError> {
	return Err(ShapoError::SystemError(String::from("N/A")))
}