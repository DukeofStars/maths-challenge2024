use super::Node;

// A direction that can be taken
// This is specifically when travelling DOWN the tree.
pub enum Direction {
    Left,
    Right,
}
// A list of directions, travelling DOWN the tre.
pub struct Path {
    directions: Vec<Direction>,
}
impl Path {
    // Convert string of L and R to Path.
    pub fn from_string(string: &str) -> Option<Path> {
        let mut directions = Vec::new();

        for c in string.chars() {
            match c {
                'L' => directions.push(Direction::Left),
                'R' => directions.push(Direction::Right),
                _ => return None,
            }
        }

        Some(Path { directions })
    }
}

// Follow a path down the tree, return the node at the end.
pub fn follow_path<'a>(node: &'a Node, path: &Path) -> Option<&'a Node> {
    let mut cur_node = node;
    for direction in &path.directions {
        match direction {
            Direction::Left => {
                if let Some(left) = &cur_node.child_left {
                    cur_node = &left
                } else {
                    return None;
                }
            }
            Direction::Right => {
                if let Some(right) = &cur_node.child_right {
                    cur_node = &right;
                } else {
                    return None;
                }
            }
        }
    }
    Some(cur_node)
}
