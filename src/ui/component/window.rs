use crate::ASSETS_PATH;
use crate::play::note::Chart;
use crate::ui::shape::image::*;
use crate::ui::shape::rectangle::Rectangle;
use crate::ui::component::combo_box::ComboBox;
use crate::system::system_function::prase_json_form_path;
use crate::ui::component::inputbox::InputBox;
use crate::ui::page::Temp;
use egui::DroppedFile;
use crate::system::system_function::to_json;
use crate::system::system_function::prase_json;
use crate::ui::shape::style::Style;
use crate::ui::shape::text::Text;
use crate::ui::shapo::Shape;
use crate::system::system_function::read_every_file;
use crate::setting::setting::read_settings;
use std::collections::HashMap;
use egui::TextureId;
use egui::TextureHandle;
use crate::log_export::log_export::print_log;
use crate::system::system_function::read_file;
use crate::language::language::Mess;
use egui::Rounding;
use crate::ui::shapo::Shapo;
use egui::{Pos2, Color32, Rect, Vec2, Align2};
use crate::ShapoError;
use crate::language::language::Language;
use crate::play::timer::Timer;
use crate::play::note::PossibleChartChange;
use crate::ui::component::button::*;
use crate::ui::ui::*;
use crate::play::play_top::PlayInfo;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Window {
	pub position: Vec2,
	pub size: Vec2,
	pub resizeable: Option<Vec2>,
	pub draggable: bool,
	pub title: Option<Language>,
	pub model: bool,
	pub id: usize,
	pub content: Vec<Component>,
	pub if_enabled: bool,
	pub label: Option<Vec<String>>,
	pub if_labeled: bool,
	pub scrollable: Option<[bool;2]>,
}

impl Default for Window {
	fn default() -> Self {
		let mut button = Button::default();
		button.shape[0].style.volume = Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 100.0, y: 100.0} };
		button.shape[0].style.fill = Color32::from_rgba_premultiplied(0,0,0,0);
		let content = vec!(Component::Button(Button::default()), Component::Button(button));
		Self {
			position: Vec2 { x: 0.0, y: 0.0},
			size: Vec2{ x: 10.0, y: 10.0 },
			resizeable: Some(Vec2 {x: 10.0, y: 10.0}),
			draggable: true,
			title: Some(Language::Code(0)),
			model: false,
			id: 1000,
			content,
			if_enabled: true,
			label: None,
			if_labeled: false,
			scrollable: None
		}
	}
}

