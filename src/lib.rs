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
	fn new<'a>(token: &'a Token, children: Vec<Node<'a>>) -> Node<'a> {
		Node { token, children }
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
				let token = tokens.next().unwrap();
				let rhs = self.equation(tokens);
				Some(Node::new(token, vec![lhs.unwrap(), rhs.unwrap()]))
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
					let parent = tokens.next().unwrap();
					let rhs = self.term(tokens);
					token = Some(Node::new(parent, vec![token.unwrap(), rhs.unwrap()]));
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
					let parent = tokens.next().unwrap();
					let rhs = self.factor(tokens);
					token = Some(Node::new(parent, vec![token.unwrap(), rhs.unwrap()]));
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
				let parent = tokens.next().unwrap();
				let rhs = self.factor(tokens);
				Some(Node::new(parent, vec![lhs.unwrap(), rhs.unwrap()]))
			},
			_ => lhs
		}
	}

	fn primary<I>(&mut self, tokens: &mut Peekable<I>) -> Option<Node<'a>>
	where I: Iterator<Item = &'a Token> {
		let token = tokens.next();
		match token {
			Some(Number(_)) => Some(Node::new(token.unwrap(), vec![])),
			Some(Identifier(_)) => Some(Node::new(token.unwrap(), vec![])),
			Some(Operator('-')) => {
				let child = self.factor(tokens);
				Some(Node::new(token.unwrap(), vec![child.unwrap()]))
			},
			_ => None
		}
	}

	pub fn parse(&mut self) {
		let tokens = self.tokenize();
		println!("{:?}\n\n", tokens);
		let mut tree = self.equation(&mut tokens.iter().peekable()).unwrap();
		
		tree = self.reduce(tree);
		println!("{:#?}", tree);

	}

	// fn traverse<'a>(&mut self, node: Node) {
	// 	match self.token {
	// 		&Operator('+') | &Operator('-') => {
	// 			println!("1: {:?}", self.token);
	// 			for child in &mut self.children {
	// 				child.traverse(node);
	// 			}
	// 		},
	// 		_ => println!("2: {:?}", self.token)
	// 	}
	// }

	fn test(&mut self, rhs: &'a mut Node) -> Option<Node<'a>> {
		loop {
			println!("{:?}", rhs);
			match rhs.token {
				&Operator('+') | &Operator('-') => {
					for child in &mut rhs.children {
						self.test(child);
					}
				},
				_ => {
					rhs.children.push(Node::new(&Number(42.0), vec![]));
				}
			}
		}
	}

	fn reduce(&mut self, mut tree: Node<'a>) -> Node<'a> {
		match tree.token {
			&Operator('=') => {
				let mut rhs = tree.children.pop().unwrap();
				let lhs = tree.children.pop().unwrap();
				// rhs.traverse(&lhs);
				while let Some(node) = self.test(&mut rhs) {
					println!("node: {:?}", node);
				}

				println!("\n\n\n\nrhs: {:#?}\n\n\n\n", rhs);
				tree.children.extend_from_slice(&[lhs, Node::new(&Number(0.0), vec![])]);
			},
			_ => ()
		}
		tree
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
