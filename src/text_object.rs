use std::ops::Range;

#[derive(Debug)]
pub struct TextObject {
    content: ropey::Rope,
}

impl TextObject {
    pub fn new(source_code: &'static str) -> TextObject {
        TextObject {
            content: ropey::Rope::from_str(source_code),
        }
    }

    pub fn get_within(&self, range: Range<usize>) -> impl Iterator<Item = &str> {
        self.content
            .lines_at(range.start)
            .take(range.end - range.start)
            .map(|s| s.as_str().unwrap_or(""))
    }
}
