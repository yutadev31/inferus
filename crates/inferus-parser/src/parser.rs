use rowan::GreenNodeBuilder;

use crate::{
    lexer::Token,
    syntax::{SyntaxKind, SyntaxNode},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    input: &'a str,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            input,
            pos: 0,
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(&mut self) {
        self.parse_document();
    }

    pub fn build(self) -> SyntaxNode {
        let green = self.builder.finish();
        SyntaxNode::new_root(green)
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) {
        let token = match self.current() {
            Some(t) => t,
            None => return,
        };

        let text = &self.input[token.range.clone()];
        self.builder.token(token.kind.into(), text);

        self.pos += 1;
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.current().map_or(false, |token| token.kind == kind)
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn at_line_start(&self) -> bool {
        self.pos == 0 || self.tokens[self.pos - 1].kind == SyntaxKind::NewLine
    }

    fn at_blank_line(&self) -> bool {
        self.pos >= self.tokens.len() || self.tokens[self.pos].kind == SyntaxKind::NewLine
    }

    fn next_is_whitespace(&mut self) -> bool {
        self.current()
            .map_or(false, |token| token.kind == SyntaxKind::Whitespace)
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn parse_document(&mut self) {
        self.start_node(SyntaxKind::Document);

        while !self.eof() {
            self.parse_block();
        }

        self.finish_node();
    }

    fn parse_block(&mut self) {
        if self.at_line_start() && self.at(SyntaxKind::Hash) && self.next_is_whitespace() {
            self.parse_heading();
        } else if self.at_line_start() && self.at(SyntaxKind::Dash) {
            self.parse_list();
        } else {
            self.parse_paragraph();
        }
    }

    fn parse_heading(&mut self) {
        self.start_node(SyntaxKind::Heading);

        self.consume_hashes();
        self.expect_whitespace();
        self.parse_inline_until_newline();

        self.finish_node();
    }

    fn parse_list(&mut self) {
        self.start_node(SyntaxKind::List);

        while self.at_line_start() && self.at(SyntaxKind::Dash) && self.next_is_whitespace() {
            self.parse_list_item();
        }

        self.finish_node();
    }
    fn parse_list_item(&mut self) {
        self.start_node(SyntaxKind::ListItem);
        self.bump(); // -
        self.expect_whitespace();

        self.parse_inline_until_newline();

        self.eat(SyntaxKind::NewLine);

        self.builder.finish_node();
    }

    fn consume_hashes(&mut self) {
        while self.eat(SyntaxKind::Hash) {}
    }

    fn expect_whitespace(&mut self) {
        if !self.next_is_whitespace() {
            self.start_node(SyntaxKind::Error);
            self.bump();
            self.finish_node();
        }
    }

    fn parse_paragraph(&mut self) {
        self.start_node(SyntaxKind::Paragraph);
        self.parse_inline_until_newline();
        self.finish_node();
    }

    fn parse_inline_until_newline(&mut self) {
        while !self.eof() && !self.at_blank_line() {
            self.parse_inline();
        }
    }

    fn parse_inline(&mut self) {
        self.bump();
    }
}