impl Window {
	pub fn render(&mut self, size: &Vec2, timer: &mut Vec<Timer>, ctx: &egui::Context, out_ui: &mut egui::Ui, texture: &HashMap<TextureId,TextureHandle>, if_enabled: bool, file: &Vec<DroppedFile>,temp: &Temp) 
	-> Result<Vec<Back>, ShapoError> {
		if !self.label.is_none() && !self.if_labeled && if_enabled {
			self.handle_label(file, temp)?;
		}
		let mut window:egui::Window;
		match &self.title {
			Some(t) => {
				window = egui::Window::new(t.get_language()?);
			},
			None => {
				window = egui::Window::new(format!("{:#?}{}",self.label, self.id)).title_bar(false);
			}
		}
		if let Some(t) = self.scrollable {
			window = window.scroll2(t);
		}
		window = window.enabled(self.if_enabled);
		window = window.scroll2([true,true]);
		if self.model {
			window = window.anchor(Align2::CENTER_CENTER, Vec2{x:0.0,y:0.0});
			window = window.fixed_size(self.size/100.0 * *size);
			window = window.resizable(false);
			window = window.movable(false);
			window = window.collapsible(false);
			let click_outside = Button::new(Some(Logic::Ignore),
					vec!(Shapo::from_rect(
						Vec2 { x: 0.0, y: 0.0},
						Vec2 { x: 100.0, y: 100.0 },
						Rounding::none(),
						Color32::from_rgba_premultiplied(0,0,0,50),
						Rect{
							min: Vec2{x:0.0,y:0.0}.to_pos2(), 
							max: Vec2{ x: 100.0, y: 100.0 }.to_pos2()
						},
						Some(egui::layers::LayerId{
							order: egui::layers::Order::PanelResizeLine,
							id: egui::Id::new(rand::random::<i32>())
						}))
					),
					None
				).render(out_ui, size, &mut Vec::new(), None, true, texture, temp)?;
			if let Some(t) = window.show(ctx, |ui| -> Result<Vec<Back>,ShapoError> { 
				let mut vec_push = Vec::new();
				let offect = Vec2{x: ui.max_rect().left(), y: ui.cursor().top()};
				let width = Vec2{x: ui.available_width(), y:ui.available_height()};
				for a in &mut self.content {
					for b in a.render(ui, &width, timer, Some(offect), true, ctx, texture, temp)? {
						vec_push.push(b);
					}
				}
				Ok(vec_push)
			}) {
				if let Some(b) = t.inner {
					let mut pull = b?;
					for a in click_outside {
						pull.push(a);
					}
					return Ok(pull);
				} 
			}
		}else if if_enabled {
			match &self.resizeable {
				Some(t) => {
					window = window.default_size(self.size/100.0 * *size);
					window = window.min_width(t.x/100.0 * size.x);
					window = window.min_height(t.y/100.0 * size.y);
					window = window.resizable(true);
				},
				None => {
					window = window.fixed_size(self.size/100.0 * *size);
					window = window.resizable(false);
				}
			}
			window = window.interactable(if_enabled);
			window = window.movable(self.draggable);
			if self.draggable {
				window = window.default_pos((self.position/100.0 * *size).to_pos2());
			}else {
				window = window.fixed_pos((self.position/100.0 * *size).to_pos2());
			}
			if let Some(t) = window.show(ctx, |ui| -> Result<Vec<Back>,ShapoError> { 
				let mut vec_push = Vec::new();
				let offect = Vec2{x: ui.max_rect().left(), y: ui.cursor().top()};
				let width = Vec2{x: ui.available_width(), y:ui.available_height()};
				for a in &mut self.content {
					for b in a.render(ui, &width, timer, Some(offect), self.if_enabled, ctx, texture, temp)? {
						vec_push.push(b);
					}
				}
				Ok(vec_push)
			}) {
				if let Some(b) = t.inner {
					return Ok(b?);
				} 
			}
		}
		return Ok(vec!(Back::Nothing));
	}

	pub fn error_model(message: String, ui: &mut egui::Ui) -> Self {
		let size = Vec2{x: ui.available_width(), y:ui.available_height()};
		let text = Shapo::from_string(message, Vec2{x: 0.0, y: 0.0 }, Color32::WHITE, Some(Rect{
			min: Vec2{ x: 0.0, y: 0.0 }.to_pos2(),
			max: Vec2{ x: 100.0, y: 80.0 }.to_pos2()
		}), 13.0, Some(ui), &size);
		let mut window = Window::default();
		window.id = 0;
		window.model = true;
		window.size = Vec2{ x: 70.0, y: 30.0 };
		window.title = Some(Language::Error(Mess::Code(0)));
		window.content[0] = Component::Button(Button{
			shape: vec!(text),
			..Button::default()
		});
		window.content.push(Component::Button(Button::new(
			Some(Logic::Ignore), 
			vec!(
				Shapo::from_language(Language::Error(Mess::Code(1)), 
				Vec2{ x: 25.0, y: 80.0 }, 
				Color32::WHITE, 
				Rect{
					min: Vec2{ x: 0.0, y: 80.0 }.to_pos2(),
					max: Vec2{ x: 50.0, y: 100.0 }.to_pos2()
				}))
			, None)));
		window.content.push(Component::Button(Button::new(
			Some(Logic::Close), 
			vec!(Shapo::from_language(
					Language::Error(Mess::Code(2)), 
					Vec2{ x: 75.0, y: 80.0 }, 
					Color32::WHITE, 
					Rect{
						min: Vec2{ x: 50.0, y: 80.0 }.to_pos2(),
						max: Vec2{ x: 100.0, y: 100.0 }.to_pos2()
					}
				))
			, None)));
		window
	}

