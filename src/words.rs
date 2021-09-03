/* words.rs
 * -------- */
use crate::code::Opcode;

use core::f64::NAN;
use ndarray::{Array1, Array2};
use std::collections::HashMap;

pub type BoolResult = Result<bool, String>;
pub type StaxResult = Result<Option<Word>, String>;
pub type WordResult = Result<Word, String>;

#[derive(Debug, Clone)]
pub enum Word {
	Null,
	Bool(bool),
	BoolVect(Box<Array1<bool>>),
	Dict(Box<HashMap<String, Word>>),
	Int(i32),
	List(Box<Vec<Word>>),
	MacroOp(Opcode),
	Mat(Box<Array2<f64>>),
	Num(f64),
	Program(Box<Vec<Opcode>>),
	Quote(Box<Vec<Opcode>>),
	Str(Box<String>),
	StrVect(Box<Array1<String>>),
	Sym(Box<String>),
	Vect(Box<Array1<f64>>)
}


impl Word {
	pub fn sprint(&self) -> String {
		match self {
			Word::Null 			=> { return format!("none") },
			Word::Bool(b) 		=> { return format!("{}", b) },
			Word::BoolVect(b)	=> { return format!("{}", b) },
			Word::Dict(_) 		=> { return format!("dict") },
			Word::Int(i) 		=> { return format!("{}", i) },
			Word::List(l) 		=> { return format!("[{}]", l.iter().map(|x| x.sprint_short(0)).collect::<Vec<String>>().join(" ")) },
			Word::MacroOp(_)	=> { return format!("macro_op") },
			Word::Mat(m) 		=> { return format!("{}", m) },
			Word::Num(f) 		=> { return format!("{}", f) },
			Word::Program(_) 		=> { return format!("program") },
			Word::Quote(_) 		=> { return format!("quote") },
			Word::Str(s) 		=> { return format!("\"{}\"", s) }
			Word::StrVect(s) 	=> { return format!("{}", s) }
			Word::Sym(s) 		=> { return format!("{}", s) },
			Word::Vect(v)		=> { return format!("{}", v) }
		}
	}

	pub fn sprint_short(&self, level: usize) -> String {
		let rep = " ".repeat(level*4);
		match self {
			Word::Null 			=> { return format!("{}none", rep) },
			Word::Bool(b) 		=> { return format!("{}{}", rep, b) },
			Word::BoolVect(_)	=> { return format!("{}vec<bool>", rep) },
			Word::Dict(_) 		=> { return format!("{}dict", rep) },
			Word::Int(i) 		=> { return format!("{}{}", rep, i) },
			Word::List(l) 		=> { return format!("{}[{}:...]", rep, l.len()) },	
			Word::MacroOp(_)	=> { return format!("{}macro_op", rep) },
			Word::Mat(m) 		=> { return format!("{}mat, {}x{}", rep, m.nrows(), m.ncols()) },
			Word::Num(f) 		=> { return format!("{}{}", rep, f) },
			Word::Program(_)	=> { return format!("{}program", rep) }, 	
			Word::Quote(_) 		=> { return format!("{}quote", rep) },
			Word::Str(s) 		=> { return format!("{}\"{}\"", rep, s) }, // SHORTEN! REMOVE "\n"
			Word::StrVect(_)	=> { return format!("{}vec<str>", rep) },
			Word::Sym(s) 		=> { return format!("{}{}", rep, s) },
			Word::Vect(_)		=> { return format!("{}vec<num>", rep) }
		}
	}

	pub fn sprint_type(&self) -> String {
		match self {
			Word::Null 			=> { return format!("none") },
			Word::Bool(_) 		=> { return format!("bool") },
			Word::BoolVect(_)	=> { return format!("vec<bool>") },
			Word::Dict(_) 		=> { return format!("dict") },
			Word::Int(_) 		=> { return format!("int") },
			Word::List(_) 		=> { return format!("list") },
			Word::MacroOp(_)	=> { return format!("macro") },
			Word::Mat(_) 		=> { return format!("mat<num>") },
			Word::Num(_) 		=> { return format!("num") },
			Word::Program(_)	=> { return format!("program") },
			Word::Quote(_) 		=> { return format!("quote") },
			Word::Str(_) 		=> { return format!("str") }
			Word::StrVect(_) 	=> { return format!("vec<str>") }
			Word::Sym(_) 		=> { return format!("sym") },
			Word::Vect(_)		=> { return format!("vec<num>") }
		}
	}

	pub fn print(&self) {
		match self {
			Word::Str(s) 	=> { print!("{}", s) },
			_ 	=> { print!("{}", self.sprint()) }
		}
	}
}

pub fn toF64(w: &Word) -> f64 {
	match w {
		Word::Bool(b) 		=> { if *b { return 1.0 } else { return 0.0 } },
		Word::Int(i) 		=> { return *i as f64 },
		Word::Mat(m) 		=> { if m.nrows() == 1 && m.ncols() == 1 { return *m.get((0, 0)).unwrap() } else { return NAN } },
		Word::Num(f) 		=> { return *f },
		Word::Str(s) 		=> match s.parse::<f64>() {
			Ok(x)	=> { return x },
			Err(_)	=> { return NAN }
		},
		Word::Vect(v)		=> { if v.dim() == 1 { return *v.get(0).unwrap() } else { return NAN } },
		_ 	=> { return NAN }
	}
}

pub fn list2array1(list: &Vec<Word>) -> Array1<f64> {
	return list.iter().map(|i| toF64(i)).collect::<Array1<f64>>()
}


