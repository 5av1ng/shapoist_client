use crate::log::log_export::print_log;
use egui::Color32;
use crate::error::error::PraseError;
use pest::iterators::Pair;
use crate::ShapoError;
use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;

// parser("
// fn oh(b: Bool, c: String) -> Number {
// 	let a = b.to_number()
// 	let i = 0
// 	sin(a)
// 	loop {
// 		i = i + 1
// 		c = a.to_sting() + (c + c)
// 		if i > 0 {
// 			break
// 		}
// 	}
// 	return a + c.len() 
// 	// uwu
// }
// oh()
// /*owo*/
// ".to_string());

#[derive(Parser)]
#[grammar = "system/shapo_language.pest"]
pub struct ShapoParser;

pub enum Variable {
	Number(f32),
	String(String),
	Vector(Vec<f32>),
	Bool(bool),
	Matrix(Vec<Vec<f32>>) // 行向量形式存储，如果各个 Vec 长度不等则末位补零
}

enum Operator {
	Add,
	Minus,
	Mul,
	Pow,
	Mod,
	And,
	Or,
	Dot,
	ParenthesesLeft,
	ParenthesesRight
}

struct Function {
	input: HashMap<String, Variable>,
	output: HashMap<String, Variable>,
	code: String
}

struct Storge {
	variable: HashMap<String, Variable>,
	function: HashMap<String, Function>
}

pub fn parser(code: String) -> Result<HashMap<String, Variable>,ShapoError> {
	let code_parsed = prase_code(&code);
	println!("{:#?}", code_parsed);
	let mut storge = Storge::new();
	for mut statement in code_parsed?.into_inner() {
		match statement.as_rule() {
			Rule::If => {
				let _ = handle_operator(&mut storge, &mut statement);
			},
			Rule::Function => {},
			Rule::Let => {},
			Rule::Const => {},
			Rule::Note1 => {},
			Rule::Note2 => {},
			Rule::Loop => {},
			Rule::FunctionUse => {},
			Rule::VariableFunction => {},
			Rule::VariableChange => {},
			Rule::WHITESPACE => {}
			_=> unreachable!()
		}
	}
	let _storge = Storge::new();
	Ok(HashMap::new())
}

fn prase_code(code: &str) -> Result<Pair<'_, Rule>, ShapoError> {
	let code_parsed = ShapoParser::parse(Rule::Statements, code);
	match code_parsed {
		Ok(mut t) => {
			match t.next() {
				Some(f) => {
					return Ok(f)
				},
				None => return Err(ShapoError::PraseError(PraseError::NoInput))
			}
		},
		Err(e) => { 
			println!("{}", e.variant.message());
			return Err(ShapoError::PraseError(PraseError::PraseError(e)));
		}
	}
}

fn handle_operator(_input: &mut Storge, prase: &mut Pair<'_, Rule>) -> Result<Storge, ShapoError> {
	for operator in prase.clone().into_inner() {
		match operator.as_rule() {
			Rule::Parentheses => {},
			Rule::Number => {},
			Rule::Bool => {},
			Rule::Condition => {},
			Rule::FunctionUse => {},
			Rule::VariableFunction => {}, 
			Rule::Text => {},
			Rule::String => {},
			Rule::Vector => {},
			Rule::Matrix => {},
			Rule::Add => {},
			Rule::Minus => {},
			Rule::Mul => {},
			Rule::Divide => {},
			Rule::Pow => {},
			Rule::Mod => {},
			Rule::And => {},
			Rule::Or => {}, 
			Rule::Dot => {},
			_ => unreachable!(),

		}
	}
	Ok(Storge::new())
}

pub fn shader_praser(input: String, time: u128, size: [usize;2]) -> Result<Vec<Color32>,ShapoError> {
	let mut vec_to_return = vec!();
	for y in 0..size[1]{
		for x in 0..size[0] {
			let result = parser(format!("fn shader(time: Number, size: Vector) -> Vector {{ {0} }} let output = shader({1},{2})",input,time,format!("[{},{}]", x, y)))?;
			let mut out = result.get("output").unwrap();
			if let Variable::Vector(t) = &mut out {
				let mut a = vec!();
				for b in t {
					if b > &1.0 {
						a.push(1.0)
					}else if b < &0.0 {
						a.push(0.0)
					}else {
						a.push(*b)
					}
				}
				if a.len() == 1 {
					vec_to_return.push(Color32::from_rgb((a[0] * 255.0) as u8, (a[0] * 255.0) as u8, (a[0] * 255.0) as u8))
				}else if a.len() == 3 {
					vec_to_return.push(Color32::from_rgb((a[0] * 255.0) as u8, (a[1] * 255.0) as u8, (a[2] * 255.0) as u8))
				}else if a.len() == 4 {
					vec_to_return.push(Color32::from_rgba_premultiplied((a[0] * 255.0) as u8, (a[1] * 255.0) as u8, (a[2] * 255.0) as u8, (a[3] * 255.0) as u8))
				}else {
					return Err(ShapoError::PraseError(PraseError::InvalidOutPut))
				}
			}else {
				print_log("[ERROR] entered unreachable place during parsing shader");
				unreachable!("")
			}
		}
	}
	Ok(vec_to_return)
}

impl Storge {
	fn new() -> Self {
		Self {
			variable: HashMap::new(),
			function: HashMap::new()
		}
	}
}