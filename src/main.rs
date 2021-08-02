











use computorv1::Computor;
use std::env;



fn main () {
    let mut computor = Computor::default();


    for arg in env::args().skip(1) {
        computor.ingest(arg);
        computor.tokenize();
		computor.parse();
        computor.print();
    }

}
