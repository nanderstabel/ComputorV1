use dot;
use std::borrow::Cow;
use std::io::Write;

// struct Edges(Vec<(usize, usize)>);

use crate::node::{Edge, Edges};

pub fn render_to<W: Write>(output: &mut W, edges: Edges) {
    dot::render(&edges, output).unwrap()
}
