use std::ops::Sub;
use std::ops::Add;
use egui::Rect;
use egui::Color32;
use egui::Stroke;
use crate::ShapoError;
use egui::Vec2;
use crate::ui::shape::style::Style;
use crate::ui::shapo::ShapeRender;
use egui::epaint::CubicBezierShape;
use egui::epaint::CircleShape;
use egui::Pos2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
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
				Pos2 { x: 0.0, y: 0.0 },
				Pos2 { x: 0.0, y: 0.0 },
				Pos2 { x: 0.0, y: 0.0 },
			]
		}
	}
}

impl ShapeRender for CubicBezier {
	fn render(&self, ui: &mut egui::Ui, size: &Vec2, offect: Option<Vec2>, style: &Style) -> Result<(), ShapoError> {
		fn draw(painter: &egui::Painter, actual_points: [Pos2;4], style: &Style, if_close: bool) {
			let paint = CubicBezierShape::from_points_stroke(
				actual_points,
				if_close,
				style.fill,
				style.stroke);
			painter.add(paint);
			painter.add(CircleShape{
				center: actual_points[0],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			painter.add(CircleShape{
				center: actual_points[1],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			painter.add(CircleShape{
				center: actual_points[2],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
			painter.add(CircleShape{
				center: actual_points[3],
				radius: 2.0,
				fill: style.fill,
				stroke: style.stroke,
			});
		}

		let actual_points = [(style.get_vec2(size, offect, self.points[0].to_vec2() + style.position)).to_pos2(),
			(style.get_vec2(size, offect, self.points[1].to_vec2() + style.position)).to_pos2(),
			(style.get_vec2(size, offect, self.points[2].to_vec2() + style.position)).to_pos2(),
			(style.get_vec2(size, offect, self.points[3].to_vec2() + style.position)).to_pos2()];
		if let Some(t) = style.layer {
			draw(&ui.ctx().layer_painter(t), actual_points, style, self.if_close)
		}else {
			draw(ui.painter(), actual_points, style, self.if_close)
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

impl Add for CubicBezier {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			points: [
				(self.points[0].to_vec2() + other.points[0].to_vec2()).to_pos2(),
				(self.points[1].to_vec2() + other.points[1].to_vec2()).to_pos2(),
				(self.points[2].to_vec2() + other.points[2].to_vec2()).to_pos2(),
				(self.points[3].to_vec2() + other.points[3].to_vec2()).to_pos2()
			],
			if_close: other.if_close,
		}
	}
}

impl Sub for CubicBezier {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			points: [
				(self.points[0].to_vec2() - other.points[0].to_vec2()).to_pos2(),
				(self.points[1].to_vec2() - other.points[1].to_vec2()).to_pos2(),
				(self.points[2].to_vec2() - other.points[2].to_vec2()).to_pos2(),
				(self.points[3].to_vec2() - other.points[3].to_vec2()).to_pos2()
			],
			if_close: self.if_close,
		}
	}
}