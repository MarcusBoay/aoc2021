use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

mod utils;

struct Node {
    val: String,
    is_small: bool,
    visited: bool,
    pub neighbours: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val: val.clone(),
            is_small: val.chars().collect::<Vec<char>>()[0].is_lowercase(),
            visited: false,
            neighbours: vec![],
        }))
    }
}

struct Program {
    unique_paths: i32,
    nodes: HashMap<String, Rc<RefCell<Node>>>,
}

impl Program {
    pub fn new() -> Self {
        let edges = utils::fast_get_file_data_as_vec()
            .iter()
            .map(|n| n.split('-').map(String::from).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();

        // Get unique nodes.
        let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
        let mut edge_list: HashMap<String, HashSet<String>> = HashMap::new();
        for edge in &edges {
            let a = &edge[0];
            let b = &edge[1];
            if !nodes.contains_key(a) {
                nodes.insert(a.clone(), Node::new(a.clone()));
                edge_list.insert(a.clone(), HashSet::new());
            }
            if !nodes.contains_key(b) {
                nodes.insert(b.clone(), Node::new(b.clone()));
                edge_list.insert(b.clone(), HashSet::new());
            }
        }

        // Add edges.
        for edge in &edges {
            let a = &edge[0];
            let b = &edge[1];
            if !edge_list.get(a).unwrap().contains(b) {
                edge_list.get_mut(a).unwrap().insert(b.clone());
                let mut a_node = nodes.get(a).unwrap().borrow_mut();
                a_node.neighbours.push(Rc::clone(nodes.get(b).unwrap()));
            }
            if !edge_list.get(b).unwrap().contains(a) {
                edge_list.get_mut(b).unwrap().insert(a.clone());
                let mut b_node = nodes.get(b).unwrap().borrow_mut();
                b_node.neighbours.push(Rc::clone(nodes.get(a).unwrap()));
            }
        }

        Self {
            unique_paths: 0,
            nodes,
        }
    }

    pub fn find_all_paths(&mut self) {
        let mut start = Rc::clone(self.nodes.get("start").unwrap());
        for neighbour in &start.borrow().neighbours {
            if !neighbour.borrow().is_small || !neighbour.borrow().visited {
                self.find_paths(&mut Rc::clone(neighbour));
                // self.find_paths(&mut Rc::clone(neighbour));
            }
        }
        println!("Unique paths: {}", self.unique_paths);
    }

    fn find_paths(&mut self, node: &mut Rc<RefCell<Node>>) {
        println!("{:#?}", node.borrow().val);
        let n = node.borrow();
        if n.val.as_str() == "start" {
            return;
        }
        if n.val.as_str() == "end" {
            self.unique_paths += 1;
            return;
        }

        for neighbour in &n.neighbours {
            if !neighbour.borrow().is_small || !neighbour.borrow().visited {
                println!("nei: {}", neighbour.borrow().val);
                neighbour.borrow_mut().visited = true;
                self.find_paths(&mut Rc::clone(neighbour));
                neighbour.borrow_mut().visited = false;
            }
        }
    }
}

fn main() {
    let mut program = Program::new();
    program.find_all_paths();
}
