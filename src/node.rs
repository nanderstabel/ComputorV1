use dot::{LabelText, Style};

use crate::tokenizer::Token;
use std::borrow::Cow;
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

// pub type Branch = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
pub struct Branch(Rc<RefCell<Node>>);

impl Branch {
    pub fn new(node: Node) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    pub fn borrow(&self) -> Ref<'_, Node> {
        self.0.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<'_, Node> {
        self.0.borrow_mut()
    }
}

pub fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

macro_rules! node {
    ($token:expr) => {
        node!($token, None, None)
    };
    ($token:expr, $left:expr) => {
        node!($token, $left, None)
    };
    ($token:expr, $left:expr, $right:expr) => {
        Some(Branch::new(Node::new($token, $left, $right)))
    };
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub token: Token,
    pub left: Option<Branch>,
    pub right: Option<Branch>,
}

#[derive(Clone)]
pub struct Edge((usize, Token), (usize, Token));
pub struct Edges(pub Vec<Edge>);

impl<'a> dot::Labeller<'a, (usize, Token), Edge> for Edges {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("example1").unwrap()
    }

    fn node_id(&'a self, n: &(usize, Token)) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.0)).unwrap()
    }

    fn node_label(&'a self, n: &(usize, Token)) -> LabelText<'a> {
        LabelText::label(format!("{}", n.1))
    }

    fn node_color(&'a self, node: &(usize, Token)) -> Option<LabelText<'a>> {
        Some(LabelText::html(match node.1 {
            Token::Operator(operator) => match operator {
                '+' | '-' => "#F1E2A7",
                '*' | '/' | '%' => "#E9D172",
                '^' => "#E1C03D",
                _ => unimplemented!(),
            },
            Token::Parenthesis(_) => unreachable!(),
            Token::Number(_) => "#00A0B0",
            Token::Identifier(_) => "#D3643B",
        }))
    }

    fn node_style(&'a self, _n: &(usize, Token)) -> Style {
        Style::Filled
    }
}

impl<'a> dot::GraphWalk<'a, (usize, Token), Edge> for Edges {
    fn nodes(&self) -> dot::Nodes<'a, (usize, Token)> {
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len());
        for Edge((source_id, source_token), (target_id, target_token)) in v.into_iter() {
            nodes.push((*source_id, source_token.clone()));
            nodes.push((*target_id, target_token.clone()));
        }
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge> {
        let &Edges(ref edges) = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &Edge) -> (usize, Token) {
        e.0.clone()
    }

    fn target(&self, e: &Edge) -> (usize, Token) {
        e.1.clone()
    }
}

impl Node {
    pub fn new(token: Token, left: Option<Branch>, right: Option<Branch>) -> Node {
        Node {
            id: get_id(),
            token,
            left,
            right,
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut ret = vec![];
        if let Some(left) = &self.left {
            ret.push(Edge(
                (self.id, self.token.clone()),
                (left.borrow().id, left.borrow().token.clone()),
            ));
            if let Some(right) = &self.right {
                ret.push(Edge(
                    (self.id, self.token.clone()),
                    (right.borrow().id, right.borrow().token.clone()),
                ));
            }
        }
        ret
    }
}

impl IntoIterator for Node {
    type Item = Branch;
    type IntoIter = NodeIter;

    fn into_iter(self) -> Self::IntoIter {
        let left = self.left.clone();
        let right = self.right.clone();

        NodeIter {
            node: Some(Branch::new(self)),
            children: if let Some(mut left) = left {
                let iter = left.borrow_mut().clone().into_iter();
                if let Some(mut right) = right {
                    Some(Rc::new(RefCell::new(
                        iter.chain(right.borrow_mut().clone().into_iter()),
                    )))
                } else {
                    Some(Rc::new(RefCell::new(iter)))
                }
            } else {
                None
            },
        }
    }
}

pub struct NodeIter {
    node: Option<Branch>,
    children: Option<Rc<RefCell<dyn Iterator<Item = Branch>>>>,
}

impl Iterator for NodeIter {
    type Item = Branch;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().or_else(|| {
            self.children
                .as_ref()
                .and_then(|iter| iter.borrow_mut().next())
        })
    }
}
