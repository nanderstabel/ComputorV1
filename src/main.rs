// https://petermalmgren.com/three-rust-parsers/
// https://mail.google.com/mail/u/0/#inbox/KtbxLthqCqfmTnHQpDJCWDbpmMKpWVvgsq?projector=1
// 92
// https://app.diagrams.net/#G14rzgULr5arR4jENATQecRsNx08YcXdhu

#[macro_use]
mod node;
mod parser;
mod tokenizer;
mod visualizer;

use anyhow::{Context, Result};
use node::{Branch, Edge, Edges};
use parser::Parser;
use std::fs::File;
use visualizer::render_to;

fn main() -> Result<()> {
    let parser = Parser::new();
    // let tree = parser.parse("3 + (4 * 5) = 0").context("Unable to parse")?.unwrap();
    let tree: Branch = parser
        // .parse("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0")
        .parse("8 * X^0 * 2 * X^1 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 - 42 = 3 * X^0")
        .context("Unable to parse")?
        .unwrap();

    // println!("{:#?}", tree);

    let test = tree.borrow().clone().into_iter();

    let edges = Edges(test.flat_map(|n| n.borrow().edges()).collect::<Vec<Edge>>());

    let mut f = File::create("example1.dot").unwrap();
    render_to(&mut f, edges);

    Ok(())
}
