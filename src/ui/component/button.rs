use crate::ui::page::Temp;
use std::collections::HashMap;
use egui::TextureId;
use egui::TextureHandle;
use crate::setting::setting::read_settings;
use crate::system::system_function::*;
use egui::Rounding;
use crate::ui::ui::*;
use egui::{Rect, Response, Vec2, Color32};
use crate::setting::setting::Setting;
use crate::ShapoError;
use crate::play::timer::Timer;
use crate::ui::shapo::Shapo;
use crate::ui::component::window::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ChangeType {
	Setting(Setting)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum Logic {
	To(Router),
	Close,
	OpenWindow(WindowToOpen),
	CloseWindow(usize),
	Change(ChangeType),
	Remove(PathToRemove),
	Read(String),
	Write([String; 2]),
	CreateDir(String),
	CreateFile(String),
	CopyFile([String; 2]),
	Animation(usize),
	MusicPlay(String),
	Pause,
	Play,
	Ignore,
	Retry,
	NewProject,
	OpenProject
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum WindowToOpen {
	FromBuilt(Window),
	FromPath(String),
	FromLabel(Option<Vec<String>>),
	FromLabelAndId(Option<Vec<String>>, usize),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum PathToRemove {
	FromTemp,
	FromPath(String),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Button {
	pub shape: Vec<Shapo>,
	pub click_logic: Option<Logic>,
	pub hold_logic: Option<Logic>
}

impl Test for Button {
	fn all(size: &Vec2) -> Vec<Component> {
		let mut component = Vec::new();
		let mut i = 0;
		loop {
			let position = i as f32 * Vec2 {x: 0.0, y: 10.0};
			let bottom = Vec2 {x: 40.0, y: 5.0};
			component.push(Component::Button(Button::new(Some(Logic::from_id(i)), 
				vec!(Shapo::from_rect(position, bottom, Rounding::same(5.0), Color32::BLACK, Rect { min: position.to_pos2(), max: (position + bottom).to_pos2()}, None),
					Shapo::from_string(id_to_string(i), position + Vec2{x: 2.0, y: 2.5}, Color32::WHITE, Some(Rect { min: position.to_pos2(), max: bottom.to_pos2()}), 13.0, None, size)), 
				None)));
			i = i + 1;
			if i > 9 {
				break;
			}
		}
		component
	}
}

impl Default for Button {
	fn default() -> Self {
		let mut vec_push = Vec::new();
		vec_push.push(Shapo::default());
		Self {
			shape: vec_push,
			click_logic: None,
			hold_logic: None
		}
	}
}

impl Button {
	fn shape(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId,TextureHandle>) 
	-> Result<(Response,Vec<Back>), ShapoError>{
		let mut min_size = Vec2{x:20000.0,y:20000.0};
		let mut max_size = Vec2{x:0.0,y:0.0};
		let mut back = Vec::new();
		for a in &self.shape {
			let rect = a.get_rect(size, offect);
			if rect.max.x > max_size.x {
				max_size.x = rect.max.x
			}
			if rect.max.y > max_size.y {
				max_size.y = rect.max.y
			}
			if rect.min.x < min_size.x {
				min_size.x = rect.min.x
			}
			if rect.min.y < min_size.y {
				min_size.y = rect.min.y
			}
		}
		let vol_rect = Rect { min: min_size.to_pos2(), max: max_size.to_pos2()};
		for a in &mut self.shape {
			let render_back = a.render(ui, size, timer, offect, if_enabled,texture)?;
			for a in render_back {
				back.push(a)
			}
		}
		let (_, response) = ui.allocate_ui_at_rect(vol_rect, |ui| {
			ui.centered_and_justified(|ui| ui.allocate_exact_size(Vec2{x: vol_rect.max.x - vol_rect.min.x, y: vol_rect.max.y - vol_rect.min.y}, egui::Sense::click())).inner
		}).inner;
		Ok((response, back))
	}

	pub fn new(click_logic: Option<Logic>, shape: Vec<Shapo>, hold_logic: Option<Logic>) -> Self {
		Self {
			click_logic,
			hold_logic,
			shape
		}
	}
}

impl Content for Button {
	fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId,TextureHandle>, temp: &Temp) -> Result<Vec<Back>, ShapoError> {
		let (response,mut back) = self.shape(ui, size, timer, offect, if_enabled,texture)?;
		if response.clicked() && if_enabled {
			back.push(handle_logic(&self.click_logic,temp)?);
		}else if response.hovered() && if_enabled {
			back.push(handle_logic(&self.hold_logic,temp)?);
		}
		return Ok(back);
	}
}

impl WindowToOpen {
	pub fn to_window(&self) -> Result<Window, ShapoError> {
		match self {
			WindowToOpen::FromBuilt(t) => Ok(t.clone()),
			WindowToOpen::FromPath(p) => {
				let setting = read_settings()?;
				Ok(Window::from_path(format!("data/data/com.saving.shapoist/assets/styles/{}/{}", setting.ui_theme, p))?)
			},
			WindowToOpen::FromLabel(l) => {
				Ok(Window::from_label(l.clone()))
			},
			WindowToOpen::FromLabelAndId(l, id) => {
				let mut window = Window::from_label(l.clone());
				window.id = *id;
				Ok(window)
			}
		}
	} 
}

fn handle_logic(input: &Option<Logic>, temp: &Temp) -> Result<Back,ShapoError> {
	match input {
		Some(t) => match t {
			Logic::To(g) => return Ok(Back::Router(g.clone())),
			Logic::Animation(t) => return Ok(Back::Animation(*t)),
			Logic::OpenWindow(t) => return Ok(Back::OpenWindow(t.to_window()?)),
			Logic::CloseWindow(t) => return Ok(Back::CloseWindow(*t)),
			Logic::CreateFile(t) => {
				create_file(t)?;
				Ok(Back::Nothing)
			},
			Logic::CreateDir(t) => {
				create_dir(t)?;
				Ok(Back::Nothing)
			},
			Logic::Write(t) => {
				write_file(&t[0], &t[1])?;
				Ok(Back::Nothing)
			},
			Logic::Remove(t) => {
				let path = t.to_string(temp); 
				let split:Vec<&str> = path.split("\\\\").collect(); 
				let mut actual_path = String::new();
				for a in split {
					actual_path = actual_path + "/" + a
				}
				actual_path = actual_path[2..actual_path.len()-1].to_string();
				remove_path(&actual_path)?;
				Ok(Back::Nothing)
			},
			Logic::CopyFile(t) => {
				copy_file(&t[0], &t[1])?;
				Ok(Back::Nothing)
			},
			Logic::Close =>return Ok(Back::Close),
			Logic::Ignore => return Ok(Back::Ignore),
			Logic::Change(_) => todo!(),
			Logic::Read(_) => todo!(),
			Logic::Pause => return Ok(Back::Pause),
			Logic::Play => return Ok(Back::Play),
			Logic::MusicPlay(t) => return Ok(Back::MusicPlay(t.to_string(),-1.0, 0.0, 0.0)),
			Logic::NewProject => return Ok(Back::NewProject),
			Logic::OpenProject => return Ok(Back::OpenProject),
			Logic::Retry => return Ok(Back::Retry),
		}
		None => Ok(Back::Nothing),
	}
}

impl Logic {
	fn from_id(id: usize) -> Logic {
		match id {
			0 => Logic::To(Router::MainPage),
			1 => Logic::OpenWindow(WindowToOpen::FromBuilt(Window::default())),
			2 => Logic::CloseWindow(1000),
			3 => Logic::CreateDir("data/data/com.saving.shapoist/assets/garbage/".to_string()),
			4 => Logic::Remove(PathToRemove::FromPath("data/data/com.saving.shapoist/assets/garbage/test.garbage".to_string())),
			5 => Logic::Read("data/data/com.saving.shapoist/assets/garbage/test.garbage".to_string()),
			6 => Logic::Write(["data/data/com.saving.shapoist/assets/garbage/test.garbage".to_string(), "Hello World! ".to_string()]),
			7 => Logic::CreateFile("data/data/com.saving.shapoist/assets/garbage/test.garbage".to_string()),
			8 => Logic::Animation(1),
			9 => Logic::CopyFile(["data/data/com.saving.shapoist/assets/garbage/test.garbage".to_string(), "data/data/com.saving.shapoist/assets/garbage/another.garbage".to_string()]),
			_=> unreachable!()
		}
	}
}

impl PathToRemove {
	fn to_string(&self, temp: &Temp) -> String {
		match self {
			PathToRemove::FromPath(t) => return t.to_string(),
			PathToRemove::FromTemp => return temp.now_project_path.clone(),
		}
	}
}


fn id_to_string(id: usize) -> String {
	let back = match id {
		0 => "Logic::To(Router::MainPage)",
		1 => "Logic::OpenWindow(Window::default())",
		2 => "Logic::CloseWindow(1000)",
		3 => "Logic::CreateDir(\"data/data/com.saving.shapoist/assets/garbage/\")",
		4 => "Logic::Remove(\"data/data/com.saving.shapoist/assets/garbage/test.garbage\")",
		5 => "Logic::Read(\"data/data/com.saving.shapoist/assets/garbage/test.garbage\")",
		6 => "Logic::Write([\"data/data/com.saving.shapoist/assets/garbage/test.garbage\", \"Hello World! \"])",
		7 => "Logic::Create(\"data/data/com.saving.shapoist/assets/garbage/test.garbage\")",
		8 => "Logic::Animation(1)",
		9 => "Logic::CopyFile(\"data/data/com.saving.shapoist/assets/garbage/test.garbage\", \"data/data/com.saving.shapoist/assets/garbage/another.garbage\"",
		_=> unreachable!()
	}.to_string();
	back
}