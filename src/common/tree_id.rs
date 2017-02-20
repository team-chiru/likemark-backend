#[derive(Debug, Clone)]
pub struct TreeId {
    value: String
}

static TREE_ID_STEP: usize = 1;

impl TreeId {
    pub fn new(s: String) -> Self {
        TreeId {
            value: s
        }
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn key(&self, level: usize) -> String {
        let sub_index = level*TREE_ID_STEP;
        if sub_index == self.len() {
            self.value()
        } else if sub_index < self.len() {
            String::from(&self.value[..sub_index])
        } else {
            panic!("No matching key exists!")
        }
    }

    pub fn level(&self) -> usize {
        self.value.len() / TREE_ID_STEP
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}
