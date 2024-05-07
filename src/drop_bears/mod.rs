use ascii_tree::Tree;

pub mod path;

/// A node in the drop bears tree.
#[derive(Debug)]
pub struct Node {
    num_a: u8,
    num_b: u8,
    child_left: Option<Box<Node>>,
    child_right: Option<Box<Node>>,
}
impl Node {
    // Convert a tree into a list of all the numbers in it.
    pub fn into_list(self) -> Vec<u16> {
        let mut vec = vec![format!("{}{}", self.num_a, self.num_b).parse().unwrap()];
        if let Some(child_left) = self.child_left {
            vec.append(&mut child_left.into_list())
        }
        if let Some(child_right) = self.child_right {
            vec.append(&mut child_right.into_list())
        }
        vec
    }

    // Get the display id of the node (e.g. a: 15, b: 8 -> 158)
    pub fn id(&self) -> String {
        format!("{}{}", self.num_a, self.num_b)
    }

    // Convert the tree to ascii-tree
    // This is just to print the whole tree so it looks nice.
    pub fn display(&self) -> ascii_tree::Tree {
        if self.child_left.is_none() && self.child_right.is_none() {
            Tree::Leaf(vec![format!("{}{}", self.num_a, self.num_b)])
        } else {
            Tree::Node(
                format!("{}{}", self.num_a, self.num_b),
                vec![&self.child_left, &self.child_right]
                    .into_iter()
                    .filter_map(|child| child.as_ref().map(|child| child.display()))
                    .collect(),
            )
        }
    }

    // Createa a new node.
    pub fn new(num_a: u8, num_b: u8) -> Node {
        // Generate the children of this node.
        let (child_left, child_right) = Node::generate_children(num_a, num_b);
        Node {
            num_a,
            num_b,
            child_left,
            child_right,
        }
    }

    // Generate all the children of a number.
    fn generate_children(num_a: u8, num_b: u8) -> (Option<Box<Node>>, Option<Box<Node>>) {
        // If num_a is less than 3, no more children can be generated.
        let child_left = if (num_a - 1) > num_b {
            Some(Box::new(Node::new(num_a - 1, num_b)))
        } else {
            None
        };
        let child_right = if num_b > 1 {
            Some(Box::new(Node::new(num_a - 1, num_b - 1)))
        } else {
            None
        };
        (child_left, child_right)
    }
}
