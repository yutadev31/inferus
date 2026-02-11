use inferus_parser::{ast::heading::Heading, parse};

pub fn format_markdown(input: &str) -> String {
    let tree = parse(input);

    for node in tree.descendants() {
        if let Some(heading) = Heading::cast(node) {
            println!("Heading: {}", heading.text())
        }
    }

    todo!()
}
