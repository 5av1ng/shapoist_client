use pest::iterators::Pair;
use crate::play::note::Note;
use crate::play::note::JudgeField;
use crate::ui::shapo::Shapo;
use crate::system::system_function::parse_toml;
use crate::play::note::PossibleChartSelection;
use crate::error::error::ParseError;
use crate::ShapoError;
use crate::ui::page::Temp;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./system/command.pest"]
pub struct CommandParser;

#[derive(Debug)]
pub enum DeltaType {
	Note(Note),
	JudgeField(JudgeField),
	Shape(Shapo),
	Nothing
} 

pub fn command_parse(input: &String, temp: &mut Temp, if_save_command: bool) -> Result<(), ShapoError> {
	let file = match match CommandParser::parse(Rule::Command, &input) {
		Ok(t) => t,
		Err(e) => return Err(ShapoError::ParseError(ParseError::ParseError(e)))
	}.next() {
		Some(t) => t,
		None => return Err(ShapoError::ParseError(ParseError::InvaildCommand))
	}.into_inner().next().unwrap();

	match file.as_rule() {
		Rule::Change => {
			let inner_rules = file.into_inner();
			let mut to_change = temp.project.multi_select.clone();
			for change in inner_rules {
				match change.as_rule() {
					Rule::SelectText => {},
					Rule::Vector => {
						to_change = vec!();
						let inner_rule_vec = change.into_inner();
						let mut element = PossibleChartSelection::Shape(0);
						for type_vec in inner_rule_vec {
							if let Rule::TypeVector = type_vec.as_rule() {
								let mut type_inner = type_vec.into_inner();
								match type_inner.next().unwrap().as_rule() {
									Rule::Note => {
										let judge_field_id = match type_inner.next().unwrap().as_str().parse::<usize>() {
											Ok(t) => t,
											Err(e) => return Err(ShapoError::ParseError(ParseError::NaN(e))),
										};
										element = PossibleChartSelection::Note(judge_field_id, 0);
									},
									Rule::Shape => {},
									Rule:: JudgeField => {
										element = PossibleChartSelection::JudgeField(0)
									},
									_ => todo!(),
								}
							}else if let Rule::Number = type_vec.as_rule() {
								let id = match type_vec.into_inner().next().unwrap().as_str().parse::<usize>() {
									Ok(t) => t,
									Err(e) => return Err(ShapoError::ParseError(ParseError::NaN(e))),
								};
								match element {
									PossibleChartSelection::Note(i, _) => element = PossibleChartSelection::Note(i,id),
									PossibleChartSelection::Shape(_) => element = PossibleChartSelection::Shape(id),
									PossibleChartSelection::JudgeField(_) => element = PossibleChartSelection::JudgeField(id),
								}
								to_change.push(element.clone());
							}
						}
					},
					Rule::Toml => {
						let text = change.as_str();
						let now_change = match &temp.project.now_select {
							Some(t) => t,
							None => {
								return Err(ShapoError::SystemError(String::from("no selection")))
							}
						};
						let mut delta = DeltaType::Nothing;
						match now_change {
							PossibleChartSelection::Note(i, j) => { 
								let notes = match temp.chart.note.get_mut(&i) {
									Some(t) => t,
									None => return Err(ShapoError::SystemError(String::from("invaild selection")))
								};
								let backup = notes[*j].clone();
								let note: Note = parse_toml(&text.to_string())?;
								notes[*j] = backup.clone() | note.clone();
								delta = DeltaType::Note(notes[*j].clone() - backup);
							},
							PossibleChartSelection::Shape(i) => { 
								for a in 0..temp.chart.shape.len() {
									if temp.chart.shape[a].label.len() > 0 {
										if temp.chart.shape[a].label[0] == i.to_string() {
											let backup = temp.chart.shape[a].clone();
											temp.chart.shape[a] = temp.chart.shape[a].clone() | parse_toml(&text.to_string())?;
											delta = DeltaType::Shape(temp.chart.shape[a].clone() - backup);
											break;
										}
									}
								}
							},
							PossibleChartSelection::JudgeField(i) => {
								for a in 0..temp.chart.judge_field.len() {
									if &temp.chart.judge_field[a].id == i {
										let backup = temp.chart.judge_field[a].clone();
										temp.chart.judge_field[a] = temp.chart.judge_field[a].clone() | parse_toml(&text.to_string())?;
										delta = DeltaType::JudgeField(temp.chart.judge_field[a].clone() - backup);
										break;
									}
								}
							}
						}
						for inside in &to_change {
							match inside {
								PossibleChartSelection::Note(i, j) => { 
									if let DeltaType::Note(ref n) = delta {
										let notes = match temp.chart.note.get_mut(&i) {
											Some(t) => t,
											None => return Err(ShapoError::SystemError(String::from("invaild selection")))
										};
										notes[*j] = notes[*j].clone() + n.clone();
									}
								},
								PossibleChartSelection::Shape(i) => {
									if let DeltaType::Shape(ref s) = delta {
										for a in 0..temp.chart.shape.len() {
											if temp.chart.shape[a].label.len() > 0 {
												if temp.chart.shape[a].label[0] == i.to_string() {
													temp.chart.shape[a] = temp.chart.shape[a].clone() + s.clone();
													break;
												}
											}
										}
									}
								},
								PossibleChartSelection::JudgeField(i) => {
									if let DeltaType::JudgeField(ref jf) = delta {
										for a in 0..temp.chart.judge_field.len() {
											if &temp.chart.judge_field[a].id == i {
												temp.chart.judge_field[a] = temp.chart.judge_field[a].clone() + jf.clone();
												break;
											}
										}
									}
								}
							}
						}
					},
					_ => unreachable!()
				}
			}
		},
		Rule::Repeat => {
			let mut inner_rules = file.into_inner();
			let mut reapeat_time = 1;
			let mut reapeat_id = 1;
			if let Some(t) = inner_rules.next() {
				reapeat_id = match t.as_str().parse::<usize>() {
					Ok(t) => t,
					Err(e) => return Err(ShapoError::ParseError(ParseError::NaN(e))),
				}
			}
			if let Some(t) = inner_rules.next() {
				reapeat_time = match t.as_str().parse::<usize>() {
					Ok(t) => t,
					Err(e) => return Err(ShapoError::ParseError(ParseError::NaN(e))),
				}
			}
			if reapeat_time == 0 {
				reapeat_time = 1;
			}
			if reapeat_id == 0 {
				reapeat_id = 1;
			}
			if let Some(t) = temp.commands.len().checked_sub(reapeat_id) {
				for _ in 0..reapeat_time {
					command_parse(&temp.commands[t].clone(), temp, false)?;
				}
			}else {
				return Err(ShapoError::ParseError(ParseError::InvaildCommand))
			}
		},
		Rule::Default => {
			let mut inner_rules = file.into_inner();
			match inner_rules.next().unwrap().as_str() {
				"tap" => {
					temp.project.default.tap = temp.project.default.tap.clone() | parse_toml(&inner_rules.next().unwrap().as_str().to_string())?;
				},
				"slide" => {
					temp.project.default.slide = temp.project.default.slide.clone() | parse_toml(&inner_rules.next().unwrap().as_str().to_string())?;
				},
				"rectangle" => {
					temp.project.default.rectangle = temp.project.default.rectangle.clone() | parse_toml(&inner_rules.next().unwrap().as_str().to_string())?;
				},
				"circle" => {
					temp.project.default.circle = temp.project.default.circle.clone() | parse_toml(&inner_rules.next().unwrap().as_str().to_string())?;
				},
				"bezier_curve" => {
					temp.project.default.bezier_curve = temp.project.default.bezier_curve.clone() | parse_toml(&inner_rules.next().unwrap().as_str().to_string())?;
				},
				"image" => {
					temp.project.default.image = temp.project.default.image.clone() | parse_toml(&inner_rules.next().unwrap().as_str().to_string())?;
				},
				"text" => {
					temp.project.default.text = temp.project.default.text.clone() | parse_toml(&inner_rules.next().unwrap().as_str().to_string())?;
				},
				"judge_field" => {
					temp.project.default.judge_field = temp.project.default.judge_field.clone() | parse_toml(&inner_rules.next().unwrap().as_str().to_string())?;
				},
				_ => unreachable!(),
			}
		},
		Rule::Select => {
			let mut inner_rules = file.into_inner();
			let select = inner_rules.next().unwrap();
			match select.as_rule() {
				Rule::Vector => {
					temp.project.multi_select = parse_vector(&select)?;
				},
				_ => unreachable!()
			};
		},
		Rule::Time => {
			temp.project.current_time = match file.into_inner().next().unwrap().as_str().parse::<i64>() {
				Ok(t) => t,
				Err(e) => return Err(ShapoError::ParseError(ParseError::NaN(e)))
			}
		},
		Rule::Add => {
			let mut inner_rule = file.into_inner();
			let type_rule = inner_rule.next().unwrap().as_str();
			let toml = inner_rule.next().unwrap().as_str();
			let uspb = (60.0 * 1e6 / temp.project.chart.bpm) as i64;
			fn handle_note(input: Note, toml: &str, uspb: i64, temp: &mut Temp) -> Result<(), ShapoError>{
				match temp.project.chart.note.get_mut(&temp.project.now_judge_field_id) {
					Some(t) => t,
					None => return Err(ShapoError::SystemError(String::from("invaild judge field")))
				}.push(Note {
					start_time: temp.project.current_time - 4 * uspb,
					click_time: temp.project.current_time,
					id: temp.project.now_note_id,
					..input | parse_toml(&toml.to_string())?
				});
				temp.project.now_note_id = temp.project.now_note_id + 1;

				Ok(())
			}

			fn handle_shapo(input: Shapo, toml: &str, uspb: i64, temp: &mut Temp) -> Result<(), ShapoError>{
				let out = Shapo {
					label: vec!(temp.project.now_shape_id.to_string()),
					sustain_time: Some((temp.project.current_time - 4 * uspb, temp.project.current_time)),
					..input | parse_toml(&toml.to_string())?
				};
				temp.project.chart.shape.push(out);
				temp.project.now_shape_id = temp.project.now_shape_id + 1;

				Ok(())
			}

			match type_rule {
				"tap" => {
					handle_note(temp.project.default.tap.clone(), toml, uspb, temp)?;
				},
				"slide" => {
					handle_note(temp.project.default.slide.clone(), toml, uspb, temp)?;
				}, 
				"rectangle" => {
					handle_shapo(temp.project.default.rectangle.clone(), toml, uspb, temp)?;
				},
				"circle" => {
					handle_shapo(temp.project.default.circle.clone(), toml, uspb, temp)?;
				},
				"bezier_curve" => {
					handle_shapo(temp.project.default.bezier_curve.clone(), toml, uspb, temp)?;
				},
				"image" => {
					handle_shapo(temp.project.default.image.clone(), toml, uspb, temp)?;
				},
				"text" => {
					handle_shapo(temp.project.default.text.clone(), toml, uspb, temp)?;
				},
				"judge_field" => {
					let judge_field = temp.project.default.judge_field.clone() | parse_toml(&toml.to_string())?;
					temp.project.chart.judge_field.push(JudgeField {
						id: temp.project.new_judge_field_id,
						start_time: temp.project.current_time - 4 * uspb,
						end_time: temp.project.current_time,
						..judge_field
					});
					temp.project.new_judge_field_id = temp.project.chart.judge_field.len();
				},
				_ => unreachable!(),
			}
		},
		Rule::Delete => {
			let inner_rules = file.into_inner().next().unwrap();
			let to_delete;
			match inner_rules.as_rule() {
				Rule::SelectText => {
					to_delete = temp.project.multi_select.clone();
				},
				Rule::Vector => {
					to_delete = parse_vector(&inner_rules)?;
				},
				_ => unreachable!()
			}

			for a in to_delete {
				match a {
					PossibleChartSelection::Note(a,b) => {
						let note = match temp.project.chart.note.get_mut(&a) {
							Some(t) => t,
							None => return Err(ShapoError::SystemError(String::from("invaild judge field")))
						};
						note[b].if_delete = true
					},
					PossibleChartSelection::JudgeField(a) => {
						temp.project.chart.judge_field[a].if_delete = true;
						temp.project.chart.note.remove(&a);
					},
					PossibleChartSelection::Shape(i) => {
						for a in 0..temp.project.chart.shape.len() {
							if temp.project.chart.shape[a].label[0] == i.to_string() {
								temp.project.chart.shape[a].if_delete = true;
							}
						}
					}
				}
			}
		},
		Rule::Undo => todo!(),
		Rule::Redo => todo!(),
		Rule::Play => todo!(),
		Rule::Pause => todo!(),
		Rule::Save => todo!(),
		Rule::Export => todo!(),
		Rule::Quit => todo!(),
		Rule::Restore => todo!(),
		Rule::Import => todo!(),
		Rule::Open => todo!(),
		_ => unreachable!(),
	}

	if if_save_command {
		temp.commands.push(input.to_string());
	}

	Ok(())
}

