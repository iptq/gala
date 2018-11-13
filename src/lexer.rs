use std::str::CharIndices;

use failure::Error;

#[derive(Clone)]
pub enum Tok {
    KwExtern,
    SymColon,
}

pub type Loc = usize;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'i> {
    chars: CharIndices<'i>,
    v: Vec<(Loc, Tok, Loc)>,
    i: usize,
}

impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Self {
        let mut lexer = Lexer {
            chars: input.char_indices(),
            v: Vec::new(),
            i: 0,
        };
        lexer.process();
        lexer
    }
    fn process(&mut self) {
        // must preprocess ahead of time because indents push two tokens at once
    }
}

impl<'i> Iterator for Lexer<'i> {
    type Item = Spanned<Tok, Loc, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.v.get(self.i).map(|x| Ok(x.clone()))
    }
}
