use std::time::UNIX_EPOCH;
use crate::ShapoError;
use std::time::SystemTime;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum TimerError {
	Running,
	Paused,
	CouldNotRead(String),
	CouldNotSet(String)
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Timer {
	last_pause_time: Option<i64>,
	last_start_time: Option<i64>,
	if_paused:bool,
	pub id: usize
}

impl Default for Timer {
	fn default() -> Self {
		Self {
			last_pause_time: None,
			last_start_time: None,
			if_paused: true,
			id: 0
		}
	}
}

impl Timer {
	pub fn new(id: usize) -> Self {
		Self {
			last_pause_time: None,
			last_start_time: None,
			if_paused: true,
			id
		}
	}

	pub fn start(&mut self) -> Result<(),ShapoError> {
		if self.if_paused {
			let time_read = read_time()?;
			let last_start_time = match self.last_start_time {
				Some(t) => {
					let pause = match self.last_pause_time {
						Some(t) => t,
						None => 0,
					};
					t + time_read - pause
				},
				None => time_read,
			};
			self.last_start_time = Some(last_start_time);
			self.if_paused = false;
			Ok(())
		}else {
			Err(ShapoError::TimerError(TimerError::Running))
		}
	}

	// pub fn pause(&mut self) -> Result<(),ShapoError> {
	// 	if !self.if_paused {
	// 		let time_read = read_time()?;
	// 		let last_pause_time = Some(time_read);
	// 		self.if_paused = true;
	// 		self.last_start_time = last_pause_time;
	// 		Ok(())
	// 	}else {
	// 		Err(ShapoError::TimerError(TimerError::Paused))
	// 	}
	// }

	pub fn read(&self) -> Result<i64,ShapoError> {
		let time: i64;
		if self.if_paused {
			let pause = match self.last_pause_time {
				Some(t) => t,
				None => return Err(ShapoError::TimerError(TimerError::CouldNotRead(String::from("havn't pause yet"))))
			};
			let start = match self.last_start_time {
				Some(t) => t,
				None => return Err(ShapoError::TimerError(TimerError::CouldNotRead(String::from("havn't start yet"))))
			};
			time = pause - start;
			Ok(time)
		}else {
			let now = read_time()?;
			let start = match self.last_start_time {
				Some(t) => t,
				None => return Err(ShapoError::TimerError(TimerError::CouldNotRead(String::from("havn't start yet"))))
			};
			time = now - start;
			Ok(time) 
		}
	}

	pub fn set(&mut self, delay: i64) -> Result<(),ShapoError> {
		let last_start_time = match self.last_start_time {
			Some(t) => t - delay,
			None => return Err(ShapoError::TimerError(TimerError::CouldNotSet(String::from("havn't start yet"))))
		};
		self.last_start_time = Some(last_start_time);
		Ok(())
	}
}

fn read_time() -> Result<i64, ShapoError> {
	let time = SystemTime::now().duration_since(UNIX_EPOCH);
	match time {
		Ok(t) => return Ok(t.as_micros() as i64),
		Err(e) => return Err(ShapoError::TimerError(TimerError::CouldNotRead(e.to_string()))),
	}
}

impl Copy for Timer {}