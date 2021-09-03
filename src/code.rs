/*	code.rs
 *	------- */
use crate::parser::Parser;
use crate::words::Word;

use std::cell::RefCell;


/*  == PRIMARY OPCODE  ==   */
#[derive(Clone, Debug, PartialEq)]
pub enum Opcode {
    GenOp(GenOp),
    Const(ConstCode),
    Num(f64),
    Prog(Box<Vec<Opcode>>),
    Quote(Box<Vec<Opcode>>),
    Str(Box<String>),
    Sym(Box<String>),
    BinOp(BinOp),
    BoolOp(BoolOp),
    StackOp(StackOp),
    Comb1(Comb1),
    Comb2(Comb2),
    Comb3(Comb3),
    UnOp(UnOp),
    MathOp(MathOp),
    AutoOp(AutoOp),
    Set(Box<String>),
    SetProg(Box<String>),

    // Implement!
    Chain2Math(MathOp, MathOp),
    Chain3Math(MathOp, MathOp, MathOp),
    Chain4Math(MathOp, MathOp, MathOp,MathOp),
    ChainConstBool(ConstCode, BinOp), // No need to const - unary since can do automatically!
    Chain2Bool(BoolOp, BoolOp),
    Chain3Bool(BoolOp, BoolOp, BoolOp),
    Blank, 
}

