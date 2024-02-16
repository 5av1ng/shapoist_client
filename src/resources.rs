use nablo::prelude::*;
use anyhow::*;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

pub enum Icon {
	Home,
	Shop,
	Setting,
	Add,
	Search,
	Filter,
	More,
	Edit,
}

impl Icon {
	#[cfg(not(target_arch = "wasm32"))]
	pub fn init(ui: &mut Ui) -> Result<()> {
		log::info!("inititialing icons...");
		let path_read = fs::read_dir("./assets/icons")?;
		for path in path_read {
			let path = path?.path();
			log::debug!("find: {}", path.display());
			if path.is_file() && path.extension().is_some() {
				let extension = path.extension().unwrap();
				if let Some(t) = extension.to_str() {
					if t == "png" {
						log::debug!("latest finding is a png file, processing...");
						ui.create_texture_from_path(path.clone(), path.file_name().unwrap().to_str().unwrap())?;
					}
				}
			}
		}
		Ok(())
	}

	#[cfg(target_arch = "wasm32")]
	pub fn init(ui: &mut Ui) -> Result<()> {
		ui.create_texture(include_bytes!("../assets/icons/add.png"), "add.png")?;
		ui.create_texture(include_bytes!("../assets/icons/filter.png"), "filter.png")?;
		ui.create_texture(include_bytes!("../assets/icons/home.png"), "home.png")?;
		ui.create_texture(include_bytes!("../assets/icons/more.png"), "more.png")?;
		ui.create_texture(include_bytes!("../assets/icons/setting.png"), "setting.png")?;
		ui.create_texture(include_bytes!("../assets/icons/search.png"), "search.png")?;
		ui.create_texture(include_bytes!("../assets/icons/shop.png"), "shop.png")?;
		ui.create_texture(include_bytes!("../assets/icons/user.png"), "user.png")?;
		ui.create_texture(include_bytes!("../assets/icons/edit.png"), "edit.png")?;
		Ok(())
	}

	pub fn draw(&self, painter: &mut Painter, size: Vec2) {
		match &self {
			Self::Home => painter.image("home.png", size),
			Self::Shop => painter.image("shop.png", size),
			Self::Setting => painter.image("setting.png", size),
			Self::Add => painter.image("add.png", size),
			Self::Search => painter.image("search.png", size),
			Self::Filter => painter.image("filter.png", size),
			Self::More => painter.image("more.png", size),
			Self::Edit => painter.image("edit.png", size),
		};
	}
}