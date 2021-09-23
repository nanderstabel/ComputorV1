use std::iter::Peekable;
use Token::*;

#[derive(Debug, Clone)]
pub enum Token {
    Operator(char),
    Number(f64),
	Identifier(String),
    Paren(char)
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
	token: &'a Token,
	children: Vec<Node<'a>>
}

impl Node<'_> {
	fn reduce(&mut self) {
		match self.token {
			&Operator('=') => {

			},
			_ => ()
			
		}
	}
}

#[derive(Debug, Default)]
pub struct Computor {
    buf: String,
}

impl<'a> Computor {
    pub fn ingest(&mut self, buf: &str) {
        self.buf = buf.to_string();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut lexer = self.buf.chars().peekable();
		let mut tokens: Vec<Token> = vec![];
        while let Some(c) = lexer.next() {
            match c {
                '+' | '-' | '*' | '/' | '^' | '=' => tokens.push(Operator(c)),
				'a' ..= 'z' | 'A' ..= 'Z' => tokens.push(Identifier(get_identifier(&mut lexer, c))),
                '0' ..= '9' => tokens.push(Number(get_number(&mut lexer, c))),
                '(' | ')' => tokens.push(Paren(c)),
				c if c.is_whitespace() => {},
                _ => panic!("Unexpected char: {}", c)
            }
        }
		tokens
    }

	fn equation<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		let lhs = self.expression(tokens);
		match tokens.peek() {
			Some(Operator('=')) => {
				let token = tokens.next();
				let rhs = self.equation(tokens);
				Some(Node {token: token.unwrap(), children: vec![lhs.unwrap(), rhs.unwrap()]})
			},
			_ => lhs
		}
	}

	fn expression<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		let mut token = self.term(tokens);
		loop {
			match tokens.peek() {
				Some(Operator('+')) | Some(Operator('-')) => {
					let parent = tokens.next();
					let rhs = self.term(tokens);
					token = Some(Node {token: parent.unwrap(), children: vec![token.unwrap(), rhs.unwrap()]});
				},
				_ => break
			}
		};
		token
	}

	fn term<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		let mut token = self.factor(tokens);
		loop {
			match tokens.peek() {
				Some(Operator('*')) | Some(Operator('/')) => {
					let parent = tokens.next();
					let rhs = self.factor(tokens);
					token = Some(Node {token: parent.unwrap(), children: vec![token.unwrap(), rhs.unwrap()]});
				},
				_ => break
			}
		};
		token
	}

	fn factor<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		let lhs = self.primary(tokens);
		match tokens.peek() {
			Some(Operator('^')) => {
				let parent = tokens.next();
				let rhs = self.factor(tokens);
				Some(Node {token: parent.unwrap(), children: vec![lhs.unwrap(), rhs.unwrap()]})
			},
			_ => lhs
		}
	}

	fn primary<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		let token = tokens.next();
		match token {
			Some(Number(_)) => Some(Node {token: token.unwrap(), children: vec![]}),
			Some(Identifier(_)) => Some(Node {token: token.unwrap(), children: vec![]}),
			_ => None
		}
	}

	pub fn parse(&mut self) {
		let tokens = self.tokenize();
		println!("{:?}", tokens);
		let mut tree = self.equation(&mut tokens.iter().peekable()).unwrap();
		
		println!("{:#?}", tree);

		tree.reduce();
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