	pub fn from_path(path: String) -> Result<Self, ShapoError> {
		let file_read = read_file(&path)?;
		let window_read: Window = prase_json(&file_read)?;
		Ok(window_read)
	}

	pub fn from_label(label: Option<Vec<String>>) -> Self {
		Self {
			label,
			..Default::default()
		}
	}

	fn edit_window_new(&mut self, _: &Vec<DroppedFile>, temp: &Temp) -> Result<(),ShapoError> {
		let music_path = match read_every_file("storage/emulated/0/Shapoist/Music") {
			Ok(t) => t,
			Err(_) => vec!(), 
		};
		let image_path = match read_every_file("storage/emulated/0/Shapoist/Image"){
			Ok(t) => t,
			Err(_) => vec!(), 
		};
		let mut music_path_json = vec!();
		let mut image_path_json = vec!();
		for a in &music_path {
			music_path_json.push(to_json(&a.clone())?);
		}
		for a in &image_path {
			image_path_json.push(to_json(&a.clone())?);
		}
		let setting = read_settings()?;
		let mut window:Window = prase_json_form_path(&format!("{}/assets/styles/{}/Window/Window.json", *ASSETS_PATH, setting.ui_theme))?;
		let content = vec!(
			Component::InputBox(InputBox{
				shape: vec!(),
				text: temp.chart.songtitle.clone(),
				hover_text: Language::Code(32),
				change_type: ChangeType::ChartTemp(PossibleChartChange::Songtitle)
			}),
			Component::InputBox(InputBox{
				shape: vec!(),
				text: temp.chart.mapname.clone(),
				hover_text: Language::Code(33),
				change_type: ChangeType::ChartTemp(PossibleChartChange::Mapname)
			}),
			Component::InputBox(InputBox{
				shape: vec!(),
				text: temp.chart.bpm.to_string(),
				hover_text: Language::Code(34),
				change_type: ChangeType::ChartTemp(PossibleChartChange::Bpm)
			}),
			Component::InputBox(InputBox{
				shape: vec!(),
				text: temp.chart.producer.clone(),
				hover_text: Language::Code(35),
				change_type: ChangeType::ChartTemp(PossibleChartChange::Producer)
			}),
			Component::InputBox(InputBox{
				shape: vec!(),
				text: temp.chart.charter.clone(),
				hover_text: Language::Code(36),
				change_type: ChangeType::ChartTemp(PossibleChartChange::Charter)
			}),
			Component::InputBox(InputBox{
				shape: vec!(),
				text: temp.chart.painter.clone(),
				hover_text: Language::Code(37),
				change_type: ChangeType::ChartTemp(PossibleChartChange::Painter)
			}),
			Component::ComboBox(ComboBox{
				shape: vec!(),
				name: Language::Code(38),
				value: to_json(&temp.music_path.clone())?,
				value_show: temp.music_path.clone(),
				possible_value_json: music_path_json.clone(),
				possible_value_show: music_path.clone(),
				change_type: ChangeType::MuiscPath
			}),
			Component::ComboBox(ComboBox{
				shape: vec!(),
				name: Language::Code(39),
				value: to_json(&temp.image_path.clone())?,
				value_show: temp.image_path.clone(),
				possible_value_json: image_path_json,
				possible_value_show: image_path,
				change_type: ChangeType::ImagePath
			}),
			Component::Button(Button{
				shape: vec!(Shapo {
					shape: Shape::Text(Text{
						text: Language::Code(40)
					}),
					style: Style {
						position: Vec2 {x: 70.0 , y: 95.0},
						volume: Rect { 
							min: Vec2 {x: 50.0 , y: 90.0}.to_pos2(), 
							max: Vec2 {x: 100.0 , y: 100.0}.to_pos2() 
						},
						fill: Color32::from_rgba_premultiplied(255,255,255,255),
						anchor: Align2::CENTER_CENTER,
						..Default::default()
					},
					..Default::default()
				}),
				..Default::default()
			}),
			Component::Button(Button{
				shape: vec!(Shapo {
					shape: Shape::Text(Text{
						text: Language::Code(41)
					}),
					style: Style {
						position: Vec2 {x: 0.0 , y: 95.0},
						volume: Rect { 
							min: Vec2 {x: 0.0 , y: 90.0}.to_pos2(), 
							max: Vec2 {x: 50.0 , y: 100.0}.to_pos2() 
						},
						fill: Color32::from_rgba_premultiplied(255,255,255,255),
						anchor: Align2::LEFT_CENTER,
						..Default::default()
					},
					..Default::default()
				}),
				click_logic: Some(Logic::NewProject),
				..Default::default()
			}),
			Component::Button(Button{
				shape: vec!(Shapo {
					shape: Shape::Text(Text{
						text: Language::Code(43)
					}),
					style: Style {
						position: Vec2 {x: 0.0 , y: 85.0},
						volume: Rect { 
							min: Vec2 {x: 0.0 , y: 80.0}.to_pos2(), 
							max: Vec2 {x: 50.0 , y: 90.0}.to_pos2() 
						},
						fill: Color32::from_rgba_premultiplied(255,255,255,255),
						anchor: Align2::LEFT_CENTER,
						..Default::default()
					},
					..Default::default()
				}),
				click_logic: Some(Logic::OpenWindow(WindowToOpen::FromPath("Window/Edit.json".to_string()))),
				..Default::default()
			}),
		);
		window.content = content;
		window.id = 1001;
		*self = window;
		self.if_labeled = true;
		Ok(())
	}

