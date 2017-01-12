use common::entity_t::Entity;
use common::types::StructType;
use common::types::FnType;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: i32,
    pub parent_id: String,
    pub name: String,
    pub url: String,
    pub struct_type: StructType,
    pub fn_type: FnType,
    pub rev_no: i32,
}

impl Entity for Link {
    fn id(&self) -> i32 {
        self.id
    }

    fn parent_id(&self) -> String {
        self.parent_id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn url(&self) -> String {
        self.url.clone()
    }

    fn struct_type(&self) -> StructType {
        self.struct_type.clone()
    }

    fn fn_type(&self) -> FnType {
        self.fn_type.clone()
    }

    fn rev_no(&self) -> i32 {
        self.rev_no.clone()
    }
}
