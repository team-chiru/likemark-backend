enum Attribut {
    Href(String),
    Date(String),
    Private(String),
    Tags(String),
    Content(String),
    None
}

impl<'a> Into<String> for &'a Attribut {
    fn into(self) -> String {
        match self {
            &BookmarkAttribut::HREF => String::from("HREF"),
            &BookmarkAttribut::DATE => String::from("DATE"),
            &BookmarkAttribut::PRIVATE => String::from("PRIVATE"),
            &BookmarkAttribut::TAGS => String::from("TAGS"),
            &BookmarkAttribut::CONTENT => String::from("CONTENT"),
            &BookmarkAttribut::None => String::from("")
        }
    }
}


impl From<String> for BookmarkAttribut {
    fn from(s: String) -> BookmarkAttribut {
        match s.to_uppercase().as_ref() {
            "HREF" =>  BookmarkAttribut::HREF,
            "DATE" => BookmarkAttribut::DATE,
            "PRIVATE" => BookmarkAttribut::PRIVATE,
            "TAGS" => BookmarkAttribut::TAGS,
            "CONTENT" => BookmarkAttribut::CONTENT,
            _ => BookmarkAttribut::None
        }
    }
}


enum BookmarkTag {
    P,
    H3,
    A,
    TITLE,
    BR,
    DL,
    DT,
    DD
}
