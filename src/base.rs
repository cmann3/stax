/* base.rs
 * ------- */
use crate::code::{Opcode, StackOp, BinOp, BoolOp, Comb1, Comb2, Comb3, UnOp, MathOp, AutoOp};
use crate::env::Env;
use crate::words::Word;

use core::f64;
use std::collections::HashMap;
use std::f64::consts;

pub fn create_base() -> Env {
	let mut map = HashMap::<String, Word>::new();

	/* 	== CONSTANTS == */
	map.insert("e".to_string(), Word::Num(consts::E));
	map.insert("pi".to_string(), Word::Num(consts::PI));
	map.insert("pi2".to_string(), Word::Num(consts::FRAC_PI_2));
	map.insert("pi3".to_string(), Word::Num(consts::FRAC_PI_3));
	map.insert("pi4".to_string(), Word::Num(consts::FRAC_PI_4));
	map.insert("pi6".to_string(), Word::Num(consts::FRAC_PI_6));
	map.insert("pi8".to_string(), Word::Num(consts::FRAC_PI_8));
	map.insert("tau".to_string(), Word::Num(consts::TAU));
	map.insert("NaN".to_string(), Word::Num(f64::NAN));
	map.insert("inf".to_string(), Word::Num(f64::INFINITY));
	map.insert("ln2".to_string(), Word::Num(consts::LN_2));
	map.insert("sqrt2".to_string(), Word::Num(consts::SQRT_2));



	/* 	== STACK OPERATIONS == */
	insertMop(&mut map, "bury", Opcode::StackOp(StackOp::Bury));
	insertMop(&mut map, "clear", Opcode::StackOp(StackOp::Clear));
	insertMop(&mut map, "dig", Opcode::StackOp(StackOp::Dig));
	insertMop(&mut map, "dup", Opcode::StackOp(StackOp::Dup));
	insertMop(&mut map, "dupd", Opcode::StackOp(StackOp::Dupd));
	insertMop(&mut map, "flip", Opcode::StackOp(StackOp::Flip));
	insertMop(&mut map, "over", Opcode::StackOp(StackOp::Over));
	insertMop(&mut map, "swap", Opcode::StackOp(StackOp::Swap));
	insertMop(&mut map, "swapd", Opcode::StackOp(StackOp::Swapd));
	insertMop(&mut map, "zap", Opcode::StackOp(StackOp::Zap));
	insertMop(&mut map, "zapd", Opcode::StackOp(StackOp::Zapd));

	/* 	== ARITH OPERATIONS == */
	insertMop(&mut map, "add", Opcode::BinOp(BinOp::Add));
	insertMop(&mut map, "sub", Opcode::BinOp(BinOp::Sub));
	insertMop(&mut map, "mul", Opcode::BinOp(BinOp::Mul));
	insertMop(&mut map, "div", Opcode::BinOp(BinOp::Div));
	insertMop(&mut map, "pow", Opcode::BinOp(BinOp::Pow));
	insertMop(&mut map, "mod", Opcode::BinOp(BinOp::Mod));
	insertMop(&mut map, "grt", Opcode::BoolOp(BoolOp::Grt));
	insertMop(&mut map, "lst", Opcode::BoolOp(BoolOp::Lst));
	insertMop(&mut map, "eq",  Opcode::BoolOp(BoolOp::Eqt));
	insertMop(&mut map, "neq", Opcode::BoolOp(BoolOp::Neq));
	insertMop(&mut map, "gte", Opcode::BoolOp(BoolOp::Gte));
	insertMop(&mut map, "lte", Opcode::BoolOp(BoolOp::Lte));
	insertMop(&mut map, "and", Opcode::BoolOp(BoolOp::And));
	insertMop(&mut map, "or", Opcode::BoolOp(BoolOp::Or));

	/* 	== COMBINATORS == */
	insertMop(&mut map, "do", Opcode::Comb1(Comb1::Do));

	insertMop(&mut map, "cleave", Opcode::Comb2(Comb2::Cleave));
	insertMop(&mut map, "dip", Opcode::Comb2(Comb2::Dip));
	insertMop(&mut map, "ifthen", Opcode::Comb2(Comb2::Ifthen));

	insertMop(&mut map, "ifelse", Opcode::Comb3(Comb3::Ifelse));

	/* 	== UNARY OPERATORS == */
	insertMop(&mut map, "print", Opcode::UnOp(UnOp::Print));

	/* 	== MATH OPERATIONS == */
	insertMop(&mut map, "abs", Opcode::MathOp(MathOp::Abs));
	insertMop(&mut map, "acos", Opcode::MathOp(MathOp::Acos));
	insertMop(&mut map, "acosh", Opcode::MathOp(MathOp::Acosh));
	insertMop(&mut map, "asin", Opcode::MathOp(MathOp::Asin));
	insertMop(&mut map, "asinh", Opcode::MathOp(MathOp::Asinh));
	insertMop(&mut map, "atan", Opcode::MathOp(MathOp::Atan));
	insertMop(&mut map, "atanh", Opcode::MathOp(MathOp::Atanh));
	insertMop(&mut map, "cbrt", Opcode::MathOp(MathOp::Cbrt));
	insertMop(&mut map, "ceil", Opcode::MathOp(MathOp::Ceil));
	insertMop(&mut map, "cos", Opcode::MathOp(MathOp::Cos));
	insertMop(&mut map, "cosh", Opcode::MathOp(MathOp::Cosh));
	insertMop(&mut map, "exp", Opcode::MathOp(MathOp::Exp));
	insertMop(&mut map, "floor", Opcode::MathOp(MathOp::Floor));
	insertMop(&mut map, "fract", Opcode::MathOp(MathOp::Fract));
	insertMop(&mut map, "ln", Opcode::MathOp(MathOp::Ln));
	insertMop(&mut map, "log10", Opcode::MathOp(MathOp::Log10));
	insertMop(&mut map, "log2", Opcode::MathOp(MathOp::Log2));
	insertMop(&mut map, "max", Opcode::MathOp(MathOp::Max));
	insertMop(&mut map, "mean", Opcode::MathOp(MathOp::Mean));
	insertMop(&mut map, "min", Opcode::MathOp(MathOp::Min));
	insertMop(&mut map, "recip", Opcode::MathOp(MathOp::Recip));
	insertMop(&mut map, "round0", Opcode::MathOp(MathOp::Round0));
	insertMop(&mut map, "sd", Opcode::MathOp(MathOp::Sd));
	insertMop(&mut map, "sign", Opcode::MathOp(MathOp::Sign));
	insertMop(&mut map, "sin", Opcode::MathOp(MathOp::Sin));
	insertMop(&mut map, "sinh", Opcode::MathOp(MathOp::Sinh));
	insertMop(&mut map, "sqrt", Opcode::MathOp(MathOp::Sqrt));
	insertMop(&mut map, "tan", Opcode::MathOp(MathOp::Tan));
	insertMop(&mut map, "tanh", Opcode::MathOp(MathOp::Tanh));
	insertMop(&mut map, "trunc", Opcode::MathOp(MathOp::Trunc));
	insertMop(&mut map, "var", Opcode::MathOp(MathOp::Var));


	/* 	== AUTO OPERATIONS == */
	insertMop(&mut map, "input", Opcode::AutoOp(AutoOp::Input));

	return Env { dict: map, inst: vec![], ip: 0 }
}


fn insertMop(map: &mut HashMap<String, Word>, name: &str, op: Opcode) {
	map.insert(name.to_string(), Word::MacroOp(op));
}