/*  == SUB OPCODES  ==   */
#[derive(Clone, Debug, PartialEq)]
pub enum AutoOp {
    Input,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Seq,
    Cat,
    Del,
    Rep,
    Spl
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoolOp {
    Grt,
    Lst,
    Gte,
    Lte,
    Eqt,
    Neq,
    And,
    Or
}

#[derive(Clone, Debug, PartialEq)]
pub enum Comb1 {
    Do,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Comb2 {
    Ifthen,
    Dip,
    Cleave,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Comb3 {
    Ifelse,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstCode {
    Int(i32),
    True,
    False,
    Null
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenOp {
    NewLine,
    AddLine(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MathOp {
    Abs,
    Acos,
    Acosh,
    Asin,
    Asinh,
    Atan,
    Atanh,
    Cbrt,
    Ceil,
    Cos,
    Cosh,
    Exp,
    Floor,
    Fract,
    Ln,
    Log10,
    Log2,
    Max,
    Mean,
    Min,
    Neg,
    Recip,
    Round0,
    Sd,
    Sign,
    Sin,
    Sinh,
    Sqrt,
    Tan,
    Tanh,
    Trunc,
    Var
}

#[derive(Clone, Debug, PartialEq)]
pub enum StackOp {
    Dup,
    Swap,
    Dupd,
    Swapd,
    Dig,
    Bury,
    Flip,
    Zap,
    Zapd,
    Over,
    Clear
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnOp {
    Print
}

/*  == PRINTING  ==   */
pub fn sprint_opcode(code: &Opcode) -> String {
    match code {
        Opcode::Blank       => { return format!("noop") },
        Opcode::Const(c)    => { return format!("const {}", sprint_constcode(c)) },
        Opcode::GenOp(g)    => { return format!("op {}", sprint_genop(g)) },
        Opcode::Num(f)      => { return format!("const num '{}'", f) },
        Opcode::Str(s)      => { return format!("const str '{}'", *s) }, // Shorten!
        Opcode::Sym(s)      => { return format!("sym '{}'", *s) },
        Opcode::Prog(q)  => {
            return format!("program \n\t-{}", q.iter().map(|x| sprint_opcode(x)).collect::<Vec<String>>().join("\n\t-"))
        },
        Opcode::Quote(q)    => {
            return format!("quote \n\t-{}", q.iter().map(|x| sprint_opcode(x)).collect::<Vec<String>>().join("\n\t-"))
        },
        Opcode::BinOp(b)    => { return format!("binop {} '{}'", sprint_binop(b), sprint_binop_sym(b)) },
        Opcode::BoolOp(b)   => { return format!("boolop {} '{}'", sprint_boolop(b), sprint_boolop_sym(b)) },
        Opcode::StackOp(s)  => { return format!("stackop {}", sprint_stackop(s)) },
        Opcode::MathOp(m)   => { return format!("mathop {}", sprint_mathop(m)) },
        Opcode::UnOp(u)     => { return format!("unop {}", sprint_unop(u)) },
        Opcode::Comb1(c)    => { return format!("comb1 {}", sprint_comb1(c)) },
        Opcode::Comb2(c)    => { return format!("comb2 {}", sprint_comb2(c)) },
        Opcode::Comb3(c)    => { return format!("comb3 {}", sprint_comb3(c)) },
        Opcode::AutoOp(a)   => { return format!("autoop {}", sprint_autoop(a)) },
        Opcode::Set(s)      => { return format!("set '{}'", *s) },
        Opcode::SetProg(s)  => { return format!("set '{}'", *s) },
        _ => { return format!("Not Implemented yet ...") }
    }
}

pub fn sprint_autoop(code: &AutoOp) -> String {
    match code {
        AutoOp::Input       => { return format!("input") },
    }
}

pub fn sprint_binop(code: &BinOp) -> String {
    match code {
        BinOp::Add           => { return format!("add") },
        BinOp::Sub           => { return format!("sub") },
        BinOp::Mul           => { return format!("mul") },
        BinOp::Div           => { return format!("div") },
        BinOp::Pow           => { return format!("pow") },
        BinOp::Mod           => { return format!("mod") },
        BinOp::Seq           => { return format!("seq") },
        BinOp::Cat           => { return format!("cat") },
        BinOp::Del           => { return format!("del") },
        BinOp::Rep           => { return format!("rep") },
        BinOp::Spl           => { return format!("spl") },
    }
}

pub fn sprint_binop_sym(code: &BinOp) -> String {
    match code {
        BinOp::Add           => { return format!("+") },
        BinOp::Sub           => { return format!("-") },
        BinOp::Mul           => { return format!("*") },
        BinOp::Div           => { return format!("/") },
        BinOp::Pow           => { return format!("^") },
        BinOp::Mod           => { return format!("%") },
        BinOp::Seq           => { return format!("..") },
        BinOp::Cat           => { return format!("++") },
        BinOp::Del           => { return format!("--") },
        BinOp::Rep           => { return format!("**") },
        BinOp::Spl           => { return format!("//") },
    }
}

pub fn sprint_boolop(code: &BoolOp) -> String {
    match code {
        BoolOp::Grt           => { return format!("grt") },
        BoolOp::Lst           => { return format!("lst") },
        BoolOp::Gte           => { return format!("gte") },
        BoolOp::Lte           => { return format!("lte") },
        BoolOp::Eqt           => { return format!("eqt") },
        BoolOp::Neq           => { return format!("neq") }, 
        BoolOp::And           => { return format!("and") },
        BoolOp::Or            => { return format!("or") },
    }
}

pub fn sprint_boolop_sym(code: &BoolOp) -> String {
    match code {
        BoolOp::Grt           => { return format!(">") },
        BoolOp::Lst           => { return format!("<") },
        BoolOp::Gte           => { return format!(">=") },
        BoolOp::Lte           => { return format!("<=") },
        BoolOp::Eqt           => { return format!("==") },
        BoolOp::Neq           => { return format!("!=") }, 
        BoolOp::And           => { return format!("&") },
        BoolOp::Or            => { return format!("|") },
    }
}

pub fn sprint_comb1(code: &Comb1) -> String {
    match code {
        Comb1::Do      => { return format!("do") },
    }
}

pub fn sprint_comb2(code: &Comb2) -> String {
    match code {
        Comb2::Ifthen   => { return format!("ifthen") },
        Comb2::Dip      => { return format!("dip") },
        Comb2::Cleave   => { return format!("cleave") }
    }
}

pub fn sprint_comb3(code: &Comb3) -> String {
    match code {
        Comb3::Ifelse      => { return format!("ifelse") },
    }
}

pub fn sprint_constcode(code: &ConstCode) -> String {
    match code {
        ConstCode::Int(i)      => { return format!("int '{}'", i) },
        //ConstCode::Num(f)      => { return format!("num '{}'", f) },
        ConstCode::True        => { return format!("bool 'true'") },
        ConstCode::False       => { return format!("bool 'false'") },
        ConstCode::Null        => { return format!("none ") },
    }
}

pub fn sprint_constval(code: &ConstCode) -> String {
    match code {
        ConstCode::Int(i)      => { return format!("{}", i) },
        ConstCode::True        => { return format!("true") },
        ConstCode::False       => { return format!("false") },
        ConstCode::Null        => { return format!("none") },
    }
}

pub fn sprint_genop(code: &GenOp) -> String {
    match code {
        GenOp::NewLine      => { return format!("newline") },
        GenOp::AddLine(u)   => { return format!("addline'{}'", u)}  
    }
}

pub fn sprint_mathop(code: &MathOp) -> String {
    match code {
        MathOp::Abs         => { return format!("abs") },
        MathOp::Acos        => { return format!("acos") },
        MathOp::Acosh       => { return format!("acosh") },
        MathOp::Asin        => { return format!("asin") },
        MathOp::Asinh       => { return format!("asinh") },
        MathOp::Atan        => { return format!("atan") },
        MathOp::Atanh       => { return format!("atanh") },
        MathOp::Cbrt        => { return format!("cbrt") },
        MathOp::Ceil        => { return format!("ceil") },
        MathOp::Cos         => { return format!("cos") },
        MathOp::Cosh        => { return format!("cosh") },
        MathOp::Exp         => { return format!("exp") },
        MathOp::Floor       => { return format!("floor") },
        MathOp::Fract       => { return format!("fract") },
        MathOp::Ln          => { return format!("ln") },
        MathOp::Log10       => { return format!("log10") },
        MathOp::Log2        => { return format!("log2") },
        MathOp::Max         => { return format!("max") },
        MathOp::Mean        => { return format!("mean") },
        MathOp::Min         => { return format!("min") },
        MathOp::Neg         => { return format!("neg") },
        MathOp::Recip       => { return format!("recip") },
        MathOp::Round0      => { return format!("round0") },
        MathOp::Sd          => { return format!("sd") },
        MathOp::Sign        => { return format!("sign") },
        MathOp::Sin         => { return format!("sin") },
        MathOp::Sinh        => { return format!("sinh") },
        MathOp::Sqrt        => { return format!("sqrt") },
        MathOp::Tan         => { return format!("tan") },
        MathOp::Tanh        => { return format!("tanh") },
        MathOp::Trunc       => { return format!("trunc") },
        MathOp::Var         => { return format!("var") },
    }
}

pub fn sprint_stackop(code: &StackOp) -> String {
    match code {
        StackOp::Dup        => { return format!("dup") },
        StackOp::Swap       => { return format!("swap") },
        StackOp::Dupd       => { return format!("dupd") },
        StackOp::Swapd      => { return format!("swapd") },
        StackOp::Dig        => { return format!("dig") },
        StackOp::Bury       => { return format!("bury") },
        StackOp::Flip       => { return format!("flip") },
        StackOp::Zap        => { return format!("zap") },
        StackOp::Zapd       => { return format!("zapd") },
        StackOp::Over       => { return format!("over") },
        StackOp::Clear      => { return format!("clear") }
    }
}

pub fn sprint_unop(code: &UnOp) -> String {
    match code {
        UnOp::Print         => { return format!("print") },
    }
}

/*  == OPCODE METHODS == */
impl Opcode {
    pub fn is_infix(&self) -> bool {
        match self {
            Opcode::BinOp(_) => { return true },
            _ => { return false}
        }
    }

    pub fn is_sym(&self) -> bool {
        match self {
            Opcode::Sym(_) => { return true },
            _ => { return false }
        }
    }

    pub fn is_unary(&self) -> bool {
        match self {
            Opcode::BinOp(o) => match o {
                BinOp::Sub  => { return true },
                _ => { return false }
            },
            _   => { return false }
        }
    }

    pub fn prec(&self) -> u8 {
        match self {
            Opcode::BinOp(o)  => { return get_prec(o) }, 
            _                 => { return 3 } // above assignment (and ,) but below everything else!  
        }
    }

    pub fn token(&self) -> u16 {
        match self {
            _   => { return 0 }
        }
    }
}

/*  == MISC OPCODE FUNCTIONS == */
pub fn get_prec(op: &BinOp) -> u8 {
    match op {
        BinOp::Add | BinOp::Sub => { return 15 },
        BinOp::Mul | BinOp::Div | BinOp::Mod    => { return 17 },
        BinOp::Pow => { return 19 },
        BinOp::Seq => { return 14 },
        BinOp::Cat | BinOp::Del | BinOp::Rep | BinOp::Spl => { return 15 },
        _ => { return 15 }
    }
}

pub fn get_prec_bool(op: &BoolOp) -> u8 {
    match op {
        BoolOp::Grt | BoolOp::Lst | BoolOp::Gte | BoolOp::Lte  => { return 13 },
        BoolOp::Eqt | BoolOp::Neq  => { return 12 },
        BoolOp::And => { return 11 },
        BoolOp::Or  => { return 10 },
        _ => { return 15 }
    }
}