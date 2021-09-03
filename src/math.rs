/* math.rs
 * ------- */
use crate::code::{MathOp, sprint_mathop};
use crate::code::MathOp::*;
use crate::vm::Vm;
use crate::words::{Word, WordResult, list2array1};

use core::f64::NAN;
use ndarray::{Array1, Array2};
use ndarray_stats::{QuantileExt, SummaryStatisticsExt};

pub fn do_math(vm: &mut Vm, op: MathOp, word: &Word) -> WordResult {
	match word {
		Word::Int(i) 	=> { return do_num(op, *i as f64) },
		Word::Num(f)	=> { return do_num(op, *f) },
		Word::List(l) 	=> { return do_vect(op, &list2array1(l)) },
		Word::Quote(q)	=> {
			match vm.run_newstack(q.to_vec()) {
				Ok(w) => { return do_vect(op, &list2array1(&w)) },
				Err(e) 	=> { return Err(e) }
			}
		},
		Word::Vect(v)	=> { return do_vect(op, v)},
		_ => { return Err(format!("Operation '{}' cannot be completed on objects of type '{}'.", sprint_mathop(&op), word.sprint_type())) }
	}
}

fn do_num(op: MathOp, f: f64) -> WordResult {
	match op {
		Abs 	=> { return Ok(Word::Num(f.abs())) },
	    Acos 	=> { return Ok(Word::Num(f.acos())) },
	    Acosh 	=> { return Ok(Word::Num(f.acosh())) },
	    Asin 	=> { return Ok(Word::Num(f.asin())) },
	    Asinh 	=> { return Ok(Word::Num(f.asinh())) },
	    Atan 	=> { return Ok(Word::Num(f.atan())) },
	    Atanh 	=> { return Ok(Word::Num(f.atanh())) },
	    Cbrt 	=> { return Ok(Word::Num(f.cbrt())) },
	    Ceil 	=> { return Ok(Word::Num(f.ceil())) },
	    Cos 	=> { return Ok(Word::Num(f.cos())) },
	    Cosh 	=> { return Ok(Word::Num(f.cosh())) },
	    Exp 	=> { return Ok(Word::Num(f.exp())) },
	    Floor 	=> { return Ok(Word::Num(f.floor())) },
	    Fract 	=> { return Ok(Word::Num(f.fract())) },
	    Ln 		=> { return Ok(Word::Num(f.ln())) },
	    Log10 	=> { return Ok(Word::Num(f.log10())) },
	    Log2 	=> { return Ok(Word::Num(f.log2())) },
	    Max 	=> { return Ok(Word::Num(f)) },
	    Mean 	=> { return Ok(Word::Num(f)) },
	    Min 	=> { return Ok(Word::Num(f)) },
	    Neg 	=> { return Ok(Word::Num(-f)) },
	    Recip 	=> { return Ok(Word::Num(f.recip())) },
	    Round0 	=> { return Ok(Word::Num(f.round())) },
	    Sd 		=> { return Ok(Word::Num(0.0)) },
	    Sign 	=> { return Ok(Word::Num(f.signum())) },
	    Sin 	=> { return Ok(Word::Num(f.sin())) },
	    Sinh 	=> { return Ok(Word::Num(f.sinh())) },
	    Sqrt 	=> { return Ok(Word::Num(f.sqrt())) },
	    Tan 	=> { return Ok(Word::Num(f.tan())) },
	    Tanh 	=> { return Ok(Word::Num(f.tanh())) },
	    Trunc 	=> { return Ok(Word::Int(f.trunc() as i32)) },
	    Var 	=> { return Ok(Word::Num(0.0)) }
	}
}

fn do_vect(op: MathOp, v: &Array1<f64>) -> WordResult {
	match op {
		Abs 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.abs())))) },
	    Acos 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.acos())))) },
	    Acosh 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.acosh())))) },
	    Asin 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.asin())))) },
	    Asinh 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.asinh())))) },
	    Atan 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.atan())))) },
	    Atanh 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.atanh())))) },
	    Cbrt 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.cbrt())))) },
	    Ceil 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.ceil())))) },
	    Cos 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.cos())))) },
	    Cosh 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.cosh())))) },
	    Exp 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.exp())))) },
	    Floor 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.floor())))) },
	    Fract 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.fract())))) },
	    Ln 		=> { return Ok(Word::Vect(Box::new(v.map(|f| f.ln())))) },
	    Log10 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.log10())))) },
	    Log2 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.log2())))) },
	    Max 	=> { return Ok(Word::Num(*v.max_skipnan())) },
	    Mean 	=> { return Ok(Word::Num( mean_vec(v) )) },
	    Min 	=> { return Ok(Word::Num(*v.min_skipnan())) },
	    Neg 	=> { return Ok(Word::Vect(Box::new(v.map(|f| -f)))) },
	    Recip 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.recip())))) },
	    Round0 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.round())))) },
	    Sd 		=> { return Ok(Word::Num( var_vec(v).sqrt() )) },
	    Sign 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.signum())))) },
	    Sin 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.sin())))) },
	    Sinh 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.sinh())))) },
	    Sqrt 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.sqrt())))) },
	    Tan 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.tan())))) },
	    Tanh 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.tanh())))) },
	    Trunc 	=> { return Ok(Word::Vect(Box::new(v.map(|f| f.trunc())))) },
	    Var 	=> { return Ok(Word::Num( var_vec(v) )) },
	}
}

pub fn mean_vec(v: &Array1<f64>) -> f64 {
	let n = v.dim();
	if n == 0 { return NAN } 
	if n == 1 { return *v.get(0).unwrap() }
	let mut nans = 0.0;
	let foldr = |x: f64, y: &f64| if y.is_nan() { nans += 1.0; x } else { x + y };
	let sum = v.fold(0.0, foldr);
	let newn = n as f64 - nans;
	if newn == 0.0 { return NAN }
	return sum / newn
}

pub fn var_vec(v: &Array1<f64>) -> f64 {
	let n = v.dim();
	if n == 0 { return NAN } 
	if n == 1 { return 0.0 }
	let mut nans = 0.0;
	let foldm = |x: f64, y: &f64| if y.is_nan() { nans += 1.0; x } else { x + y };
	let temp = v.fold(0.0, foldm);
	let newn = n as f64 - nans;
	if newn == 0.0 { return 0.0 }
	let mean = temp / newn;
	let foldv = |x: f64, y: &f64| if y.is_nan() { x } else { x + y };
	return v.map(|i| (i - mean).powi(2)).fold(0.0, foldv) / (newn - 1.0)
}


