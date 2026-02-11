use rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens
    Hash,
    Dash,
    Star,
    Backtick,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Text,
    Whitespace,
    NewLine,

    // Nodes
    Document,
    Heading,
    Paragraph,
    List,
    ListItem,
    CodeBlock,
    Error,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InferusLanguage {}

impl Language for InferusLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<InferusLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<InferusLanguage>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
