use crate::ui::page::Temp;
use crate::language::language::Language;
use egui::TextureId;
use egui::TextureHandle;
use crate::play::timer::Timer;
use egui::Vec2;
use std::collections::HashMap;
use crate::ShapoError;
use crate::ui::ui::Back;
use crate::ui::ui::Content;
use crate::ui::ui::ChangeType;
use crate::ui::shapo::Shapo;
use egui::Color32;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct ColorPicker {
	pub text: Language,
	pub shape: Vec<Shapo>,
	pub value: Color32,
	pub change_type: ChangeType,
	pub if_open: bool
}

impl Default for ColorPicker {
	fn default() -> Self {
		Self {
			text: Language::Text(String::from("TEST")),
			shape: vec!(),
			value: Color32::TRANSPARENT,
			change_type: ChangeType::ProjectPath,
			if_open: false
		}
	}
}

impl Content for ColorPicker {
	fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId, TextureHandle>, temp: &Temp) -> Result<Vec<Back>, ShapoError> {	
		let backup = self.value.clone();
		let text = self.text.get_language()?;
		let mut vec_back = self.shape(ui, size,timer,offect,if_enabled,texture)?;
		ui.label(text);
		egui::widgets::color_picker::color_picker_color32(ui, &mut self.value, egui::widgets::color_picker::Alpha::OnlyBlend);
		if self.value != backup {
			vec_back.push(self.change_type.change(&self.value, temp, None)?);
		}
		Ok(vec_back)
	}
}

impl ColorPicker {
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