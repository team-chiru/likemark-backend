#[derive(Debug, Clone)]
pub enum FnType {
    Simple,
    None,
}

impl<'a> Into<String> for &'a FnType {
    fn into(self) -> String {
        match self {
            &FnType::Simple => String::from("SIMPLE"),
            _ => String::from(""),
        }
    }
}

impl Into<String> for FnType {
    fn into(self) -> String {
        (&self).into()
    }
}

impl From<String> for FnType {
    fn from(s: String) -> FnType {
        match s.as_ref() {
            "SIMPLE" => FnType::Simple,
            _ => FnType::None,
        }
    }
}

impl PartialEq for FnType {
    fn eq(&self, st: &FnType) -> bool {
        let s: String = self.into();
        let o: String = st.into();
        s == o
    }
}

#[derive(Debug, Clone)]
pub enum StructType {
    Link,
    Node,
}

impl<'a> Into<String> for &'a StructType {
    fn into(self) -> String {
        match self {
            &StructType::Link => String::from("LINK"),
            &StructType::Node => String::from("NODE"),
        }
    }
}

impl Into<String> for StructType {
    fn into(self) -> String {
        (&self).into()
    }
}

impl From<String> for StructType {
    fn from(s: String) -> StructType {
        match s.as_ref() {
            "LINK" => StructType::Link,
            "NODE" => StructType::Node,
            _ => panic!("Type not found!"),
        }
    }
}

impl PartialEq for StructType {
    fn eq(&self, ft: &StructType) -> bool {
        let s: String = self.into();
        let o: String = ft.into();
        s == o
    }
}