	fn edit_window(&mut self, temp: &Temp) -> Result<(),ShapoError> {
		let projects = read_every_file(&format!("{}/assets/chart", *ASSETS_PATH))?;
		let mut choice = vec!();
		let mut choice_json = vec!();
		for a in projects {
			let slice = utf8_slice::from(&a, ASSETS_PATH.len() + 14);
			choice.push(slice.to_string());
			choice_json.push(to_json(&a)?);
		}
		let setting = read_settings()?;
		let mut window:Window = prase_json_form_path(&format!("{}/assets/styles/{}/Window/Window.json",*ASSETS_PATH , setting.ui_theme))?;
		let content = vec!(
			Component::ComboBox(ComboBox{
				shape: vec!(),
				name: Language::Code(46),
				value: to_json(&temp.now_project_path.clone())?,
				value_show: temp.now_project_path.clone(),
				possible_value_json: choice_json,
				possible_value_show: choice,
				change_type: ChangeType::ProjectPath
			}),
			Component::Button(Button{
				shape: vec!(Shapo {
					shape: Shape::Text(Text{
						text: Language::Code(156)
					}),
					style: Style {
						position: Vec2 {x: 0.0 , y: 75.0},
						volume: Rect { 
							min: Vec2 {x: 0.0 , y: 70.0}.to_pos2(), 
							max: Vec2 {x: 50.0 , y: 80.0}.to_pos2() 
						},
						fill: Color32::from_rgba_premultiplied(255,255,255,255),
						anchor: Align2::LEFT_CENTER,
						..Default::default()
					},
					..Default::default()
				}),
				click_logic: Some(Logic::Remove(PathToRemove::FromTemp)),
				..Default::default()
			}),
			Component::Button(Button{
				shape: vec!(Shapo {
					shape: Shape::Text(Text{
						text: Language::Code(45)
					}),
					style: Style {
						position: Vec2 {x: 0.0 , y: 95.0},
						volume: Rect { 
							min: Vec2 {x: 0.0 , y: 90.0}.to_pos2(), 
							max: Vec2 {x: 50.0 , y: 100.0}.to_pos2() 
						},
						fill: Color32::from_rgba_premultiplied(255,255,255,255),
						anchor: Align2::LEFT_CENTER,
						..Default::default()
					},
					..Default::default()
				}),
				click_logic: Some(Logic::OpenProject),
				..Default::default()
			}),
			Component::Button(Button{
				shape: vec!(Shapo {
					shape: Shape::Text(Text{
						text: Language::Code(44)
					}),
					style: Style {
						position: Vec2 {x: 0.0 , y: 85.0},
						volume: Rect { 
							min: Vec2 {x: 0.0 , y: 80.0}.to_pos2(), 
							max: Vec2 {x: 100.0 , y: 90.0}.to_pos2() 
						},
						fill: Color32::from_rgba_premultiplied(255,255,255,255),
						anchor: Align2::LEFT_CENTER,
						..Default::default()
					},
					..Default::default()
				}),
				click_logic: Some(Logic::OpenWindow(WindowToOpen::FromPath("Window/EditNew.json".to_string()))),
				..Default::default()
			}),
		);
		window.content = content;
		window.id = 1001;
		*self = window;
		self.if_labeled = true;
		Ok(())
	}

