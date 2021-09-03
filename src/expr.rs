/* expr.rs
 * ------- */
use crate::code::Opcode;
use crate::lexer::Token;

use std::mem;

#[derive(Debug, Clone)]
pub enum Expr {
	Blank,
	Infix       (Opcode, u8),
	Single 		(Opcode),
	Double 		(Opcode, Opcode),
	Group     	(Vec<Opcode>),
	Primary     (Vec<Opcode>), // TODO: Remove Primary!
    Unary       (Vec<Opcode>, Box<Expr>),
    Binary      (Opcode, Box<Expr>, Box<Expr>),
    LeftCode	(Opcode, Box<Expr>),
    RightCode	(Opcode, Box<Expr>),
    Call 		(Vec<Opcode>, Box<Expr>),
    Incomplete  (Opcode, Box<Expr>), 	// TODO:: REMOVE
    Quote		(Box<Expr>),
}


pub fn walk_expr(expr : Expr) -> Vec<Opcode> {
    let mut result = Vec::new();
    match expr {
    	Expr::Group(mut code) => result.append(&mut code),
    	Expr::Double(c1, c2)	=> {
    		result.push(c1);
    		result.push(c2);
    	},
        Expr::Primary(mut code) => result.append(&mut code), // TODO: Remove Primary
        Expr::Unary(mut code, rhs) => {
            result.append( &mut walk_expr(*rhs) );
            result.append( &mut code );
        },
        Expr::Binary(code, lhs, rhs) => {
            result.append( &mut walk_expr(*lhs) );
            result.append( &mut walk_expr(*rhs) );
            result.push(code);
        },
        Expr::LeftCode(code, rhs) => {
        	result.push(code);
        	result.append( &mut walk_expr(*rhs) );
        },
        Expr::RightCode(code, rhs) => {
            result.append( &mut walk_expr(*rhs) );
            result.push(code);
        },
        Expr::Call(mut code, lhs) => {
        	result.append( &mut code );
        	result.append( &mut walk_expr(*lhs) );
        },
        Expr::Incomplete(code, rhs) => {
            result.append( &mut walk_expr(*rhs) ); 	// TODO:: REMOVE
            result.push(code);
        },
        Expr::Quote(inside) => {
        	let q = Box::new( walk_expr(*inside) );
        	result.push(Opcode::Quote(q));
        },
        Expr::Single(code) => { result.push(code) },
        Expr::Infix(code, _) => { result.push(code) },
        _ => { }
    }
    return result
}


pub fn walk_expr_assign(expr : Expr, tok: Token) -> Vec<Opcode> {
    match expr {
        Expr::Single(code)  => match code {
            Opcode::Str(mut s) | Opcode::Sym(mut s) => match tok {
                Token::Equal        => { return vec![Opcode::Set(mem::replace(&mut s, Box::new("".to_string())))] },
                Token::EqualProg    => { return vec![Opcode::SetProg(mem::replace(&mut s, Box::new("".to_string())))] },
                _ => { return vec![] }
            },
            _   => { return vec![code] }  
        },
        _ => { return walk_expr(expr) }
    }
}





