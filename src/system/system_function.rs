use crate::ASSETS_PATH;
use kira::sound::Region;
use kira::sound::EndPosition;
use kira::sound::PlaybackPosition;
use kira::clock::ClockHandle;
use kira::StartTime;
use kira::manager::AudioManager;
use kira::clock::ClockTime;
use kira::clock::ClockSpeed;
use crate::setting::setting::read_settings;
use egui::ColorImage;
use core::ops::RangeFrom;
use kira::sound::static_sound::StaticSoundSettings;
use kira::sound::static_sound::StaticSoundData;
use crate::error::error::*;
use std::io::{Write, BufRead};
use std::{io, fs};

pub fn create_dir(path: &str) -> Result<(), ShapoError>{
	//文件夹创建
	match fs::create_dir(&path){
		Ok(_) => {
			// print_log(&format!("path: \" {}\" created", &path));
			return Ok(());
		},
		Err(e) => {
			// print_log(&format!("[ERROR] creating path: \" {} \" failed. info: {}", &path, &e.to_string()));
			return Err(ShapoError::SystemError(format!("creating path: \" {} \" failed. info: {}", &path, &e.to_string())));
		}
	}
}

pub fn create_file(path: &str) -> Result<(), ShapoError>{
	//创建文件
	match fs::File::create(path){
		Ok(_) => {
			// print_log(&format!("file: \" {} \" created", &path));
			return Ok(());
		},
		Err(e) => {
			// print_log(&format!("[ERROR] creating file: \" {} \" failed. info: {}", &path, &e.to_string()));
			return Err(ShapoError::SystemError(format!("creating file: \" {} \" failed. info: {}", &path, &e.to_string())));
		}
	}
}

pub fn write_file(path: &str, input: &str) -> Result<(), ShapoError>{
	//写入文件
	let mut file = match fs::OpenOptions::new().append(true).open(&path) {
		Ok(t) => t,
		Err(e) => {
			// print_log(&format!("[ERROR] file \" {} \" failed to write. info: {}",&path,&e.to_string()));
			return Err(ShapoError::SystemError(format!("file \" {} \" failed to write. info: {}",&path,&e.to_string())));
		}
	};
	match file.write_all(input.as_bytes()){
		Ok(_) => {
			// print_log(&format!("file \" {} \" have written \" {} \"", &path, &input));
			return Ok(())
		},
		Err(e) => {
			// print_log(&format!("[ERROR] file \" {} \" failed to write. info: {}",&path,&e.to_string()));
			return Err(ShapoError::SystemError(format!("file \" {} \" failed to write. info: {}",&path,&e.to_string())));
		}
	};
}

pub fn read_file_split(path:&str) -> Result<Vec<String>, ShapoError>{
	let file_open = fs::File::open(&path);
	match file_open {
		Ok(file) => {
			let file_open = io::BufReader::new(file).lines();
			let mut file_lines = Vec::new();
			for line in file_open{
				if let Ok(data) = line {
					file_lines.push(data);
				}
			};
			return Ok(file_lines);
		},
		Err(e) => {
			// print_log(&format!("[ERROR] file \" {} \" failed to open. info: {}",&path,&e.to_string()));
			return Err(ShapoError::SystemError(format!("file \" {} \" failed to open. info: {}",&path,&e.to_string())));
		},
	}
}

pub fn read_file(path:&str) -> Result<String, ShapoError>{
	let file_open = fs::File::open(&path);
	match file_open {
		Ok(file) => {
			let file_open = io::BufReader::new(file).lines();
			let mut file_lines_collect = String::new();
			for line in file_open{
				if let Ok(data) = line {
					file_lines_collect = file_lines_collect + "\n" + &data;
				}
			};
			return Ok(file_lines_collect);
		},
		Err(e) => {
			// print_log(&format!("[ERROR] file: \" {} \" failed to open. info: {}",&path,&e.to_string()));
			return Err(ShapoError::SystemError(format!("file: \" {} \" failed to open. info: {}",&path,&e.to_string())));
		},
	}
}

