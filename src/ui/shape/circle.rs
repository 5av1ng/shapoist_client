use crate::ShapoError;
use crate::ui::shape::style::Style;
use egui::Vec2;
use crate::ui::shapo::ShapeRender;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Circle {
	pub radius: f32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum CircleChange {
	Radius,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum CircleAnimate {
	Radius,
}

impl Default for Circle {
	fn default() -> Self {
		Self {
			radius: 5.0
		}
	}
}

impl ShapeRender for Circle {
	fn render(&self, ui: &mut egui::Ui, size: &Vec2, offect: Option<Vec2>, style: &Style) -> Result<(), ShapoError> {
		let size_min;
		if size.x > size.y {
			size_min = size.y
		}else {
			size_min = size.x
		}
		let style_size_min;
		if style.size.x > style.size.y {
			style_size_min = style.size.y
		}else {
			style_size_min = style.size.x
		}
		let actual_position = style.get_position(size, offect);
		if let Some(t) = style.layer {
			ui.ctx().layer_painter(t).circle(
				actual_position.to_pos2(), 
				self.radius/100.0 * size_min * style_size_min, 
				style.fill, 
				style.stroke);
		}else {
			ui.painter().circle(
				actual_position.to_pos2(), 
				self.radius/100.0 * size_min * style_size_min, 
				style.fill, 
				style.stroke);
		}
		Ok(())
	}
}