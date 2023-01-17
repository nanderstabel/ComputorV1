use dot::{LabelText, Style};

use crate::tokenizer::Token;
use crate::types::Type;
use std::borrow::Cow;
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::fmt::{Debug};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use derive_more::Display;

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

#[derive(Clone)]
pub struct Edge((usize, NodeObject), (usize, NodeObject));
pub struct Edges(pub Vec<Edge>);

impl<'a> dot::Labeller<'a, (usize, NodeObject), Edge> for Edges {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("example1").unwrap()
    }

    fn node_id(&'a self, n: &(usize, NodeObject)) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.0)).unwrap()
    }

    fn node_label(&'a self, n: &(usize, NodeObject)) -> LabelText<'a> {
        LabelText::label(format!("{}", n.1))
    }

    fn node_color(&'a self, node: &(usize, NodeObject)) -> Option<LabelText<'a>> {
        Some(LabelText::html(match node.1.clone() {
            NodeObject::Operator(Token::Operator(operator)) => match operator {
                '+' | '-' => "#F1E2A7",
                '*' | '/' | '%' => "#E9D172",
                '^' => "#E1C03D",
                _ => unimplemented!(),
            },
            NodeObject::Operator(Token::Number(_)) => "#00A0B0",
            NodeObject::Operand(operand) => operand.node_color(),
            _ => unreachable!(),
        }))
    }

    fn node_style(&'a self, _n: &(usize, NodeObject)) -> Style {
        Style::Filled
    }
}

impl<'a> dot::GraphWalk<'a, (usize, NodeObject), Edge> for Edges {
    fn nodes(&self) -> dot::Nodes<'a, (usize, NodeObject)> {
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

    fn source(&self, e: &Edge) -> (usize, NodeObject) {
        e.0.clone()
    }

    fn target(&self, e: &Edge) -> (usize, NodeObject) {
        e.1.clone()
    }
}

#[derive(Debug, Clone, Display)]
pub enum NodeObject {
    Operator(Token),
    Operand(Rc<dyn Type>)
}

impl Into<Token> for NodeObject {
    fn into(self) -> Token {
        match self {
            NodeObject::Operator(token) => token,
            NodeObject::Operand(_) => panic!()
        }
    }
}

impl From<&Token> for NodeObject {
    fn from(token: &Token) -> Self {
        Self::Operator(token.clone())
    }
}

pub fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

macro_rules! node {
    ($object:expr) => {
        node!($object, None, None)
    };
    ($object:expr, $left:expr) => {
        node!($object, $left, None)
    };
    ($object:expr, $left:expr, $right:expr) => {
        Some(Branch::new(Node::new($object, $left, $right)))
    };
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub object: NodeObject,
    pub left: Option<Branch>,
    pub right: Option<Branch>,
}

impl Node {
    pub fn new(object: NodeObject, left: Option<Branch>, right: Option<Branch>) -> Node {
        Node {
            id: get_id(),
            object,
            left,
            right,
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut ret = vec![];
        if let Some(left) = &self.left {
            ret.push(Edge(
                (self.id, self.object.clone()),
                (left.borrow().id, left.borrow().object.clone()),
            ));
            if let Some(right) = &self.right {
                ret.push(Edge(
                    (self.id, self.object.clone()),
                    (right.borrow().id, right.borrow().object.clone()),
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
