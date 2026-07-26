use crate::token::Token;

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    pub fn moi(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn peek2(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            return None;
        }

        let t = &self.tokens[self.index];
        self.index += 1;
        Some(t)
    }

    pub fn eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn vi_tri(&self) -> usize {
        self.index
    }
}
