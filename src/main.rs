

// https://petermalmgren.com/three-rust-parsers/
// https://mail.google.com/mail/u/0/#inbox/KtbxLthqCqfmTnHQpDJCWDbpmMKpWVvgsq?projector=1
// 92
// https://app.diagrams.net/#G14rzgULr5arR4jENATQecRsNx08YcXdhu

use computorv1::Computor;
use std::env;

fn main () {
    let mut computor = Computor::default();


    for arg in env::args().skip(1) {
        computor.ingest(&arg);
		computor.parse();
        // computor.print();
    }

}
