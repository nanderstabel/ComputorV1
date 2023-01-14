use crate::tokenizer::Token;
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
    left: Option<Branch>,
    right: Option<Branch>,
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

    pub fn edges(&self) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        if let Some(left) = &self.left {
            ret.push((self.id, left.borrow().id));
            if let Some(right) = &self.right {
                ret.push((self.id, right.borrow().id));
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
