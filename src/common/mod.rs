pub mod utils;

pub mod entity;
pub mod criteria;

pub mod bookmark {
    type Link = Entity;
    type Folder = Entity;

    pub use common::entity::Entity;
    pub use common::criteria::Criteria;
}
