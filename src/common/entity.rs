#[derive(Debug, Clone)]
pub struct Entity {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub url: String,
    pub struct_type: String,
    pub fn_type: String,
    pub rev_no: i32
}

impl Entity {
    pub fn to_yaml(self) -> String {
        let mut btree = BTreeMap::new();

        btree.insert("id", QueryValue::Integer(self.id));
        btree.insert("parent_id", QueryValue::Integer(self.id));
        btree.insert("name", QueryValue::String(self.name));
        btree.insert("url", QueryValue::String(self.url));
        btree.insert("struct_type", QueryValue::String(self.structure_type));
        btree.insert("fn_type", QueryValue::String(self.function_type));
        btree.insert("rev_no", QueryValue::Integer(self.rev_no));

        for (k, v) in btree {
            yaml.push_str("\t");
            yaml.push_str(k);
            yaml.push_str(": ");

            let s: String = (&v).into();
            yaml.push_str(s.as_str());
            yaml.push_str("\n");
        }

        yaml
    }

    pub fn to_btree(self) -> BTreeMap<String, QueryValue> {
        let mut btree : BTreeMap<String, QueryValue> = BTreeMap::new();

        btree.insert("id", QueryValue::Integer(self.id));
        btree.insert("parent_id", QueryValue::Integer(self.id));
        btree.insert("name", QueryValue::String(self.name));
        btree.insert("url", QueryValue::String(self.url));
        btree.insert("struct_type", QueryValue::String(self.structure_type));
        btree.insert("fn_type", QueryValue::String(self.function_type));
        btree.insert("rev_no", QueryValue::Integer(self.rev_no));

        btree
    }

}

impl PartialEq for Entity {
    fn eq(&self, e: &Entity) -> bool {
        if self.id != e.id {
            return false;
        }

        if self.parent_id != e.parent_id{
            return false;
        }

        if self.name != e.name {
            return false;
        }

        if self.url != e.url {
            return false;
        }

        if self.struct_type != e.struct_type{
            return false;
        }

        if self.fn_type != e.fn_type{
            return false;
        }

        if self.rev_no != e.rev_no {
            return false;
        }

        return true;
    }
}
