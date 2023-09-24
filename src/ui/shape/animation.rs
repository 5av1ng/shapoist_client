use std::ops::Sub;
use std::ops::Add;
use crate::ShapoError;
use crate::setting::setting::*;
use egui::Vec2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Animation {
	pub control_point_one: Vec2,
	pub control_point_two: Vec2
}

impl Default for Animation {
	fn default() -> Self {
		Self {
			control_point_one: Vec2 { x: 0.0, y: 0.0},
			control_point_two: Vec2 { x: 0.0, y: 0.0}
		}
	}
}

impl Add for Animation {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			control_point_one: self.control_point_one + other.control_point_one,
			control_point_two: self.control_point_two + other.control_point_two,
		}
	}
}

impl Sub for Animation {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			control_point_one: self.control_point_one - other.control_point_one,
			control_point_two: self.control_point_two - other.control_point_two,
		}
	}
}

/*
$$
f(t) = P_0 (1-t)^3 + 3P_1 (1-t)^2 t + 3 P_2 (1-t) t^2 + P_3 t^3 \\
f'(t) = -3 (P_1 (-3 t^2 + 4 t - 1) + P_0 (1 - t)^2 + t (P_2 (3 t - 2) - P_3 t)) \\
$$
*/
impl Animation {
	pub fn caculate(&self, x: f32) -> Result<f32, ShapoError> {
		if x < 0.0 || x > 1.0 {
			return Err(ShapoError::SystemError(String::from("invaild x")))
		}
		let setting = read_settings()?;
		let t: f32;
		let mut left = 0.0;
		let mut right = 1.0;
		loop {
			let middle = (left + right)/2.0;
			let result = 3.0 * middle * (1.0 - middle) * (1.0 - middle) * self.control_point_one.x +
			3.0 * middle * middle * (1.0 - middle) * self.control_point_two.x +
			middle * middle * middle;
			if result == x {
				t = middle;
				break;
			}else if result < x {
				left = middle
			}else {
				right = middle
			}
			if (right - left) < setting.accuracy {
				t = middle;
				break;
			}
		}
		let y = 3.0 * t * (1.0 - t) * (1.0 - t) * self.control_point_one.y +
		3.0 * t * t * (1.0 - t) * self.control_point_two.y +
		t * t * t;
		if y < 0.0 {
			return Ok(0.0);
		}else if y > 1.0 {
			return Ok(1.0);
		}
		return Ok(y);
	}

	// pub fn compress(&mut self) {
	// 	if self.control_point_one.x > 1.0 {
	// 		self.control_point_one.x = 1.0
	// 	}else if self.control_point_one.x < 0.0 {
	// 		self.control_point_one.x = 0.0
	// 	}

	// 	if self.control_point_one.y > 1.0 {
	// 		self.control_point_one.y = 1.0
	// 	}else if self.control_point_one.y < 0.0 {
	// 		self.control_point_one.y = 0.0
	// 	}

	// 	if self.control_point_two.x > 1.0 {
	// 		self.control_point_two.x = 1.0
	// 	}else if self.control_point_two.x < 0.0 {
	// 		self.control_point_two.x = 0.0
	// 	}

	// 	if self.control_point_two.y > 1.0 {
	// 		self.control_point_two.y = 1.0
	// 	}else if self.control_point_two.y < 0.0 {
	// 		self.control_point_two.y = 0.0
	// 	}
	// }
}