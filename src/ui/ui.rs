use crate::system::system_function::parse_json;
use crate::ASSETS_PATH;
use crate::ui::end_page::end_page;
use crate::ui::edit_page::edit_page;
use crate::ui::page::Temp;
use crate::play::note::PossibleChartChange;
use crate::system::system_function::to_json;
use crate::setting::setting::Setting;
use egui::DroppedFile;
use crate::ui::component::color_picker::ColorPicker;
use crate::system::system_function::parse_toml;
use crate::ui::component::check_box::CheckBox;
use crate::ui::component::combo_box::ComboBox;
use crate::setting::setting::PossibleSettingChange;
use crate::language::language::Language;
use egui::Label;
use egui::Rect;
use std::collections::HashMap;
use egui::TextureId;
use egui::TextureHandle;
use crate::ui::shapo::Shapo;
use crate::play::play_top::PlayTop;
use egui::Color32;
use crate::ui::shape::style::StyleAnimation;
use crate::ui::component::slider::*;
use crate::ui::component::inputbox::*;
use crate::ui::component::shader::*;
use crate::ui::component::window::*;
use crate::ui::component::button::*;
use crate::play::timer::Timer;
use crate::setting::setting::read_settings;
use crate::system::system_function::read_file;
use crate::error::error::ShapoError;
use egui::Vec2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum Component {
	Button(Button),
	InputBox(InputBox),
	Slider(Slider),
	Shader(Shader),
	Shapo(Vec<Shapo>),
	ComboBox(ComboBox),
	CheckBox(CheckBox),
	ColorPicker(ColorPicker),
	Nothing
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Display {
	pub component: Option<Vec<Component>>,
	pub play_top: Option<PlayTop>,
	pub timer: Vec<Timer>,
	pub window: Vec<Window>,
	pub if_normal: bool,
	pub tip: Option<Language>,
	pub label: Option<Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Router {
	MainPage,
	ChartPage,
	PlayPage(String),
	ShopPage,
	EditPage,
	UserPage,
	TestPage,
	EndPage(PlayTop)
}

pub enum Back {
	Router(Router),
	Animation(usize),
	AnimateDone(usize),
	OpenWindow(Window),
	CloseWindow(usize),
	Close,
	Error(ShapoError),
	Nothing,
	Pause,
	Retry,
	Play,
	JudgeNote(Vec<(usize,usize)>),
	MusicPlay(String, f32, f32, f32), //path bpm(-1.0 for unknown) beatnumber offect(s)
	Ignore,
	LoadedTexture((TextureId,TextureHandle)),
	PlaySound(String),
	PauseSound,
	Change(ChangeType,String),
	NewProject,
	OpenProject,
	Save,
	Undo,
	Redo,
	Export,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum ChangeType {
	Setting(PossibleSettingChange),
	ChartTemp(PossibleChartChange),
	MuiscPath,
	ImagePath,
	ProjectPath
}

pub trait Content {
	fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId,TextureHandle>, temp: &Temp) -> Result<Vec<Back>, ShapoError>;
}

pub trait Test {
	fn all(size: &Vec2) -> Vec<Component>;
}

impl Default for Display {
	fn default() -> Self {
		Self {
			component: None,
			play_top: None,
			window: Vec::new(),
			timer: Vec::new(),
			if_normal: true,
			label: None,
			tip: None
		}
	}
}

impl Display {
	pub fn read_component(&mut self, router: &Router, size: &Vec2, temp: &mut Temp) -> Result<Back, ShapoError> {
		let mut back = Back::Nothing;
		let setting = read_settings()?;
		if let Router::TestPage = router {
			*self = Self::test_page(size);
			return Ok(Back::Nothing);
		}else if let Router::PlayPage(t) = router {
			let play_top_read = PlayTop::read(&format!("{}/assets/chart/{}/map.shapoistmap",*ASSETS_PATH,t))?;
			temp.chart = play_top_read.chart.clone();
			*self = Self {
				play_top: Some(play_top_read),
				component: Some(vec!(Component::Nothing)),
				..Self::default()
			};
			return Ok(Back::PauseSound);
		}else if let Router::MainPage = router {
			back = Back::MusicPlay(format!("{}/assets/styles/{}/Music/MainTheme.mp3",*ASSETS_PATH,setting.ui_theme), -1.0, 0.0, 0.0)
		}else if let Router::EndPage(playtop) = router {
		 	*self = end_page(playtop.clone())?;
		 	return Ok(Back::PauseSound);
		};
		let path = format!("{}/assets/styles/{}/{}.toml",*ASSETS_PATH,&setting.ui_theme, &router.to_string());
		let read =  read_file(&path)?;
		let mut display_read:Self = parse_toml(&read)?;
		if !display_read.label.is_none() {
			for a in display_read.label.clone().unwrap() {
				if a == "test".to_string() {
					display_read = Display{
						component: Some(vec!()),
						..Default::default()
					}
				}
			}
		}
		*self = display_read;
		Ok(back)
	}

	pub fn test_page(size: &Vec2) -> Self {
		let mut buttons = Button::all(size);
		let mut animation_test = Button::default();
		animation_test.shape[0].animation.push(StyleAnimation::default());
		animation_test.shape[0].style.fill = Color32::from_rgba_premultiplied(0,233,0,255);
		buttons.push(Component::Button(animation_test));
		let component = Some(buttons);
		let window = vec!(Window::default());
		let out = Self {
			component,
			window,
			play_top: None,
			timer: Vec::new(),
			tip: None,
			if_normal: true,
			label: None
		};
		out
	}

	pub fn render(&mut self, ui:&mut egui::Ui, router: &Router, size: &Vec2, ctx: &egui::Context, texture: &HashMap<TextureId,TextureHandle>, file: &Vec<DroppedFile>, temp: &mut Temp) -> Result<Vec<Back>, ShapoError> {
		let setting = read_settings()?;
		let mut back_vec = Vec::new();
		for a in &mut self.window {
			for b in a.render(size, &mut self.timer, ctx, ui, texture, self.if_normal, file, temp)? {
				back_vec.push(b);
			};
		}
		if let Router::EditPage = router {
			for a in edit_page(ui, size, &mut self.timer, self.if_normal, texture, temp, file, size)? {
				back_vec.push(a);
			}
		}else {
			match &mut self.component {
				Some(t) => {
					for a in t {
						for b in a.render(ui, size, &mut self.timer, None, self.if_normal, ctx, texture, temp)? {
							back_vec.push(b);
						}
					}
				},
				None => {
					if self.if_normal {
						back_vec.push(self.read_component(router, size, temp)?);
					}
					if let Some(t) = &mut self.component {
						for a in t {
							for b in a.render(ui, size, &mut self.timer, None, self.if_normal, ctx, texture, temp)? {
								back_vec.push(b);
							}
						}
					}
				}
			}
		}
		match &mut self.play_top {
			Some(t) => {
				for a in t.render(ui, size, texture, self.if_normal)? {
					back_vec.push(a);
				}
			},
			None => {}
		}
		if let None = self.tip {
			self.tip = Some(match Language::random_tip() {
				Ok(t) => t,
				Err(_) => Language::Text(format!("couldn't find tips :(")),
			})
		}
		if setting.if_tip {
			if let Router::MainPage | Router::ShopPage | Router::ChartPage | Router::UserPage = router {
				ui.put(Rect{min: Vec2{ 
					x: 0.0,
					y: size.y - 60.0
				}.to_pos2(), max: size.to_pos2()}, Label::new(self.tip.clone().unwrap().get_language().unwrap()));
			}
		}
		Ok(back_vec)
	}
}

impl Component {
	pub fn render(&mut self, ui: &mut egui::Ui, 
		size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, ctx: &egui::Context, texture: &HashMap<TextureId,TextureHandle>, temp: &Temp) -> Result<Vec<Back>, ShapoError> {
		match self {
			Component::Button(t) => t.render(ui, size, timer, offect, if_enabled, texture,temp),
			Component::Nothing => Ok(vec!(Back::Nothing)),
			Component::Shader(t) => t.render(ui, size, timer, offect, if_enabled, ctx),
			Component::InputBox(t) => t.render(ui, size, timer, offect, if_enabled, texture,temp),
			Component::Slider(t) => t.render(ui, size, timer, offect, if_enabled, texture, temp),
			Component::ComboBox(t) => t.render(ui, size, timer, offect, if_enabled, texture, temp),
			Component::CheckBox(t) => t.render(ui, size, timer, offect, if_enabled, texture,temp),
			Component::ColorPicker(t) => t.render(ui, size, timer, offect, if_enabled, texture, temp),
			Component::Shapo(t) => {
				let mut vec_back = vec!();
				for a in t {
					for b in a.render(ui,size,timer,offect,if_enabled, texture)? {
						vec_back.push(b);
					};
				}
				Ok(vec_back)
			},
		}
	}
}

impl Back {
	pub fn to_string(&self) -> String {
		match self {
			Self::Router(r) => format!("Router: {:#?}", r),
			Self::Animation(r) => format!("Animation: {:#?}", r),
			Self::AnimateDone(r) => format!("AnimateDone: {:#?}", r),
			Self::OpenWindow(r) => format!("OpenWindow: {}", r.id),
			Self::CloseWindow(r) => format!("CloseWindow: {:#?}", r),
			Self::Close => format!("close program"),
			Self::Error(r) => format!("Error: {:#?}", r),
			Self::Nothing => format!("nothing"),
			Self::Pause => format!("game pause"),
			Self::Play => format!("game stat"),
			Self::JudgeNote(r) => format!("JudgeNote: {:#?}", r),
			Self::MusicPlay(r, _, _, _) => format!("MusicPlay: {:#?}", r),
			Self::Ignore => format!("Ignore Error"),
			Self::LoadedTexture(_) => format!("Texture"),
			Self::PlaySound(r) => format!("Will Play Sound: {}",r),
			Self::Retry => format!("User Retry"),
			Self::PauseSound => format!("SoundPause"),
			Self::Change(t,_) => format!("user changed {:?}", t),
			Self::NewProject => format!("user try to crate new project"),
			Self::OpenProject => format!("user try to open a project"),
			Self::Save => format!("user save project"),
			Self::Undo => format!("user undo"),
			Self::Redo => format!("user redo"),
			Self::Export => format!("user export"),
		}
	}
}

impl Router {
	fn to_string(&self) -> String {
		match self {
			Router::MainPage => String::from("MainPage"),
			Router::ChartPage => String::from("ChartPage"),
			Router::PlayPage(_) => String::from("PlayPage"),
			Router::ShopPage => String::from("ShopPage"),
			Router::EditPage => String::from("EditPage"),
			Router::UserPage => String::from("UserPage"),
			Router::TestPage => String::from("TestPage"),
			Router::EndPage(_) => String::from("EndPage"),
		}
	}
}

impl ChangeType {
	pub fn change<T: serde::Serialize + std::fmt::Debug>(&self, value_to_change: T, temp: &Temp, value_no_need_to_change: Option<String>) -> Result<Back,ShapoError> {
		match &self {
			ChangeType::Setting(t) => {
				let value_json:String;
				if let Some(t) = value_no_need_to_change {
					value_json = t;
				}else {
					value_json = to_json(&value_to_change)?;
				}
				let setting = Setting::from_change(t, value_json)?;
				let setting = to_json(&setting)?;
				Ok(Back::Change(self.clone(),setting))
			},
			ChangeType::ChartTemp(t) => {
				let value_json:String;
				if let Some(t) = value_no_need_to_change {
					value_json = t;
				}else {
					value_json = to_json(&value_to_change)?;
				}
				let chart = temp.chart.clone().change(t, &value_json)?;
				let chart = to_json(&chart)?;
				Ok(Back::Change(self.clone(),chart))
			},
			ChangeType::MuiscPath => {
				let value_json:String;
				if let Some(t) = value_no_need_to_change {
					value_json = t;
				}else {
					value_json = to_json(&value_to_change)?;
				}
				let path: String = parse_json(&value_json)?;
				let split:Vec<&str> = path.split(".").collect();
				if split.len() <= 1 {
					return Err(ShapoError::SystemError(format!("not a file")))
				}
				if utf8_slice::slice(split[1], 0, 3) != "mp3" {
					return Err(ShapoError::SystemError(format!("not mp3 file")))
				}
				Ok(Back::Change(self.clone(),to_json(&value_json)?))
			},
			ChangeType::ImagePath => {
				let value_json:String;
				if let Some(t) = value_no_need_to_change {
					value_json = t;
				}else {
					value_json = to_json(&value_to_change)?;
				}
				let path: String = parse_json(&value_json)?;
				let split:Vec<&str> = path.split(".").collect();
				if split.len() <= 1 {
					return Err(ShapoError::SystemError(format!("not a file")))
				}
				if split[1] != "png" {
					return Err(ShapoError::SystemError(format!("not png file")))
				}
				Ok(Back::Change(self.clone(),to_json(&value_json)?))
			},
			ChangeType::ProjectPath => {
				let value_json:String;
				if let Some(t) = value_no_need_to_change {
					value_json = t;
				}else {
					value_json = to_json(&value_to_change)?;
				}
				Ok(Back::Change(self.clone(),to_json(&value_json)?))
			},
		}
	}
}