use egui::{Rect, Color32};
use crate::ShapoError;
use crate::ui::shape::style::Style;
use crate::ui::shapo::ShapeRender;
use egui::Vec2;
use egui::FontId;
use egui::FontFamily;
use crate::language::language::Language;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Text {
	pub text: Language
}

impl Default for Text {
	fn default() -> Self {
		Self {
			text: Language::Code(0)
		}
	}
}

impl ShapeRender for Text {
	fn render(&self, ui: &mut egui::Ui, size: &Vec2, offect: Option<Vec2>, style: &Style) -> Result<(), ShapoError> {
		let offect_vec = match offect {
			Some(t) => t,
			None => Vec2 {x: 0.0, y: 0.0},
		};
		let actual_position: Vec2;
		if !style.if_absolute {
			actual_position = ((style.position)/100.0 * style.size) * *size + offect_vec;
		}else {
			actual_position = style.position * style.size + offect_vec
		}
		let text = self.text.get_language()?;
		let mut text_out = String::new();
		let mut if_compressed = false;
		let delta = style.volume.max.x - style.volume.min.x;
		for a in text.split("\n") {
			let rectangle = get_rect(a.to_string(), style.text_size, ui, size).max.x - get_rect(a.to_string(), style.text_size, ui, size).min.x;
			let text_length: usize;
			if ((a.chars().count() as f32 * delta / rectangle).floor() as i32) - 3 < 0 {
				text_length = 0;
			}else {
				text_length = ((a.chars().count() as f32 * delta / rectangle).floor() as usize) - 3;
			};
			if rectangle > delta {
				text_out = text_out + "\n" + &(format!("{}...",utf8_slice::slice(a, 0, text_length)));
				if_compressed = true;
			}else {
				text_out = text_out + "\n" + a;
			}
		}
		if let Some(t) = style.layer {
			ui.ctx().layer_painter(t).text(
				actual_position.to_pos2(),
				style.anchor,
				text_out,
				FontId::new(style.text_size, FontFamily::Proportional),
				style.fill);
			if if_compressed {
				let vol_rect = Rect {
					min: (style.volume.min.to_vec2() / 100.0 * *size + offect_vec).to_pos2(),
					max: (style.volume.max.to_vec2() / 100.0 * *size + offect_vec).to_pos2()
				};
				let (_, response) = ui.allocate_ui_at_rect(vol_rect, |ui| {
					ui.centered_and_justified(|ui| ui.allocate_exact_size(Vec2{x: vol_rect.max.x - vol_rect.min.x, y: vol_rect.max.y - vol_rect.min.y}, egui::Sense::click())).inner
				}).inner;
				response.on_hover_text(text);
			}
		}else {
			ui.painter().text(
				actual_position.to_pos2(),
				style.anchor,
				text_out,
				FontId::new(style.text_size, FontFamily::Proportional),
				style.fill);
			if if_compressed {
				let vol_rect = Rect {
					min: (style.volume.min.to_vec2() / 100.0 * *size + offect_vec).to_pos2(),
					max: (style.volume.max.to_vec2() / 100.0 * *size + offect_vec).to_pos2()
				};
				let (_, response) = ui.allocate_ui_at_rect(vol_rect, |ui| {
					ui.centered_and_justified(|ui| ui.allocate_exact_size(Vec2{x: vol_rect.max.x - vol_rect.min.x, y: vol_rect.max.y - vol_rect.min.y}, egui::Sense::click())).inner
				}).inner;
				response.on_hover_text(text);
			}
		}
		Ok(())
	}
}

impl Text {
	pub fn new_from_string(input: String) -> Self {
		Self {
			text: Language::Text(input)
		}
	}

	pub fn new(text: Language) -> Self {
		Self {
			text
		}
	}
}

pub fn get_rect(string: String, text_size: f32, ui: &mut egui::Ui, size: &Vec2) -> Rect {
	let mut back = ui.painter().layout_no_wrap(
		string,
		FontId::new(text_size, FontFamily::Proportional),
		Color32::WHITE).rect;
	back.max.x = back.max.x / size.x * 100.0;
	back.max.y = back.max.y / size.y * 100.0;
	back
}