use common::deep::tree_path::{ TreePath, TREE_ID_STEP, TREE_ID_BASE };
use std::marker;

pub trait Traversal {
    fn right(&self) -> Option<Self> where Self: marker::Sized;
    fn up(&self) -> Option<Self> where Self: marker::Sized;
    fn left(&self) -> Option<Self> where Self: marker::Sized;
    fn down(&self) -> Option<Self> where Self: marker::Sized;
}

const CODEGEN: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn incr_string(to_incr: &String) -> Option<String> {
    let mut is_incr = false;
    let rev = to_incr.chars().rev().collect::<String>();
    let mut from_incr = String::from("");

    for char in rev.chars() {
        if let Some(index) = CODEGEN.find(char) {
            if is_incr {
                from_incr.push_str(&char.to_string());
            } else if index == CODEGEN.len() - 1 {
                from_incr.push_str(&CODEGEN[0 .. 1]);
            } else {
                from_incr.push_str(&CODEGEN[index + 1 .. index + 2]);
                is_incr = !is_incr;
            }
        } else {
            return None;
        }
    }

    if !is_incr {
        None
    } else {
        Some(from_incr.chars().rev().collect::<String>())
    }
}

fn decrement_string(to_decr: &String) -> Option<String> {
    let mut is_decr = false;
    let rev = to_decr.chars().rev().collect::<String>();
    let mut from_decr = String::from("");

    for char in rev.chars() {
        if let Some(index) = CODEGEN.find(char) {
            if is_decr {
                from_decr.push_str(&char.to_string());
            } else if index == 0 {
                let len = CODEGEN.len();
                from_decr.push_str(&CODEGEN[len - 1 .. len]);
            } else {
                from_decr.push_str(&CODEGEN[index - 1 .. index]);
                is_decr = !is_decr;
            }
        } else {
            return None;
        }
    }

    if !is_decr {
        None
    } else {
        Some(from_decr.chars().rev().collect::<String>())
    }
}

impl Traversal for TreePath {
    fn right(&self) -> Option<TreePath> {
        Some(
            TreePath {
                id: self.id.clone() + &String::from(TREE_ID_BASE)
            }
        )
    }

    fn up(&self) -> Option<TreePath> {
        if self.id.len() <= 0 {
            return None;
        }

        let mut parent_id = self.id();
        let child_id = parent_id.split_off(self.len() - TREE_ID_STEP);

        incr_string(&child_id).map(|new_id| {
            TreePath {
                id: parent_id + &new_id
            }
        })
    }

    fn left(&self) -> Option<TreePath> {
        if self.level() > 0 {
            let len = self.len() - TREE_ID_STEP;

            Some(
                TreePath {
                    id: String::from(&self.id[..len])
                }
            )
        } else {
            None
        }
    }

    fn down(&self) -> Option<TreePath> {
        if self.id.len() <= 0 {
            return None;
        }

        let mut parent_id = self.id();
        let child_id = parent_id.split_off(self.len() - TREE_ID_STEP);

        decrement_string(&child_id).map(|new_id| {
            TreePath {
                id: parent_id + &new_id
            }
        })
    }
}
