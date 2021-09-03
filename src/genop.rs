/* genop.rs
 * -------- */
use crate::code::{GenOp};
use crate::code::GenOp::*;
use crate::vm::Vm;
use crate::words::{BoolResult};


pub fn do_general(vm: &mut Vm, op: GenOp) -> BoolResult {
	match op {
		AddLine(u)	=> { 
			vm.line += u as u16;
			return Ok(false)
		},
		_ => { return Ok(false) }
	}
}
