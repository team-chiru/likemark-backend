use common::deep::tree_path::{ TreePath, TREE_ID_STEP, TREE_ID_BASE };
use std::marker;

trait Traversal {
    fn right(&self) -> Option<Self> where Self: marker::Sized;
    fn up(&self) -> Option<Self> where Self: marker::Sized;
    fn left(&self) -> Option<Self> where Self: marker::Sized;
    fn down(&self) -> Option<Self> where Self: marker::Sized;
}

pub const TREE_ID_LOW_BOUND: &'static str = "0";
pub const TREE_ID_HIGH_BOUND: &'static str = "z";

fn incr(to_incr: &String) -> Option<String> {
    let mut is_incr = false;
    let rev = to_incr.chars().rev().collect::<String>();
    let mut from_incr = String::from("");

    for byte in rev.as_bytes() {
        let old = match String::from_utf8( vec![*byte] ) {
            Ok(s) => s,
            Err(_) => return None
        };

        if is_incr {
            from_incr.push_str(&old);
        } else if old == TREE_ID_HIGH_BOUND.to_string() {
            from_incr.push_str(TREE_ID_LOW_BOUND);
        } else if let Ok(new) = String::from_utf8( vec![*byte + 1] ) {
            from_incr.push_str(&new);
            is_incr = !is_incr;
        }
    }

    if !is_incr {
        None
    } else {
        Some(from_incr.chars().rev().collect::<String>())
    }
}

fn decr(to_decr: &String) -> Option<String> {
    let mut is_decr = false;
    let rev = to_decr.chars().rev().collect::<String>();
    let mut from_decr = String::from("");

    for byte in rev.as_bytes() {
        let old = match String::from_utf8( vec![*byte] ) {
            Ok(s) => s,
            Err(_) => return None
        };

        if is_decr {
            from_decr.push_str(&old);
        } else if old == TREE_ID_LOW_BOUND.to_string() {
            from_decr.push_str(TREE_ID_HIGH_BOUND);
        } else if let Ok(new) = String::from_utf8( vec![*byte - 1] ) {
            from_decr.push_str(&new);
            is_decr = !is_decr;
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

        match incr(&child_id) {
            Some(new_id) => Some(
                TreePath {
                    id: parent_id + &new_id
                }
            ),
            None => None
        }
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

        match decr(&child_id) {
            Some(new_id) => Some(
                TreePath {
                    id: parent_id + &new_id
                }
            ),
            None => None
        }
    }
}
