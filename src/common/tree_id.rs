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

pub fn level(ti: &TreeId) -> usize {
    ti.id.len() / TREE_ID_STEP
}

pub fn key(ti: &TreeId, level: usize) -> Option<String> {
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
