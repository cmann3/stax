/* stackop.rs
 * ---------- */
use crate::code::StackOp;
use crate::code::StackOp::*;
use crate::vm::Vm;


pub fn do_stack(vm: &mut Vm, op: StackOp) -> Result<bool, String> {
	let n = vm.stack.len();
	match op {
		Dup 	=> {
			let last = vm.stack.last();
			match last {
				Some(x) => { 
					let z = x.clone();
					return vm.push_const(z) 
				},
				None => { return Err(format!("'dup' requires 1 item on top of the stack, but none found.")) }
			}
		},
		Swap 	=> {
			if n >= 2 {
				vm.stack.swap(n-1, n-2);
			} else { return Err(format!("'swap' requires 2 items on top of the stack, '{}' found.", n)) }
		},
		Dupd 	=> {
			if n >= 2 {
				let top = vm.stack.pop().unwrap();
				vm.push_const_nocheck(vm.stack.last().unwrap().clone());
				return vm.push_const(top);
			} else { return Err(format!("'dupd' requires 2 items on top of the stack, '{}' found.", n)) }
		},
		Swapd 	=> {
			if n >= 3 {
				vm.stack.swap(n-2, n-3);
			} else { return Err(format!("'swapd' requires 3 items on top of the stack, '{}' found.", n)) }
		},
		Flip 	=> {
			if n >= 3 {
				vm.stack.swap(n-1, n-3);
			} else { return Err(format!("'flip' requires 3 items on top of the stack, '{}' found.", n)) }
		},
		Bury 	=> {
			if n >= 3 {
				vm.stack.swap(n-1, n-3);
				vm.stack.swap(n-2, n-1);
			} else { return Err(format!("'bury' requires 3 items on top of the stack, '{}' found.", n)) }
		},
		Dig 	=> {
			if n >= 3 {
				vm.stack.swap(n-1, n-3);
				vm.stack.swap(n-2, n-3);
			} else { return Err(format!("'bury' requires 3 items on top of the stack, '{}' found.", n)) }
		},
		Zap 	=> {
			vm.stack.pop();
		}
		Zapd 	=> {
			match vm.stack.pop() {
				Some(x) => {
					vm.stack.pop();
					vm.push_const_nocheck(x);
				},
				None => { }
			}
		},
		Over 	=> {
			if n >= 2 { vm.push_const(vm.stack[n-2].clone());
			} else { return Err(format!("'over' requires 2 items on top of the stack, '{}' found.", n)) }
		},
		Clear 	=> { vm.stack.clear() }
	}
	return Ok(false)
}