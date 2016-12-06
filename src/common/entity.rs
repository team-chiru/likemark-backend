#[derive(Debug, Clone)]
pub struct Entity {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub url: String,
    pub rev_no: i32
}
