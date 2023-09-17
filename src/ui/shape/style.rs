use egui::Align;
use crate::ui::shape::rectangle::RectangleAnimate;
use crate::ui::shape::circle::CircleAnimate;
use crate::ui::shape::bezier_curve::CubicBezierAnimate;
use egui::LayerId;
use crate::ui::shape::bezier_curve::CubicBezier;
use crate::ui::shape::animation::Animation;
use crate::setting::setting::*;
use crate::ShapoError;
use egui::Align2;
use egui::Color32;
use egui::Stroke;
use egui::Rect;
use egui::Vec2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Copy)]
pub enum Unit {
	Vc,
	Em,
	Px,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct StyleGrid {
	pub grid: [usize;2],
	pub position: [usize;2],
	pub anchor: Align2
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Style {
	pub position: Vec2,
	pub position_unit: [Unit;2],
	pub size: Vec2,
	pub rotate: f32,
	pub rotate_center: Vec2,
	pub rotate_center_unit: [Unit;2],
	pub scale_center: Vec2,
	pub scale_center_unit: [Unit;2],
	pub fill: Color32,
	pub stroke: Stroke,
	pub volume: Rect,
	pub volume_unit: [[Unit;2];2],
	pub anchor: Align2,
	pub text_size: f32,
	pub layer: Option<LayerId>,
	pub grid: StyleGrid,
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
	pub start_time: Option<u64>,
	pub animate_time: u64,
	pub if_animating: bool,
	pub id: usize
}

impl Style {
	pub fn new(position: Vec2, fill: Color32, volume: Rect, layer: Option<LayerId>) -> Self {
		Self {
			position,
			volume,
			fill,
			layer,
			..Default::default()
		}
	}

	pub fn get_position(&self, size: &Vec2, offect: Option<Vec2>) -> Vec2 {
		self.get_vec2(size, offect, self.position)
	}

	pub fn get_rectangle(&self, size: &Vec2, offect: Option<Vec2>) -> Rect {
		let offect = match offect {
			Some(t) => t,
			None => Vec2::new(0.0,0.0)
		};
		let min = (get_true_cartesian(&self.volume.min.to_vec2(), &self.volume_unit[0], size) + offect).to_pos2();
		let max = (get_true_cartesian(&self.volume.max.to_vec2(), &self.volume_unit[1], size) + offect).to_pos2();
		Rect {
			min,
			max
		}
	}

	pub fn get_vec2(&self, size: &Vec2, offect: Option<Vec2>, input_position: Vec2) -> Vec2 {
		fn rotate(rotate_center: Vec2, vec_to_rotate: Vec2, rotate: f32) -> Vec2 {
			let delta = vec_to_rotate - rotate_center;
			let middle = Vec2 {
				x: delta.x * rotate.cos() - delta.y * rotate.sin(),
				y: delta.x * rotate.sin() + delta.y * rotate.cos()
			};
			middle + rotate_center
		}

		fn scale(scale_center: Vec2, vec_to_scale: Vec2, size: Vec2) -> Vec2 {
			let delta = vec_to_scale - scale_center;
			let middle = Vec2::new(
				delta.x * size.x,
				delta.y * size.y,
			);
			middle + scale_center
		}

		let grid_ident_vec = *size/Vec2::new(self.grid.grid[0] as f32, self.grid.grid[1] as f32);
		let grid_anchor = Vec2::new(match self.grid.anchor[0] {
			Align::Min => 0.0,
			Align::Max => 1.0,
			Align::Center => 0.5,
		},match self.grid.anchor[1] {
			Align::Min => 0.0,
			Align::Max => 1.0,
			Align::Center => 0.5,
		});
		let grid_vec = grid_ident_vec * Vec2::new((self.grid.position[0] - 1) as f32, (self.grid.position[1] - 1) as f32) + grid_ident_vec * grid_anchor;

		let position = grid_vec + get_true_cartesian(&input_position, &self.position_unit, size);
		let scale_center = grid_vec + get_true_cartesian(&self.scale_center, &self.scale_center_unit,size);
		let rotate_center = grid_vec + get_true_cartesian(&self.rotate_center, &self.rotate_center_unit, size);

		let offect = match offect {
			Some(t) => t,
			None => Vec2::new(0.0,0.0)
		};

		scale(scale_center,rotate(rotate_center,position, self.rotate),self.size) + offect
	}
}

impl Default for Style{
	fn default() -> Self {
		Self {
			position: Vec2 { x: 0.0, y: 0.0 },
			position_unit: [Unit::Vc;2],
			size: Vec2 { x: 1.0, y:1.0 },
			rotate: 0.0,
			rotate_center: Vec2 { x: 0.0, y: 0.0 },
			rotate_center_unit: [Unit::Vc;2],
			scale_center: Vec2::new(0.0,0.0),
			scale_center_unit: [Unit::Vc;2],
			fill: Color32::WHITE,
			stroke: Stroke { width: 0.0, color: Color32::TRANSPARENT },
			volume: Rect {
				min: Vec2::new(-1.0,-1.0).to_pos2(),
				max: Vec2::new(-1.0,-1.0).to_pos2()
			},
			anchor: Align2::LEFT_TOP,
			text_size: 12.0,
			layer: None,
			volume_unit: [[Unit::Vc;2];2],
			grid: StyleGrid {
				grid: [1,1],
				position: [1,1],
				anchor: Align2::LEFT_TOP
			},
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
			animate_time: 5 * 1e6 as u64,
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

fn get_true_cartesian(position: &Vec2 ,units: &[Unit;2] , size: &Vec2) -> Vec2 {
	Vec2::new(match units[0]{
		Unit::Vc => {
			position.x /100.0 * size.x
		}
		Unit::Em => {
			position.x * 16.0
		}
		Unit::Px => {
			position.x
		}
	},
	match units[1]{
		Unit::Vc => {
			position.y /100.0 * size.y
		}
		Unit::Em => {
			position.y * 16.0
		}
		Unit::Px => {
			position.y
		}
	},)
}