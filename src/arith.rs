/* arith.rs
 * -------- */ 
use crate::code::BinOp;
use crate::vm::Vm;
use crate::words::Word;

use ndarray::{Array, Array1, Array2, array};

type ArithResult = Result<Word, String>;

pub fn do_arith(vm: &Vm, op: BinOp, lhs: &Word, rhs: &Word) -> ArithResult {
	match lhs {
		Word::Int(l) 	=> { return do_int(*l, rhs, op) },
		Word::Num(l) 	=> { return do_num(*l, rhs, op) },
		Word::Str(l) 	=> { return do_str(&*l, rhs, op) },
		Word::Vect(l)	=> { return do_vect{&*l, rhs, op} },
		_ => return Ok(Word::Null)
	}
}

fn do_int(lhs: i32, rhs: &Word, op: BinOp) -> ArithResult {
	match rhs {
		Word::Int(r) 	=> { return do_int_int(lhs, *r, op) },
		Word::Num(r) 	=> { return do_num_num(lhs as f64, *r, op) },
		Word::Str(r) 	=> { return do_str_int(&*r, lhs, op, false) },
		Word::Vect(r)	=> { return do_vec_num(r, lhs as f64, false) },
		_ => return Ok(Word::Null)
	}
}

fn do_num(lhs: f64, rhs: &Word, op: BinOp) -> ArithResult {
	match rhs {
		Word::Int(r) 	=> { return do_num_num(lhs, *r as f64, op) },
		Word::Num(r) 	=> { return do_num_num(lhs, *r, op) },
		Word::Str(r) 	=> { return do_str_num(&*r, lhs, op, false) },
		Word::Vect(r)	=> { return do_vec_num(r, lhs, false) },
		_ => return Ok(Word::Null)
	}
}

fn do_str(lhs: &String, rhs: &Word, op: BinOp) -> ArithResult {
	match rhs {
		Word::Int(r) 	=> { return do_str_int(lhs, *r, op, true) },
		Word::Num(r) 	=> { return do_str_num(lhs, *r, op, true) },
		Word::Str(r) 	=> { return do_str_str(lhs, &*r, op) },
		_ => return Ok(Word::Null)
	}
}

fn do_int_int(lhs: i32, rhs: i32, op: BinOp) -> ArithResult {
	match op {
		BinOp::Add	=> { return Ok(Word::Int(lhs + rhs)) },
		BinOp::Sub	=> { return Ok(Word::Int(lhs - rhs)) },
		BinOp::Mul	=> { return Ok(Word::Int(lhs * rhs)) },
		BinOp::Div	=> { return Ok(Word::Num(lhs as f64 / rhs as f64)) },
		BinOp::Pow	=> { return Ok(Word::Num((lhs as f64).powi(rhs))) },
		BinOp::Mod	=> { return Ok(Word::Int(lhs % rhs)) },
		BinOp::Seq  => {
			let diff = (lhs - rhs).abs() as usize;
			return Ok(Word::Vect(Box::new(Array1::<f64>::linspace(lhs as f64, rhs as f64, diff+1))))
		},
		BinOp::Cat  => { return Ok(Word::Vect(Box::new(array![lhs as f64, rhs as f64]))) },
		BinOp::Rep  => {
			if rhs < 0 { return Err(format!("Value can only be repeated a positive number of times. Given: '{}'.", rhs)) }
			return Ok(Word::Vect(Box::new(Array1::from_elem(rhs as usize, lhs as f64))))
		},
		_ => { return Err("Binary operation not recognized!".to_string()) }
	}
}