pub fn remove_file(path:&str) -> Result<(), ShapoError> {
	match fs::remove_file(&path){
		Ok(_) => {
			// print_log(&format!("file \"{} \" have been removed", &path));
			return Ok(());
		},
		Err(e) => {
			// print_log(&format!("[ERROR] file \" {} \" failed to be removed. info: {}", &path, &e.to_string()));
			return Err(ShapoError::SystemError(format!("file \" {} \" failed to be removed. info: {}", &path, &e.to_string())));
		},
	}
}

pub fn remove_path(path:&str) -> Result<(), ShapoError> {
	match fs::remove_dir_all(&path){
		Ok(_) => {
			// print_log(&format!("file \"{} \" have been removed", &path));
			return Ok(());
		},
		Err(e) => {
			// print_log(&format!("[ERROR] file \" {} \" failed to be removed. info: {}", &path, &e.to_string()));
			return Err(ShapoError::SystemError(format!("path \" {} \" failed to be removed. info: {}", &path, &e.to_string())));
		},
	}
}

pub fn read_every_file(path: &str) -> Result<Vec<String>, ShapoError>{
	let path_read = fs::read_dir(path);
	let mut vec_back = Vec::new();
	let mut vec_err = Vec::new();
	match path_read {
		Ok(t) => {
			for path in t {
				match path {
					Ok(sth) => vec_back.push(sth.path().display().to_string()),
					Err(err) => vec_err.push("[ERROR] beacuse: ".to_string() + &err.to_string()),
				}
			}
			if vec_err.len() >= 1{
				// print_log(&format!("[ERROR] path \" {} \" failed to read. info: read nothing",&path));
				return Err(ShapoError::SystemError("read nothing".to_string()));
			}
			return Ok(vec_back);
		},
		Err(e) => {
			// print_log(&format!("[ERROR] path \" {} \" failed to read. info: {}",&path,&e.to_string()));
			return Err(ShapoError::SystemError(format!("path \" {} \" failed to read. info: {}",&path,&e.to_string())));
		},
	}
}

pub fn copy_file(file_path: &str, copy_path: &str) -> Result<(), ShapoError>{
	match fs::copy(file_path, copy_path) {
		Ok(_) => {
			// print_log(&format!("file \"{} \" have been copyed to \" {} \"", &file_path, &copy_path));
			return Ok(());
		},
		Err(e) => {
			// print_log(&format!("[ERROR] file \" {} \" failed to be copyed to \" {} \" info: {}", &file_path, &copy_path, &e.to_string()));
			return Err(ShapoError::SystemError(format!("file \" {} \" failed to be copyed to \" {} \" info: {}", &file_path, &copy_path, &e.to_string())));
		},
	}
}

pub fn clear_log() -> Result<(), ShapoError>{
	let file = read_every_file(&format!("{}/assets/log", *ASSETS_PATH))?;
	if file.len() > 5 {
		for a in 0..file.len() {
			remove_file(&file[a])?
		}
	}
	Ok(())
}

pub fn load_icon(path: &str) -> Result<eframe::IconData,ShapoError> {
	let (icon_rgba, icon_width, icon_height) = {
		let image = match image::open(path) {
			Ok(t) => t,
			Err(e) => {
				// print_log(&format!("[ERROR] read icon failed, info: {}",&e.to_string()));
				return Err(ShapoError::SystemError(format!("icon failed to be read info: {}", &e.to_string())))
			}
		}.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	// print_log(&("Icon read"));

	Ok(eframe::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	})
}

