use crate::system::system_function::to_json;
use crate::ui::page::Temp;
use crate::language::language::Language;
use crate::ui::ui::ChangeType;
use egui::SelectableLabel;
use crate::ShapoError;
use crate::ui::ui::Back;
use crate::play::timer::Timer;
use egui::Vec2;
use std::collections::HashMap;
use egui::TextureHandle;
use egui::TextureId;
use crate::ui::ui::Content;
use crate::ui::shapo::Shapo;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct ComboBox {
	pub shape: Vec<Shapo>,
	pub name: Language,
	pub value: String, // Json Value
	pub value_show: String,
	pub possible_value_json: Vec<String>,
	pub possible_value_show: Vec<String>,
	pub change_type: ChangeType
}

impl Default for ComboBox {
	fn default() -> Self {
		Self {
			shape: vec!(),
			name: Language::Text(String::from("TEST")),
			value: String::new(),
			value_show: String::new(),
			possible_value_json: vec!(),
			possible_value_show: vec!(),
			change_type: ChangeType::ProjectPath
		}
	}
}

impl Content for ComboBox {
	fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId, TextureHandle>, temp: &Temp) -> Result<Vec<Back>, ShapoError> { 
		if self.possible_value_show.len() != self.possible_value_json.len() {
			return Err(ShapoError::SystemError(format!("[ERROR] invailed value in combo box")));
		}
		let backup = self.value.clone();
		let mut vec_back = self.shape(ui,size,timer,offect,if_enabled,texture)?; 
		let text = self.name.get_language()?;
		ui.label(text.clone());
		if let Some(t) = egui::ComboBox::from_id_source(text).selected_text(format!("{}", self.value_show)).show_ui(ui, |ui| -> Result<(), ShapoError> {
			for a in 0..self.possible_value_show.len() {
				if ui.add(SelectableLabel::new(self.value == self.possible_value_json[a].clone(),format!("{}",self.possible_value_show[a]))).clicked() {
					self.value = self.possible_value_json[a].clone();
				};
			}
			if let Some((value, value_show)) = open_file(&self.change_type, ui)? {
				self.value = value;
				self.value_show = value_show;
			}
			Ok(())
		}).inner {
			t?;
		};
		if self.value != backup {
			for a in 0..self.possible_value_show.len() {
				if self.value == self.possible_value_show[a] {
					self.value = self.possible_value_json[a].clone();
					self.value_show = self.possible_value_show[a].clone();
				}
			}
			vec_back.push(self.change_type.change(&self.value,temp, Some(self.value.clone()))?);
		}
		Ok(vec_back)
	}
}

impl ComboBox {
	fn shape(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId,TextureHandle>) -> Result<Vec<Back>, ShapoError>{
		let mut back = Vec::new();
		for a in &mut self.shape {
			let render_back = a.render(ui, size, timer, offect, if_enabled,texture)?;
			for a in render_back {
				back.push(a)
			}
		}
		Ok(back)
	}
}

#[cfg(not(target_os = "android"))]
fn open_file(change_type: &ChangeType, ui: &mut egui::Ui) -> Result<Option<(String, String)>, ShapoError> {
	use pollster::block_on;

	let mut value = String::new();
	let mut value_show = String::new();
	let open_text = Language::Code(59).get_language()?;
	let music_text = Language::Code(38).get_language()?;
	let image_text = Language::Code(39).get_language()?;
	match change_type {
		ChangeType::ImagePath => {
			if ui.button(open_text).clicked() {
				let file = block_on(async { 
					let file = rfd::AsyncFileDialog::new().add_filter(&image_text, &["png"]).pick_file().await;
					file
				});
				if let Some(path) = file{
					let path = path.path().display().to_string();
					value = to_json(&path)?;
					value_show = path;
				}
			}
		},
		ChangeType::MuiscPath => {
			if ui.button(open_text).clicked() {
				let file = block_on(async { 
					let file = rfd::AsyncFileDialog::new().add_filter(&music_text, &["mp3"]).pick_file().await;
					file
				});
				if let Some(path) = file {
					let path = path.path().display().to_string();
					value = to_json(&path)?;
					value_show = path;
				}
			}
		},
		_ => {}
	}
	if !value_show.is_empty() && !value.is_empty() {
		return Ok(Some((value,value_show)))
	}else {
		return Ok(None)
	}
}

#[cfg(target_os = "android")]
fn open_file(change_type: &ChangeType, ui: &mut egui::Ui)  -> Result<Option<(String, String)>, ShapoError> {

	let music_text = Language::Code(158).get_language()?;
	let image_text = Language::Code(159).get_language()?;
	match change_type {
		ChangeType::ImagePath => {
			ui.label(image_text);
		},
		ChangeType::MuiscPath => {
			ui.label(music_text);
		},
		_ => {}
	}
	return Ok(None)
}