use std::iter::Peekable;
use Token::*;

#[derive(Debug, Clone)]
pub enum Token {
    Operator(char),
    Number(f64),
	Identifier(String),
    Paren(char)
}

#[derive(Debug)]
pub struct Node<'a> {
	token: &'a Token,
	children: Vec<Node<'a>>
}

#[derive(Debug, Default)]
pub struct Computor {
    buf: String,
    tokens: Vec<Token>,
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
		let mut tokens = self.tokens.iter().peekable();
		let mut tree : Vec<Node> = Vec::new();
		while let Some(token) = tokens.next() {
			match token {
				Number(num) => println!("number"),
				Operator('+') | Operator('-') => println!("operator"),
				_ => ()
			};

			tree.push( Node {
				token: token,
				children: Vec::new()
			});

		}
		println!("{:#?}", tree);
	}

    pub fn print(&mut self) {
        // println!("{}", self.buf);
        println!("{:?}", self.tokens);
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
