use crate::system::system_function::read_every_file;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::ShapoError;
use crate::system::system_function::remove_path;
use crate::clear_log;

pub fn check() -> Result<(),ShapoError> {
	match clear_log() {
		Ok(()) => {},
		Err(_) => return init()
	}
	let files = match read_every_file("storage/emulated/0/Shapoist/Chart"){
		Ok(t) => t,
		Err(_) => vec!(),
	};
	if !files.is_empty() {
		for a in files {
			let split:Vec<&str> = a.split(".").collect();
			if split.len() >= 2 {
				if split[split.len() - 1] == "shapoistcompress" {
					let file = match File::open(a.clone()){
						Ok(t) => t,
						Err(e) => return Err(ShapoError::SystemError(format!("error because: {}", e.to_string())))
					};
					let tar = GzDecoder::new(file);
					let mut archive = Archive::new(tar);
					match archive.unpack(format!("data/data/com.saving.shapoist/assets/chart/")){
						Ok(_) => {}
						Err(e) => return Err(ShapoError::SystemError(e.to_string()))
					};
				}
			}
		}
	}
	return Ok(());
}

pub fn init() -> Result<(),ShapoError> {

	let _ = remove_path("data/data/com.saving.shapoist/assets");

	let path = include_bytes!("../../assets.tar.gz");
	let tar = GzDecoder::new(&path[..]);
	let mut archive = Archive::new(tar);
	match archive.unpack("data/data/com.saving.shapoist/"){
		Ok(_) => return Ok(()),
		Err(e) => return Err(ShapoError::SystemError(e.to_string()))
	};
}