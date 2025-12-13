use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::atomic::AtomicU32,
};

static ID_COUNT: std::sync::atomic::AtomicU32 = AtomicU32::new(0);

type SharedNode = Rc<RefCell<TNode>>;

#[derive(Debug, Default, Clone)]
struct TNode {
    left: Option<SharedNode>,
    right: Option<SharedNode>,
    id: u32,
}

struct BinaryTree {
    root: SharedNode,
    map_position: HashMap<usize, SharedNode>,
    map_id: HashMap<u32, SharedNode>,
}

impl BinaryTree {
    pub fn new() -> Self {
        let node = Rc::new(RefCell::new(TNode::default()));
        {
            node.borrow_mut().id = ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        let mut map_id = HashMap::new();
        map_id.insert(node.borrow().id, node.clone());
        BinaryTree {
            root: node,
            map_position: HashMap::new(),
            map_id,
        }
    }

    pub fn add_root(&mut self, node_position: usize) {
        self.map_position.insert(node_position, self.root.clone());
    }

    pub fn add_to_left(&mut self, current_position: usize, node_position: usize) {
        let current = match self.map_position.get(&current_position) {
            Some(v) => v.clone(),
            None => panic!(
                "{}",
                format!("node {current_position} is not part of the tree")
            ),
        };

        if self.map_position.contains_key(&node_position) {
            let pos_node = self.map_position.get(&node_position).unwrap();
            current.borrow_mut().left = Some(pos_node.clone());
            return;
        }
        let node = Rc::new(RefCell::new(TNode {
            left: None,
            right: None,
            id: ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }));

        current.borrow_mut().left = Some(node.clone());
        self.map_position.insert(node_position, node.clone());
        self.map_id.insert(node.borrow().id, node.clone());
    }

    pub fn add_to_right(&mut self, current_position: usize, node_position: usize) {
        let current = match self.map_position.get(&current_position) {
            Some(v) => v.clone(),
            None => panic!(
                "{}",
                format!("node {current_position} is not part of the tree")
            ),
        };

        if self.map_position.contains_key(&node_position) {
            let pos_node = self.map_position.get(&node_position).unwrap();
            current.borrow_mut().right = Some(pos_node.clone());
            return;
        }
        let node = Rc::new(RefCell::new(TNode {
            left: None,
            right: None,
            id: ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }));

        current.borrow_mut().right = Some(node.clone());
        self.map_position.insert(node_position, node.clone());
        self.map_id.insert(node.borrow().id, node.clone());
    }

    pub fn remove_position(&mut self, node_position: usize) {
        self.map_position.remove(&node_position);
    }
}

fn traverse(node: &SharedNode, memo: &mut HashMap<u32, u64>) -> u64 {
    let node = node.borrow();
    let id = node.id;

    if let Some(&count) = memo.get(&id) {
        return count;
    }

    if node.left.is_none() && node.right.is_none() {
        memo.insert(id, 1);
        return 1;
    }

    let mut path_count = 0;

    if let Some(ref left_node) = node.left {
        path_count += traverse(left_node, memo);
    }

    if let Some(ref right_node) = node.right {
        path_count += traverse(right_node, memo);
    }

    memo.insert(id, path_count);

    path_count
}

fn main() {
    let input = shared::get_input(7);
    let mut beams: HashSet<usize> = HashSet::new();
    let mut tree = BinaryTree::new();
    for (n, v) in input.lines().next().unwrap().chars().enumerate() {
        if v == 'S' {
            beams.insert(n);
            tree.add_root(n);
            break;
        }
    }

    let l_size = input.lines().next().unwrap().len();
    let mut count = 0;

    for (ln, line) in input.lines().skip(2).enumerate() {
        if ln & 1 == 1 {
            continue;
        }
        for (n, v) in line.chars().enumerate() {
            if v == '^' && beams.contains(&n) {
                count += 1;
                beams.remove(&n);

                if n > 0 {
                    beams.insert(n - 1);
                    tree.add_to_left(n, n - 1);
                }
                if n + 1 < l_size {
                    beams.insert(n + 1);
                    tree.add_to_right(n, n + 1);
                }
                tree.remove_position(n);
            }
        }
    }

    let mut memo: HashMap<u32, u64> = HashMap::new();
    let count2 = traverse(&tree.root, &mut memo);

    println!("Part 1: {count}");
    println!("Part 2: {}", count2);
}
