/* vm.rs
 * ----- */
use crate::code::Opcode;
use crate::env::Env;
use crate::eval::eval_inst;
use crate::words::Word;

use std::mem;

pub const ENV_SIZE : usize = 16;
pub const STACK_SIZE : usize = 2048;

pub struct Vm {
	pub envs 	: Vec<Env>,
	pub ep 		: usize,
	pub last 	: Option<Word>,
	pub line 	: u16,
	pub stack 	: Vec<Word>
}

impl Vm {
	pub fn new() -> Self {
		return Vm {
			envs 	: Vec::with_capacity(ENV_SIZE),
			ep 		: 0,
			last 	: None,
			line 	: 0,
			stack 	: Vec::with_capacity(STACK_SIZE)
		}
	}

	pub fn clear(&mut self) {
		self.last = self.stack.pop();
		self.stack.clear();
	}

	pub fn clear_last(&mut self) {
		self.last = None;
	}

	pub fn curr(&self) -> &Env {
		return self.envs.last().unwrap()
	}

	pub fn curr_mut(&mut self) -> &mut Env {
		return self.envs.last_mut().unwrap()
	}

	pub fn env_pop(&mut self) -> Option<Env> {
		if self.envs.len() > 2 { 
			self.ep -= 1;
			return self.envs.pop() 
		}
		return None
	}

	pub fn env_push(&mut self, inst: Vec<Opcode>) {
		self.envs.push(Env::new(inst));
		self.ep += 1;
	}

	pub fn eval(&mut self) -> Result<bool, String> {
		let n = self.curr().inst.len();
		while self.curr().ip < n {
			let code = self.curr_mut().adv_get(1);
			match eval_inst(code, self) {
				Ok(_) => { },
				Err(e) => { return Err(e) }
			}
		}
		return Ok(false)
	}

	pub fn exec_word(&mut self, w : Word) -> Result<bool, String> {
		/* Similar to 'run_word' except quotes are executed too! */
		match w {
			Word::Program(m) => { return self.run_opcodes(*m.clone()) },
			Word::MacroOp(op) => {
				return eval_inst(op, self)
			},
			Word::Quote(q) => { return self.run_opcodes(*q.clone()) },
			_ 	=> { return self.push_const(w.clone()) }
		}
	}

	pub fn get(&self, key: &String) -> Option<&Word> {
		for i in 0 .. self.ep {
			match self.envs[self.ep-1-i].dict.get(key) {
				Some(x) => { return Some(x) },
				None => { }
			}
		}
		return None
	}

	pub fn global(&mut self) -> &mut Env {
		return self.envs.get_mut(1).unwrap()
	}

	pub fn print_stack(&self) {
		println!("{}\n", self.stack.iter().map(|x| x.sprint()).collect::<Vec<String>>().join("\n"));
	}

	pub fn push_const(&mut self, w: Word) -> Result<bool, String> {
		if self.stack.len() >= STACK_SIZE { return Err(format!("Stack overflow.")) }
		self.stack.push(w);
		return Ok(false);
	}

	pub fn push_const_nocheck(&mut self, w: Word) {
		self.stack.push(w);
	}

	pub fn run_newstack(&mut self, ops: Vec<Opcode>) -> Result<Vec<Word>, String> {
		let mut s = Vec::<Word>::with_capacity(STACK_SIZE);
		mem::swap(&mut s, &mut self.stack);
		let result = self.run_opcodes(ops);
		mem::swap(&mut s, &mut self.stack);
		match result {
			Ok(_) 	=> { return Ok(s) },
			Err(e) 	=> { return Err(e) }
		}
	}

	pub fn run_opcodes(&mut self, ops : Vec<Opcode>) -> Result<bool, String> {
		self.env_push(ops);
		let result = self.eval();
		self.env_pop();
		return result
	}

	pub fn run_word(&mut self, w : Word) -> Result<bool, String> {
		match w {
			Word::Program(m) => { return self.run_opcodes(*m.clone()) },
			Word::MacroOp(op) => {
				return eval_inst(op, self)
			},
			_ 	=> { return self.push_const(w.clone()) }
		}
	}
} 