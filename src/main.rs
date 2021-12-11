// https://petermalmgren.com/three-rust-parsers/
// https://mail.google.com/mail/u/0/#inbox/KtbxLthqCqfmTnHQpDJCWDbpmMKpWVvgsq?projector=1
// 92
// https://app.diagrams.net/#G14rzgULr5arR4jENATQecRsNx08YcXdhu

use anyhow::{Context, Result};
use computorv1::*;

fn main() -> Result<()> {
    let mut parser = Parser::new();
    let tree = parser
        .parse("3 - (4 - 5) = 0")
        .context("Unable to parse")?;

    println!("{:#?}", tree);

    Ok(())
}
