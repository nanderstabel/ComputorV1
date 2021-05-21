/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   lib.rs                                             :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/05/06 18:20:27 by nstabel       #+#    #+#                 */
/*   Updated: 2021/05/21 17:41:53 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

// #[derive(Debug, Clone)]
// pub enum Operator {
//     Addition,
//     Subtract,
//     Multiplication,
//     Division,
//     Exponent
    
// }

#[derive(Debug, Clone)]
pub enum Token {
    Operator(char),
    Number(u32),
    Paren(char)
}

// #[derive(Debug, Clone)]
// pub struct Node {
//     pub entry: Token,
//     pub children: Vec<Node>
// }

// impl Node {
//     pub fn new() -> Node {
//         Node {
//             entry: Token::Paren,
//             children: Vec::new()
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum LexItem {
//     Paren(char),
//     Op(char),
//     Num(u64)
// }

// fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
//     let mut number = c.to_string().parse::<u64>().expect("The caller should have passed a digit.");
//     while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
//         number = number * 10 + digit;
//         iter.next();
//     }
//     number
// }



#[derive(Debug, Default)]
pub struct Computor {
    buf: String,
    tokens: Vec<Token>
}

impl Computor {
    pub fn ingest(&mut self, buf: String) {
        self.buf = buf;
    }

    pub fn print(&mut self) {
        println!("{}", self.buf);
        println!("{:?}", self.tokens);
    }

    pub fn scanner(&mut self) {
        let mut scanner = self.buf.chars().peekable();

        while let Some(c) = scanner.next() {
            match c {
                '+' | '-' | '*' | '/' | '^' => self.tokens.push(Token::Operator(c)),
                '0' ..= '9' => self.tokens.push(Token::Number(c.to_digit(10).unwrap())),
                '(' | ')' => self.tokens.push(Token::Paren(c)),
                _ => {}
            }
            // println!("1: {}, 2: {:?}", c, scanner);
        }
    }
}



#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_still_works() {
        assert_eq!(2 + 1, 3);
    }
}
 