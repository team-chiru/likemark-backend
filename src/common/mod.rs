pub mod link;
pub mod node;
pub mod utils;
pub mod link_criteria;

pub mod bookmark {
    pub use common::link::Link;
    pub use common::node::Node;
    pub use common::link_criteria::LinkCriteria;
}
