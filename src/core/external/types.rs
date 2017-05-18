use html5ever::QualName;

pub enum Attribut {
    Href,
    AddDate,
    Private,
    Tags,
    LastModified,
    None
}

impl<'a> From<&'a QualName> for Attribut {
    fn from(name: &'a QualName) -> Attribut {
        match name.local.as_ref() {
            "HREF" => Attribut::Href,
            "ADD_DATE" => Attribut::AddDate,
            "PRIVATE" => Attribut::Private,
            "TAGS" => Attribut::Tags,
            "LAST_MODIFIED" => Attribut::LastModified,
            _ => Attribut::None
        }
    }
}

pub enum Tag {
    H3,
    H1,
    A,
    TITLE,
    DL,
    DT,
    DD,
    None
}

impl<'a> From<&'a QualName> for Tag {
    fn from(name: &'a QualName) -> Tag {
        let tag_name = name.local.as_ref().to_uppercase();

        match tag_name.as_str() {
            "H3" => Tag::H3,
            "H1" => Tag::H1,
            "A" => Tag::A,
            "TITLE" => Tag::TITLE,
            "DL" => Tag::DL,
            "DT" => Tag::DT,
            "DD" => Tag::DD,
            _ => Tag::None
        }
    }
}
