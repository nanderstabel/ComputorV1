use dot;
use std::borrow::Cow;
use std::io::Write;

struct Edges(Vec<(usize, usize)>);

pub fn render_to<W: Write>(output: &mut W, edges: Vec<(usize, usize)>) {
    let edges = Edges(edges);
    dot::render(&edges, output).unwrap()
}

impl<'a> dot::Labeller<'a, usize, (usize, usize)> for Edges {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("example1").unwrap()
    }

    fn node_id(&'a self, n: &usize) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", *n)).unwrap()
    }
}

impl<'a> dot::GraphWalk<'a, usize, (usize, usize)> for Edges {
    fn nodes(&self) -> dot::Nodes<'a, usize> {
        // (assumes that |N| \approxeq |E|)
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len());
        for &(s, t) in v {
            nodes.push(s);
            nodes.push(t);
        }
        nodes.sort();
        nodes.dedup();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, (usize, usize)> {
        let &Edges(ref edges) = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &(usize, usize)) -> usize {
        e.0
    }

    fn target(&self, e: &(usize, usize)) -> usize {
        e.1
    }
}
