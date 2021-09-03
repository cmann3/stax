/* lexer.rs
 * -------- */
use crate::code::{Opcode, ConstCode, BinOp, BoolOp};
use crate::words::{BoolResult, Word};

use unicode_segmentation::UnicodeSegmentation;
use rustyline::Editor;
use std::cmp::Ordering;
use std::mem;


/*  HELPER FUNCTIONS    */
fn num_chars(ch: &String) -> bool { return "0123456789".contains(ch) }
fn sym_chars(ch: &String) -> bool { return "=-+!@#$%^&*:.<>?/|\\~".contains(ch) }
fn stop_chars(ch: &String) -> bool { return " \t\r\n[](){}!@#$%^&*-+=:;,.<>?/|\\~`'\"".contains(ch) }
fn stop_chars_num(ch: &String) -> bool { return " \t\r\n[](){}!@#$%^&*-+=:;,<>?/|\\~`'\"".contains(ch) }
fn alpha_chars(ch: &String) -> bool { return "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(ch) }

/*  HELPER TYPES  */
type TokenResult = Result<Token, String>;


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Blank,
	Colon,
	Comma,
	Const(Opcode),
	Elif,
	Else,
	Eof,
	Eol,
	Equal,
	EqualProg,
	If,
	Infix(Opcode, Prec),
	Num(f64),
	Lparen, Rparen,
	Lbrack, Rbrack,
	Lbrace, Rbrace,
	Minus,
	Semicolon,
	Str(Box<String>),
	Sym(Box<String>),
	White(u8)
}

pub fn sprint_token(tok: &Token) -> String {
	match tok {
		Token::Blank			=> { format!("token: blank") },
		Token::Colon			=> { format!("token: colon") },
		Token::Comma			=> { format!("token: comma") },
		Token::Const(_)			=> { format!("token: const") },
		Token::Elif				=> { format!("token: elif") },
		Token::Else				=> { format!("token: else") },
		Token::Eof				=> { format!("token: eof") },
		Token::Eol				=> { format!("token: eol") },
		Token::Equal 			=> { format!("token: equal") },
		Token::EqualProg		=> { format!("token: equalprog") },
		Token::If				=> { format!("token: if") },
		Token::Infix(_,_)		=> { format!("token: infix") },
		Token::Num(_)			=> { format!("token: num") },
		Token::Lparen			=> { format!("token: lparen") },
		Token::Lbrack			=> { format!("token: lbrack") },
		Token::Lbrace			=> { format!("token: lbrace") },
		Token::Rparen			=> { format!("token: rparen") },
		Token::Rbrack			=> { format!("token: rbrack") },
		Token::Rbrace			=> { format!("token: rbrace") },
		Token::Minus			=> { format!("token: minus") },
		Token::Semicolon		=> { format!("token: semicolon") },
		Token::Str(_)			=> { format!("token: str") },
		Token::Sym(_)			=> { format!("token: sym") },
		Token::White(_)			=> { format!("token: white") },
	}
}

// Add Traits: Ord, PartialOrd
#[derive(Clone, Copy, Debug, Eq)]
pub enum Prec {
	Min = 0,
	Assign,
	Conditional,
	Nullish,
	Or,
	And,
	Equality,
	Inequality,
	Add,
	Mul,
	General,
	Seq,
	Unary,
	Pow,
	Call
}

impl PartialOrd for Prec {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Prec {
	fn cmp(&self, other: &Self) -> Ordering {
		(*self as u8).cmp(&(*other as u8))
	}
}

impl PartialEq for Prec {
	fn eq(&self, other: &Self) -> bool {
		(*self as u8) == (*other as u8)
	}
}



/*  MAIN PARSER  */
pub struct Lexer {
    pub breaks      : Vec<usize>,
    pub chars       : Vec<String>,
    pub current 	: Token,
    file	 		: String,
    interactive 	: bool,
    line 			: u16,
    pos             : usize,
    pub future		: Token,
}


impl Lexer {
	pub fn new(input: String, interactive: bool, file: &str) -> Self {
		let lexer = Lexer {
			breaks      : Vec::new(),
            chars       : UnicodeSegmentation::graphemes(&input[..], true).into_iter().map(|x| x.to_string()).collect::<Vec<String>>(),
            current 	: Token::Blank,
            file        : file.to_string(),
            interactive : interactive,  
            line        : 0,
            pos         : 0,
           	future 		: Token::Blank,
		};
		return lexer
	}


