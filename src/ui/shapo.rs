use std::collections::HashMap;
use egui::TextureHandle;
use egui::TextureId;
use crate::language::language::Language;
use crate::ui::shape::text::get_rect;
use egui::Rounding;
use egui::Color32;
use crate::ui::shape::image::Image;
use crate::ui::shape::text::Text;
use crate::ui::shape::circle::*;
use egui::Rect;
use crate::ui::shape::style::Style;
use egui::Vec2;
use crate::ui::shape::style::arc_length;
use crate::ui::shape::bezier_curve::*;
use crate::ui::shape::style::*;
use crate::ui::shape::rectangle::*;
use crate::ui::ui::Back;
use crate::play::timer::Timer;
use crate::ShapoError;

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug, PartialEq)]
pub struct Shapo {
	pub style: Style,
	pub shape: Shape,
	pub animation: Vec<StyleAnimation>,
	pub label: Option<Vec<String>>,
	pub sustain_time: Option<(u128,u128)>,
	pub if_delete: bool
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Shape {
	Circle(Circle),
	Rectangle(Rectangle),
	Text(Text),
	CubicBezier(CubicBezier),
	Image(Image),
}

pub trait ShapeRender {
	fn render(&self, ui: &mut egui::Ui, size: &Vec2, offect: Option<Vec2>, style: &Style) -> Result<(), ShapoError>;
}

impl Shapo {
	pub fn default() -> Self {
		Self {
			style: Style::default(),
			shape: Shape::default(),
			label: None,
			animation: Vec::new(),
			sustain_time: None,
			if_delete: false
		}
	}

	pub fn trig_animation(&mut self, id: usize) {
		for a in &mut self.animation {
			if a.id == id {
				a.if_animating = true;
			}
		}
	}

	pub fn rect_normalize(&mut self) {
		match &self.shape {
			Shape::Circle(c) => {
				let position = self.style.position;
				self.style.volume = Rect {
					min: (position - Vec2 {x: c.radius, y: c.radius}).to_pos2(),
					max: (position + Vec2 {x: c.radius, y: c.radius}).to_pos2(),
				}
			},
			Shape::Rectangle(r) => {
				let position = self.style.position;
				self.style.volume = Rect {
					min: position.to_pos2(),
					max: (position + r.bottom_right_point).to_pos2(),
				}
			},
			Shape::Text(_) => todo!(),
			Shape::CubicBezier(c) => {
				let rect = c.get_rectangle();
				self.style.volume = Rect {
					min: (rect.min.to_vec2() + self.style.position).to_pos2(),
					max: (rect.max.to_vec2() + self.style.position).to_pos2(),
				};
			},
			Shape::Image(i) => {
				let position = self.style.position;
				self.style.volume = Rect {
					min: position.to_pos2(),
					max: (position + i.bottom_right_point).to_pos2(),
				}
			}
		}
	}

	pub fn get_rect(&self, size: &Vec2, offect: Option<Vec2>) -> Rect {
		let offect_vec = match offect {
			Some(t) => t,
			None => Vec2 {x: 0.0, y: 0.0},
		};
		let mut rect_back = self.style.volume;
		if !self.style.if_absolute {
			rect_back.min = (rect_back.min.to_vec2()/100.0 * *size + offect_vec).to_pos2();
			rect_back.max = (rect_back.max.to_vec2()/100.0 * *size + offect_vec).to_pos2();
		}else {
			rect_back.min = (rect_back.min.to_vec2() + offect_vec).to_pos2();
			rect_back.max = (rect_back.max.to_vec2() + offect_vec).to_pos2();
		}
		rect_back
	}