fn parse_vector(input: &Pair<'_, Rule>) -> Result<Vec<PossibleChartSelection>, ShapoError> {
	let mut back = vec!();
	let inner_rule_vec = input.clone().into_inner();
	let mut element = PossibleChartSelection::Shape(0);
	for type_vec in inner_rule_vec {
		if let Rule::TypeVector = type_vec.as_rule() {
			let mut type_inner = type_vec.into_inner();
			match type_inner.next().unwrap().as_rule() {
				Rule::Note => {
					let judge_field_id = match type_inner.next().unwrap().as_str().parse::<usize>() {
						Ok(t) => t,
						Err(e) => return Err(ShapoError::ParseError(ParseError::NaN(e))),
					};
					element = PossibleChartSelection::Note(judge_field_id, 0);
				},
				Rule::Shape => {},
				Rule:: JudgeField => {
					element = PossibleChartSelection::JudgeField(0)
				},
				_ => todo!(),
			}
		}else if let Rule::Number = type_vec.as_rule() {
			let id = match type_vec.into_inner().next().unwrap().as_str().parse::<usize>() {
				Ok(t) => t,
				Err(e) => return Err(ShapoError::ParseError(ParseError::NaN(e))),
			};
			match element {
				PossibleChartSelection::Note(i, _) => element = PossibleChartSelection::Note(i,id),
				PossibleChartSelection::Shape(_) => element = PossibleChartSelection::Shape(id),
				PossibleChartSelection::JudgeField(_) => element = PossibleChartSelection::JudgeField(id),
			}
			back.push(element.clone());
		}
	}

	Ok(back)
}