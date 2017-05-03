use html5ever::QualName;

pub enum Attribut {
    Href(String),
    Date(String),
    Private(String),
    Tags(String),
    Content(String),
    None
}

impl<'a> Into<(String, String)> for &'a Attribut {
    fn into(self) -> (String, String) {
        match self {
            &Attribut::Href(ref s) => (String::from("HREF"), s.clone()),
            &Attribut::Date(ref s) => (String::from("DATE"), s.clone()),
            &Attribut::Private(ref s) => (String::from("PRIVATE"), s.clone()),
            &Attribut::Tags(ref s) => (String::from("TAGS"), s.clone()),
            &Attribut::Content(ref s) => (String::from("CONTENT"), s.clone()),
            &Attribut::None => (String::from(""), String::from(""))
        }
    }
}


impl From<(String, String)> for Attribut {
    fn from(t: (String, String)) -> Attribut {
        let (ref key, ref value) = t;
        match (key.as_str(), value.clone()) {
            ("HREF", v) =>  Attribut::Href(v),
            ("DATE", v) => Attribut::Date(v),
            ("PRIVATE", v) => Attribut::Private(v),
            ("TAGS", v) => Attribut::Tags(v),
            ("CONTENT", v) => Attribut::Content(v),
            (_, _) => Attribut::None
        }
    }
}

pub enum Tag {
    P,
    H3,
    H1,
    A,
    TITLE,
    DL,
    DT,
    DD
}

impl<'a> From<&'a QualName> for Tag {
    fn from(name: &'a QualName) -> Tag {
        println!("{:?}", name.local);
        match name.local.as_ref() {
            "DL" => Tag::DL,
            _ => Tag::DT
        }
    }
}
