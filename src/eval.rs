/* eval.rs
 * ------- */
use crate::arith::do_arith;
use crate::auto::do_auto;
use crate::code::{Opcode, ConstCode};
use crate::combinator::{do_comb1, do_comb2, do_comb3};
use crate::compare::do_compare;
use crate::genop::do_general;
use crate::math::do_math;
use crate::parser::Parser;
use crate::stackop::do_stack;
use crate::unop::do_un;
use crate::vm::Vm;
use crate::words::Word;

use std::mem;

pub fn eval_inst(op: Opcode, vm: &mut Vm) -> Result<bool, String> {
	match op {
		Opcode::Const(c) => match c {
			ConstCode::Int(i) 	=> { return vm.push_const(Word::Int(i)) },
			ConstCode::True 	=> { return vm.push_const(Word::Bool(true)) },
			ConstCode::False	=> { return vm.push_const(Word::Bool(false)) },
			ConstCode::Null		=> { return vm.push_const(Word::Null) },
		},
		Opcode::Num(f) => { vm.push_const(Word::Num(f)) },
		Opcode::Str(s) => { vm.push_const(Word::Str(s.clone())) },
		Opcode::Sym(s) => { 
			match vm.get(&s) {
				Some(w) => { 
					let word = w.clone();
					return vm.run_word(word) 
				},
				None => { return Err(format!("Object '{}' could not be found.", s)) }
			}
			
		}, // TODO!!
		Opcode::Quote(q) 	=> { return vm.push_const(Word::Quote(q.clone())) },
		Opcode::Prog(q) 	=> { return vm.run_opcodes(q.to_vec()) },
		Opcode::BinOp(o) 	=> {
			match vm.stack.pop() {
				Some(rhs) => match vm.stack.pop() {
					Some(lhs) => match do_arith(vm, o, &lhs, &rhs) {
						Ok(x)  => { vm.push_const_nocheck(x); return Ok(false) },
						Err(e) => { return Err(e) }
					},
					None => { return Err(format!("Operation requires 2 objects on top of the stack. Only 1 found.")) }
				},
				None => { return Err(format!("Operation requires 2 objects on top of the stack. None found.")) }
			}
		},
		Opcode::BoolOp(o) => {
			match vm.stack.pop() {
				Some(rhs) => match vm.stack.last() {
					Some(lhs) => match do_compare(o, lhs.clone(), rhs) {
						Ok(x)  => { vm.push_const_nocheck(x); return Ok(false) },
						Err(e) => { return Err(e) }
					},
					None => { return Err(format!("Operation requires 2 objects on top of the stack. Only 1 found.")) }
				},
				None => { return Err(format!("Operation requires 2 objects on top of the stack. None found.")) }
			}
		},
		Opcode::UnOp(o) => {
			match vm.stack.pop() {
				Some(arg) => match do_un(o, arg) { 
					Ok(result) => match result {
						Some(val) 	=> { vm.push_const_nocheck(val); return Ok(false) },
						None 		=> { return Ok(false) }
					},
					Err(e) => { return Err(e) }
				},
				None => { return Err(format!("Operation requires at least 1 object on top of the stack. None found.")) }
			}
		},
		Opcode::StackOp(o) 	=> { return do_stack(vm, o) },
		Opcode::Comb1(c) 	=> { return do_comb1(vm, c) },
		Opcode::Comb2(c) 	=> { return do_comb2(vm, c) },
		Opcode::Comb3(c) 	=> { return do_comb3(vm, c) },
		Opcode::MathOp(m) 	=> {
			match vm.stack.pop() {
				Some(w) => { 
					match do_math(vm, m, &w) {
						Ok(result) 	=> { vm.push_const_nocheck(result); return Ok(false) },
						Err(e) 		=> { return Err(e) }
					} 
				},
				None => { return Err(format!("Operation requires at least 1 object on top of the stack. None found.")) }
			}

		},
		Opcode::AutoOp(a)	=> { return do_auto(vm, a) },
		Opcode::GenOp(g)	=> { return do_general(vm, g) },
		Opcode::Set(s) 		=> {
			match vm.stack.pop() {
				Some(word) => {
					vm.curr_mut().dict.insert(*s, word);
					return Ok(false)
				},
				None => { return Err(format!("No objects found with which to set to '{}' with '='.", *s)) }
			}
		},
		Opcode::SetProg(s)	=> {
			match vm.stack.pop() {
				Some(word) => match word {
					Word::Quote(mut v) 	=> {
						vm.curr_mut().dict.insert(*s, Word::Program(mem::replace(&mut v, Box::new(vec![]))));
						return Ok(false)
					},
					_ => {
						vm.curr_mut().dict.insert(*s, word);
						return Ok(false)
					}
				},
				None => { return Err(format!("No objects found with which to set to '{}' with '='.", *s)) }
			}
		},

		Opcode::Blank 		=> { return Ok(false) },
		_ => { return Ok(true) } // TODO: REMOVE! NEED TO COVER ALL CASES!
	}
}










