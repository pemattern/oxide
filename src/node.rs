use crate::router::Handler;

pub struct Node {
    pub children: Vec<Node>,
    pub key: String,
    pub handler: Option<Handler>,
}

impl Node {
    pub fn new(key: &str) -> Self {
        Node {
            children: Vec::new(),
            key: String::from(key),
            handler: None,
        }
    }

    pub fn insert(&mut self, path: &str, handler: Handler) {
        match path.split_once('/') {
            Some((root, "")) => {
                self.key = String::from(root);
                self.handler = Some(handler);
            },
            Some(("", path)) => self.insert(path, handler),
            Some((root, path)) => {
                let node = self.children.iter_mut().find(|m| root == &m.key);
                match node {
                    Some(n) => n.insert(path, handler),
                    None => {
                        let mut node = Node::new(root);
                        node.insert(path, handler);
                        self.children.push(node);
                    }
                }
            },

            None => {
                let mut node = Node::new(path);
                node.handler = Some(handler);
                self.children.push(node);
            },
        }
    }

    pub fn get(&self, path: &str) -> Option<Handler> {
        match path.split_once('/') {
            Some((root, "")) => {
                if root == &self.key {
                    self.handler
                } else {
                    None
                }
            }
            Some(("", path)) => self.get(path),
            Some((root, path)) => {
                let node = self.children.iter().find(|m| root == &m.key);
                if let Some(node) = node {
                    node.get(path)
                } else {
                    None
                }
            }
            None => {
                let node = self.children.iter().find(|m| path == &m.key);
                if let Some(node) = node {
                    node.handler
                } else {
                    None
                }
            }
        }
    }
}