	pub fn animate(&mut self, timer: &mut Timer) -> Result<Vec<Back>,ShapoError> {
		let mut vec_back = Vec::new();
		for a in &mut self.animation {
			if a.if_animating {
				let time_read = timer.read()?;
				let delay:u128;
				if let Some(t) = a.start_time {
					delay = t;
				}else {
					delay = 0;
				}
				if time_read < delay {
					vec_back.push(Back::Nothing);
				}else {
					let x;
					if time_read > a.animate_time + delay{
						a.if_animating = false;
						vec_back.push(Back::AnimateDone(timer.id));
						x = 1.0;
					}else {
						x = (time_read - delay) as f32 / a.animate_time as f32;
					}
					let length = a.caculate(x)?;
					match &a.style {
						StyleAnimate::Position(t) => self.style.position = arc_length(length, t)?,
						StyleAnimate::Size(t) => self.style.size = arc_length(length, t)?,
						StyleAnimate::Rotate => self.style.rotate = length,
						StyleAnimate::Fill => todo!(),
						StyleAnimate::Stroke => todo!(),
						StyleAnimate::RoutateCenter(t) => self.style.rotate_center = arc_length(length, t)?,
						StyleAnimate::Volume(_) => todo!(),
						StyleAnimate::TextSize => self.style.text_size = length,
						StyleAnimate::ShapeAnimate(animate) => {
							match animate {
								ShapeAnimate::Rectangle(ra) => {
									if let Shape::Rectangle(rect) = &mut self.shape {
										match ra {
											RectangleAnimate::BottomRightPoint(t) => {
												rect.bottom_right_point = arc_length(length, t)?;
											}
										}
									}
								},
								ShapeAnimate::Circle(ca) => {
									if let Shape::Circle(cir) = &mut self.shape {
										match ca {
											CircleAnimate::Radius => {
												cir.radius = length;
											}
										}
									}
								},
								ShapeAnimate::Bezier(cba) => {
									if let Shape::CubicBezier(cb) = &mut self.shape {
										match cba {
											CubicBezierAnimate::Point1(t) => {
												cb.points[0] = arc_length(length, t)?.to_pos2();
											},
											CubicBezierAnimate::Point2(t) => {
												cb.points[1] = arc_length(length, t)?.to_pos2();
											},
											CubicBezierAnimate::Point3(t) => {
												cb.points[2] = arc_length(length, t)?.to_pos2();
											},
											CubicBezierAnimate::Point4(t) => {
												cb.points[3] = arc_length(length, t)?.to_pos2();
											},
										}
									}
								},
							}
						},
						StyleAnimate::Alpha => {
							fn hard_compresser(input: f32) -> f32 {
								if input > 1.0{
									return 1.0;
								}else if input < 0.0 {
									return 0.0;
								}
								input
							}
							self.style.fill = Color32::from_rgba_unmultiplied(self.style.fill.r(),self.style.fill.g(),self.style.fill.b(),(hard_compresser(length) * 255.0) as u8);
						}
					}
				}
			}
		};
		Ok(vec_back)
	}

	pub fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, timer: &mut Vec<Timer>, offect: Option<Vec2>, if_enabled: bool, texture: &HashMap<TextureId,TextureHandle>) -> Result<Vec<Back>, ShapoError>{
		let mut vec_back = vec!(); 
		if let Some((s,e)) = self.sustain_time {
			for a in &mut *timer {
				if a.read()? < s || a.read()? > e {
					return Ok(vec_back);
				}
			}
		}
		match &mut self.shape {
			Shape::Circle(cir) => cir.render(ui, size, offect, &self.style)?,
			Shape::Rectangle(rec) => rec.render(ui, size, offect, &self.style)?,
			Shape::Text(strings) => strings.render(ui, size, offect, &self.style)?,
			Shape::CubicBezier(cb) => cb.render(ui, size, offect, &self.style)?,
			Shape::Image(img) => vec_back.push(img.render(ui,size,offect,&self.style,&texture)?),
		}
		if if_enabled {
			for a in timer {
				self.trig_animation(a.id);
				return self.animate(a); 
			}
		}
		Ok(vec_back)
	}

	pub fn from_string(string: String, position: Vec2, fill: Color32, rect: Option<Rect>, text_size: f32, ui: Option<&mut egui::Ui>, size: &Vec2) -> Self {
		let volume = match rect {
			Some(t) => t,
			None => {
				let mut got = get_rect(string.clone(), text_size, ui.unwrap(), size);
				got.min = got.min + position;
				got.max = got.max + position;
				got
			}
		};
		Self {
			style: Style::new(position, fill, volume, None),
			shape: Shape::Text(Text::new_from_string(string)),
			label: None,
			animation: Vec::new(),
			..Default::default()
		}
	}

	pub fn from_language(string: Language, position: Vec2, fill: Color32, rect: Rect) -> Self {
		Self {
			style: Style::new(position, fill, rect, None),
			label: None,
			shape: Shape::Text(Text::new(string)),
			animation: Vec::new(),
			..Default::default()
		}
	}

	pub fn from_rect(position: Vec2, bottom_right_point: Vec2, rounding: Rounding, fill: Color32, volume: Rect, layer: Option<egui::layers::LayerId>) -> Self{
		Self {
			style: Style::new(position, fill, volume, layer),
			shape: Shape::Rectangle(Rectangle::new(bottom_right_point, rounding)),
			label: None,
			animation: Vec::new(),
			..Default::default()
		}
	}

	pub fn empty(volume: Rect) -> Self {
		Self {
			shape: Shape::Rectangle(Rectangle{
				bottom_right_point: volume.max.to_vec2(),
				..Default::default()
			}),
			style: Style {
				volume,
				fill: Color32::TRANSPARENT,
				..Default::default()
			},
			..Default::default()
		}
	}
}

impl Default for Shape {
	fn default() -> Self {
		Shape::Rectangle(Rectangle::default())
	}
}

pub fn rotate(rotate_center: Vec2, vec_to_rotate: Vec2, rotate: f32) -> Vec2 {
	let mut delta = vec_to_rotate - rotate_center;
	let middle = Vec2 {
		x: delta.x * rotate.cos() - delta.y * rotate.sin(),
		y: delta.x * rotate.sin() + delta.y * rotate.cos()
	};
	delta = middle + rotate_center;
	delta
}