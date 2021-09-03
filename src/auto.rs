/* auto.rs
 * ------- */
use crate::code::{AutoOp};
use crate::code::AutoOp::*;
use crate::vm::Vm;
use crate::words::{BoolResult, Word};

use std::io::{stdin, stdout};

pub fn do_auto(vm: &mut Vm, op: AutoOp) -> BoolResult {
	match op {
		Input => {
			let mut s = String::new();
			stdin().read_line(&mut s).expect("Could not recognize entered string.");
			s.pop(); // Remove trailing "\r\n"
			return vm.push_const(Word::Str(Box::new(s)))
		}
	}	
}