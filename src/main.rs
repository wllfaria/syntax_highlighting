mod colors;
mod text_object;
mod viewport;

use std::collections::HashMap;

fn main() {
    let content = include_str!("main.rs");
    let mut viewport = viewport::Viewport::new();
    let text_object = text_object::TextObject::new(content);

    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_rust::language();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(content, None).unwrap();
    let query = tree_sitter::Query::new(&language, tree_sitter_rust::HIGHLIGHTS_QUERY).unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let root_node = tree.root_node();
    let matches = cursor.matches(&query, root_node, content.as_bytes());

    let mut capture_map = HashMap::new();
    for m in matches {
        for capture in m.captures {
            let node = capture.node;
            let start = node.start_position();
            let end = node.end_position();
            let name = query.capture_names()[capture.index as usize];
            let line_list = capture_map.entry(start.row).or_insert(vec![]);
            line_list.push((start.column..end.column, name))
        }
    }

    let colors = colors::make_colors();

    let style_extractor = |col: usize, row: usize| {
        let Some(name) = capture_map
            .get(&row)
            .and_then(|entry| entry.iter().find(|(range, _)| range.contains(&col)))
            .map(|(_, name)| name)
        else {
            return crossterm::style::Color::Reset;
        };

        colors
            .get(name)
            .cloned()
            .unwrap_or(crossterm::style::Color::Reset)
    };

    viewport.fill(text_object.get_within(0..viewport.size.1), style_extractor);
    viewport.render();
}
