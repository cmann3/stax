/* compiler.rs
 * ----------- */
use crate::code::{Opcode, ConstCode, GenOp, MathOp};
use crate::expr::{Expr, walk_expr, walk_expr_assign};
use crate::lexer::{Lexer, Prec, Token, sprint_token};
use crate::words::{BoolResult};


type ParseResult = Result<Expr, String>;

struct Compiler {
	indent	: 	u8,
	stop 	: 	Token
}

pub fn compile(lexer: &mut Lexer) -> Result<Vec<Vec<Opcode>>, String> {
	lexer.next_token()?;
	lexer.next_token()?;
	let mut compiler = Compiler::new();
	let mut results  = Vec::new(); 
	while !lexer.is_match(&vec![Token::Eof]) {
		let expr = compiler.parse_until(lexer, &vec![Token::Eol, Token::Semicolon])?;
		results.push(expr);
	}
	return Ok(results)
}


impl Compiler {
	pub fn new() -> Self {
		return Compiler {
			indent	: 0,
			stop 	: Token::Blank
		}
	}

	/* == MAIN PARSING FUNCTIONS == */

	fn parse_binary(&mut self, lexer: &mut Lexer, lhs: Expr, prec: Prec, stop: &Vec<Token>) -> Result<(Expr, Prec), String> {
		match lexer.current.clone() {
			Token::White(_) | Token::Blank => {
				lexer.next_token()?;
				if lexer.is_match(stop) { return Ok((lhs, prec)) }
				return self.parse_binary(lexer, lhs, prec, stop)
			},
			Token::Eol => {
				let mut nlines = 1;
				'outer: loop {
					lexer.next_token()?;
					match lexer.current {
						Token::Eol 		=> { nlines += 1 }
						Token::White(u)	=> { self.indent = u },
						_ => { 
							self.indent = 0;
							break 'outer;
						}
					}
				}
				if lexer.is_match(stop) { 
					return Ok((Expr::Single(Opcode::GenOp(GenOp::AddLine(nlines))), prec)) 
				}
				return self.parse_binary(lexer, Expr::RightCode(
					Opcode::GenOp(GenOp::AddLine(nlines)), Box::new(lhs)
				), prec, stop)			
			},
			Token::Equal 	=> {
				lexer.next_token()?;
				let mut results = self.parse_until(lexer, &vec![Token::Eol, Token::Semicolon])?;
				results.append( &mut walk_expr_assign(lhs, Token::Equal) );
				return Ok((Expr::Group(results), prec))
			},
			Token::EqualProg 	=> {
				lexer.next_token()?;
				let mut results = vec![Opcode::Quote(Box::new(
					self.parse_until(lexer, &vec![Token::Eol, Token::Semicolon])?
				))];
				results.append( &mut walk_expr_assign(lhs, Token::EqualProg) );
				return Ok((Expr::Group(results), prec))
			},
			Token::Infix(op, op_prec) => {
				if op_prec > prec {
					lexer.next_token()?;
					if lexer.is_match(stop) { return Err(format!("Stop token was reached before infix operation was finished parsing.")) }
					let rhs = self.parse_primary(lexer, false, stop)?;
					let (expr, new_prec) = self.parse_binary(lexer, rhs, op_prec, stop)?;
					let new_code = Expr::Binary(op.clone(), Box::new(lhs), Box::new(expr));
					if new_prec <= prec { return Ok((new_code, new_prec)) }
					return self.parse_binary(lexer, new_code, prec, stop)
				} else {
					return Ok((lhs, prec))
				}
			},
			Token::Eof => { return Ok((lhs, prec))}
			_ => { return Ok((lhs, prec))}
		}
	}

	fn parse_primary(&mut self, lexer: &mut Lexer, first: bool, stop: &Vec<Token>) -> ParseResult {
		let exp : Expr;
		match lexer.current.clone() {
			Token::White(_) | Token::Blank	=> {
				lexer.next_token()?;
				if lexer.is_match(stop) { return Ok(Expr::Blank) }
				return self.parse_primary(lexer, first, stop)
			},
			Token::Eof => { return Ok(Expr::Blank) },
			Token::Eol => {
				let mut nlines = 1;
				'outer: loop {
					lexer.next_token()?;
					match lexer.current {
						Token::Eol 		=> { nlines += 1 }
						Token::White(u)	=> { self.indent = u },
						_ => { 
							self.indent = 0;
							break 'outer;
						}
					}
				}
				if lexer.is_match(stop) { 
					return Ok(Expr::Single(Opcode::GenOp(GenOp::AddLine(nlines)))) 
				}
				let new_prime = self.parse_primary(lexer, first, stop)?;
				return Ok( Expr::LeftCode(
					Opcode::GenOp(GenOp::AddLine(nlines)), Box::new(new_prime)
				))				
			},
			Token::Lparen 	=> {
				lexer.next_token()?;
				exp = Expr::Primary(self.parse_until(lexer, &vec![Token::Rparen])?);
			},
			Token::Lbrack 	=> {
				lexer.next_token()?;
				let quote = self.parse_until(lexer, &vec![Token::Rbrack])?;
				exp = Expr::Single(Opcode::Quote(Box::new(quote)));
			},
			//TODO: LBRACE
			Token::Minus 	=> {
				lexer.next_token()?;
				match &lexer.current {
					Token::Const(op)	=> match op {
						Opcode::Const(c)	=> match c {
							ConstCode::Int(i)	=> { return Ok(Expr::Single(Opcode::Const(ConstCode::Int(-i)))) },
							ConstCode::True 	=> { return Ok(Expr::Single(Opcode::Const(ConstCode::False))) },
							ConstCode::False 	=> { return Ok(Expr::Single(Opcode::Const(ConstCode::True))) },
							ConstCode::Null 	=> { return Ok(Expr::Single(Opcode::Const(ConstCode::Null))) },
						},
						_ 	=> { return Err(format!("Opcode misplaced by lexer!")) }
					},
					Token::Num(f)	=> { return Ok(Expr::Single(Opcode::Num(-f))) },
					Token::Sym(s)	=> {
						return Ok(Expr::Double(Opcode::Sym(s.clone()), Opcode::MathOp(MathOp::Neg)))
					},
					_ => { return Err("yep".to_string()) }// ?What to do here??}
				}
			},
			Token::Const(c)	=> { exp = Expr::Single(c) },
			Token::Num(f)	=> { exp = Expr::Single(Opcode::Num(f)) },
			Token::Str(s)	=> { exp = Expr::Single(Opcode::Str(s)) },
			Token::Sym(s)	=> { exp = Expr::Single(Opcode::Sym(s)) },

			Token::Infix(o,_)	=> {
				lexer.next_token()?;
				if lexer.is_match(stop) { return Err(format!("Stop token was reached before infix operation was finished parsing.")) }
				let res = self.parse_primary(lexer, false, stop)?;  // ?? Should parse binary after so no stop?
				let rhs = walk_expr(Expr::RightCode(o.clone(), Box::new(res)));
				if first { return Ok( Expr::Primary(rhs) ) }
				return Ok( Expr::Single(Opcode::Quote(Box::new(rhs))) )
			},

			_ => { return Err(format!("Token not recognized for parsing!")) }
		}
		return self.ending(lexer, exp, stop)
	}

	pub fn parse_until(&mut self, lexer: &mut Lexer, stop: &Vec<Token>) -> Result<Vec<Opcode>, String> {
		let mut results = Vec::new();
		'outer: while !lexer.is_match(stop) {
			lexer.skipWhite()?;
			let prime = self.parse_primary(lexer, true, stop)?;
			if lexer.is_match(stop) { 
				results.append( &mut walk_expr(prime) );
				break 'outer; 
			} 
			match self.parse_binary(lexer, prime, Prec::Min, stop) {
				Ok((code, _)) 	=> { results.append( &mut walk_expr(code) ) },
				Err(e)			=> { return Err(e) }
			}
			//lexer.next_token()?; // TODO: Check!
			if lexer.current == Token::Eof { return Ok(results) }
		}
		self.stop = lexer.current.clone();
		if lexer.current == Token::Eol {
			// Emit result with newline!
			lexer.next_token()?;
			match lexer.current {
				Token::White(u)	=> {
					self.indent = u;
					lexer.next_token()?;
				},
				// Check if newline??
				_ 	=> { self.indent = 0 }
			}
		} else { lexer.next_token()?; }
		return Ok(results)
	}


	/* 	== HELPER FUNCTIONS == */
	fn ending(&mut self, lexer: &mut Lexer, expr: Expr, stop: &Vec<Token>) -> ParseResult {
		lexer.next_token()?;
		match lexer.current {
			Token::White(_) | Token::Blank	=> { lexer.next_token()?; },
			Token::Lparen => { 
				lexer.next_token()?;
				let mut results = Vec::new();
				while self.stop != Token::Rparen {
					let arg = self.parse_until(lexer, &vec![Token::Comma, Token::Rparen])?;
					results.push(Opcode::Quote(Box::new(arg)));
				}
				self.stop = Token::Blank;
				// TODO: Check here for assignment!
				let exp = Expr::Call(results, Box::new(expr));
				return self.ending(lexer, exp, stop)
			},
			// TODO: Lbrack, Lbrace, (period?, infix?)
			_ => { }
		}
		// separate match because whitespace & don't want a loop!
		match lexer.current {
			Token::Equal 	=> {

			},
			_ => { }
		}
		return Ok(expr)
	}

}

