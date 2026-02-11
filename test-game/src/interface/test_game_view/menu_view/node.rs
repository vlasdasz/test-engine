#[derive(Default, Debug)]
pub struct Node {
    pub open:  bool,
    pub index: usize,
    pub depth: usize,
    pub value: String,
    pub leafs: Vec<Node>,
}

impl Node {
    pub fn empty(value: impl ToString) -> Self {
        Self {
            open:  false,
            index: 0,
            depth: 0,
            value: value.to_string(),
            leafs: vec![],
        }
    }

    pub fn new(value: impl ToString, leafs: Vec<Node>) -> Self {
        Self {
            open: false,
            index: 0,
            depth: 0,
            value: value.to_string(),
            leafs,
        }
    }

    pub fn open(mut self) -> Self {
        self.open = true;
        self.update_indices(0, 0);
        self
    }

    pub fn is_leaf(&self) -> bool {
        self.leafs.is_empty()
    }

    pub fn length(&self) -> usize {
        let mut length = 1;

        if self.open {
            for leaf in &self.leafs {
                length += leaf.length();
            }
        }

        length
    }

    pub fn update_indices(&mut self, index: usize, depth: usize) -> usize {
        self.index = index;
        self.depth = depth;
        let mut last_index = index + 1;

        if !self.open {
            return last_index;
        }

        for node in &mut self.leafs {
            last_index = node.update_indices(last_index, depth + 1);
        }

        last_index
    }

    pub fn val_at_index(&mut self, index: usize) -> &mut Self {
        self.find(index)
            .unwrap_or_else(|| panic!("Failed to find node at index {index}"))
    }

    fn find(&mut self, index: usize) -> Option<&mut Self> {
        if self.index == index {
            return Some(self);
        }

        if !self.open {
            return None;
        }

        for leaf in &mut self.leafs {
            if let Some(val) = leaf.find(index) {
                return Some(val);
            }
        }

        None
    }
}

impl Iterator for Node {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::interface::test_game_view::Node;

    #[test]
    fn test_node() {
        let mut root = Node::new(
            "Root",
            vec![
                Node::empty("A"),
                Node::new("Ooo", vec![Node::empty("a"), Node::empty("b")]),
                Node::empty("C"),
            ],
        );

        assert_eq!(root.length(), 1);

        root.open = true;

        assert_eq!(root.length(), 4);

        root.update_indices(0, 0);

        assert_eq!(root.val_at_index(3).value, "C");

        root.leafs[1].open = true;
        root.update_indices(0, 0);

        assert_eq!(root.val_at_index(3).value, "a");
        assert_eq!(root.val_at_index(5).value, "C");

        dbg!(&root);
    }
}
