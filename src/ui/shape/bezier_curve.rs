use egui::Rect;
use egui::Color32;
use egui::Stroke;
use crate::ui::shapo::rotate;
use crate::ShapoError;
use egui::Vec2;
use crate::ui::shape::style::Style;
use crate::ui::shapo::ShapeRender;
use egui::epaint::CubicBezierShape;
use egui::epaint::CircleShape;
use egui::Pos2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct CubicBezier {
	pub points : [Pos2; 4],
	pub if_close: bool
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum CubicBezierChange {
	Point1,
	Point2,
	Point3,
	Point4,
	IfClose,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum CubicBezierAnimate {
	Point1(CubicBezier),
	Point2(CubicBezier),
	Point3(CubicBezier),
	Point4(CubicBezier),
}

impl Default for CubicBezier {
	fn default() -> Self {
		Self {
			if_close: false,
			points: [
				Pos2 { x: 0.0, y: 0.0 },
				Pos2 { x: 5.0, y: 30.0 },
				Pos2 { x: 25.0, y: 0.0 },
				Pos2 { x: 30.0, y: 10.0 },
			]
		}
	}
}

impl ShapeRender for CubicBezier {
	fn render(&self, ui: &mut egui::Ui, size: &Vec2, offect: Option<Vec2>, style: &Style) -> Result<(), ShapoError> {
		let offect_vec = match offect {
			Some(t) => t,
			None => Vec2 {x: 0.0, y: 0.0},
		};
		let actual_points: [Pos2; 4];
		if !style.if_absolute {
			actual_points = [((rotate(style.rotate_center, self.points[0].to_vec2(), style.rotate) + style.position)/100.0 * *size * style.size + offect_vec).to_pos2(),
			((rotate(style.rotate_center, self.points[1].to_vec2(), style.rotate) + style.position)/100.0 * *size * style.size + offect_vec).to_pos2(),
			((rotate(style.rotate_center, self.points[2].to_vec2(), style.rotate) + style.position)/100.0 * *size * style.size + offect_vec).to_pos2(),
			((rotate(style.rotate_center, self.points[3].to_vec2(), style.rotate) + style.position)/100.0 * *size * style.size + offect_vec).to_pos2()];
		}else {
			actual_points = [((rotate(style.rotate_center, self.points[0].to_vec2(), style.rotate) + style.position) * style.size + offect_vec).to_pos2(),
			((rotate(style.rotate_center, self.points[1].to_vec2(), style.rotate) + style.position) * style.size + offect_vec).to_pos2(),
			((rotate(style.rotate_center, self.points[2].to_vec2(), style.rotate) + style.position) * style.size + offect_vec).to_pos2(),
			((rotate(style.rotate_center, self.points[3].to_vec2(), style.rotate) + style.position) * style.size + offect_vec).to_pos2()];
		}
		let paint = CubicBezierShape::from_points_stroke(
			actual_points,
			self.if_close,
			style.fill,
			style.stroke);
		if let Some(t) = style.layer {
			ui.ctx().layer_painter(t).add(paint);
			ui.ctx().layer_painter(t).add(CircleShape{
				center: actual_points[0],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			ui.ctx().layer_painter(t).add(CircleShape{
				center: actual_points[1],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			ui.ctx().layer_painter(t).add(CircleShape{
				center: actual_points[2],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			ui.ctx().layer_painter(t).add(CircleShape{
				center: actual_points[3],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
		}else {
			ui.painter().add(paint);
			ui.painter().add(CircleShape{
				center: actual_points[0],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			ui.painter().add(CircleShape{
				center: actual_points[1],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			ui.painter().add(CircleShape{
				center: actual_points[2],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			ui.painter().add(CircleShape{
				center: actual_points[3],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
		}
		Ok(())
	}
}

impl CubicBezier {
	pub fn get_rectangle(&self) -> Rect {
		let paint = CubicBezierShape::from_points_stroke(
			self.points,
			self.if_close,
			Color32::BLACK,
			Stroke {
				width: 3.0,
				color: Color32::BLACK,
			});
		paint.visual_bounding_rect()
	}
}