







// https://petermalmgren.com/three-rust-parsers/
// https://mail.google.com/mail/u/0/#inbox/KtbxLthqCqfmTnHQpDJCWDbpmMKpWVvgsq?projector=1
// 92


use computorv1::Computor;
use std::env;

fn main () {
    let mut computor = Computor::new(
        lexer: Box::new()
    );


    for arg in &env::args().skip(1) {
        computor.ingest(&arg);
        // computor.tokenize();
		// computor.parse();
        // computor.print();
    }

}