pub fn load_sound(path: &str, loop_region: Option<RangeFrom<f64>>, bpm: f32, beatnumber: f32, offect: f32, manager: &mut AudioManager) -> Result<ClockHandle, ShapoError> {
	let mut setting = StaticSoundSettings::new();
	let mut offect = offect + read_settings()?.offect / 1000.0;
	if bpm > 0.0 {
		let spb = 60.0 / bpm;
		offect = offect + beatnumber*spb;
	}
	if offect == 0.0 {
		offect = 0.0000001;
	};
	let clock_offect = match manager.add_clock(ClockSpeed::SecondsPerTick(offect.abs() as f64)) {
		Ok(t) => t,
		Err(e) => return Err(ShapoError::SystemError(e.to_string())),
	};
	if offect > 0.0 {
		setting = setting.start_time(StartTime::ClockTime(ClockTime {
			clock: clock_offect.id(),
			ticks: 1,
		}));
	}else {
		setting.playback_region = Region {
			start: PlaybackPosition::Seconds(offect.abs() as f64),
    		end: EndPosition::EndOfAudio,
		}
	}
	if let Some(t) = loop_region {
		setting = setting.loop_region(t);
	}
	let sound = match StaticSoundData::from_file(path, setting){
		Ok(t) => t,
		Err(e) => {
			// print_log(&format!("[ERROR] play sound failed, info: {}",&e.to_string()));
			return Err(ShapoError::SystemError(e.to_string()))
		},
	};
	match manager.play(sound) {
		Ok(_) => {},
		Err(e) => {
			// print_log(&format!("[ERROR] play sound failed, info: {}",&e.to_string()));
			return Err(ShapoError::SystemError(e.to_string()));
		},
	};
	match clock_offect.start() {
		Ok(_) => {},
		Err(e) => {
			// print_log(&format!("[ERROR] play sound failed, info: {}",&e.to_string()));
			return Err(ShapoError::SystemError(e.to_string()));
		},
	};
	return Ok(clock_offect);
	
}

pub fn load_image(path: &str) -> Result<ColorImage,ShapoError> {
	let image = match image::io::Reader::open(path) {
		Ok(t) => t,
		Err(e) => {
			// print_log(&format!("[ERROR] read image \"{}\" failed, info: {}",path,&e.to_string()));
			return Err(ShapoError::SystemError(format!("read image \"{}\" failed, info: {}",path,e.to_string())));
		}
	};
	let image = match image.decode() {
		Ok(t) => t,
		Err(e) => {
			// print_log(&format!("[ERROR] read image \"{}\" failed, info: {}",path,&e.to_string()));
			return Err(ShapoError::SystemError(format!("decode image \"{}\" failed, info: {}",path,e.to_string())));
		}
	};
	let size = [image.width() as _, image.height() as _];
	let binding = image.to_rgba8();
	let pixels  = binding.as_flat_samples();
	Ok(
		ColorImage::from_rgba_unmultiplied(
		size,
		pixels.as_slice(),
	))
}

pub fn to_json<T: serde::Serialize>(input: &T) -> Result<String, ShapoError> {
	match serde_json::to_string_pretty(input) {
		Ok(t) => return Ok(t),
		Err(e) => {
			// print_log(&format!("[ERROR] error while converting to json. info: {}", e.to_string()));
			return Err(ShapoError::ConvertError(format!("[ERROR] error while converting to json. info: {}", e.to_string())))
		}
	};
}

pub fn prase_json<'a, T: serde::Deserialize<'a>>(input: &'a String) -> Result<T, ShapoError> {
	match serde_json::from_str(input) {
		Ok(t) => return Ok(t),
		Err(e) => {
			// print_log(&format!("[ERROR] error while converting to variable. info: {}", e.to_string()));
			return Err(ShapoError::ConvertError(format!("[ERROR] error while converting to variable. info: {}", e.to_string())))
		}
	};
}

pub fn prase_json_form_path<T: for<'a> serde::Deserialize<'a>>(path: & str) -> Result<T, ShapoError> {
	let input = read_file(path)?;
	match serde_json::from_str(&input) {
		Ok(t) => return Ok(t),
		Err(e) => {
			// print_log(&format!("[ERROR] error while converting to variable. info: {}", e.to_string()));
			return Err(ShapoError::ConvertError(format!("[ERROR] error while converting to variable. info: {}", e.to_string())))
		}
	};
}