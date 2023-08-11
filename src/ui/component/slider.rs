use crate::ui::page::Temp;
use std::ops::RangeInclusive;
use crate::language::language::Language;
use crate::ui::ui::Back;
use crate::ui::ui::ChangeType;
use crate::ui::shapo::Shapo;
use crate::ui::ui::Content;
use crate::play::timer::Timer;
use std::collections::HashMap;
use egui::TextureId;
use egui::TextureHandle;
use crate::ShapoError;
use egui::Vec2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Slider {
	pub shape: Vec<Shapo>,
	pub value: f32,
	pub change_type: ChangeType,
	pub range: RangeInclusive<f32>,
	pub text: Language
}

impl Content for Slider {
	fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId, TextureHandle>, temp: &Temp) -> Result<Vec<Back>, ShapoError> {
		let backup = self.value.clone();
		let mut vec_back = self.shape(ui,size,timer,offect,if_enabled,texture)?;
		let text = self.text.get_language()?;
		ui.label(text);
		ui.add(egui::Slider::new(&mut self.value, self.range.clone()));
		if backup != self.value {
			vec_back.push(self.change_type.change(&self.value,temp,None)?);
		}
		Ok(vec_back)
	}
}

impl Slider {
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