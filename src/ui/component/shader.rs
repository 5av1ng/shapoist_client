use egui::LayerId;
use crate::setting::setting::read_settings;
use egui::Rect;
use egui::pos2;
use core::f32::consts::PI;
use egui::Color32;
use egui::ImageData;
use egui::ColorImage;
use egui::epaint::textures::TextureOptions;
// use crate::system::shapo_language::shader_praser;
use egui::TextureId;
use crate::ShapoError;
use crate::ui::ui::Back;
use crate::play::timer::Timer;
use egui::Vec2;
use std::thread;
use std::sync::mpsc;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Shader {
	pub position:Vec2,
	pub size: [usize; 2],
	pub zoom: Vec2,
	pub code: ShaderCode,
	pub timer: Option<Timer>,
	pub name: String,
	pub repeat_x: usize,
	pub repeat_y: usize,
	pub registered_info: Option<TextureId>
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ShaderCode {
	Code(String),
	Built(Built)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum Built {
	MeshWithLight,
}

impl Shader {
	pub fn render(&mut self, ui: &mut egui::Ui, size: &Vec2, _: &mut Vec<Timer>, _: Option<Vec2>, if_enabled: bool, ctx: &egui::Context) -> Result<Vec<Back>,ShapoError> {
		let setting = read_settings()?;
		if !setting.if_shader {
			return Ok(vec!())
		}
		let time:u128;
		if let Some(t) = self.timer {
			time = t.read()?;
		}else {
			let mut timer = Timer::new(0);
			timer.start()?;
			time = timer.read()?;
			self.timer = Some(timer);
		}
		let vec_back = vec!();
		if let None = self.registered_info {
			let handle = ctx.load_texture(self.name.clone(), ColorImage::example(), TextureOptions::LINEAR);
			self.registered_info = Some(handle.id());
		}
		let mut new_texture;
		if if_enabled {
			match &self.code {
				ShaderCode::Code(_) => {
					todo!()
					// new_texture = ColorImage { 
					// 	pixels: shader_praser(t.to_string(),time,self.size)?,
					// 	size: self.size
					// };
				},
				ShaderCode::Built(u) => {
					new_texture = ColorImage { 
						pixels: u.render(&self.size, time)?,
						size: self.size
					};
				}
			}
			let grid = new_texture.clone();
			for _ in 1..self.repeat_y * self.repeat_x {
				for a in &grid.pixels {
					new_texture.pixels.push(*a)
				}
			}
			new_texture.size = [self.size[0] * self.repeat_x, self.size[1] * self.repeat_y];
			let id = ctx.load_texture(self.name.clone(), ImageData::Color(new_texture), TextureOptions::LINEAR).id();
			self.registered_info = Some(id);
			ui.ctx().layer_painter(LayerId::background()).image(self.registered_info.unwrap(), 
				Rect{min:self.position.to_pos2(),max: (*size * self.zoom).to_pos2()}, 
				Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), 
				Color32::WHITE);
			
		}
		Ok(vec_back)
	}
}

impl Built {
	fn render(&self, size: &[usize;2], time: u128) -> Result<Vec<Color32>,ShapoError> {
		let mut out_vec = vec!();
		let mut thread_handler = vec!();
		let setting = read_settings()?;
		let other_size = size.clone();
		let (tx, rx) = mpsc::channel();
		let (tx1, rx1) = mpsc::channel();
		let (tx2, rx2) = mpsc::channel();
		let (tx3, rx3) = mpsc::channel();
		let (tx4, rx4) = mpsc::channel();
		let (tx5, rx5) = mpsc::channel();
		let tx_vec = vec!(tx,tx1,tx2,tx3,tx4,tx5);
		let thread_number = 7;
		for i in 2..=thread_number {
			let input_tx = tx_vec.clone();
			thread_handler.push(thread::spawn(move || {
				let mut out = vec!();
				for y in other_size[0] * (i - 1) / thread_number..other_size[0] * i / thread_number {
					for x in 0..other_size[1] {
						match self {
							Built::MeshWithLight => {
								out.push(mesh_with_light(x, y, &other_size, time, setting.background_color));
							},
						}
					}
				}
				let _ = input_tx[i - 2].send(out);
			}));
		}
		for y in 0..size[0] / thread_number {
			for x in 0..size[1] {
				match self {
					Built::MeshWithLight => {
						out_vec.push(mesh_with_light(x, y, size, time, setting.background_color));
					},
				}
			}
		}
		for a in rx.recv().unwrap() {
			out_vec.push(a)
		}
		for a in rx1.recv().unwrap() {
			out_vec.push(a)
		}
		for a in rx2.recv().unwrap() {
			out_vec.push(a)
		}
		for a in rx3.recv().unwrap() {
			out_vec.push(a)
		}
		for a in rx4.recv().unwrap() {
			out_vec.push(a)
		}
		for a in rx5.recv().unwrap() {
			out_vec.push(a)
		}
		// Ok(convolver(out_vec,size))
		Ok(out_vec)
	}
}

// fn convolver(input: Vec<Color32>,size: &[usize;2]) -> Vec<Color32> {
// 	let mut output = input.clone();
// 	fn average(input: &Vec<u8>, power: &Vec<f32>) -> u8 {
// 		let mut sum: f32 = 0.0;
// 		for a in 0..input.len() {
// 			sum = sum + input[a] as f32 * power[a];
// 		}
// 		sum as u8
// 	}

// 	let power = vec!(0.05,0.1,0.05,0.1,0.4,0.1,0.01,0.1,0.05);

// 	for y in 0..size[1]{
// 		for x in 0..size[0] {
// 			let now = x + y * size[0];
// 			for i in 0..=3 {
// 				let mut to_average = vec!();
// 				for j in -1..=1 {
// 					for k in -1..=1 {
// 						let summary = now as i32 + k + j * size[0] as i32;
// 						if summary > 0 && summary < (size[1] * size[0]) as i32 {
// 							to_average.push(input[summary as usize][i]);
// 						}
// 					}
// 				}
// 				output[now][i] = average(&to_average,&power);
// 			}
// 		}
// 	}
// 	output
// }

fn mesh_with_light(x: usize,y:usize,size: &[usize;2], time: u128, background_color: Color32) -> Color32 {
	let mut u = (x as f32 - size[1] as f32 / 2.0) / size[1] as f32;
	let mut v = (y as f32 - size[0] as f32 / 2.0) / size[0] as f32;
	u = u.abs();
	v = v.abs();
	let mut final_color = [0.0;4];
	let t = 1.0;
	let mut distance:f32;
	let time_s = time as f32 / 1e6;
	let d = (((u.powf(t) + v.powf(t))).powf(1.0/t) + time_s * PI).sin() * 0.5 + 0.5;
	for i in 0..2 {
		u = repeat(u * 1.5) - 0.5;
		v = repeat(v * 1.5) - 0.5;
		distance = ((u.powf(t) + v.powf(t))).powf(1.0/t);
		distance = (distance * 8.0 + time_s).sin().abs();
		distance = 0.02 / distance * (1.0/(i as f32 + 1.0));
		let colorize = colorize([[0.0,1.0,1.0,0.0];4], d + distance);
		for a in 0..final_color.len() {
			final_color[a] = final_color[a] + distance * colorize[a]
		}
	}
	for a in 0..final_color.len() {
		final_color[a] = final_color[a].abs().powf(0.5) * 2.0;
	}
	u = (x as f32 - size[1] as f32 / 2.0) / size[1] as f32;
	v = (y as f32 - size[0] as f32 / 2.0) / size[0] as f32;
	let d = - ((u.powf(2.0) + v.powf(2.0)).powf(0.5)) / f32::sqrt(2.0) + 0.7;
	for a in 0..final_color.len() {
		final_color[a] = final_color[a] * d
	}
	color_normalized(final_color[0] * background_color[0] as f32 / 255.0,final_color[1] * background_color[1] as f32 / 255.0,final_color[2] * background_color[2] as f32 / 255.0,1.0)
}

fn color_normalized(a: f32,b: f32,c: f32,d: f32) -> Color32 {
	let a = (hard_compresser(a, 0.0, 1.0) * 255.0) as u8;
	let b = (hard_compresser(b, 0.0, 1.0) * 255.0) as u8;
	let c = (hard_compresser(c, 0.0, 1.0) * 255.0) as u8;
	let d = (hard_compresser(d, 0.0, 1.0) * 255.0) as u8;
	Color32::from_rgba_unmultiplied(a,b,c,d)
}

fn hard_compresser(input: f32, min: f32, max: f32) -> f32 {
	if input > max {
		max
	}else if input < min{
		min
	}else {
		input
	}
}

fn repeat(input:f32) -> f32 {
	input - input.floor()
}

fn colorize(color: [[f32;4];4], input: f32) -> [f32; 4] {
	let mut output = [1.0;4];
	for a in 0..color.len() {
		output[a] = color[a][0] + color[a][1] * (0.5 * f32::cos(2.0 * PI *(color[a][2]* input + color[a][3]) + 0.5))
	}
	output
}