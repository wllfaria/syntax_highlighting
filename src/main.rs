mod text_object;
mod viewport;

fn main() {
    let content = include_str!("viewport.rs");
    let mut viewport = viewport::Viewport::new();
    let text_object = text_object::TextObject::new(content);
    viewport.fill(text_object.get_within(0..viewport.size.1));
    viewport.render();
}
