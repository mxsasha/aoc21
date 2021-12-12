use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read};
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum NodeKind {
    Start,
    End,
    Small,
    Big,
}

#[derive(Eq, Clone)]
struct Node {
    id: String,
    kind: NodeKind,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .finish()
    }
}

impl Node {
    fn new(id: &str) -> Self {
        let kind = match id {
            "start" => NodeKind::Start,
            "end" => NodeKind::End,
            _ => {
                if id.chars().all(|c| c.is_ascii_lowercase()) {
                    NodeKind::Small
                } else if id.chars().all(|c| c.is_ascii_uppercase()) {
                    NodeKind::Big
                } else {
                    panic!("Invalid Node ID")
                }
            }
        };
        Node {
            id: String::from(id),
            kind,
            neighbors: vec![],
        }
    }
    fn add_neighbor(&mut self, neighbor: &Rc<RefCell<Node>>) {
        self.neighbors.push(Rc::clone(neighbor));
    }
}

fn build_nodes(input: &str) -> Rc<RefCell<Node>> {
    let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    let tunnels: Vec<(&str, &str)> = input
        .trim()
        .lines()
        .map(|tunnel| {
            let parts: Vec<&str> = tunnel.split('-').collect();
            (parts[0], parts[1])
        })
        .collect();
    for (node_id1, node_id2) in tunnels {
        nodes = get_or_create_node(nodes, node_id1);
        nodes = get_or_create_node(nodes, node_id2);
        let node1 = nodes.get(node_id1).unwrap();
        let node2 = nodes.get(node_id2).unwrap();
        node1.borrow_mut().add_neighbor(node2);
        node2.borrow_mut().add_neighbor(node1);
    }
    nodes.remove("start").unwrap()
}

fn get_or_create_node(
    mut nodes: HashMap<String, Rc<RefCell<Node>>>,
    node_id: &str,
) -> HashMap<String, Rc<RefCell<Node>>> {
    if nodes.contains_key(node_id) {
        nodes
    } else {
        let new_node = Rc::new(RefCell::new(Node::new(node_id)));
        nodes.insert(String::from(node_id), new_node);
        nodes
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Path {
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl Path {
    fn valid_neighbors(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut small_visit_counts: HashMap<Node, usize> = HashMap::new();
        for node in self.nodes.iter().filter(|rc| rc.borrow().kind == NodeKind::Small).map(|rc| rc.borrow().deref().clone()) {
            *small_visit_counts.entry(node).or_default() += 1;
        }
        let small_visit_limit = small_visit_counts.iter().filter(|&(_k, v)| *v > 1).count() > 0;


        self.nodes
            .last()
            .unwrap()
            .borrow()
            .neighbors
            .iter()
            .filter(|rc| !self.nodes.contains(rc) || rc.borrow().kind == NodeKind::Big || (rc.borrow().kind == NodeKind::Small && !small_visit_limit)) // wrong
            .cloned()
            .collect()
    }
    fn is_ended(&self) -> bool {
        self.nodes.last().unwrap().borrow().kind == NodeKind::End
    }
}

fn calculate(input: &str) -> usize {
    let root_rc: Rc<RefCell<Node>> = build_nodes(input);
    // let start_node: &Node = &root_rc.borrow();

    let mut path_stack: Vec<Path> = vec![Path {
        nodes: vec![root_rc],
    }];
    let mut final_paths: Vec<Path> = vec![];
    while !path_stack.is_empty() {
        let path = path_stack.pop().unwrap();

        if path.is_ended() {
            final_paths.push(path.clone());
        }
        for neighbor in path.valid_neighbors() {
            let mut new_path = path.clone();
            new_path.nodes.push(neighbor);
            path_stack.push(new_path);
        }
    }
    println!("FINAL === {:?}", final_paths);

    final_paths.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let score = calculate(&input);
    println!("result: {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_calculate() {
        let count = calculate(
            "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
        );
        assert_eq!(count, 103);
    }
}
