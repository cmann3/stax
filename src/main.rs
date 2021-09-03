use stax::base::create_base;
use stax::parser::Parser;
use stax::code::sprint_opcode;
use stax::vm::Vm;

use stax::code::Opcode;
use stax::words::Word;

use stax::lexer::Lexer;
use stax::compiler::compile;

use rustyline::Editor;

use std::mem::size_of;

fn main() {
    let flag_byteprint = true;

    println!("Size of opcode: {}", size_of::<Opcode>());
    println!("Size of word: {}", size_of::<Word>());

    let mut cli = Editor::<()>::new();
    let mut vm  = Vm::new();
    vm.envs.push(create_base());
    vm.ep += 1;
    vm.env_push(vec![]); // GLOBAL!

    loop {
        let s = cli.readline("> ");
        if s.is_err() {
            println!("There has been an error!");
            continue;
        }
        let input = s.unwrap();
        if input == "".to_string() { continue }
        if input == "quit".to_string() { break }
        
        let mut lexer = Lexer::new(input, true, "interactive");
        match compile(&mut lexer) {
            Ok(v)   => {
                if flag_byteprint {
                    for statement in v.iter() {
                        if statement.len() > 0 {
                            for op in 0.. statement.len() {
                                println!("{:0>6} {}", op, sprint_opcode(&statement[op]));
                            }
                            println!("");
                        }
                    }
                }
                for statement in v {
                    if statement.len() > 0 {
                        vm.global().reset(statement);
                        let result = vm.eval();
                        match result {
                            Ok(_)   => {
                                if vm.stack.len() > 0 { 
                                    // TODO: clear except for last, then print last!
                                    vm.print_stack();
                                    vm.clear(); 
                                }
                            },
                            Err(e)  => { println!("ERROR: {}", e) }
                        }
                    }
                }
            },
            Err(e)  => { println!("ERROR: {}\n", e) }
        }
    }

}