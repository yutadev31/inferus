use crate::syntax::SyntaxKind;

#[derive(Debug)]
pub struct Token {
    pub kind: SyntaxKind,
    pub range: std::ops::Range<usize>,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let c = self.peek_char()?;

        match c {
            '#' => self.single(SyntaxKind::Hash),
            '-' => self.single(SyntaxKind::Dash),
            '*' => self.single(SyntaxKind::Star),
            '\n' => self.single(SyntaxKind::NewLine),
            ' ' | '\t' => self.consume_while_whitespace(),
            _ => self.consume_text(),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, ch) = iter.next()?;
        let next_pos = match iter.next() {
            Some((idx, _)) => self.pos + idx,
            None => self.input.len(),
        };
        self.pos = next_pos;
        Some(ch)
    }

    fn single(&mut self, kind: SyntaxKind) -> Option<Token> {
        self.bump()?;
        Some(Token {
            kind,
            range: self.pos - 1..self.pos,
        })
    }

    fn consume_while_whitespace(&mut self) -> Option<Token> {
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c == ' ' || c == '\t' {
                self.bump();
            } else {
                break;
            }
        }
        Some(Token {
            kind: SyntaxKind::Whitespace,
            range: start..self.pos,
        })
    }

    fn consume_text(&mut self) -> Option<Token> {
        let start = self.pos;

        while let Some(c) = self.peek_char() {
            if is_special(c) {
                break;
            }

            self.bump();
        }

        Some(Token {
            kind: SyntaxKind::Text,
            range: start..self.pos,
        })
    }
}

fn is_special(c: char) -> bool {
    matches!(
        c,
        '#' | '-' | '*' | '>' | '`' | '[' | ']' | '(' | ')' | '\n' | ' ' | '\t'
    )
}