	pub fn next_token(&mut self) -> BoolResult {
		if self.pos >= self.chars.len() {
			return self.token(Token::Eof)
		}
		let ch = self.curr();
		match &ch[..] {
			" "	 | "\r"		=> { 
				let n = self.whitespace(1);
				return self.token(Token::White(n)) 
			},
			"\t"			=> { 
				let n = self.whitespace(4);
				return self.token(Token::White(n)) 
			},
			"\n" | "\r\n" 	=> { return self.newline(self.pos) },
			"("				=> { return self.token_adv(Token::Lparen) },
			"["				=> { return self.token_adv(Token::Lbrack) },
			"{"				=> { return self.token_adv(Token::Lbrace) },
			")"				=> { return self.token_adv(Token::Rparen) },
			"]"				=> { return self.token_adv(Token::Rbrack) },
			","				=> { return self.token_adv(Token::Comma) },
			";"				=> { return self.token_adv(Token::Semicolon) },
			"\"" | "'"		=> { return self.string(ch, self.pos) },
			_ 	=> {
				if num_chars(&ch) { return self.number(self.pos, 0) 
				} else if sym_chars(&ch) { return self.infix(self.pos) 
				} else { return self.symbol(self.pos) }
			}
		}
	}

	/* HELPER FUNCTIONS */
	fn adv(&mut self, n: usize) { self.pos += n }

	fn curr(&mut self) -> String {
		match self.chars.get(self.pos) {
			Some(x)	=> return x.clone(),
			None 	=> return "".to_string()
		}
	}

	fn get(&self, start: usize, end: usize) -> String { return self.chars[start..end].join("") }

	fn infix(&mut self, start: usize) -> BoolResult {
		loop {
            self.adv(1);
            let ch = self.curr();
            if ch == "" || !sym_chars(&ch) { break }
        }
        let s 	= self.get(start, self.pos);
        let op : Opcode;
        let prec : Prec;
        match &s[..] {
            "="		=> { return self.token(Token::Equal); },
            ":="	=> { return self.token(Token::EqualProg); },
            ":"		=> { return self.token(Token::Colon); },
            "+"     => { op = Opcode::BinOp(BinOp::Add); prec = Prec::Add; },
            "-"     => { return self.token(Token::Minus); },
            "*"     => { op = Opcode::BinOp(BinOp::Mul); prec = Prec::Mul; },
            "/"     => { op = Opcode::BinOp(BinOp::Div); prec = Prec::Mul; },
            "^"     => { op = Opcode::BinOp(BinOp::Pow); prec = Prec::Pow; },
            "%"     => { op = Opcode::BinOp(BinOp::Mod); prec = Prec::Mul; },
            "=="    => { op = Opcode::BoolOp(BoolOp::Eqt); prec = Prec::Equality; },
            "!="    => { op = Opcode::BoolOp(BoolOp::Neq); prec = Prec::Equality; },
            ">"     => { op = Opcode::BoolOp(BoolOp::Grt); prec = Prec::Inequality; },
            "<"     => { op = Opcode::BoolOp(BoolOp::Lst); prec = Prec::Inequality; },
            ">="    => { op = Opcode::BoolOp(BoolOp::Gte); prec = Prec::Inequality; },
            "<="    => { op = Opcode::BoolOp(BoolOp::Lte); prec = Prec::Inequality; },
            "&"     => { op = Opcode::BoolOp(BoolOp::And); prec = Prec::And; },
            "|"     => { op = Opcode::BoolOp(BoolOp::Or); prec = Prec::Or; },
            ".."    => { op = Opcode::BinOp(BinOp::Seq); prec = Prec::Seq; },
            "++"    => { op = Opcode::BinOp(BinOp::Cat); prec = Prec::General; },
            "--"    => { op = Opcode::BinOp(BinOp::Del); prec = Prec::General; },
            "**"    => { op = Opcode::BinOp(BinOp::Rep); prec = Prec::General; },
            "//"    => { op = Opcode::BinOp(BinOp::Spl); prec = Prec::General; },
            _ => { op = Opcode::Sym(Box::new(s)); prec = Prec::General; }
        }
        return self.token(Token::Infix(op, prec))
	}

	pub fn is_match(&self, tok: &Vec<Token>) -> bool {
    	for i in tok {
    		if self.current == *i { return true }
    	}
    	return false
    }

