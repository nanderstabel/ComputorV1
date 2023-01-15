use crate::node::{Branch, Edges};
use dot;
use std::fs::File;

pub fn render_graph(tree: &Branch) {
    let edges = Edges(
        tree.borrow()
            .clone()
            .into_iter()
            .flat_map(|n| n.borrow().edges())
            .collect(),
    );

    let mut output = File::create("example1.dot").unwrap();

    dot::render(&edges, &mut output).unwrap()
}
