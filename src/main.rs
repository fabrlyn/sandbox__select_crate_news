use select::document::Document;
use select::predicate::Predicate;
use select::predicate::{Attr, Name};

fn main() {
    let webpage_content = include_str!("./index.html");
    let document = Document::from(webpage_content);
    let head_line = document
        .find(
            Name("div")
                .and(Attr("data-key", "main-story"))
                .descendant(Name("a").and(Attr("data-key", "card-headline"))),
        )
        .into_selection()
        .children()
        .first()
        .map(|n| n.text())
        .map(|x| {
            x.trim()
                .split("\n")
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();

    println!("{}", head_line);
}
