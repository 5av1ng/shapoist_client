use crate::ASSETS_PATH;
use crate::ui::shape::style::Style;
use crate::ui::shapo::Shape;
use crate::ui::shape::text::Text;
use crate::ui::component::button::Logic;
use egui::Vec2;
use crate::ui::component::button::Button;
use crate::ui::component::color_picker::ColorPicker;
use crate::ui::component::slider::Slider;
use crate::ui::component::check_box::CheckBox;
use crate::language::language::Language;
use crate::ui::ui::ChangeType;
use egui::Rect;
use egui::Pos2;
use egui::Align2;
use crate::ui::shapo::Shapo;
use crate::ui::component::combo_box::ComboBox;
use crate::ui::ui::Component;
use crate::ui::component::window::Window;
use egui::Color32;
use crate::system::system_function::*;
use crate::error::error::ShapoError;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Setting {
	pub ui_theme: String,
	pub language: String,
	pub accuracy: f32,
	pub if_immaculate: bool,
	pub tap_prompt_color: Option<Color32>,
	pub slide_prompt_color: Option<Color32>,
	pub note_prompt_radius: f32,
	pub judge_field_prompt_color: Option<Color32>,
	pub judge_field_prompt_size: f32,
	pub drop_velocity: f32,
	pub immaculate_color: Color32,
	pub extra_color: Color32,
	pub normal_color: Color32,
	pub fade_color: Color32,
	pub miss_color: Color32,
	pub background_color: Color32,
	pub search_depth: usize,
	pub if_tip: bool,
	pub volume: f32,
	pub undo_steps: usize,
	pub if_shader: bool,
	pub offect: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RootFindRule {
	BisectionMethod,
	NewtonMethod
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UiTheme {

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PossibleSettingChange {
	UiTheme,
	Language,
	Accuracy,
	IfImmaculate,
	TapPromptColor,
	SlidePromptColor,
	NotePromptRadius,
	JudgeFieldPromptColor,
	JudgeFieldPromptSize,
	DropVelocity,
	ImmaculateColor,
	ExtraColor,
	NormalColor,
	FadeColor,
	MissColor,
	BackgroundColor,
	SearchDepth,
	IfTip,
	Volume,
	UndoSteps,
	IfShader,
	Offect,
}

pub fn read_settings() -> Result<Setting, ShapoError> {
	let setting_read = match read_file(&format!("{}/assets/setting.toml", *ASSETS_PATH)){
		Ok(t) => parse_toml(&t)?,
		Err(_) => {
			write_file(&format!("{}/assets/setting.toml", *ASSETS_PATH), &to_toml(&Setting::default())?)?;
			Setting::default()
		}
	};
	Ok(setting_read)
}

impl Default for Setting {
	fn default() -> Self {
		Self {
			ui_theme: "Default".to_string(),
			language: "en-US".to_string(),
			accuracy: 0.0001,
			if_immaculate: true,
			tap_prompt_color: Some(Color32::from_rgba_premultiplied(255,255,255,100)),
			slide_prompt_color: Some(Color32::from_rgba_premultiplied(100,100,100,100)),
			note_prompt_radius: 1.0,
			judge_field_prompt_color: Some(Color32::from_rgba_premultiplied(255,255,255,100)),
			judge_field_prompt_size: 1.0,
			drop_velocity: 0.00001,
			immaculate_color: Color32::from_rgba_premultiplied(246,239,80,255),
			extra_color: Color32::from_rgba_premultiplied(67,187,252,255),
			normal_color: Color32::from_rgba_premultiplied(21,223,112,255),
			fade_color: Color32::from_rgba_premultiplied(107,55,34,255),
			miss_color: Color32::from_rgba_premultiplied(255,255,255,255),
			background_color: Color32::from_rgba_premultiplied(107,55,255,255),
			search_depth: 20,
			if_tip: true,
			volume: 1.0,
			undo_steps: 30,
			if_shader: false,
			offect: 0.0,
		}
	}
}

impl Setting {
	pub fn from_change(change_type: &PossibleSettingChange, json: String) -> Result<Setting, ShapoError> {
		let mut setting = read_settings()?;
		match change_type {
			PossibleSettingChange::UiTheme => setting.ui_theme = parse_json(&json)?,
			PossibleSettingChange::Language => setting.language = parse_json(&json)?,
			PossibleSettingChange::Accuracy => setting.accuracy = parse_json(&json)?,
			PossibleSettingChange::IfImmaculate => setting.if_immaculate = parse_json(&json)?,
			PossibleSettingChange::TapPromptColor => setting.tap_prompt_color = parse_json(&json)?,
			PossibleSettingChange::SlidePromptColor => setting.slide_prompt_color = parse_json(&json)?,
			PossibleSettingChange::NotePromptRadius => setting.note_prompt_radius = parse_json(&json)?,
			PossibleSettingChange::JudgeFieldPromptColor => setting.judge_field_prompt_color = parse_json(&json)?,
			PossibleSettingChange::JudgeFieldPromptSize => setting.judge_field_prompt_size = parse_json(&json)?,
			PossibleSettingChange::DropVelocity => setting.drop_velocity = parse_json(&json)?,
			PossibleSettingChange::ImmaculateColor => setting.immaculate_color = parse_json(&json)?,
			PossibleSettingChange::ExtraColor => setting.extra_color = parse_json(&json)?,
			PossibleSettingChange::NormalColor => setting.normal_color = parse_json(&json)?,
			PossibleSettingChange::FadeColor => setting.fade_color = parse_json(&json)?,
			PossibleSettingChange::MissColor => setting.miss_color = parse_json(&json)?,
			PossibleSettingChange::BackgroundColor => setting.background_color = parse_json(&json)?,
			PossibleSettingChange::SearchDepth => setting.search_depth = parse_json(&json)?,
			PossibleSettingChange::IfTip => setting.if_tip = parse_json(&json)?,
			PossibleSettingChange::Volume => setting.volume = parse_json(&json)?,
			PossibleSettingChange::UndoSteps => setting.undo_steps = parse_json(&json)?,
			PossibleSettingChange::IfShader => setting.if_shader = parse_json(&json)?,
			PossibleSettingChange::Offect => setting.offect = parse_json(&json)?,
		}
		return Ok(setting);
	}
}

impl Setting {
	pub fn ui(&self) -> Result<Window,ShapoError> {
		let mut content = vec!();
		let ui_theme = read_every_file(&format!("{}/assets/styles", *ASSETS_PATH))?;
		let mut ui_theme_true= vec!();
		let mut ui_theme_json= vec!();
		for a in &ui_theme {
			let slice = utf8_slice::from(a, ASSETS_PATH.len() + 15);
			if slice != "Font.ttf" {
				ui_theme_true.push(slice.to_string());
				ui_theme_json.push(to_json(&slice)?);
			}
		}
		let language = read_every_file(&format!("{}/assets/language", *ASSETS_PATH))?;
		let mut language_true = vec!();
		let mut language_json = vec!();
		let mut value_show = String::new();
		for a in &language {
			let slice = utf8_slice::from(a, ASSETS_PATH.len() + 17);
			if slice == self.language {
				value_show = read_file(&format!("{}/assets/language/{}/info.ini",*ASSETS_PATH , slice))?.trim().to_string();
			}
			language_true.push(read_file(&format!("{}/assets/language/{}/info.ini",*ASSETS_PATH , slice))?.trim().to_string());
			language_json.push(to_json(&slice)?);
		}
		content.push(Component::ComboBox(ComboBox {
			name: Language::Code(10),
			value_show: self.ui_theme.clone(),
			shape: vec!(Shapo::empty(Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 100.0, y: 5.0 } } )),
			value: to_json(&self.ui_theme)?,
			possible_value_json: ui_theme_json,
			possible_value_show: ui_theme_true,
			change_type: ChangeType::Setting(PossibleSettingChange::UiTheme)
		}));
		content.push(Component::ComboBox(ComboBox {
			name: Language::Code(11),
			value_show,
			shape: vec!(),
			value: to_json(&self.language)?,
			possible_value_json: language_json,
			possible_value_show: language_true,
			change_type: ChangeType::Setting(PossibleSettingChange::Language)
		}));
		content.push(Component::CheckBox(CheckBox {
			hover_text: Language::Code(12),
			if_checked: self.if_immaculate,
			shape: vec!(),
			change_type: ChangeType::Setting(PossibleSettingChange::IfImmaculate)
		}));
		content.push(Component::Slider(Slider {
			text: Language::Code(21),
			value: self.drop_velocity,
			range: 0.000001..=0.00005,
			shape: vec!(),
			change_type: ChangeType::Setting(PossibleSettingChange::DropVelocity)
		}));
		content.push(Component::Slider(Slider {
			text: Language::Code(134),
			value: self.offect,
			range: -5000.0..=5000.0,
			shape: vec!(),
			change_type: ChangeType::Setting(PossibleSettingChange::Offect)
		}));
		content.push(Component::ColorPicker(ColorPicker {
			text: Language::Code(27),
			value: self.background_color,
			shape: vec!(),
			change_type: ChangeType::Setting(PossibleSettingChange::BackgroundColor),
			if_open: false,
		}));
		content.push(Component::CheckBox(CheckBox {
			hover_text: Language::Code(29),
			if_checked: self.if_tip,
			shape: vec!(),
			change_type: ChangeType::Setting(PossibleSettingChange::IfTip)
		}));
		content.push(Component::CheckBox(CheckBox {
			hover_text: Language::Code(42),
			if_checked: self.if_shader,
			shape: vec!(),
			change_type: ChangeType::Setting(PossibleSettingChange::IfShader)
		}));
		content.push(Component::Button(Button {
			shape: vec!(Shapo{
				shape: Shape::Text(Text{ text:Language::Code(31)}),
				style: Style {
					position: Vec2 { x: 50.0, y: 94.0}, 
					fill: Color32::WHITE, 
					volume: Rect { min: Pos2 { x: 0.0, y: 90.0 }, max: Pos2 { x: 100.0, y: 100.0 } },
					anchor: Align2::CENTER_CENTER,
					..Default::default()
				}, 
				..Default::default()
			}),
			click_logic: Some(Logic::CloseWindow(1002)),
			hold_logic: None
		}));
		let window_read = read_file(&format!("{}/assets/styles/{}/Window/Setting.toml",*ASSETS_PATH , self.ui_theme))?;
		let mut window: Window = parse_toml(&window_read)?;
		window.content = content;
		Ok(window)
	}
}