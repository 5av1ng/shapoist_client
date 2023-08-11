// use crate::system::shapo_language::Rule;
use crate::play::timer::TimerError;

#[derive(Debug)]
pub enum ShapoError {
	SystemError(String),
	ConvertError(String),
	TimerError(TimerError),
	// PraseError(PraseError),
}

// #[derive(Debug, Clone)]
// pub enum PraseError {
// 	PraseError(pest::error::Error<Rule>),
// 	InvalidOutPut,
// 	NoInput
// }