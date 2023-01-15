// https://petermalmgren.com/three-rust-parsers/
// https://mail.google.com/mail/u/0/#inbox/KtbxLthqCqfmTnHQpDJCWDbpmMKpWVvgsq?projector=1
// 92
// https://app.diagrams.net/#G14rzgULr5arR4jENATQecRsNx08YcXdhu

#[macro_use]
mod node;
mod parser;
mod polynomial;
mod tokenizer;
mod visualizer;

use crate::polynomial::Polynomial;
use anyhow::{Context, Result};
use indoc::indoc;
use node::Branch;
use parser::Parser;
use visualizer::render_graph;

fn main() -> Result<()> {
    let parser = Parser::new();
    // let tree = parser.parse("3 + (4 * 5) = 0").context("Unable to parse")?.unwrap();
    let tree: Branch = parser
        // .parse("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0")
        .parse("5 * X^0 + 4 * X^1 = 4 * X^0")
        // .parse("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0")
        // .parse("(X+3) * (X + 1) = 0")
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
