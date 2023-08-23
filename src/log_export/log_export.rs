use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use crate::LOGPATH;
use colored::*;

pub fn print_log(strings: &str) {
	let log_text = print_out(strings);
	match OpenOptions::new().append(true).open(&*LOGPATH){
		Ok(mut log_write) => {
			if let Err(e) = log_write.write_all(log_text.as_bytes()){
				log::error!("Could not write log because error: {}", e);
			};
		},
		Err(e) => {
			log::error!("failed to write log beacuse: {}", ("\"".to_string() + &e.to_string() + &"\"").italic());
		},
	};
}

pub fn print_out(strings: &str) -> String {
	let fmt = "%Y-%m-%d %H:%M:%S";
	let now = Local::now().format(fmt).to_string();
	let log_text_split: Vec<&str> = strings.split(" ").collect();
	if log_text_split[0] == "[ERROR]"{
		let log_text_final = &strings[8..];
		log::error!("{}", log_text_final);
	}
	else{
		log::info!("{}", strings);
	}
	let log_text = format!("[{}] {}\n",&now, &strings);
	log_text
}