	fn make_number(&mut self, start: usize, end: usize, nperiod: usize, isSym: bool) -> BoolResult {
        let s = self.get(start, end);
        if isSym { return self.token(Token::Sym(Box::new(s))) }
        if nperiod == 0 {
            match s.parse::<i32>() {
                Ok(x)  => { return self.token(Token::Const(Opcode::Const(ConstCode::Int(x)))) },
                Err(_) => { }
            }
        }
        match s.parse::<f64>() {
            Ok(x)   => { return self.token(Token::Num(x)) },
            Err(_)  => { return self.token(Token::Sym(Box::new(s))) }
        }
    }

	fn newline(&mut self, n: usize) -> BoolResult {
		self.breaks.push(n);
		self.line += 1;
		return self.token_adv(Token::Eol)
	}

	fn next_word(&mut self, start: usize) -> String {
        loop {
            self.adv(1);
            let ch = self.curr();
            if ch == "" || stop_chars(&ch) { break }
        }
        return self.get(start, self.pos)
    }

	fn number(&mut self, start: usize, mut nperiod: usize) -> BoolResult {
        let mut non_digit = false;
        loop {
            self.adv(1);
            let ch = self.curr();
            if ch  == "." {
                if self.peek(1) == "." { return self.make_number(start, self.pos, nperiod, non_digit) }
                nperiod += 1; 
                continue
            } else if ch == "" { return self.make_number(start, self.pos, nperiod, non_digit) }
            if stop_chars_num(&ch) { return self.make_number(start, self.pos, nperiod, non_digit) }
            if !num_chars(&ch) { non_digit = true }
        }
    }

    fn peek(&self, n: usize) -> String {
        match self.chars.get(self.pos + n) {
            Some(x) => return x.clone(),
            None    => return "".to_string()
        }
    }

    fn prompt(&mut self) -> bool {
        let mut cli = Editor::<()>::new();
        while self.pos >= self.chars.len() {
            match cli.readline(".. ") {
                Ok(input) => {
                    self.chars.append( &mut UnicodeSegmentation::graphemes(&input[..], true).into_iter().map(|x| x.to_string()).collect::<Vec<String>>() );
                },
                Err(_) => { return true }
            }
        }
        return false
    }

	fn string(&mut self, stop: String, start: usize) -> BoolResult {
		'outer: loop {
            self.adv(1);
            let ch = self.curr();
            if ch == "" {
                if self.interactive {
                    let hasErr = self.prompt();
                    if hasErr { return Err( format!("Unable to read user input.") ) }
                } else {
                    return Err( format!("File ended before reaching '{}'. Command could not be determined.", stop))
                }
                continue
            } else if ch == stop {
                break 'outer
            } else if ch == "\\" { self.adv(2) }
        }
        let s = self.get(start+1, self.pos);
        self.adv(1);
        return self.token(Token::Str(Box::new(s)))
	}

	fn symbol(&mut self, start: usize) -> BoolResult {
        let s = self.next_word(start);
        match &s[..] {
        	"if"	=> { return self.token(Token::If) },
        	"elif"	=> { return self.token(Token::Elif) },
        	"else"	=> { return self.token(Token::Else) },
            "true"  => { return self.token(Token::Const(Opcode::Const(ConstCode::True))) },
            "false" => { return self.token(Token::Const(Opcode::Const(ConstCode::False))) },
            "none"  => { return self.token(Token::Const(Opcode::Const(ConstCode::Null))) },
            _ => { }
        }
        return self.token(Token::Sym(Box::new(s)))
    }

    fn token(&mut self, tok: Token) -> BoolResult {
		self.current = mem::replace(&mut self.future, tok);
		return Ok(false)
	}

	fn token_adv(&mut self, tok: Token) -> BoolResult {
		self.current = mem::replace(&mut self.future, tok);
		self.pos += 1;
		return Ok(false)
	}

	fn whitespace(&mut self, mut nwhite : u8) -> u8 {
		loop {
			self.adv(1);
			match &self.curr()[..] {
				" " | "\r"	=> { nwhite += 1 },
				"\t"		=> { nwhite += 4 },
				_ 	=> { return nwhite }
			}
		}
	}

	pub fn skipWhite(&mut self) -> BoolResult {
		match self.current {
			Token::White(_)	=> { return self.next_token() },
			_ 	=> { return Ok(false) }
		}
	}
}


















