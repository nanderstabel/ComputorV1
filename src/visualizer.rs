use dot;
use std::io::Write;

use crate::node::Edges;

pub fn render_to<W: Write>(output: &mut W, edges: Edges) {
    dot::render(&edges, output).unwrap()
}
