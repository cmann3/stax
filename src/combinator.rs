/* combinator
 * ---------- */
use crate::code::{Comb1, Comb2, Comb3, sprint_comb1, sprint_comb2, sprint_comb3};
use crate::code::Comb1::*;
use crate::code::Comb2::*;
use crate::code::Comb3::*;
use crate::vm::Vm;
use crate::words::{Word, BoolResult};


pub fn do_comb1(vm: &mut Vm, op: Comb1) -> BoolResult {
	match vm.stack.pop() {
		Some(arg) => match op {
			Do => { return vm.exec_word(arg) }
		},
		None => { return Err(format!("Combinator '{}' requires at least 1 item on top of the stack. None found.", sprint_comb1(&op))) }
	}
}

pub fn do_comb2(vm: &mut Vm, op: Comb2) -> BoolResult {
	match vm.stack.pop() {
		Some(arg2) => match vm.stack.pop() {
			Some(arg1) => match op {
				Ifthen => { return do_ifthen(vm, arg1, arg2) },
				Dip => {
					match vm.exec_word(arg2) {
						Ok(_)  => { },
						Err(e) => { return Err(e) }
					}
					return vm.push_const(arg1);
				},
				Cleave => { return do_cleave(vm, arg1, arg2) },
			},
			None => { return Err(format!("Combinator '{}' requires at least 2 items on top of the stack. Only 1 found.", sprint_comb2(&op))) }
		},
		None => { return Err(format!("Combinator '{}' requires at least 2 items on top of the stack. None found.", sprint_comb2(&op))) }
	}
}

pub fn do_comb3(vm: &mut Vm, op: Comb3) -> BoolResult {
	match vm.stack.pop() {
		Some(arg3) => match vm.stack.pop() {
			Some(arg2) => match vm.stack.pop() {
				Some(arg1) => match op {
					Ifelse => { return do_ifelse(vm, arg1, arg2, arg3) },
				},
				None => { return Err(format!("Combinator '{}' requires at least 3 items on the stack. Only 2 found.", sprint_comb3(&op))) }
			},
			None => { return Err(format!("Combinator '{}' requires at least 3 items on the stack. Only 1 found.", sprint_comb3(&op))) }
		},
		None => { return Err(format!("Combinator '{}' requires at least 3 items on the stack. None found.", sprint_comb3(&op))) }
	}
}

pub fn do_cleave(vm: &mut Vm, fun1: Word, fun2: Word) -> BoolResult {
	let top: Word;
	match vm.stack.last() {
		Some(x) => { top = x.clone() },
		None 	=> { return Err(format!("'cleave' requires at least 3 items on top of the stack. Only 2 found.")) }
	}
	match vm.exec_word(fun1) {
		Ok(_) 	=> { },
		Err(e) 	=> { return Err(e) }
	}
	match vm.push_const(top) {
		Ok(_) 	=> { },
		Err(e) 	=> { return Err(e) }
	}
	return vm.exec_word(fun2)
}

pub fn do_ifelse(vm: &mut Vm, cond: Word, then: Word, elsew: Word) -> BoolResult {
	match cond {
		Word::Bool(b) => {
			if b { return vm.exec_word(then) }
			return vm.exec_word(elsew)
		},
		Word::Quote(q) => {
			let result = vm.run_opcodes(q.to_vec());
			match result {
				Ok(_) => match vm.stack.pop() {
					Some(w) => match w {
						Word::Bool(b) => {
							if b { return vm.exec_word(then) }
							return vm.exec_word(elsew)
						},
						_ => { return Err(format!("Condition for 'ifelse' did not evaluate to a boolean <true/false> value.")) }
					},
					None => { return Err(format!("Condition for 'ifelse' did not evaluate to a value. Expression could not be tested.")) }
				},
				Err(e) => { return Err(e) }
			}
		},
		_ => { return Err(format!("Condition for 'ifelse' did not evaluate to a boolean <true/false> value.")) }
	}
}

pub fn do_ifthen(vm: &mut Vm, cond: Word, then: Word) -> BoolResult {
	match cond {
		Word::Bool(b) => {
			if b { return vm.exec_word(then) }
			return Ok(false)
		},
		Word::Quote(q) => {
			let result = vm.run_opcodes(q.to_vec());
			match result {
				Ok(_) => match vm.stack.pop() {
					Some(w) => match w {
						Word::Bool(b) => {
							if b { return vm.exec_word(then) }
							return Ok(false)
						},
						_ => { return Err(format!("Condition for 'ifthen' did not evaluate to a boolean <true/false> value.")) }
					},
					None => { return Err(format!("Condition for 'ifthen' did not evaluate to a value. Expression could not be tested.")) }
				},
				Err(e) => { return Err(e) }
			}
		},
		_ => { return Err(format!("Condition for 'ifthen' did not evaluate to a boolean <true/false> value.")) }
	}
}


