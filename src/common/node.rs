use common::bookmark::Link;

pub struct Node {
    pub links: Vec<Link>,
    pub node: Box<FnMut() -> Node>
}

impl Node {
    pub fn hello(self) {
        println!("coucou je suis un node");
    }

    // NODE IMPLEMENTATION
}
