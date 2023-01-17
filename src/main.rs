// https://petermalmgren.com/three-rust-parsers/
// https://mail.google.com/mail/u/0/#inbox/KtbxLthqCqfmTnHQpDJCWDbpmMKpWVvgsq?projector=1
// 92
// https://app.diagrams.net/#G14rzgULr5arR4jENATQecRsNx08YcXdhu

#[macro_use]
mod node;
mod parser;
mod types;
mod tokenizer;
mod visualizer;

use std::env::args;

use crate::types::polynomial::Polynomial;
use anyhow::{Context, Result};
use indoc::indoc;
use node::Branch;
use parser::Parser;
use visualizer::render_graph;

fn main() -> Result<()> {
    let input: Vec<String> = args().collect();
    let parser = Parser::new();
    let tree: Branch = parser
        .parse(input[1].as_str())
        .context("Unable to parse")?
        .unwrap();

    render_graph(&tree);

    let mut polynomial = Polynomial::from(tree);
    polynomial.reduce();
    let degree = polynomial.degree();
    polynomial.solve();

    println!("Reduced form: {polynomial}= 0");
    println!("Polynomial degree: {degree}");

    let output = polynomial.solve();

    if output.len() == 2 {
        println!(
            indoc! {
                "Discriminant is strictly positive, the two solutions are:
                {:.6}
                {:.6}"
            },
            output[0], output[1]
        );
    } else {
        println!(
            indoc! {
                "The solution is:
                {:.6}"
            },
            output[0]
        );
    }
    Ok(())
}
