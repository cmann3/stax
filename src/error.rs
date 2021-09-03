/* error.rs
 * -------- */ 
use crate::parser::Parser;
use crate::text_format::*;

#[derive(Debug, Clone)]
pub enum Message {
	Custom(String),
	Error(ErrorType),
	Message,
	Note,
	Traceback(String),
	Warning 
}

#[derive(Debug, Clone)]
pub enum ErrorType {
	Index,
	Overflow,
	Type
}

impl ErrorType {
	fn sprint(&self) -> String {
		match self {
			ErrorType::Index 		=> { return "INDEX".to_string() },
			ErrorType::Overflow 	=> { return "OVERFLOW".to_string() },
			ErrorType::Type 		=> { return "TYPE".to_string() },
		}
	}
}


pub fn format_message(msg: String, typ: Message, parser: &Parser, tok: Option<u16>) -> String {
	let header : String;
	match typ {
		Message::Custom(s)		=> { header = format!("{}: {}", bold(green(s)), msg) },
		Message::Error(e) 		=> { header = bold(format!("{}: {}", red(format!("{} ERROR", e.sprint())), msg)) },
		Message::Message  		=> { header = msg },
		Message::Note 	 		=> { header = format!("{}: {}", bold(cyan("NOTE".to_string())), msg) },
		Message::Traceback(s)	=> { header = format!("{}: {}", bold(format!("{}, '{}'", red("TRACEBACK".to_string()), s)), msg) },
		Message::Warning		=> { header = bold(format!("{}: {}", yellow("WARNING".to_string()), msg)) }
	}
	match tok {
		Some(u) => {
			let (start, _, wordn) = parser.extract_token(u);
			let mid = format!("    {} {}:{}:{}",
				blue("-->".to_string()),
				parser.file,
				parser.find_line(start),
				wordn
			);
			return format!("{}\n{}", header, mid)
		},
		None => { return header }
	}
}