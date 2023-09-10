use crate::ui::shape::bezier_curve::CubicBezier;
use crate::ui::shapo::rotate;
use egui::Align;
use egui::Pos2;
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
		let offect_vec = match offect {
			Some(t) => t,
			None => Vec2 {x: 0.0, y: 0.0},
		};
		if style.rotate == 0.0 {
			let mut actual_position: Vec2;
			let mut actual_bottom_left_point: Vec2;
			if !style.if_absolute {
				actual_position = (((style.position)/100.0 * style.size) * *size + offect_vec) * style.size;
				actual_bottom_left_point = ((style.position + self.bottom_right_point)/100.0 * *size + offect_vec) * style.size
			}else {
				actual_position = style.position;
				actual_bottom_left_point = style.position + self.bottom_right_point;
			}
			if self.if_keep {
				actual_bottom_left_point.y = actual_position.y + (actual_bottom_left_point.x - actual_position.x) * self.bottom_right_point.y / self.bottom_right_point.x;
			}
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
			let point_1: Pos2;
			let point_2: Pos2;
			let point_3: Pos2;
			let point_4: Pos2;
			if !style.if_absolute {
				point_1 = (((rotate(style.rotate_center,style.position,style.rotate))/100.0 * *size + offect_vec) * style.size).to_pos2();
				point_2 = (((rotate(style.rotate_center,Vec2{x: (style.position + self.bottom_right_point).x, y: style.position.y},style.rotate))/100.0 * *size + offect_vec) * style.size).to_pos2();
				point_3 = (((rotate(style.rotate_center,Vec2{x: style.position.x, y: (style.position + self.bottom_right_point).y},style.rotate))/100.0 * *size + offect_vec) * style.size).to_pos2();
				point_4 = (((rotate(style.rotate_center,style.position + self.bottom_right_point,style.rotate))/100.0 * *size + offect_vec) * style.size).to_pos2();
			}else {
				point_1 = (((rotate(style.rotate_center,style.position,style.rotate)) + offect_vec) * style.size).to_pos2();
				point_2 = (((rotate(style.rotate_center,Vec2{x: (style.position + self.bottom_right_point).x, y: style.position.y},style.rotate)) + offect_vec) * style.size).to_pos2();
				point_3 = (((rotate(style.rotate_center,Vec2{x: style.position.x, y: (style.position + self.bottom_right_point).y},style.rotate)) + offect_vec) * style.size).to_pos2();
				point_4 = (((rotate(style.rotate_center,style.position + self.bottom_right_point,style.rotate)) + offect_vec) * style.size).to_pos2();
			}
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