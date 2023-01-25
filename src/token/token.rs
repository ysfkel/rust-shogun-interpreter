use std::collections::HashMap;

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LT_EQ: &str = "<=";
pub const GT_EQ: &str = ">=";

pub const OR: &str = "||";
pub const AND: &str = "&&";

//bitwise
pub const B_OR: &str = "|";
pub const B_AND: &str = "&";
pub const L_SHIFT: &str = "<<";
pub const R_SHIFT: &str = ">>";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = "(";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

pub type TokenType = String;

#[derive(Clone)]
pub struct Token {
    pub token: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token: &str, literal: &str) -> Self {
        Self {
            token: token.to_string(),
            literal: literal.to_string(),
        }
    }

    pub fn keywords() -> HashMap<String, TokenType> {
        let _keywords: HashMap<String, TokenType> = HashMap::<String, TokenType>::from([
            ("fn".to_owned(), String::from(FUNCTION)),
            ("let".to_owned(), String::from(LET)),
            ("true".to_owned(), String::from(TRUE)),
            ("false".to_owned(), String::from(FALSE)),
            ("if".to_owned(), String::from(IF)),
            ("else".to_owned(), String::from(ELSE)),
            ("return".to_owned(), String::from(RETURN)),
        ]);

        _keywords
    }

    pub fn lookup_indent(ident: &str) -> String {
        if let Some(val) = Self::keywords().get(ident) {
            val.clone()
        } else {
            IDENT.to_owned()
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token: "".to_owned(),
            literal: "".to_owned(),
        }
    }
}
