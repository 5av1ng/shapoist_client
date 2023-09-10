use crate::ui::page::Temp;
use crate::language::language::Language;
use egui::Vec2;
use crate::ui::ui::Content;
use crate::play::timer::Timer;
use std::collections::HashMap;
use egui::TextureId;
use egui::TextureHandle;
use crate::ui::ui::Back;
use crate::ShapoError;
use crate::ui::ui::ChangeType;
use crate::ui::shapo::Shapo;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct InputBox {
	pub shape: Vec<Shapo>,
	pub text: String,
	pub hover_text: Language,
	pub change_type: ChangeType
}

impl Default for InputBox {
	fn default() -> Self {
		Self {
			shape: vec!(),
			text: String::new(),
			hover_text: Language::Text(String::from("TEST")),
			change_type: ChangeType::ProjectPath
		}
	}
}

impl Content for InputBox {
	fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId, TextureHandle>, temp: &Temp) -> Result<Vec<Back>, ShapoError> {
		let backup = self.text.clone();
		let mut vec_back = self.shape(ui,size,timer,offect,if_enabled,texture)?;
		let text = self.hover_text.get_language()?;
		ui.label(format!("{}",text.clone()));
		ui.add(egui::TextEdit::singleline(&mut self.text).hint_text(text));
		if backup != self.text {
			vec_back.push(self.change_type.change(&self.text,temp, None)?);
		}
		Ok(vec_back)
	}
}

impl InputBox {
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