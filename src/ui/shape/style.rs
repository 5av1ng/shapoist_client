use crate::ui::shape::rectangle::RectangleAnimate;
use crate::ui::shape::circle::CircleAnimate;
use crate::ui::shape::bezier_curve::CubicBezierAnimate;
use egui::LayerId;
use crate::ui::shape::bezier_curve::CubicBezier;
use crate::ui::shape::animation::Animation;
use crate::setting::setting::*;
use crate::ShapoError;
use egui::Pos2;
use egui::Align2;
use egui::Color32;
use egui::Stroke;
use egui::Rect;
use egui::Vec2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct Style {
	pub position: Vec2,
	pub if_absolute: bool,
	pub size: Vec2,
	pub rotate: f32,
	pub rotate_center: Vec2,
	pub fill: Color32,
	pub stroke: Stroke,
	pub volume: Rect,
	pub anchor: Align2,
	pub text_size: f32,
	pub layer: Option<LayerId>
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum ShapeAnimate {
	Rectangle(RectangleAnimate),
	Circle(CircleAnimate),
	Bezier(CubicBezierAnimate),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum StyleAnimate {
	Position(CubicBezier),
	Size(CubicBezier),
	Rotate,
	Fill,
	Stroke,
	RoutateCenter(CubicBezier),
	Volume(CubicBezier),
	TextSize,
	ShapeAnimate(ShapeAnimate),
	Alpha
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct StyleAnimation {
	pub style: StyleAnimate,
	pub start_value: f32,
	pub end_value: f32,
	pub animation: Animation,
	pub start_time: Option<u128>,
	pub animate_time: u128,
	pub if_animating: bool,
	pub id: usize
}

impl Style {
	pub fn new(position: Vec2, fill: Color32, volume: Rect, layer: Option<LayerId>) -> Self {
		Self {
			position,
			size: Vec2 { x: 1.0, y:1.0 },
			rotate: 0.0,
			rotate_center: Vec2 { x: 0.0, y: 0.0 },
			fill,
			stroke: Stroke { width: 0.0, color: Color32::from_rgba_premultiplied(0,0,0,0) },
			volume,
			anchor: Align2::LEFT_TOP,
			text_size: 12.0,
			if_absolute: false,
			layer
		}
	}
}

impl Default for Style{
	fn default() -> Self {
		Self {
			position: Vec2 { x: 0.0, y: 0.0 },
			size: Vec2 { x: 1.0, y:1.0 },
			rotate: 0.0,
			rotate_center: Vec2 { x: 0.0, y: 0.0 },
			fill: Color32::WHITE,
			stroke: Stroke { width: 0.0, color: Color32::TRANSPARENT },
			volume: Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 10.0, y: 10.0} },
			anchor: Align2::LEFT_TOP,
			text_size: 12.0,
			if_absolute: false,
			layer: None
		}
	}
}

impl Default for StyleAnimation {
	fn default()-> Self {
		Self{
			style: StyleAnimate::Position(CubicBezier::default()),
			start_value: 0.0,
			end_value: 6.0,
			animation: Animation::default(),
			start_time: None,
			animate_time: 5 * 1e6 as u128,
			if_animating: false,
			id: 1
		}
	}
}

impl StyleAnimation {
	pub fn caculate(&self, x: f32) -> Result<f32, ShapoError> {
		let output = self.animation.caculate(x)?;
		Ok(self.start_value + output * (self.end_value - self.start_value))
	}
}

pub fn arc_length(mut input: f32, bezier_curve: &CubicBezier) -> Result<Vec2,ShapoError> {
	if input < 0.0 {
		input = 0.0
	}else if input == 0.0 {
		return Ok(Vec2 { x:0.0, y: 0.0 });
	}
	
	fn bezier_curve_sqrt(t: f32, bezier_curve: &CubicBezier) -> f32 {
		let a = bezier_curve.points[0].x;
		let b = bezier_curve.points[1].x;
		let c = bezier_curve.points[2].x;
		let d = bezier_curve.points[3].x;
		let e = bezier_curve.points[0].y;
		let f = bezier_curve.points[1].y;
		let g = bezier_curve.points[2].y;
		let h = bezier_curve.points[3].y;
		let middle = ((-3.0 * a - 3.0 * e) * (-1.0 + t) * (-1.0 + t)  + (3.0 * d + 3.0 * h) * t * t + (c + g) * (6.0 * t - 9.0 * t * t) + (b + f) * (3.0 - 12.0 * t + 9.0 * t * t)).abs();
		let back = f32::sqrt(middle);
		back
	}

	fn simpsons_rule_integration(b: f32, bezier_curve: &CubicBezier) -> f32 {
		b / 8.0 * (bezier_curve_sqrt(0.0, &bezier_curve) + 
		3.0 * bezier_curve_sqrt(b / 3.0, &bezier_curve) + 
		3.0 * bezier_curve_sqrt(2.0 * b / 3.0, &bezier_curve) + 
		bezier_curve_sqrt(b, &bezier_curve))
	}

	fn bezier_curve_caculate(t: f32, bezier_curve: &CubicBezier) -> Vec2 {
		let x = (1.0 - t) * (1.0 - t) * (1.0 - t) * bezier_curve.points[0].x +
		3.0 * t * (1.0 - t) * (1.0 - t) * bezier_curve.points[1].x +
		3.0 * t * t * (1.0 - t) * bezier_curve.points[2].x +
		t * t * t * bezier_curve.points[3].x;
		let y = (1.0 - t) * (1.0 - t) * (1.0 - t) * bezier_curve.points[0].y +
		3.0 * t * (1.0 - t) * (1.0 - t) * bezier_curve.points[1].y +
		3.0 * t * t * (1.0 - t) * bezier_curve.points[2].y +
		t * t * t * bezier_curve.points[3].y;
		return Vec2 {
			x,
			y
		};
	}
	if input >= simpsons_rule_integration(1.0, &bezier_curve) {
		return Ok(bezier_curve_caculate(1.0, &bezier_curve))
	}

	let setting = read_settings()?;
	let back: f32;
	let mut left = 0.0;
	let mut right = 1.0;
	loop {
		let middle = (left + right) / 2.0;
		let result = simpsons_rule_integration(middle, &bezier_curve);
		if result == input {
			back = middle;
			break;
		}else if result < input {
			left = middle;
		}else {
			right = middle;
		}
		if (right - left) < setting.accuracy {
			back = middle;
			break;
		}
	};
	let out = bezier_curve_caculate(back, bezier_curve);
	return Ok(out);
}