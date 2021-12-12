use std::collections::HashMap;
use std::io::{self, Read};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug)]
enum NodeKind {
    Start,
    End,
    Small,
    Big,
}

#[derive(Debug)]
struct Node {
    id: String,
    kind: NodeKind,
    neighbors: Vec<Rc<RefCell<Node>>>,
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
struct Path<'a> {
    nodes: Vec<&'a Node>,
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
        node1.borrow_mut().add_neighbor(&node2);
        node2.borrow_mut().add_neighbor(&node1);
    }
    nodes.remove("start").unwrap()
}

fn get_or_create_node<'a>(mut nodes: HashMap<String, Rc<RefCell<Node>>>, node_id: &str) -> HashMap<String, Rc<RefCell<Node>>> {
    if nodes.contains_key(node_id) {
        nodes
    } else {
        let new_node = Rc::new(RefCell::new(Node::new(node_id)));
        nodes.insert(String::from(node_id), new_node);
        nodes
    }
}

fn calculate(input: &str) -> usize {
    let root_rc: Rc<RefCell<Node>> = build_nodes(input);
    let start_node: &Node = &root_rc.borrow();
    println!("{:?}", root_rc);

    0
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
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        );
        assert_eq!(count, 10);
    }
}
