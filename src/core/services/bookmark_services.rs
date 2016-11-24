use common::utils;
use core::logic::html_parser::parse_html;

pub struct BookmarkServices {
    // TODO  initialisation of servives, singlteton, traits ?
}

impl BookmarkServices {
    pub fn import(src_path: String) -> Result<String, String> {
        match utils::dump_file(&src_path) {
            Ok(p) => match parse_html(p) {
                Ok(_) => Ok(String::from("Ok")),
                Err(msg) => Err(msg)
            },
            Err(_) => Err(format!("{}", "Destination path doesn't exist!"))
        }
    }
}
