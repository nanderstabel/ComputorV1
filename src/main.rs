







// https://petermalmgren.com/three-rust-parsers/



use computorv1::Computor;
use std::env;



fn main () {
    // let mut computor = Computor::default();


    // for arg in env::args().skip(1) {
    //     computor.ingest(arg);
    //     computor.tokenize();
	// 	computor.parse();
    //     computor.print();
    // }

    let hello: &str = "Hello, World!";
    let hello1 = hello;
    println!("1: {:p}, 2: {:p}", &hello, &hello1);

    let hello = String::from("Hello, World!");
    let hello1 = &hello;
    println!("1: {:p}, 2: {:p}", &hello, hello1);


}