fn do_num_num(lhs: f64, rhs: f64, op: BinOp) -> ArithResult {
	match op {
		BinOp::Add	=> { return Ok(Word::Num(lhs + rhs)) },
		BinOp::Sub	=> { return Ok(Word::Num(lhs - rhs)) },
		BinOp::Mul	=> { return Ok(Word::Num(lhs * rhs)) },
		BinOp::Div	=> { return Ok(Word::Num(lhs / rhs)) },
		BinOp::Pow	=> { return Ok(Word::Num(lhs.powf(rhs))) },
		BinOp::Mod	=> { return Ok(Word::Num(lhs % rhs)) },
		BinOp::Seq  => {
			let diff = (lhs - rhs).trunc().abs() as usize;
			return Ok(Word::Vect(Box::new(Array1::<f64>::linspace(lhs, rhs, diff+1))))
		},
		BinOp::Cat  => { return Ok(Word::Vect(Box::new(array![lhs, rhs]))) },
		BinOp::Rep  => {
			if rhs < 0.0 { return Err(format!("Value can only be repeated a positive number of times. Given: '{}'.", rhs)) }
			return Ok(Word::Vect(Box::new(Array1::from_elem(rhs as usize, lhs))))
		},
		_ => { return Err("Binary operation not recognized!".to_string()) }
	}
}

fn do_str_str(lhs: &String, rhs: &String, op: BinOp) -> ArithResult {
	match op {
		BinOp::Add 	=> { return Ok(Word::Str(Box::new(format!("{}{}", lhs, rhs)))) },
		BinOp::Sub  => { return Ok(Word::Str(Box::new(lhs.replace(rhs, "")))) },
		BinOp::Cat  => { 
			return Ok(Word::StrVect(Box::new( Array1::from_vec(vec![lhs.to_string(), rhs.to_string()]) )))
		},
		BinOp::Del	=> { 
			if lhs == rhs { return Ok(Word::StrVect(Box::new( Array1::from_vec(vec![]) ))) 
			} else { return Ok(Word::Str(Box::new(lhs.to_string()))) } 
		},
		BinOp::Spl 	=> { 
			let new_lhs = lhs.split(rhs).map(|s| s.to_string()).collect();
			return Ok(Word::StrVect(Box::new( Array1::from_vec(new_lhs) ))) 
		},
		_ => { return Err(format!("Operation cannot be completed between two string objects.")) }
	}
}

fn do_str_num(lhs: &String, rhs: f64, op: BinOp, left: bool) -> ArithResult {
	match op {
		BinOp::Add 	=> {
			if left { return Ok(Word::Str(Box::new(format!("{}{}", lhs, rhs)))) }
			return Ok(Word::Str(Box::new(format!("{}{}", rhs, lhs))))
		},
		BinOp::Cat => {
			if left { 
				return Ok(Word::StrVect(Box::new(Array1::from_vec(vec![lhs.to_string(), format!("{}", rhs)]))))
			}
			return Ok(Word::StrVect(Box::new(Array1::from_vec(vec![format!("{}", rhs), lhs.to_string()]))))
		},
		_ => { return do_str_int(lhs, rhs as i32, op, left) }
	}
}

fn do_str_int(lhs: &String, rhs: i32, op: BinOp, left: bool) -> ArithResult {
	match op {
		BinOp::Add => {
			if left { return Ok(Word::Str(Box::new(format!("{}{}", lhs, rhs)))) }
			return Ok(Word::Str(Box::new(format!("{}{}", rhs, lhs)))) 
		},
		BinOp::Mul => {
			if rhs < 0 { return Err(format!("String cannot be repeated a negative number of times.")) }
			return Ok(Word::Str(Box::new(lhs.repeat(rhs as usize))))
		},
		BinOp::Cat => {
			if left { 
				return Ok(Word::StrVect(Box::new(Array1::from_vec(vec![lhs.to_string(), format!("{}", rhs)]))))
			}
			return Ok(Word::StrVect(Box::new(Array1::from_vec(vec![format!("{}", rhs), lhs.to_string()]))))
		},
		_ => { return Err(format!("Operation cannot be completed between a string and numeric object.")) }
	}
}


