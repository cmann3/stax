/* 	lexer.rs
 * 	-------- */
use crate::code::{Opcode, BinOp, BoolOp, ConstCode, get_prec, get_prec_bool};
use crate::expr::{Expr, walk_expr};
use crate::words::Word;

use unicode_segmentation::UnicodeSegmentation;
use rustyline::Editor;


/*  HELPER FUNCTIONS    */
fn num_chars(ch: &String) -> bool { return "0123456789".contains(ch) }
fn sym_chars(ch: &String) -> bool { return "=-+!@#$%^&*:.<>?/|\\~".contains(ch) }
fn stop_chars(ch: &String) -> bool { return " \t\r\n[](){}!@#$%^&*-+=:;,.<>?/|\\~`'\"".contains(ch) }
fn stop_chars_num(ch: &String) -> bool { return " \t\r\n[](){}!@#$%^&*-+=:;,<>?/|\\~`'\"".contains(ch) }
fn alpha_chars(ch: &String) -> bool { return "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(ch) }

/*  HELPER TYPES  */
type ParseResult = Result<Expr, ParseError>;
type OpResult = Result<Vec<Opcode>, ParseError>;

pub struct Token {
    pub word   : Word,
    pub start  : usize,
    pub end    : usize,
    pub wordn  : u16
}

#[derive(Debug)]
pub struct ParseError {
    pub start   : usize,
    pub end     : usize,
    pub msg     : String,
    pub wordn   : u16
}


/*  MAIN PARSER  */
pub struct Parser {
    pub breaks      : Vec<usize>,
    pub chars       : Vec<String>,
    pub codes       : Vec<Expr>,
    pub file        : String,
    interactive     : bool,
    line            : u16,
    pos             : usize,
    ppos            : usize,
    stop            : String,
    pub tokens      : Vec<Token>,
    wordn           : u16
}

impl Parser {
    pub fn new(input: String, interactive: bool, file: &str) -> Self {
        Parser {
            breaks      : Vec::new(),
            chars       : UnicodeSegmentation::graphemes(&input[..], true).into_iter().map(|x| x.to_string()).collect::<Vec<String>>(),
            codes       : Vec::<Expr>::new(),
            file        : file.to_string(),
            interactive : interactive,  
            line        : 0,
            pos         : 0,
            ppos        : 0,
            stop        : "".to_string(),
            tokens      : Vec::new(),
            wordn       : 0
        }
    }

    /* MAIN LEXING FUNCTIONS */
    pub fn lex(&mut self) -> OpResult {
        let mut result = Vec::new();
        while self.pos < self.chars.len() {
            self.whitespace();
            match self.lex_until(self.pos, "\n".to_string()) {
                Ok(mut codes)   => { result.append(&mut codes) }, 
                Err(e)  => { return Err(e) }
            }
        }
        return Ok(result)
    }

    fn lex_next(&mut self) -> ParseResult {
        let ch = self.curr();
        match &ch[..] {
            "\n" | "\r" | "\r\n"    => return self.newline(self.pos),
            "("     => { return self.paren("(".to_string(), ")".to_string()) },
            "["     => { return self.paren("[".to_string(), "]".to_string()) },
            "{"     => { return self.paren("{".to_string(), "}".to_string()) },
            "\""    => { return self.string("\"", self.pos) },
            _ => {
                if num_chars(&ch) { return self.number(self.pos, 0) }
                if sym_chars(&ch) { return self.infix(self.pos) }
                return self.symbol(self.pos)
            }
        }
    }

    fn lex_until(&mut self, start: usize, stop: String) -> OpResult {
        let mut result = Vec::new();
        'outer: loop {
            self.whitespace();
            let curr = self.curr();
            if curr == "" {
                if stop.contains("\n") { break 'outer }
                if self.interactive {
                    let hasErr = self.prompt();
                    if hasErr { return Err( 
                        ParseError{start: start, end: self.pos, msg: "Unable to read user input.".to_string(), wordn: self.wordn} 
                    )}
                } else {
                    return Err( ParseError{
                        start: start, end: self.pos, msg: format!("File ended before reaching '{}'. Command could not be determined.", stop), wordn: self.wordn
                    })
                }
            } else if stop.contains(&curr) {
                self.stop = curr.clone();
                if stop.contains("\n") { self.newline( self.pos );
                } else { self.adv(1) }
                break 'outer;
            } else {
                match self.lex_next() {
                    Ok(v)   => { result.push(v) },
                    Err(e)      => { return Err(e) }
                }
            }
        }
        return self.parse(result)
    }

    /* MAIN PARSING FUNCTIONS */
    pub fn parse(&mut self, ops : Vec<Expr>) -> OpResult {
        self.codes  = ops;
        self.ppos   = 0;
        let mut results = Vec::new();
        while !self.check_parse() {
            match self.parse_primary(true) {
                Ok(x) => match self.parse_binary(x, 0) {
                    Ok((code, _)) => { results.append( &mut walk_expr(code) ) },
                    Err(e) => { return Err(e) }
                },
                Err(e) => { return Err(e) }
            }
        }
        self.codes.clear();
        return Ok(results)
    }

    fn parse_primary(&mut self, first: bool) -> Result<Expr, ParseError> {
        while !self.check_parse() {
            let code = self.curr_parse_adv(1);
            match code {
                Expr::Blank      => { },
                Expr::Infix(o,_)   => { 
                    // TODO: Check for UNARY
                    match self.parse_primary(false) {
                        Ok(expr_code) => {
                            let rhs = Expr::Incomplete(o, Box::new(expr_code));
                            if first { return Ok( Expr::Primary(walk_expr(rhs)) ) }
                            let op  = Opcode::Quote(Box::new(walk_expr(rhs)));
                            return Ok( Expr::Single(op) )
                        },
                        Err(e) => { return Err(e) }
                    }
                    // multiple infixes a row ... or start
                    // TODO: Allow multiple in a row
                    //let (start, end, wordn) = self.extract_token(o.token());
                    //return Err( ParseError{ start: start, end: end, msg: format!("Multiple binary operations found in succession or expression started with binary operation."), wordn: wordn} ) 
                },
                _ => return Ok( code )
            }
        }
        return Ok( Expr::Blank )
    }

    fn parse_binary(&mut self, lhs: Expr, prec: u8) -> Result<(Expr, u8), ParseError> {
        if self.check_parse() { return Ok((lhs, prec)) }
        let code = self.curr_parse();
        match code {
            Expr::Infix(op, op_prec) => {
                if op_prec > prec {
                    self.adv_parse(1);
                    let rhs_test = self.parse_primary(false);
                    if rhs_test.is_err() { return Err(rhs_test.expect_err("")) }
                    let rhs = rhs_test.expect("");
                    match self.parse_binary(rhs, op_prec) {
                        Ok((expr, new_prec)) => {
                            let new_code = Expr::Binary(op.clone(), Box::new(lhs), Box::new(expr));
                            if new_prec <= prec { return Ok((new_code, new_prec)) }
                            return self.parse_binary(new_code, prec)
                        },
                        Err(e) => { return Err(e) }
                    }
                } else {
                    return Ok((lhs, prec))
                }
            },
            /*Expr::Primary(codes) => { 
                return self.parse_binary( Expr::Unary(codes.to_vec(), Box::new(lhs)), prec )
            },*/
            _ => { return Ok((lhs, prec)) } // stops parsing expr
        }
    }


    /* HELPER FUNCTIONS */
    fn adv(&mut self, n: usize) { self.pos += n }
    fn adv_parse(&mut self, n: usize) { self.ppos += n }

    fn check_parse(&self) -> bool { return self.ppos >= self.codes.len() }

    fn curr(&mut self) -> String {
        match self.chars.get(self.pos) {
            Some(x) => return x.clone(),
            None    => return "".to_string()
        }
    }

    fn curr_parse(&mut self) -> Expr {
        (*self.codes.get(self.ppos).unwrap()).clone()
    }

    fn curr_parse_adv(&mut self, n: usize) -> Expr {
        let curr = self.curr_parse();
        self.adv_parse(n);
        return curr
    }

    pub fn extract_str(&self, loc: u16) -> String {
        match self.tokens.get(loc as usize) {
            Some(tok) => { return tok.word.sprint_short(0) },
            None => { return "".to_string() }
        }
    }

    pub fn extract_token(&self, loc: u16) -> (usize, usize, u16) {
        match self.tokens.get(loc as usize) {
            Some(tok) => { return (tok.start, tok.end, tok.wordn) },
            None => { return (0, 0, 0) }
        }
    }

    pub fn extract_word(&self, loc: u16) -> Word {
        match self.tokens.get(loc as usize) {
            Some(tok) => { return tok.word.clone() },
            None => { return Word::Null }
        }
    }

    pub fn find_line(&self, start: usize) -> usize {
        let n = self.breaks.len();
        if n == 0 { return 0 }
        for i in 0..n {
            if start <= self.breaks[i] { return i }
        }
        return n
    }

    fn get(&self, start: usize, end: usize) -> String { return self.chars[start..end].join("") }

    fn infix(&mut self, start: usize) -> ParseResult {
        loop {
            self.adv(1);
            let ch = self.curr();
            if ch == "" || !sym_chars(&ch) { break }
        }
        let s    = self.get(start, self.pos);
        let op : Opcode;
        let prec : u8;
        match &s[..] {
            //"="     => { return Ok(Expr::Assign(true)) },
            //"=>"    => { return Ok(Expr::Assign(false)) },
            "+"     => { op = Opcode::BinOp(BinOp::Add); prec = get_prec(&BinOp::Add); },
            "-"     => { op = Opcode::BinOp(BinOp::Sub); prec = get_prec(&BinOp::Add); },
            "*"     => { op = Opcode::BinOp(BinOp::Mul); prec = get_prec(&BinOp::Mul); },
            "/"     => { op = Opcode::BinOp(BinOp::Div); prec = get_prec(&BinOp::Div); },
            "^"     => { op = Opcode::BinOp(BinOp::Pow); prec = get_prec(&BinOp::Pow); },
            "%"     => { op = Opcode::BinOp(BinOp::Mod); prec = get_prec(&BinOp::Mul); },
            "=="    => { op = Opcode::BoolOp(BoolOp::Eqt); prec = get_prec_bool(&BoolOp::Eqt); },
            "!="    => { op = Opcode::BoolOp(BoolOp::Neq); prec = get_prec_bool(&BoolOp::Eqt); },
            ">"     => { op = Opcode::BoolOp(BoolOp::Grt); prec = get_prec_bool(&BoolOp::Grt); },
            "<"     => { op = Opcode::BoolOp(BoolOp::Lst); prec = get_prec_bool(&BoolOp::Lst); },
            ">="    => { op = Opcode::BoolOp(BoolOp::Gte); prec = get_prec_bool(&BoolOp::Grt); },
            "<="    => { op = Opcode::BoolOp(BoolOp::Lte); prec = get_prec_bool(&BoolOp::Lst); },
            "&"     => { op = Opcode::BoolOp(BoolOp::And); prec = get_prec_bool(&BoolOp::And); },
            "|"     => { op = Opcode::BoolOp(BoolOp::Or); prec = get_prec_bool(&BoolOp::Or); },
            ".."    => { op = Opcode::BinOp(BinOp::Seq); prec = get_prec(&BinOp::Seq); },
            "++"    => { op = Opcode::BinOp(BinOp::Cat); prec = get_prec(&BinOp::Cat); },
            "--"    => { op = Opcode::BinOp(BinOp::Del); prec = get_prec(&BinOp::Del); },
            "**"    => { op = Opcode::BinOp(BinOp::Rep); prec = get_prec(&BinOp::Rep); },
            "//"    => { op = Opcode::BinOp(BinOp::Spl); prec = get_prec(&BinOp::Spl); },
            _ => { op = Opcode::Sym(Box::new(s)); prec = 3; }
        }
        return Ok(Expr::Infix(op, prec))
    }

    fn make_error(&mut self, start: usize, end: usize, msg: String) -> ParseResult {
        return Err( ParseError{ start: start, end: end, msg: msg, wordn: self.wordn} )
    }

    fn make_number(&mut self, start: usize, end: usize, nperiod: usize, isSym: bool) -> ParseResult {
        let s = self.get(start, end);
        if isSym { return Ok(Expr::Single(Opcode::Sym(Box::new(s)))) }
        if nperiod == 0 {
            match s.parse::<i32>() {
                Ok(x)  => { return Ok(Expr::Single(Opcode::Const(ConstCode::Int(x)))) },
                Err(_) => { }
            }
        }
        match s.parse::<f64>() {
            Ok(x)   => { return Ok(Expr::Single(Opcode::Num(x))) },
            Err(_)  => { return self.make_error(start, end, format!("Object, {}, could not be converted to a number.", s)) }
        }
    }

    fn newline(&mut self, n: usize) -> ParseResult {
        self.breaks.push(n);
        self.wordn = 0;
        self.line += 1;
        self.adv(1);
        Ok(Expr::Blank)
    }

    fn next_word(&mut self, start: usize) -> String {
        loop {
            self.adv(1);
            let ch = self.curr();
            if ch == "" || stop_chars(&ch) { break }
        }
        return self.get(start, self.pos)
    }

    fn number(&mut self, start: usize, mut nperiod: usize) -> ParseResult {
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

    fn paren(&mut self, ch: String, stop: String) -> ParseResult {
        let start = self.pos;
        self.adv(1);
        match self.lex_until(start, stop) {
            Ok(v) => match &ch[..] {
                "(" => { return Ok(Expr::Primary(v)) },
                "[" => { return Ok(Expr::Single(Opcode::Quote(Box::new(v)))) },
                _ => { return Ok(Expr::Blank) }
            },
            Err(e) => { return Err(e) }
        }
    }

    fn peek(&self, n: usize) -> String {
        match self.chars.get(self.pos + n) {
            Some(x) => return x.clone(),
            None    => return "".to_string()
        }
    }

    fn prev(&mut self, n: usize) { self.pos -= n }

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

    fn push_token(&mut self, word: Word, start: usize, end: usize) -> u16 {
        self.tokens.push(Token{ word: word, start: start, end: end, wordn: self.wordn });
        self.wordn += 1;
        return self.wordn - 1
    }

    fn string(&mut self, stop: &str, start: usize) -> ParseResult {
        'outer: loop {
            self.adv(1);
            let ch = self.curr();
            if ch == "" {
                if self.interactive {
                    let hasErr = self.prompt();
                    if hasErr { return Err( ParseError{start: start, end: self.pos, msg: "Unable to read user input.".to_string(), wordn: self.wordn} )}
                } else {
                    return Err( ParseError{start: start, end: self.pos, msg: format!("File ended before reaching '{}'. Command could not be determined.", stop), wordn: self.wordn} )
                }
                continue
            } else if ch == stop {
                break 'outer
            } else if ch == "\\" { self.adv(2) }
        }
        let s = self.get(start+1, self.pos);
        self.adv(1);
        return Ok(Expr::Single(Opcode::Str(Box::new(s))));
    }

    fn symbol(&mut self, start: usize) -> ParseResult {
        let s = self.next_word(start);
        match &s[..] {
            "true"  => { return Ok(Expr::Single(Opcode::Const(ConstCode::True))) },
            "false" => { return Ok(Expr::Single(Opcode::Const(ConstCode::False))) },
            "none"  => { return Ok(Expr::Single(Opcode::Const(ConstCode::Null))) },
            _ => { }
        }
        return Ok(Expr::Single(Opcode::Sym(Box::new(s))));
    }

    fn whitespace(&mut self) -> usize {
        let mut nwhite: usize = 0;
        loop {
            match &self.curr()[..] {
                " " | "\r"  => { nwhite += 1 },
                "/t"        => { nwhite += 4 },
                _           => return nwhite
            }
            self.adv(1);
        }
    }
}




