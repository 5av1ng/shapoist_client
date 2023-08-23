use crate::ASSETS_PATH;
use crate::log_export::log_export::print_log;
use rand::Rng;
use crate::setting::setting::read_settings;
use crate::system::system_function::read_file_split;
use crate::ShapoError;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Language {
	Code(usize),
	Error(Mess),
	Text(String)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Mess {
	Code(usize),
	Text(String)
}

impl Language {
	pub fn get_language(&self) -> Result<String, ShapoError> {
		match self {
			Language::Code(t) => {
				let setting = read_settings()?;
				let path = format!("{}/assets/language/{}/language.ini", *ASSETS_PATH , &setting.language);
				let language = read_file_split(&path)?;
				if t < &language.len() {
					let send = &language[*t];
					Ok(send.to_string())
				}else {
					print_log(&format!("[ERROR] The language code didn't exist"));
					Err(ShapoError::SystemError(format!("[ERROR] The language code didn't exist")))
				}	
			},
			Language::Error(t) => {
				match t {
					Mess::Code(a) => {
						let setting = match read_settings() {
							Ok(t) => t.language,
							Err(_) => "en-US".to_string(),
						};
						let path = format!("{}/assets/language/{}/error.ini",*ASSETS_PATH , &setting);
						let language = match read_file_split(&path) {
							Ok(u) => u,
							Err(_) => vec!("Error! ".to_string(),"Ignore".to_string(), "Exit".to_string())
						};
						let back = &language[*a];
						return Ok(back.to_string())
					},
					Mess::Text(s) => return Ok(s.to_string())
				}
			}
			Language::Text(t) => {
				return Ok(t.to_string());
			} 
		}
	}

	pub fn random_tip() -> Result<Self,ShapoError> {
		let setting = read_settings()?;
		let path = format!("{}/assets/language/{}/tip.ini",*ASSETS_PATH , &setting.language);
		let language = read_file_split(&path)?;
		if language.is_empty(){
			print_log(&format!("[ERROR] tip file is empty"));
			Err(ShapoError::SystemError(format!("[ERROR] tip file is empty")))
		}else {
			let random_number = rand::thread_rng().gen_range(0..language.len());
			Ok(Self::Text(language[random_number].clone()))
		}
	}
}