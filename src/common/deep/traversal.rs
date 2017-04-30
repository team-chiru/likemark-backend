use common::deep::tree_path::{ TreePath, TREE_ID_STEP, TREE_ID_BASE };
use std::marker;

trait Traversal {
    fn right(&self) -> Option<Self> where Self: marker::Sized;
    fn up(&self) -> Option<Self> where Self: marker::Sized;
    fn left(&self) -> Option<Self> where Self: marker::Sized;
    fn down(&self) -> Option<Self> where Self: marker::Sized;
}

fn incr(to_incr: &String) -> Option<String> {
    unimplemented!()
}

fn decr(to_decr: &String) -> Option<String> {
    unimplemented!()
}

/*
fn to_ascii(i: &i32) -> String {
    match *i {
        x@0...127 => format!("{:?}", x as u8 as char),
        _ => "".into(),
    }
}

fn main() {
    for x in 0x30..0x7A {
        print!("{} ", to_ascii(&x));
    }
    println!("");
    //'0' '1' '2' '3' '4' '5' '6' '7' '8' '9' ':' ';' '<' '=' '>' '?'
}
*/


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
