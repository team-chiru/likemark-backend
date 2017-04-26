use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct TreePath {
    id: String
}

static TREE_ID_STEP: usize = 2;

impl TreePath {
    pub fn new(s: String) -> Self {
        TreePath {
            id: s
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn len(&self) -> usize {
        self.id.len()
    }
}

impl Ord for TreePath {
    fn cmp(&self, other: &TreePath) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for TreePath {
    fn partial_cmp(&self, other: &TreePath) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Eq for TreePath {}

impl PartialEq for TreePath {
    fn eq(&self, other: &TreePath) -> bool {
        self.id == other.id
    }
}

pub fn level(ti: &TreePath) -> usize {
    ti.id.len() / TREE_ID_STEP
}

pub fn key(ti: &TreePath, level: usize) -> Option<String> {
    let sub_index = level*TREE_ID_STEP;
    let ref s = ti.id;

    if sub_index == s.len() {
        Some(s.clone())
    } else if sub_index < s.len() {
        Some(String::from(&s[..sub_index]))
    } else {
        None
    }
}
