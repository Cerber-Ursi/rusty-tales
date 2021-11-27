use crate::highlight::highlight;

use pulldown_cmark::CowStr;
use pulldown_cmark::Event;
use pulldown_cmark::Tag;

/// Copied from https://github.com/raphlinus/pulldown-cmark/issues/167, migrated from syntect to rustdoc-highlight
pub fn highlight_rust<'a>(
    input: impl IntoIterator<Item = Event<'a>>,
) -> impl IntoIterator<Item = Event<'a>> {
    // We'll build a new vector of events since we can only consume the parser once
    let mut new_p = Vec::new();
    // As we go along, we'll want to highlight code in bundles, not lines
    let mut to_highlight = String::new();
    // And track a little bit of state
    let mut in_code_block = false;

    for event in input {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                // In actual use you'd probably want to keep track of what language this code is
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                if in_code_block {
                    // Format the whole multi-line code block as HTML all at once
                    let html = highlight(&to_highlight);
                    // And put it into the vector
                    new_p.push(Event::Html(CowStr::Boxed(html.into_boxed_str())));
                    to_highlight = String::new();
                    in_code_block = false;
                }
            }
            Event::Text(t) => {
                if in_code_block {
                    // If we're in a code block, build up the string of text
                    to_highlight.push_str(&t);
                } else {
                    new_p.push(Event::Text(t))
                }
            }
            e => {
                new_p.push(e);
            }
        }
    }
    new_p
}
