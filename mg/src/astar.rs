use std::vec::Vec;

struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    position: (usize, usize),
    g: usize, h: usize, f: usize,
}

impl Node<'_> {
    pub fn new<'a>(
        parent: Option<&'a Node<'a>>,
        position: (usize, usize)
    ) -> Node<'a> {
        Node {
            parent: parent,
            position: position,
            g: 0, h: 0, f: 0,
        }
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Node<'_> {}

// returns a list of tuples as a path from the given start to
// the given end in the given maze
pub fn astar(
    map: &mut [[bool; 205]; 50],
    start: (usize, usize),
    end: (usize, usize)
) -> Option<Vec<(usize, usize)>> {
    let start_node = Node::new(None, start);
    let end_node   = Node::new(None, end);

    let mut open_list   = Vec::new();
    let mut closed_list = Vec::new();

    open_list.push(start_node);

    while open_list.len() > 0 {
        // get the current node
        let mut current_node = &open_list[0];
        let mut current_index = 0;

        for index in 0..open_list.len() {
            let item = &open_list[index];
            if item.f < current_node.f {
                current_node = item.clone();
                current_index = index;
            }
        }

        // pop current off open list, add to closed
        open_list.remove(current_index);
        closed_list.push(current_node);

        // have we found the goal?
        if *current_node == end_node {
            let mut path = Vec::new();
            let mut current = Some(&current_node);

            while current != None {
                path.push(current.unwrap().position);
                current = current.unwrap().parent;
            }

            path.reverse();
            return Some(path);
        }

        let adjacent_positions = &[(0, -1), (0, 1), (-1, 0),
            (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        let mut children = Vec::new();

        for new_position in adjacent_positions {
            let node_position = (current_node.position.0 as isize + new_position.0,
                current_node.position.1 as isize + new_position.1);

            if node_position.0 > (50 - 1) || node_position.0 < 0 ||
                node_position.1 > (205 - 1) || node_position.1 < 0 {
                continue; // out of range
            }

            if !map[node_position.0 as usize][node_position.1 as usize] {
                continue; // the square is a wall
            }

            let new_node = Node::new(Some(&current_node),
                (node_position.0 as usize, node_position.1 as usize));

            children.push(new_node);
        }

        for mut child in children {
            let mut on_closed_list = false;
            for closed_child in &closed_list {
                if child == **closed_child {
                    on_closed_list = true;
                    break;
                }
            }

            if on_closed_list {
                continue;
            }

            // calculate f, g, h values
            child.g = current_node.g.clone() + 1;
            child.h = ((child.position.0 - end_node.position.0).pow(2)) +
                ((child.position.1 - end_node.position.1).pow(2));
            child.f = child.g + child.h;

            let mut on_open_list = false;
            for open_child in &open_list {
                if child == *open_child {
                    on_open_list = true;
                    break;
                }
            }

            if !on_open_list {
                open_list.push(child);
            }
        }
    }

    None
}
