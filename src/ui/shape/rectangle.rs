use crate::ui::shape::bezier_curve::CubicBezier;
use egui::Align;
use crate::ShapoError;
use egui::Rect;
use egui::epaint::PathShape;
use crate::ui::shape::style::Style;
use crate::ui::shapo::ShapeRender;
use egui::Rounding;
use egui::Vec2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Rectangle {
	pub bottom_right_point: Vec2,
	pub rounding: Rounding,
	pub if_keep: bool
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum RectangleChange {
	BottomRightPoint,
	Rounding
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum RectangleAnimate {
	BottomRightPoint(CubicBezier)
}

impl Default for Rectangle {
	fn default() -> Self {
		Self {
			bottom_right_point: Vec2 { x: 10.0, y: 10.0 },
			if_keep: false,
			rounding: Rounding::none(),
		}
	}
}

impl ShapeRender for Rectangle {
	fn render(&self, ui: &mut egui::Ui, size: &Vec2, offect: Option<Vec2>, style: &Style) -> Result<(), ShapoError> {
		if style.rotate == 0.0 {
			let mut actual_position = style.get_position(size,offect);
			let mut actual_bottom_left_point = style.get_vec2(size,offect, style.position + self.bottom_right_point);
			match style.anchor[0] {
				Align::Min => {}
				Align::Center => {
					let middle = (actual_position.x - actual_bottom_left_point.x).abs()/2.0;
					actual_position.x = actual_position.x - middle;
					actual_bottom_left_point.x = actual_bottom_left_point.x - middle;
				} 
				Align::Max => {
					let delta = (actual_position.x - actual_bottom_left_point.x).abs();
					actual_position.x = actual_position.x - delta;
					actual_bottom_left_point.x = actual_bottom_left_point.x - delta;
				}
			}
			match style.anchor[1] {
				Align::Min => {}
				Align::Center => {
					let middle = (actual_position.y - actual_bottom_left_point.y).abs()/2.0;
					actual_position.y = actual_position.y - middle;
					actual_bottom_left_point.y = actual_bottom_left_point.y - middle;
				} 
				Align::Max => {
					let delta = (actual_position.y - actual_bottom_left_point.y).abs();
					actual_position.y = actual_position.y - delta;
					actual_bottom_left_point.y = actual_bottom_left_point.y - delta;
				}
			}
			if let Some(t) = style.layer {
				ui.ctx().layer_painter(t).rect(
					Rect {
						min: actual_position.to_pos2(), 
						max: actual_bottom_left_point.to_pos2()
					},
					self.rounding, 
					style.fill, 
					style.stroke);
			}else {
				ui.painter().rect(
					Rect {
						min: actual_position.to_pos2(), 
						max: actual_bottom_left_point.to_pos2()
					},
					self.rounding,
					style.fill, 
					style.stroke);
			}
		}else {
			let point_1 = style.get_vec2(size,offect,style.position).to_pos2();
			let point_2 = style.get_vec2(size,offect,Vec2::new((style.position + self.bottom_right_point).x, style.position.y)).to_pos2();
			let point_3 = style.get_vec2(size,offect,Vec2::new(style.position.x, (style.position + self.bottom_right_point).y)).to_pos2();
			let point_4 = style.get_vec2(size,offect,style.position + self.bottom_right_point).to_pos2();
			if let Some(t) = style.layer {
				ui.ctx().layer_painter(t).add(PathShape::closed_line(vec!(point_1,point_3,point_2,point_4),style.stroke));
			}else {
				ui.painter().add(PathShape::closed_line(vec!(point_2,point_4,point_3,point_1),style.stroke));
			}
		}
		Ok(())
	}
}

impl Rectangle {
	pub fn new(bottom_right_point: Vec2, rounding: Rounding) -> Self {
		Self {
			bottom_right_point,
			if_keep: false,
			rounding
		}
	}
}