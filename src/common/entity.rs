use common::types::StructType;
use common::types::FnType;
use common::TreePath;
use std::marker;

//struct EntityBuilder {}

#[derive(Debug, Clone, Default, Builder)]
#[builder(field(private))]
pub struct Entity {
    #[builder(default="None")]
    pub id: Option<i32>,

    #[builder(default="None")]
    pub uuid: Option<String>,

    #[builder(default="None")]
    pub path: Option<TreePath>,

    #[builder(default="None")]
    pub name: Option<String>,

    #[builder(default="None")]
    pub url: Option<String>,

    #[builder(default="None")]
    pub struct_type: Option<StructType>,

    #[builder(default="None")]
    pub fn_type: Option<FnType>,

    #[builder(default="None")]
    pub rev_no: Option<i32>
}
