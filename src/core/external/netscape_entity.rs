use common::types::StructType;
use common::types::FnType;
use common::TreePath;

#[derive(Debug, Clone)]
pub struct NetscapeEntity {
    id: i32,
    path: TreePath,
    name: String,
    url: String,
    struct_type: StructType,
    fn_type: FnType,
    rev_no: i32,
}

impl NetscapeEntity {
    /*********
    GETTER
    **********/
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_path(&self) -> &TreePath{
        &self.path
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn get_struct_type(&self) -> &StructType {
        &self.struct_type
    }

    pub fn get_fn_type(&self) -> &FnType {
        &self.fn_type
    }

    pub fn get_rev_no(&self) -> i32 {
        self.rev_no
    }

    /*********
    SETTER
    **********/

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_path(&mut self, path: TreePath) {
        self.path = path;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_struct_type(&mut self, struct_type: StructType) {
        self.struct_type = struct_type;
    }

    pub fn set_fn_type(&mut self, fn_type: FnType) {
        self.fn_type = fn_type;
    }

    pub fn set_rev_no(&mut self, rev_no: i32) {
        self.rev_no = rev_no;
    }

}
