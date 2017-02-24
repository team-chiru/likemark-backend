use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct TreeId {
    id: String
}

static TREE_ID_STEP: usize = 2;

impl TreeId {
    pub fn new(s: String) -> Self {
        TreeId {
            id: s
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn key(&self, level: usize) -> Option<String> {
        let sub_index = level*TREE_ID_STEP;
        if sub_index == self.len() {
            Some(self.id())
        } else if sub_index < self.len() {
            Some(String::from(&self.id[..sub_index]))
        } else {
            None
        }
    }

    pub fn level(&self) -> usize {
        self.id.len() / TREE_ID_STEP
    }

    pub fn len(&self) -> usize {
        self.id.len()
    }
}

impl Ord for TreeId {
    fn cmp(&self, other: &TreeId) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for TreeId {
    fn partial_cmp(&self, other: &TreeId) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Eq for TreeId {}

impl PartialEq for TreeId {
    fn eq(&self, other: &TreeId) -> bool {
        self.id == other.id
    }
}

pub fn level(s: &String) -> usize {
    s.len() / TREE_ID_STEP
}
