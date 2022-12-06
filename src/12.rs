use std::{
    cell::UnsafeCell,
    collections::{HashMap, HashSet},
    process::exit,
    rc::Rc,
};

mod utils;

struct Node {
    val: String,
    is_small: bool,
    visited: bool,
    pub neighbours: Vec<Rc<UnsafeCell<Node>>>,
}

impl Node {
    pub fn new(val: String) -> Rc<UnsafeCell<Self>> {
        Rc::new(UnsafeCell::new(Self {
            val: val.clone(),
            is_small: val.chars().collect::<Vec<char>>()[0].is_lowercase(),
            visited: false,
            neighbours: vec![],
        }))
    }
}

struct Program {
    unique_paths: i32,
    nodes: HashMap<String, Rc<UnsafeCell<Node>>>,
    cur_path: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        let edges = utils::fast_get_file_data_as_vec()
            .iter()
            .map(|n| n.split('-').map(String::from).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();

        // Get unique nodes.
        let mut nodes: HashMap<String, Rc<UnsafeCell<Node>>> = HashMap::new();
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
                let a_node = nodes.get(a).unwrap().get();
                unsafe {
                    (*a_node).neighbours.push(Rc::clone(nodes.get(b).unwrap()));
                }
            }
            if !edge_list.get(b).unwrap().contains(a) {
                edge_list.get_mut(b).unwrap().insert(a.clone());
                let b_node = nodes.get(b).unwrap().get();
                unsafe {
                    (*b_node).neighbours.push(Rc::clone(nodes.get(a).unwrap()));
                }
            }
        }

        Self {
            unique_paths: 0,
            nodes,
            cur_path: vec![],
        }
    }

    pub unsafe fn find_all_paths(&mut self) {
        let start = Rc::clone(self.nodes.get("start").unwrap());
        self.cur_path.push(String::from("start"));
        for neighbour in &(*start.get()).neighbours {
            let nn = &mut *(neighbour.get());
            if !nn.is_small || !nn.visited {
                self.cur_path.push((*nn).val.clone());
                self.find_paths(&mut Rc::clone(neighbour));
                self.cur_path.pop();
            }
        }
        self.cur_path.pop();
        println!("Unique paths: {}", self.unique_paths);
    }

    /// Welcome to inferior C++...
    unsafe fn find_paths(&mut self, node: &mut Rc<UnsafeCell<Node>>) {
        let n = node.get();
        if (*n).val.as_str() == "start" || ((*n).visited && (*n).is_small) {
            return;
        }
        if (*n).val.as_str() == "end" {
            println!("path: {:?}", self.cur_path);
            self.unique_paths += 1;
            return;
        }

        (*n).visited = true;

        for neighbour in &mut (*n).neighbours {
            let nn = neighbour.get();
            if !(*nn).is_small || !(*nn).visited {
                self.cur_path.push((*nn).val.clone());
                self.find_paths(&mut Rc::clone(neighbour));
                self.cur_path.pop();
            }
        }

        (*n).visited = false;
    }
}

fn main() {
    let mut program = Program::new();
    unsafe {
        program.find_all_paths();
    }
}
