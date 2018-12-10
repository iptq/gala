use std::collections::VecDeque;
use std::str;

use failure;

const TAB_INDENT_WIDTH: usize = 8;
const MAX_DEPTH: usize = 64;

pub type Spanned<Token, Location, Error> = Result<(Location, Token, Location), Error>;

#[derive(Clone, Debug)]
pub enum Token {
    // indentation
    Newline,
    Indent,
    Dedent,
    EOF,

    // symbols
    Arrow,
    DoubleEqual,
    NotEqual,

    Colon,
    Comma,
    Dash,
    Dot,
    Equal,
    LeftParen,
    Plus,
    RightParen,
    Semicolon,
    Star,

    // literals
    Integer(u32),

    // keywords
    KeywordElse,
    KeywordExtern,
    KeywordFalse,
    KeywordFn,
    KeywordIf,
    KeywordLet,
    KeywordReturn,
    KeywordStruct,
    KeywordTrue,
    KeywordWhile,

    // types
    TypeBool,
    TypeChar,
    TypeInt,
    TypeString,

    String(String),
    Symbol(String),
    Ident(String),
    Char(char),
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "Lex error: {}", message)]
pub struct LexError {
    message: String,
}

#[derive(Clone)]
pub struct Lexer {
    source: String,
    position: usize,
    queue: VecDeque<Spanned<Token, usize, LexError>>,
    istack: Vec<usize>,
    nesting: usize,
    first: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            source: input.to_owned(),
            position: 0,
            queue: VecDeque::new(),
            istack: vec![0],
            nesting: 0,
            first: true,
        };
        lexer.precalc();
        lexer
    }
    fn rest(&self) -> &str {
        return &self.source[self.position..];
    }
    fn peek(&self, offset: usize) -> Option<char> {
        let rest = self.rest();
        if rest.len() == 0 {
            return None;
        }
        return rest.chars().nth(offset);
    }
    fn peekwhile<F>(&self, f: F, offset: usize) -> &str
    where
        F: Fn(char) -> bool,
    {
        let mut length: usize = offset;
        while let Some(ch) = self.peek(length) {
            if !f(ch) {
                break;
            }
            length += 1;
        }
        return &self.rest()[offset..length];
    }
    fn whitecount(&self, line: &str) -> (usize, usize) {
        let mut count = 0usize;
        let mut len = 0usize;
        for c in line.chars() {
            if !(c == '\t' || c == ' ') {
                break;
            }
            count += 1;
            len += match c {
                '\t' => TAB_INDENT_WIDTH,
                ' ' => 1,
                _ => 0,
            };
        }
        return (len, count);
    }
    fn indentcalc(&mut self, line: &str) -> usize {
        if self.nesting > 0 {
            return 0;
        }

        let (whitelen, whitecount) = self.whitecount(line);
        let mut level = self.istack.len() - 1;
        if whitelen == self.istack[level] {
            if !self.first {
                self.queue
                    .push_back(Ok((self.position, Token::Newline, self.position + 1)));
            }
            self.first = false;
            return 0;
        }

        if whitelen > self.istack[level] {
            self.queue.push_back(Ok((
                self.position,
                Token::Indent,
                self.position + whitecount,
            )));
            if level + 1 > MAX_DEPTH {
                panic!("exceeded max depth");
            }
            self.istack.push(whitelen);
            return whitelen;
        }

        while whitelen < self.istack[level] {
            level -= 1;
            self.queue
                .push_back(Ok((self.position, Token::Dedent, self.position)));
            self.queue
                .push_back(Ok((self.position, Token::Newline, self.position)));
            self.istack.pop();
        }

        0
    }
    fn read_comment(&mut self) {
        let mut length = 0;
        self.position += 1;
        while let Some(c) = self.peek(length) {
            if c == '\n' {
                break;
            }
            length += 1;
        }
        self.position += length;
    }
    fn read_ident(&mut self) {
        // already guaranteed that ch is not going to be a digit
        let name = self
            .peekwhile(
                |ch| {
                    (ch >= 'a' && ch <= 'z')
                        || (ch >= 'A' && ch <= 'Z')
                        || (ch >= '0' && ch <= '9')
                        || ch == '_'
                },
                0,
            ).to_owned();
        let length = name.len();
        self.queue.push_back(Ok((
            self.position,
            match name.as_ref() {
                "else" => Token::KeywordElse,
                "extern" => Token::KeywordExtern,
                "false" => Token::KeywordFalse,
                "fn" => Token::KeywordFn,
                "if" => Token::KeywordIf,
                "let" => Token::KeywordLet,
                "return" => Token::KeywordReturn,
                "struct" => Token::KeywordStruct,
                "true" => Token::KeywordTrue,
                "while" => Token::KeywordWhile,

                "bool" => Token::TypeBool,
                "char" => Token::TypeChar,
                "int" => Token::TypeInt,
                "string" => Token::TypeString,

                _ => Token::Ident(name.to_owned()),
            },
            self.position + length,
        )));
        self.position += length;
    }
    fn read_number_generic(&self, base: u32) -> Option<(u32, usize)> {
        let mut dstr = String::new();
        let mut length = 0;
        let mut float = false;
        let mut unsigned = false;
        let mut long = false;
        let mut uchecked = false;
        let mut lchecked = false;
        if base != 10 {
            length += 2;
        }
        while let Some(c) = self.peek(length) {
            match c {
                '0'...'9' => if lchecked || uchecked {
                    return None;
                } else {
                    dstr += &c.to_string();
                },
                'a'...'f' | 'A'...'F' => if lchecked || uchecked {
                    return None;
                } else if base == 16 {
                    dstr += &c.to_string();
                },
                '.' => if lchecked || uchecked {
                    return None;
                } else {
                    float = true;
                    dstr += &c.to_string();
                },
                'u' => if lchecked || float {
                    return None; // this is definitely an error
                } else {
                    unsigned = true;
                    uchecked = true;
                },
                'L' => {
                    long = true;
                    lchecked = true;
                }
                _ => break,
            }
            length += 1;
        }

        // temporarily using unwrap here, we know what chars are in this string anyway
        if float {
            // if long {
            //     Number::LongFloat(FromStr::from_str(&dstr).unwrap())
            // } else {
            //     Number::Float(FromStr::from_str(&dstr).unwrap())
            // }
            None
        } else {
            match (unsigned, long) {
                (false, false) => Some((
                    // println!("dstr: '{}'", dstr);
                    // (true, true) => Number::ULongInteger(u64::from_str_radix(&dstr, base).unwrap()),
                    // (true, false) => Number::UInteger(u32::from_str_radix(&dstr, base).unwrap()),
                    // (false, true) => Number::LongInteger(i64::from_str_radix(&dstr, base).unwrap()),
                    u32::from_str_radix(&dstr, base).unwrap(),
                    length,
                )),
                _ => None,
            }
        }
        // Some((
        //     Number::Integer(i32::from_str_radix(&dstr, base as u32).unwrap()),
        //     length,
        // ))
    }
    fn read_number(&mut self) {
        let value = match self.peek(0) {
            Some('0') => match self.peek(1) {
                // this could be bin / oct / hex
                Some('b') | Some('B') => self.read_number_generic(2),
                Some('o') | Some('O') => self.read_number_generic(8),
                Some('x') | Some('X') => self.read_number_generic(16),
                _ => self.read_number_generic(10),
            },
            Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6') | Some('7')
            | Some('8') | Some('9') => self.read_number_generic(10),
            Some(c) => panic!("invalid numeric character: {:?}", c),
            None => panic!("what"),
        };

        match value {
            Some((v, len)) => {
                self.queue
                    .push_back(Ok((self.position, Token::Integer(v), self.position + len)));
                self.position += len;
            }
            None => (),
        }
    }
    fn read_char(&mut self) {
        let retval;
        let next;
        if let Some(c) = self.peek(1) {
            if c == '\'' {
                panic!("empty character");
            }
            if c == '\\' {
                if let Some(c) = self.peek(2) {
                    retval = match c {
                        'n' => '\n',
                        't' => '\t',
                        '\\' => '\\',
                        '\'' => '\'',
                        '\"' => '\"',
                        _ => panic!("invalid character escape: \\{}", c),
                    };
                    next = 3;
                } else {
                    panic!("premature end of string (0)");
                }
            } else {
                retval = c;
                next = 2;
            }
        } else {
            panic!("premature end of string (1)");
        }
        if let Some('\'') = self.peek(next) {
            self.queue.push_back(Ok((
                self.position,
                Token::Char(retval),
                self.position + next,
            )));
            self.position += next + 1;
            return;
        }
        panic!("unterminated char literal");
    }
    fn read_string(&mut self) {
        // TODO: check triple string
        let quote_type: char = self.peek(0).unwrap();
        let mut length = 1;
        let mut chars = String::new();
        while let Some(c) = self.peek(length) {
            // eprintln!("read char: {}", c);
            if c == quote_type {
                break;
            } else if c == '\\' {
                length += 1;
                if let Some(c) = self.peek(length) {
                    chars.push(match c {
                        'n' => '\n',
                        't' => '\t',
                        '\\' => '\\',
                        '\'' => '\'',
                        '\"' => '\"',
                        _ => panic!("invalid character escape: \\{}", c),
                    });
                } else {
                    panic!("premature end of string");
                }
            } else {
                chars.push(c);
            }
            length += 1;
        }
        // eprintln!("Final: {}", chars);
        self.queue.push_back(Ok((
            self.position,
            Token::String(chars),
            self.position + length,
        )));
        self.position += length + 1; // for quote
    }
    fn skipwhite(&self) -> usize {
        let mut offset = 0;
        loop {
            match self.peek(offset) {
                Some(' ') | Some('\t') => offset += 1,
                _ => break,
            }
        }
        return offset;
    }
    fn precalc(&mut self) {
        while let Some(c) = self.peek(0) {
            if c == '\n' {
                self.queue
                    .push_back(Ok((self.position, Token::Newline, self.position)));
                self.position += 1;
                let white = {
                    // rip 0 copy
                    let chars = self.peekwhile(|c| c != '\n', 0).to_owned();
                    self.indentcalc(chars.as_ref())
                };
                self.position += white;
                continue;
            } else {
                let white = self.skipwhite();
                self.position += white;
                if white > 0 {
                    continue;
                }
            }

            // check tokens
            if let Some(c2) = self.peek(1) {
                // match double token
                let opt = match (c, c2) {
                    ('-', '>') => Some(Token::Arrow),
                    ('=', '=') => Some(Token::DoubleEqual),
                    ('!', '=') => Some(Token::NotEqual),
                    (_, _) => None,
                };
                match opt {
                    Some(token) => {
                        self.queue
                            .push_back(Ok((self.position, token, self.position + 2)));
                        self.position += 2;
                        continue;
                    }
                    None => (),
                }
            }
            match c {
                '#' => self.read_comment(),
                '(' | ')' | '=' | ':' | ';' | '.' | ',' | '+' | '-' | '*' | '/' => {
                    self.queue.push_back(Ok((
                        self.position,
                        match c {
                            ':' => Token::Colon,
                            ',' => Token::Comma,
                            '-' => Token::Dash,
                            '.' => Token::Dot,
                            '=' => Token::Equal,
                            '(' => Token::LeftParen,
                            '+' => Token::Plus,
                            ')' => Token::RightParen,
                            ';' => Token::Semicolon,
                            '*' => Token::Star,
                            _ => Token::Symbol(
                                self.source[self.position..self.position + 1].to_owned(),
                            ),
                        },
                        self.position + 1,
                    )));
                    self.position += 1;
                }
                '\'' => self.read_char(),
                '"' => self.read_string(),
                // raw literals here
                'a'...'z' | 'A'...'Z' => self.read_ident(),
                '0'...'9' => self.read_number(),
                _ => self.position += 1,
            };
        }
        self.indentcalc("");
        self.queue
            .push_back(Ok((self.position, Token::EOF, self.position)));
    }
}

impl Iterator for Lexer {
    type Item = Spanned<Token, usize, failure::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let opt = self.queue.pop_front();
        opt.map(|res| res.map_err(|err| err.into()))
    }
}
