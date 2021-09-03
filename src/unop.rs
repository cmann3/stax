/* unop.rs
 * ------- */
use crate::code::UnOp;
use crate::code::UnOp::*;
use crate::words::{StaxResult, Word};

use std::io::prelude::*;
use std::io;

pub fn do_un(op: UnOp, w: Word) -> StaxResult {
	match op {
		Print 	=> {  
			w.print(); 
			io::stdout().flush().ok().expect("stdout could not be flushed.");
			return Ok(None) 
		},
		Input  => {
			let mut s = String::new();
			stdin().read_line(&mut s).expect("String could not be read.");
			
		}
	}
}