	fn play_window(&mut self, split: Vec<&str>) -> Result<(),ShapoError> {
		let charts = read_every_file(&format!("{}/assets/chart", *ASSETS_PATH))?;
		let page_number: i32 = match split[2].parse(){
			Ok(t) => t,
			Err(_) => {
				print_log(&format!("[ERROR] invailed page number"));
				unreachable!()
			}
		};
		let delta = charts.len() as i32 - 9 * (page_number - 1);
		let mut if_next_page =  false;
		let mut if_previous_page =  false;
		if delta > 9 {
			if_next_page = true;
		}
		if page_number > 1 {
			if_previous_page = true;
		}
		if delta < 0 {
			print_log(&format!("[ERROR] invailed page number"));
			return Err(ShapoError::SystemError(format!("invailed page number")))
		}else {
			let setting = read_settings()?;
			let file_read = read_file(&format!("{}/assets/styles/{}/Component/ChartCard.json", *ASSETS_PATH, setting.ui_theme))?;
			let mut button = Button::default();
			button.shape[0].style.volume = Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 100.0, y: 100.0} };
			button.shape[0].style.fill = Color32::from_rgba_premultiplied(0,0,0,0);
			let card_read: Button = prase_json(&file_read)?;
			let mut component_vec = vec!(Component::Button(button));
			for card_number in 0..delta as usize {
				if card_number >= 9 {
					break;
				}
				let mut card_clone = card_read.clone();
				for b in &mut card_clone.shape {
					if b.label.is_none() {
						b.style.position = Vec2 {
							x: (card_number % 3) as f32 * 24.0 + (card_number % 3 + 1) as f32 * 7.0 + 15.0, 
							y: ((card_number - card_number % 3)/3) as f32 * 30.0  + 17.0
						};
					}else {
						for c in b.label.clone().unwrap() {
							if c == "Image".to_string() {
								let image_path = format!("{}/image.png",&charts[card_number + 9 * (page_number as usize - 1)][ASSETS_PATH.len() + 14..]);
								if let Shape::Image(t) = &mut b.shape {
									t.path = image_path
								}else {
									unreachable!()
								}
								b.style.position = Vec2 {
									x: (card_number % 3) as f32 * 24.0 + (card_number % 3 + 1) as f32 * 7.0 + 15.0, 
									y: ((card_number - card_number % 3)/3) as f32 * 30.0  + 17.0
								};
							}else if c == "Title".to_string() {
								let title = format!("{}",&charts[card_number + 9 * (page_number as usize - 1)][ASSETS_PATH.len() + 14..]);
								if let Shape::Text(t) = &mut b.shape {
									*t = Text::new_from_string(title);
								}else {
									unreachable!()
								}b.style.position = Vec2 {
									x: (card_number % 3) as f32 * 24.0 + (card_number % 3 + 1) as f32 * 7.0 + 5.0, 
									y: ((card_number - card_number % 3)/3) as f32 * 30.0  + 7.0
								};
							}
						}
					}
					b.style.volume = Rect{ 
						min: b.style.position.to_pos2(), 
						max: Vec2 {
							x: (card_number % 3 + 1) as f32 * 24.0 + (card_number % 3 + 1) as f32 * 7.0 + 15.0, 
							y: ((card_number - card_number % 3)/3 + 1) as f32 * 30.0  + 17.0
						}.to_pos2()
					};
				}
				card_clone.click_logic = Some(Logic::OpenWindow(WindowToOpen::FromLabel(Some(vec!("Chart".to_string(), format!("{}",charts[card_number + 9 * (page_number as usize - 1)]))))));  
				component_vec.push(Component::Button(card_clone.clone()));
			}
			if if_next_page {
				component_vec.push(Component::Button(Button{
					shape: vec!(Shapo {
						shape: Shape::Text(Text{
							text: Language::Code(8)
						}),
						style: Style {
							position: Vec2 {x: 90.0 , y: 95.0},
							volume: Rect { 
								min: Vec2 {x: 70.0 , y: 90.0}.to_pos2(), 
								max: Vec2 {x: 100.0 , y: 100.0}.to_pos2() 
							},
							fill: Color32::from_rgba_premultiplied(255,255,255,255),
							anchor: Align2::CENTER_CENTER,
							..Default::default()
						},
						..Default::default()
					}),
					click_logic: Some(Logic::OpenWindow(WindowToOpen::FromLabelAndId(Some(vec!(format!("Chart Page {}", page_number + 1))), 1001))),
					..Default::default()
				}));
			}

