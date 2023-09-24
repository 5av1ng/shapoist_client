use std::num::ParseIntError;
use crate::system::command::Rule;
use crate::play::timer::TimerError;

#[derive(Debug)]
pub enum ShapoError {
	SystemError(String),
	ConvertError(String),
	TimerError(TimerError),
	ParseError(ParseError),
}

#[derive(Debug, Clone)]
pub enum ParseError {
	ParseError(pest::error::Error<Rule>),
	NaN(ParseIntError),
	InvaildCommand
}