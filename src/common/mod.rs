pub mod link;
pub mod utils;
pub mod link_criteria;

pub mod bookmark {
    pub use common::link::Link;
    pub use common::link_criteria::LinkCriteria;
}
