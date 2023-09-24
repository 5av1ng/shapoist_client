use std::ops::Sub;
use std::ops::Add;
use crate::ASSETS_PATH;
use egui::Align;
use std::collections::HashMap;
use egui::TextureHandle;
use crate::ui::ui::Back;
use egui::TextureId;
use crate::setting::setting::read_settings;
use egui::Rect;
use crate::system::system_function::load_image;
use egui::Ui;
use crate::ShapoError;
use crate::ui::shape::style::Style;
use egui::epaint::textures::TextureOptions;
use egui::Vec2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Image {
	pub name: String,
	pub first_path: Path,
	pub path: String,
	pub bottom_right_point: Vec2,
	pub registered_info: Option<TextureId>,
	pub if_keep: bool
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Path {
	Styles,
	Chart
}

impl Default for Image {
	fn default() -> Self {
		Self {
			name: String::from("Test"),
			first_path: Path::Styles,
			path: String::from("Icons/setting.png"),
			bottom_right_point: Vec2{x: 0.0, y: 0.0},
			if_keep: false,
			registered_info: None
		}
	}
}

impl Image {
	pub fn render(&mut self, ui: &mut Ui, size: &Vec2, offect: Option<Vec2>, style: &Style, texture: &HashMap<TextureId,TextureHandle>) -> Result<Back, ShapoError> {
		let handle:TextureHandle;
		let setting  = read_settings()?;
		let path = match self.first_path {
			Path::Styles => format!("{}/assets/styles/{}/{}",*ASSETS_PATH ,setting.ui_theme ,self.path),
			Path::Chart => format!("{}/assets/chart/{}",*ASSETS_PATH ,self.path),
		};
		if let None = self.registered_info {
			let image = load_image(&path)?;
			handle = ui.ctx().load_texture(self.name.clone(), image, TextureOptions::LINEAR);
			self.registered_info = Some(handle.id());
			return Ok(Back::LoadedTexture((handle.id(),handle)))
		}else {
			handle = texture.get(&self.registered_info.unwrap()).unwrap().clone()
		}
		let mut actual_position = style.get_position(size, offect).to_pos2();
		let mut actual_bottom_left_point = style.get_vec2(size,offect,style.position + self.bottom_right_point).to_pos2();
		let mut image_size= style.get_vec2(size,None,self.bottom_right_point);
		if self.if_keep {
			actual_bottom_left_point.y = actual_position.y + (actual_bottom_left_point.x - actual_position.x) * self.bottom_right_point.y / self.bottom_right_point.x;
			image_size.y = image_size.x  * self.bottom_right_point.y / self.bottom_right_point.x;
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
		ui.put(Rect{min: actual_position, max: actual_bottom_left_point}, egui::widgets::Image::new(texture.get(&handle.id()).unwrap(),image_size));
		Ok(Back::Nothing)
	}
}

impl Add for Image {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self {
			name: self.name,
			first_path: self.first_path,
			path: self.path,
			bottom_right_point: self.bottom_right_point + other.bottom_right_point,
			registered_info: self.registered_info,
			if_keep: other.if_keep,
		}
	}
}

impl Sub for Image {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {
			name: self.name,
			first_path: self.first_path,
			path: self.path,
			bottom_right_point: self.bottom_right_point - other.bottom_right_point,
			registered_info: self.registered_info,
			if_keep: other.if_keep,
		}
	}
}