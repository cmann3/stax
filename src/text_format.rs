pub fn bold(s: String) -> String {format!("\x1b[1m{}\x1b[0m", s)}


pub fn red(s: String) -> String {format!("\x1b[31m{}\x1b[0m", s)}

pub fn green(s: String) -> String {format!("\x1b[32m{}\x1b[0m", s)}

pub fn yellow(s: String) -> String {format!("\x1b[33m{}\x1b[0m", s)}

pub fn blue(s: String) -> String {format!("\x1b[34m{}\x1b[0m", s)}

pub fn magenta(s: String) -> String {format!("\x1b[35m{}\x1b[0m", s)}

pub fn cyan(s: String) -> String {format!("\x1b[36m{}\x1b[0m", s)}

pub fn white(s: String) -> String {format!("\x1b[37m{}\x1b[0m", s)}

pub fn bg_red(s: String) -> String {format!("\x1b[41m{}\x1b[0m", s)}

pub fn bg_green(s: String) -> String {format!("\x1b[42m{}\x1b[0m", s)}

pub fn bg_yellow(s: String) -> String {format!("\x1b[43m{}\x1b[0m", s)}

pub fn bg_blue(s: String) -> String {format!("\x1b[44m{}\x1b[0m", s)}

pub fn bg_magenta(s: String) -> String {format!("\x1b[45m{}\x1b[0m", s)}

pub fn bg_cyan(s: String) -> String {format!("\x1b[46m{}\x1b[0m", s)}

pub fn bg_white(s: String) -> String {format!("\x1b[47m{}\x1b[0m", s)}