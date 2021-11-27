use pulldown_cmark::Event;

// placeholder for now
pub fn highlight_rust<'a>(
    input: impl IntoIterator<Item = Event<'a>>,
) -> impl IntoIterator<Item = Event<'a>> {
    input
}