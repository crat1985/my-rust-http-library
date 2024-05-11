use crate::router::HandlerFn;

pub struct Node<S: Clone> {
    nodes: Vec<Node<S>>,
    key: String,
    pub(crate) handler: Option<HandlerFn<S>>,
}

impl<S: Clone> Node<S> {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            nodes: Vec::new(),
            handler: None,
        }
    }

    pub fn insert(&mut self, path: &str, handler: HandlerFn<S>) {
        if let Some((root, path)) = path.split_once('/') {
            if root.is_empty() {
                self.insert(path, handler);
                return;
            }

            let res = self.nodes.binary_search_by(|n| root.cmp(&n.key));
            match res {
                Ok(i) => {
                    let Some(node) = self.nodes.get_mut(i) else {
                        panic!("jsp");
                    };
                    node.insert(path, handler);
                }
                Err(n) => {
                    let mut node = Node::new(root);
                    node.insert(path, handler);
                    self.nodes.insert(n, node);
                }
            }
        } else {
            if path.is_empty() {
                self.handler = Some(handler);
            } else {
                let mut node = Node::new(path);
                node.handler = Some(handler);
                self.nodes.push(node);
            }
        }
    }

    pub fn get(&self, path: &str) -> Option<HandlerFn<S>> {
        if let Some((root, path)) = path.split_once('/') {
            if root.is_empty() {
                return self.get(path);
            }

            let node = self.nodes.binary_search_by(|n| root.cmp(&n.key));
            match node {
                Ok(i) => {
                    return self.nodes[i].get(path);
                }
                Err(_) => return None,
            }
        } else {
            if path.is_empty() || path == &self.key {
                return self.handler.clone();
            }

            let node = self.nodes.binary_search_by(|n| path.cmp(&n.key));
            match node {
                Ok(i) => return self.nodes[i].handler.clone(),
                Err(_) => return None,
            }
        }
    }
}
