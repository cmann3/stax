/* compare.rs
 * ---------- */
use crate::code::BoolOp;
use crate::code::BoolOp::*;
use crate::words::Word;

type BoolResult = Result<Word, String>;

pub fn do_compare(op: BoolOp, lhs: Word, rhs: Word) -> BoolResult {
	match lhs {
		Word::Int(l) 	=> { return do_int(l, rhs, op) },
		Word::Num(l) 	=> { return do_num(l, rhs, op) },
		Word::Str(l) 	=> { return do_str(*l, rhs, op) },
		_ => return Ok(Word::Null)
	}
}


fn do_int(lhs: i32, rhs: Word, op: BoolOp) -> BoolResult {
	match rhs {
		Word::Int(r) 	=> { return do_int_int(lhs, r, op) },
		Word::Num(r) 	=> { return do_num_num(lhs as f64, r, op) },
		_ => return Ok(Word::Null)
	}
}

fn do_num(lhs: f64, rhs: Word, op: BoolOp) -> BoolResult {
	match rhs {
		Word::Int(r) 	=> { return do_num_num(lhs, r as f64, op) },
		Word::Num(r) 	=> { return do_num_num(lhs, r, op) },
		_ => return Ok(Word::Null)
	}
}

fn do_str(lhs: String, rhs: Word, op: BoolOp) -> BoolResult {
	match rhs {
		Word::Str(r) 	=> { return do_str_str(lhs, *r, op) },
		_ => return Ok(Word::Null)
	}
}

fn do_int_int(lhs: i32, rhs: i32, op: BoolOp) -> BoolResult {
	match op {
		Grt	=> { return Ok(Word::Bool(lhs > rhs)) },
		Lst	=> { return Ok(Word::Bool(lhs < rhs)) },
		Gte	=> { return Ok(Word::Bool(lhs >= rhs)) },
		Lte	=> { return Ok(Word::Bool(lhs <= rhs)) },
		Eqt	=> { return Ok(Word::Bool(lhs == rhs)) },
		Neq	=> { return Ok(Word::Bool(lhs != rhs)) },
		_ => { return Err("Binary operation not recognized!".to_string()) }
	}
}

fn do_num_num(lhs: f64, rhs: f64, op: BoolOp) -> BoolResult {
	match op {
		Grt	=> { return Ok(Word::Bool(lhs > rhs)) },
		Lst	=> { return Ok(Word::Bool(lhs < rhs)) },
		Gte	=> { return Ok(Word::Bool(lhs >= rhs)) },
		Lte	=> { return Ok(Word::Bool(lhs <= rhs)) },
		Eqt	=> { return Ok(Word::Bool(lhs == rhs)) },
		Neq	=> { return Ok(Word::Bool(lhs != rhs)) },
		_ => { return Err("Binary operation not recognized!".to_string()) }
	}
}

fn do_str_str(lhs: String, rhs: String, op: BoolOp) -> BoolResult {
	match op {
		Grt	=> { return Ok(Word::Bool(lhs > rhs)) },
		Lst	=> { return Ok(Word::Bool(lhs < rhs)) },
		Gte	=> { return Ok(Word::Bool(lhs >= rhs)) },
		Lte	=> { return Ok(Word::Bool(lhs <= rhs)) },
		Eqt	=> { return Ok(Word::Bool(lhs == rhs)) },
		Neq	=> { return Ok(Word::Bool(lhs != rhs)) },
		_ => { return Err(format!("Operation cannot be completed between two string objects.")) }
	}
}

