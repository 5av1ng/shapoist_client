use crate::ASSETS_PATH;
use crate::ui::ui::Display;
use crate::ui::component::window::Window;
use crate::ui::shape::text::Text;
use crate::ui::shape::style::Style;
use crate::ui::component::button::Logic;
use crate::ui::ui::Router;
use crate::ui::shape::rectangle::Rectangle;
use crate::language::language::Language;
use crate::ui::shapo::Shape;
use crate::ui::shape::image::*;
use egui::Vec2;
use crate::play::play_top::PlayTop;
use crate::ShapoError;
use crate::ui::shapo::Shapo;
use crate::ui::component::button::Button;
use egui::Rect;
use egui::Pos2;
use egui::Color32;
use crate::setting::setting::read_settings;
use crate::ui::ui::Component;

pub fn end_page(play_top: PlayTop) -> Result<Display, ShapoError> {
	let chart = play_top.chart.clone();
	let info = play_top.get_info();
	let delta = info.write(chart.mapname.clone())?;
	let delta_text: String;
	if delta >= 0{
		delta_text = format!("+{}", delta);
	}else {
		delta_text = format!("{}", delta);
	}
	let mut button = Button::default();
	button.shape[0].style.volume = Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 100.0, y: 100.0} };
	button.shape[0].style.fill = Color32::from_rgba_premultiplied(0,0,0,0);
	let component = Some(vec!(
		Component::Shapo(vec!(Shapo {
				shape: Shape::Image(Image {
					name: String::from(play_top.chart.mapname.clone()),
					first_path: Path::Chart,
					path: format!("{}/image.png",play_top.chart.mapname),
					bottom_right_point: Vec2{x: 100.0,y: 100.0},
					..Default::default()
				}),
				..Default::default()
			},Shapo {
				shape: Shape::Rectangle(Rectangle {
					bottom_right_point: Vec2{x: 100.0,y: 100.0},
					..Default::default()
				}),
				style: Style {
					fill: Color32::from_rgba_premultiplied(0,0,0,100),
					..Default::default()
				},
				..Default::default()
			})
		),
	));
	let setting = read_settings()?;
	let mut component_vec = vec!(Component::Button(button),
		Component::Shapo(vec!(Shapo {
				shape: Shape::Image(Image {
					name: String::from(play_top.chart.mapname.clone()),
					first_path: Path::Chart,
					path: format!("{}/image.png",play_top.chart.mapname),
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
					text: Language::Text(chart.songtitle)
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
					text: Language::Text(chart.producer)
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
					text: Language::Text(format!("{}{}  {}{}",Language::Code(138).get_language()?,info.extra_number,Language::Code(139).get_language()?,info.normal_number))
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
					text: Language::Text(format!("{}{}  {}{}",Language::Code(140).get_language()?,info.fade_number,Language::Code(141).get_language()?,info.miss_number))
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
					text: Language::Text(format!("{}{:.2}%",Language::Code(143).get_language()?, info.accuracy * 100.0))
				}),
				style: Style {
					position: Vec2{x: 65.0, y: 72.0},
					text_size: 12.0,
					volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(30.0,70.0).to_pos2()},
					..Default::default()
				},
				..Default::default()
			},
			Shapo {
				shape: Shape::Text(Text {
					text: Language::Code(142)
				}),
				style: Style {
					position: Vec2{x: 65.0, y: 75.0},
					text_size: 18.0,
					volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(30.0,70.0).to_pos2()},
					..Default::default()
				},
				..Default::default()
			},
			Shapo {
				shape: Shape::Text(Text {
					text: Language::Text(format!("{}", delta_text))
				}),
				style: Style {
					position: Vec2{x: 65.0, y: 81.0},
					text_size: 12.0,
					volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(30.0,70.0).to_pos2()},
					..Default::default()
				},
				..Default::default()
			},
			Shapo {
				shape: Shape::Text(Text {
					text: Language::Text(format!("{}", info.score))
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
					name: String::from(play_top.chart.mapname.clone()),
					first_path: Path::Styles,
					path: format!("Icons/Retry.png"),
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
			click_logic: Some(Logic::To(Router::PlayPage(format!("{}",play_top.chart.mapname)))),
			..Default::default()
		}),
		Component::Button(Button {
			shape: vec!(Shapo {
				shape: Shape::Image(Image {
					name: String::from(play_top.chart.mapname),
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
			click_logic: Some(Logic::To(Router::MainPage)),
			..Default::default()
		})
	);
	if setting.if_immaculate {
		component_vec.push(Component::Shapo(vec!(Shapo {
			shape: Shape::Text(Text {
				text: Language::Text(format!("{}{}",Language::Code(136).get_language()?,info.immaculate_number))
			}),
			style: Style {
				position: Vec2{x: 0.0, y: 75.0},
				text_size: 16.0,
				volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(65.0,70.0).to_pos2()},
				..Default::default()
			},
			..Default::default()
		})));
	}else {
		component_vec.push(Component::Shapo(vec!(Shapo {
			shape: Shape::Text(Text {
				text: Language::Text(format!("{}{}",Language::Code(137).get_language()?,info.immaculate_number))
			}),
			style: Style {
				position: Vec2{x: 0.0, y: 75.0},
				text_size: 16.0,
				volume: Rect{min: Vec2::new(0.0,70.0).to_pos2(), max: Vec2::new(65.0,70.0).to_pos2()},
				..Default::default()
			},
			..Default::default()
		})));
	}
	
	let mut window = Window::from_path(format!("{}/assets/styles/{}/Window/Window.toml",*ASSETS_PATH , setting.ui_theme))?;
	window.id = 1003;
	window.size = Vec2 {x: 82.0, y: 70.0};
	window.position = Vec2 {x: 10.0, y: 10.0};
	window.content = component_vec;
	Ok(Display {
		component,
		window: vec!(window),
		..Default::default()
	})
}