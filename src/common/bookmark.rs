extern crate time;
extern crate yaml_rust;

use self::time::Timespec;
use self::yaml_rust::Yaml;
use std::collections::BTreeMap;


#[derive(Debug)]
pub struct Bookmark {
    id: i32,
    name: String,
    time_created: Timespec,
    url: String,
    stamp: Timespec,
    rev_no: i32

}

impl Bookmark{
    fn to_yaml(&self) -> String{
        let mut btree = BTreeMap::new();

        btree.insert("id", self.id);
        btree.insert("name", self.name);
        btree.insert("time_created", self.time_created);
        btree.insert("url", self.url);
        btree.insert("stamp",self.stamp);
        btree.insert("rev_no",self.rev_no);

        let yaml = String::new();
        yaml.push_str("Bookmark: \n");

        for (k,v) in btree {
            yaml.push_str(k + ":" + v + "\n");
        }

    }
}
