use std::iter::Peekable;
use Token::*;

#[derive(Debug, Clone)]
pub enum Token {
    Operator(char),
    Number(f64),
	Identifier(String),
    Paren(char)
}

#[derive(Debug, Default)]
pub struct Term {
	sign: i8,
	coefficient: Option<f64>,
	constant: Option<f64>,
    identifier: Option<String>,
    exponent: Option<f64>,
}

#[derive(Debug, Default)]
pub struct Node {
	token: Token,
	children: Vec<Node>
}

#[derive(Debug, Default)]
pub struct Computor {
    buf: String,
    tokens: Vec<Token>,
	terms: Vec<Term>
}

impl Computor {
    pub fn ingest(&mut self, buf: String) {
        self.buf = buf;
    }

    pub fn tokenize(&mut self) {
        let mut lexer = self.buf.chars().peekable();
        while let Some(c) = lexer.next() {
            match c {
                '+' | '-' | '*' | '/' | '^' | '=' => self.tokens.push(Operator(c)),
				'a' ..= 'z' | 'A' ..= 'Z' => self.tokens.push(Identifier(get_identifier(&mut lexer, c))),
                '0' ..= '9' => self.tokens.push(Number(get_number(&mut lexer, c))),
                '(' | ')' => self.tokens.push(Paren(c)),
				c if c.is_whitespace() => {},
                _ => panic!("Unexpected char: {}", c)
            }
        }
    }

	pub fn parse(&mut self) {
		let mut tokens = self.tokens.iter().Peekable();
		let mut tree = Vec<Node>;
		while let Some(token) = tokens.next() {

			tree.push( Node {
				token: token,

			})
			// println!("{}, {}, {}", sign, coefficient, identifier);


			// self.terms.push( Term {
			// 	coefficient: 0.0,
			// 	identifier: String::from("temp"),
			// 	exponent: 0.0
			// });
		}
	}

    pub fn print(&mut self) {
        // println!("{}", self.buf);
        println!("{:?}", self.tokens);
        println!("{:#?}", self.terms);
    }
}

fn get_number<I>(lexer: &mut Peekable<I>, c: char) -> f64
where I: Iterator<Item = char>, {
	let mut number = c.to_string();
	let mut is_float = false;
	while let Some(c) = lexer.peek() {
		match c {
			'0' ..= '9' => number.push(*c),
			'.' if !is_float => {
				number.push(*c);
				is_float = true;
			}
			_ => break
		}
		lexer.next();
	}
	number.parse().unwrap()
}

fn get_identifier<I>(lexer: &mut Peekable<I>, c: char) -> String
where I: Iterator<Item = char>, {
	let mut identifier = c.to_string();
	while let Some(c) = lexer.peek() {
		match c {
			'0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' => identifier.push(*c),
			_ => break
		}
		lexer.next();
	}
	identifier
}
