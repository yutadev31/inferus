use crate::syntax::{SyntaxKind, SyntaxNode};

pub struct Heading {
    node: SyntaxNode,
}

impl Heading {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Heading {
            Some(Self { node })
        } else {
            None
        }
    }

    pub fn text(&self) -> String {
        self.node
            .children_with_tokens()
            .filter_map(|e| e.into_token())
            .map(|t| t.text().to_string())
            .collect()
    }
}
