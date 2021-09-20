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

#[derive(Debug)]
pub struct Node<'a> {
	token: Option<&'a Token>,
	children: Vec<Node<'a>>
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

	fn parse_recursive<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		if let Some(token) = tokens.next() {
			let mut new_node = Node {
				token: Some(token),
				children: vec![]
			};
			match token {
				Token::Operator(op) => println!("{:?}, {}", new_node, op),
				Token::Number(num) => println!("{:?}, {}", new_node, num),
				Token::Identifier(var) => println!("{:?}, {}", new_node, var),
				_ => ()
			}
			if let Some(child) = self.parse_recursive(tokens) {
				new_node.children.push(child);
			}
			return Some(new_node);
		}
		None
	}

	pub fn parse(&mut self) {
		let tokens = self.tokenize();
		let mut tree = Node {
			token: None,
			children: vec![]
		};
		if let Some(child) = self.parse_recursive(&mut tokens.iter().peekable()) {
			tree.children.push(child);
		}
		println!("{:#?}", tree);
	}

    // pub fn print(&mut self) {
    //     // println!("{}", self.buf);
    //     println!("{:?}", self.tokens);
    // }
}

// impl fmt::Display for Computor {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		write!(f, )
// 	}
// }

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