			if if_previous_page {
				component_vec.push(Component::Button(Button{
					shape: vec!(Shapo {
						shape: Shape::Text(Text{
							text: Language::Code(146)
						}),
						style: Style {
							position: Vec2 {x: 10.0 , y: 95.0},
							volume: Rect { 
								min: Vec2 {x: 0.0 , y: 90.0}.to_pos2(), 
								max: Vec2 {x: 30.0 , y: 100.0}.to_pos2() 
							},
							fill: Color32::from_rgba_premultiplied(255,255,255,255),
							anchor: Align2::CENTER_CENTER,
							..Default::default()
						},
						..Default::default()
					}),
					click_logic: Some(Logic::OpenWindow(WindowToOpen::FromLabelAndId(Some(vec!(format!("Chart Page {}", page_number - 1))), 1001))),
					..Default::default()
				}));
			}
			
			*self = Window {
				content: component_vec,
				..Self::from_path(format!("{}/assets/styles/{}/Window/Window.json",*ASSETS_PATH , setting.ui_theme))?
			};
			self.if_labeled = true;
		}
		Ok(())
	}

	fn chart_window(&mut self, label: Vec<String>) -> Result<(),ShapoError> {
		let chart:Chart = prase_json_form_path(&format!("{}/map.shapoistmap",&label[1]))?;
		let info = PlayInfo::read(chart.mapname.clone());
		let mut button = Button::default();
		button.shape[0].style.volume = Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 100.0, y: 100.0} };
		button.shape[0].style.fill = Color32::from_rgba_premultiplied(0,0,0,0);
		let setting = read_settings()?;
		let component_vec = vec!(Component::Button(button),
			Component::Shapo(vec!(Shapo {
					shape: Shape::Image(Image {
						name: String::from(&label[1][ASSETS_PATH.len() + 14..]),
						first_path: Path::Chart,
						path: format!("{}/image.png",&label[1][ASSETS_PATH.len() + 14..]),
						bottom_right_point: Vec2{x: 100.0,y: 100.0},
						..Default::default()
					}),
					..Default::default()
				},
				Shapo {
					shape: Shape::Rectangle(Rectangle {
						bottom_right_point: Vec2{x: 100.0,y: 100.0},
						..Default::default()
					}),
					style: Style {
						fill: Color32::from_rgba_premultiplied(0,0,0,120),
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Text(format!("{}",chart.songtitle))
					}),
					style: Style {
						position: Vec2{x: 0.0, y: 55.0},
						text_size: 36.0,
						volume: Rect{min: Vec2::new(0.0,55.0).to_pos2(), max: Vec2::new(65.0,55.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Text(format!("{}{}",Language::Code(35).get_language()?,chart.producer))
					}),
					style: Style {
						position: Vec2{x: 0.0, y: 70.0},
						text_size: 16.0,
						volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(65.0,70.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Text(format!("{}{}",Language::Code(36).get_language()?,chart.charter))
					}),
					style: Style {
						position: Vec2{x: 0.0, y: 75.0},
						text_size: 16.0,
						volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(65.0,70.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Text(format!("{}{}",Language::Code(37).get_language()?,chart.painter))
					}),
					style: Style {
						position: Vec2{x: 0.0, y: 80.0},
						text_size: 16.0,
						volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(65.0,70.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Text(format!("{}{}",Language::Code(34).get_language()?,chart.bpm))
					}),
					style: Style {
						position: Vec2{x: 0.0, y: 85.0},
						text_size: 16.0,
						volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(65.0,70.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Text(format!("{}{}",Language::Code(145).get_language()?,chart.length as f64 / 1e6))
					}),
					style: Style {
						position: Vec2{x: 0.0, y: 90.0},
						text_size: 16.0,
						volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(65.0,70.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Code(135)
					}),
					style: Style {
						position: Vec2{x: 65.0, y: 78.0},
						text_size: 16.0,
						volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(30.0,70.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Text(format!("{}{:.2}%",Language::Code(143).get_language()?, info.accuracy * 100.0))
					}),
					style: Style {
						position: Vec2{x: 65.0, y: 75.0},
						text_size: 12.0,
						volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(30.0,70.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				},
				Shapo {
					shape: Shape::Text(Text {
						text: Language::Text(info.score.to_string())
					}),
					style: Style {
						position: Vec2{x: 65.0, y: 80.0},
						text_size: 32.0,
						volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(30.0,70.0).to_pos2()},
						..Default::default()
					},
					..Default::default()
				})
			),
			Component::Button(Button {
				shape: vec!(Shapo {
					shape: Shape::Image(Image {
						name: String::from(&label[1][ASSETS_PATH.len() + 14..]),
						first_path: Path::Styles,
						path: format!("Icons/Play.png"),
						if_keep: true,
						bottom_right_point: Vec2{x: 10.0,y: 10.0},
						..Default::default()
					}),
					style: Style {
						position: Vec2{x: 87.0, y: 80.0},
						volume: Rect{min: Vec2::new(75.0,75.0).to_pos2(), max: Vec2::new(100.0,100.0).to_pos2()},
						..Default::default()
					}, 
					..Default::default()
				}),
				click_logic: Some(Logic::To(Router::PlayPage(format!("{}",&label[1][ASSETS_PATH.len() + 14..])))),
				..Default::default()
			}),
			Component::Button(Button {
				shape: vec!(Shapo {
					shape: Shape::Image(Image {
						name: String::from(&label[1][ASSETS_PATH.len() + 14..]),
						first_path: Path::Styles,
						path: format!("Icons/Close.png"),
						if_keep: true,
						bottom_right_point: Vec2{x: 5.0,y: 5.0},
						..Default::default()
					}),
					style: Style {
						position: Vec2{x: 95.0, y: 0.0},
						volume: Rect{min: Vec2::new(75.0,0.0).to_pos2(), max: Vec2::new(100.0,25.0).to_pos2()},
						..Default::default()
					}, 
					..Default::default()
				}),
				click_logic: Some(Logic::CloseWindow(1003)),
				..Default::default()
			})
		);
		let mut window = Self::from_path(format!("{}/assets/styles/{}/Window/Window.json",*ASSETS_PATH , setting.ui_theme))?;
		window.id = 1003;
		window.size = Vec2 {x: 82.0, y: 70.0};
		window.position = Vec2 {x: 10.0, y: 10.0};
		window.content = component_vec;
		*self = window;
		self.if_labeled = true;
		Ok(())
	}

	pub fn pause_window() -> Result<Self, ShapoError> {
		let content = vec!(
			Component::Button(Button {
				shape: vec!(
					Shapo {
						shape: Shape::Image(Image {
							name: String::from("Close"),
							if_keep: true,
							path: format!("Icons/Close.png"),
							bottom_right_point: Vec2{x: 20.0,y: 20.0},
							..Default::default()
						}),
						style: Style {
							position: Vec2{x: 25.0, y: 50.0},
							volume: Rect{min: Vec2::new(0.0,0.0).to_pos2(), max: Vec2::new(33.0,100.0).to_pos2()},
							anchor: Align2::CENTER_CENTER, 
							..Default::default()
						},
						..Default::default()
					}
				),
				click_logic: Some(Logic::To(Router::MainPage)),
				..Default::default()
			}),
			Component::Button(Button {
				shape: vec!(
					Shapo {
						shape: Shape::Image(Image {
							name: String::from("Play"),
							if_keep: true,
							path: format!("Icons/Play.png"),
							bottom_right_point: Vec2{x: 20.0,y: 20.0},
							..Default::default()
						}),
						style: Style {
							position: Vec2{x: 50.0, y: 50.0},
							volume: Rect{min: Vec2::new(33.0,0.0).to_pos2(), max: Vec2::new(66.0,100.0).to_pos2()},
							anchor: Align2::CENTER_CENTER, 
							..Default::default()
						},
						..Default::default()
					}
				),
				click_logic: Some(Logic::Play),
				..Default::default()
			}),
			Component::Button(Button {
				shape: vec!(
					Shapo {
						shape: Shape::Image(Image {
							name: String::from("Retry"),
							if_keep: true,
							path: format!("Icons/Retry.png"),
							bottom_right_point: Vec2{x: 20.0,y: 20.0},
							..Default::default()
						}),
						style: Style {
							position: Vec2{x: 75.0, y: 50.0},
							volume: Rect{min: Vec2::new(66.0,0.0).to_pos2(), max: Vec2::new(100.0,100.0).to_pos2()},
							anchor: Align2::CENTER_CENTER, 
							..Default::default()
						},
						..Default::default()
					}
				),
				click_logic: Some(Logic::Retry),
				..Default::default()
			}),
		);
		Ok(Self {
			title: None,
			resizeable: None,
			draggable: false,
			size: Vec2{x: 70.0 ,y: 30.0},
			position: Vec2{x:15.0, y: 35.0},
			id: 1004,
			content,
			..Default::default()
		})
	}

	fn handle_label(&mut self, file: &Vec<DroppedFile>, temp: &Temp) -> Result<(), ShapoError> {
		let label = self.label.clone().unwrap();
		if label.len() == 2 && label[0] == "Chart".to_string() {
			self.chart_window(label)?;
			return Ok(());
		}
		for a in label {
			if a == "Setting" {
				*self = read_settings()?.ui()?;
				self.if_labeled = true;
				return Ok(());
			}else if a == "EditNew" {
				self.edit_window_new(file, temp)?;
				return Ok(());
			}else if a == "Edit" {
				self.edit_window(temp)?;
				return Ok(())
			}
			let split: Vec<&str> = a.split(" ").collect();
			if split.len() == 3 && split[0] == "Chart" {
				self.play_window(split)?;
				return Ok(());
			}
		}
		self.if_labeled = true;
		Ok(())
	}

	pub fn test() -> Self {
		// let mut button = Button::default();
		// button.shape[0].style.volume = Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 100.0, y: 100.0} };
		// button.shape[0].style.fill = Color32::from_rgba_premultiplied(0,0,0,0);
		// let content = vec!(
		// 	Component::Button(button)
		// );
		// let back = Self {
		// 	title: Some(Language::Code(9)),
		// 	resizeable: Some(Vec2 {x: 20.0, y: 20.0}),
		// 	draggable: true,
		// 	size: Vec2{x:30.0,y: 70.0},
		// 	position: Vec2{x:15.0, y: 15.0},
		// 	id: 999999,
		// 	content,
		// 	..Default::default()
		// };
		// println!("{}",to_json(&back).unwrap());
		// back
		Self::pause_window().unwrap()
	}
}