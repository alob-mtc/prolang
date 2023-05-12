

const ILLEGAL: &str = "ILLEGAL";
const EOF: &str = "EOF";
// Identifiers + literals
const IDENT: &str = "IDENT";
const INT: &str = "INT";
// Operators
const ASSIGN: &str = "=";
const PLUS: &str = "+";
const MINUS: &str = "-";
const BANG: &str = "!";
const ASTERISK: &str = "*";
const SLASH: &str = "/";


const LT: &str = "<";
const GT: &str = ">";

// Delimiters
const COMMA: &str = ",";
const SEMICOLON: &str = ";";


const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";

// Keywords
const FUNCTION: &str = "FUNCTION";
const LET: &str = "LET";
const TRUE: &str = "TRUE";


let keywords: std::Hashmap = std::hashmap::from()
