use std::iter::Peekable;
// use std::fmt;
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
	token: Option<&'a Token>,
	children: Vec<Node<'a>>
}

// impl<'a> fmt::Display for Node<'a> {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		match self.token {
// 			Some(token) => write!(f, "Token: {:?}\n", token),
// 			None => write!(f, "Token: None\n")
// 		};
// 		for child in &self.children {
// 			write!(f, "children:\n\t{}", child);
// 		}
// 		write!(f, "")
// 	}
// }

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

	fn parse_expression<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		// let a = self.parse_expression(tokens);
		// loop {
		// 	let token = tokens.next();
		// 	match token {
		// 		Some(Token::Operator(_)) => {
		// 			let b = self.parse_expression(tokens);
		// 			let a = Some(Node {
		// 				token: token,
		// 				children: vec![a.unwrap(), b.unwrap()]
		// 			});
		// 		},
		// 		_ => return a
		// 	}
		// }

		let lhs = self.parse_term(tokens);
		let token = tokens.next();
		match token {
			Some(Token::Operator('+')) | Some(Token::Operator('-')) => {
				let rhs = self.parse_expression(tokens);
				Some(Node {
					token: token,
					children: vec![lhs.unwrap(), rhs.unwrap()]
				})
			}
			_ => lhs
		}
	}

	fn parse_term<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		let lhs = self.parse_factor(tokens);
		let token = tokens.next();
		match token {
			Some(Token::Operator('*')) | Some(Token::Operator('/')) => {
				let rhs = self.parse_term(tokens);
				Some(Node {
					token: token,
					children: vec![lhs.unwrap(), rhs.unwrap()]
				})
			}
			_ => lhs
		}
	}

	fn parse_factor<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		let token = tokens.next();
		match token {
			Some(Token::Number(_)) => Some(Node {token: token, children: vec![]}),
			Some(Token::Identifier(_)) => Some(Node {token: token, children: vec![]}),
			_ => None
		}
	}

	pub fn parse(&mut self) {
		let tokens = self.tokenize();
		println!("{:?}", tokens);
		let tree = self.parse_expression(&mut tokens.iter().peekable());
		
		println!("{:#?}", tree);
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
