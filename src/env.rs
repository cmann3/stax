/* env.rs
 * ------ */
use crate::code::Opcode;
use crate::eval::eval_inst;
use crate::parser::Parser;
use crate::words::Word;
use crate::vm::Vm;

use std::collections::HashMap;

pub struct Env {
	pub dict 	: HashMap<String, Word>,
	pub inst 	: Vec<Opcode>,
	pub ip 		: usize
}

impl Env {
	pub fn new(inst: Vec<Opcode>) -> Self {
		return Env {
			dict 	: HashMap::new(),
			inst	: inst,
			ip 		: 0
		}
	}

	fn adv(&mut self, n: usize) {
		self.ip += n;
	}

	pub fn adv_get(&mut self, n: usize) -> Opcode {
		self.ip += n;
		return self.inst[self.ip - 1].clone()
	}


	pub fn reset(&mut self, inst: Vec<Opcode>) {
		self.inst = inst;
		self.ip   = 0;
	}

}

