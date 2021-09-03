#![allow(dead_code, unreachable_patterns, unused_imports, non_snake_case, unused_must_use)]
//#[macro_use]
extern crate rustyline;
extern crate unicode_segmentation;

pub mod arith;
pub mod auto;
pub mod base;
pub mod code;
pub mod combinator;
pub mod compare;
pub mod compiler;
pub mod env;
pub mod error;
pub mod eval;
pub mod expr;
pub mod genop;
pub mod lexer;
pub mod math;
pub mod parser; // TODO: change/remove!
pub mod stackop;
pub mod text_format;
pub mod unop;
pub mod vm;
pub mod words;