/* ADDING FUNCTIONS! */
pub fn add_int_int(lhs: i32, rhs: i32) -> Word { return Word::Int(lhs + rhs) }
pub fn add_num_num(lhs: f64, rhs: f64) -> Word { return Word::Num(lhs + rhs) }
pub fn add_str_str(lhs: &String, rhs: &String) -> Word { return Word::Str(Box::new( lhs.to_string() + rhs)) }
pub fn add_str_boo(lhs: &String, rhs: bool) -> Word { return Word::Str(Box::new( format!("{}{}", lhs, rhs))) }
pub fn add_str_int(lhs: &String, rhs: i32)  -> Word { return Word::Str(Box::new( format!("{}{}", lhs, rhs))) }
pub fn add_str_num(lhs: &String, rhs: f64)  -> Word { return Word::Str(Box::new( format!("{}{}", lhs, rhs))) }
pub fn add_boo_str(lhs: bool, rhs: &String) -> Word { return Word::Str(Box::new( format!("{}{}", lhs, rhs))) }
pub fn add_int_str(lhs: i32, rhs: &String)  -> Word { return Word::Str(Box::new( format!("{}{}", lhs, rhs))) }
pub fn add_num_str(lhs: f64, rhs: &String)  -> Word { return Word::Str(Box::new( format!("{}{}", lhs, rhs))) }

pub fn add_vec_vec(lhs: &Array1<f64>, rhs: &Array1<f64>) -> Word { return Word::Vect(Box::new( lhs + rhs )) }
pub fn add_vec_boolvec(lhs: &Array1<f64>, rhs: &Array1<bool>) -> Word {
	let new_rhs = rhs.map(|i| (*i as isize) as f64);
	return Word::Vect(Box::new( lhs + new_rhs ))
}
pub fn add_boolvec_vec(lhs: &Array1<bool>, rhs: &Array1<f64>) -> Word {
	let new_lhs = lhs.map(|i| (*i as isize) as f64);
	return Word::Vect(Box::new( new_lhs + rhs ))
}
pub fn add_strvec_strvec(lhs: &Array1<String>, rhs: &Array1<String>) -> Word { 
	let new_rhs = rhs.broadcast(lhs.shape());
	let res = lhs.iter().zip(new_rhs.iter()).map(|(l,r)| format!("{}{}", l, r)).collect::<Array1<String>>();
	return Word::StrVect(Box::new(res))
}
pub fn add_strvec_vec(lhs: &Array1<String>, rhs: &Array1<f64>) -> Word {
	let new_rhs = rhs.broadcast(lhs.shape());
	let res = lhs.iter().zip(new_rhs.iter()).map(|(l,r)| format!("{}{}", l, r)).collect::<Array1<String>>();
	return Word::StrVect(Box::new(res))
}
pub fn add_vec_strvec(lhs: &Array1<f64>, rhs: &Array1<String>) -> Word {
	let new_rhs = rhs.broadcast(lhs.shape());
	let res = lhs.iter().zip(new_rhs.iter()).map(|(l,r)| format!("{}{}", l, r)).collect::<Array1<String>>();
	return Word::StrVect(Box::new(res))
}
pub fn add_strvec_boolvec(lhs: &Array1<String>, rhs: &Array1<bool>) -> Word {
	let new_rhs = rhs.broadcast(lhs.shape());
	let res = lhs.iter().zip(new_rhs.iter()).map(|(l,r)| format!("{}{}", l, r)).collect::<Array1<String>>();
	return Word::StrVect(Box::new(res))
}
pub fn add_boolvec_strvec(lhs: &Array1<String>, rhs: &Array1<bool>) -> Word {
	let new_rhs = rhs.broadcast(lhs.shape());
	let res = lhs.iter().zip(new_rhs.iter()).map(|(l,r)| format!("{}{}", l, r)).collect::<Array1<String>>();
	return Word::StrVect(Box::new(res))
}
pub fn add_boolvec_boolvec(lhs: &Array1<bool>, rhs: &Array1<bool>) -> Word {
	let new_lhs = lhs.map(|i| (*i as isize) as f64);
	let new_rhs = rhs.map(|i| (*i as isize) as f64);
	return Word::Vect(Box::new( new_lhs + new_rhs ))
}

pub fn add_vec_mat(lhs: &Array1<f64>, rhs: &Array2<f64>) -> Word { return Word::Mat( Box::new(lhs + rhs) ) }
pub fn add_mat_vec(lhs: &Array2<f64>, rhs: &Array1<f64>) -> Word { return Word::Mat( Box::new(lhs + rhs) ) }
pub fn add_mat_mat(lhs: &Array2<f64>, rhs: &Array2<f64>) -> Word { return Word::Mat( Box::new(lhs + rhs) ) }













