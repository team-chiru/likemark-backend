extern crate regex;

use common::model::Node;
use common::entity::{ Entity, EntityBuilder };
use core::external::types::{ Tag, Attribut };

use core::external::base::Converter;
use self::regex::Regex;

pub struct Netscape {}

impl Netscape {
    pub fn sanitize(bookmark_string: String) -> String {

        let mut sanitized = bookmark_string;
        sanitized = sanitized.replace("\t"," ");
        sanitized = sanitized.replace("\r"," ");

        let mut bookmark_str: String = String::from(&sanitized[..]);

        let set = vec![
            r"(?i)(<!DOCTYPE|<META|<TITLE|<H1|<P).*\s?",
            r"(?mis)<!--.*?-->\s?",
            r"(?mis)\s?<.?br>",
        ];

        let set_unique = vec![
            r"(?mis)>(\s*?)<",
            r"@\n<br>@mis",
            r"@\n<DD@i",
        ];

        for regex in set {
            let re = Regex::new(regex).unwrap();
            let result = re.replace_all(&bookmark_str, "").to_string();
            bookmark_str = result;
        }

        let mut re_unique: Regex;

        for i in 0..3 {

            re_unique = Regex::new(set_unique[i]).unwrap();

            if i == 0{
                re_unique.replace_all(&bookmark_str, ">\n<").to_string();
            }

            else if i == 1 {
                re_unique.replace_all(&bookmark_str, "<br>").to_string();
            }

            else if i == 2 {
                re_unique.replace_all(&bookmark_str, "<DD").to_string();
            }

        }

        sanitized = bookmark_str;
        sanitized.trim();
        sanitized
    }
}

struct ExternalData {
    pub tags: Vec<Tag>,
    pub attrbutes: Vec<Attribut>,
}

impl ExternalData {
    fn new() -> Self {
        ExternalData {
            tags: Vec::new(),
            attrbutes: Vec::new(),
        }
    }
}

impl Converter for Netscape {
    fn parse(bookmark_string: String) -> Vec<Node> {
        let parse_set = vec![
            r#"(?i)([a-z]+)="([^"]*)""#,
            r#"(?i)<a.*>(.*?)</a>"#,
            r#"<[\|/]?([TITLE|H1|DL|DD|H3]+)?>"#,
            r#"<[/]([TITLE|H1|DL|DD|H3]+)?>"#,
        ];

        let bookmark_string = Netscape::sanitize(bookmark_string);

        let lines: Vec<&str> = bookmark_string.split("\n").collect();
        let attribut_regex = Regex::new(parse_set[0]).unwrap();
        let content_regex = Regex::new(parse_set[1]).unwrap();
        let tag_regex = Regex::new(parse_set[2]).unwrap();
        let end_tag_regex = Regex::new(parse_set[3]).unwrap();

        let data = ExternalData::new();
        let root = EntityBuilder::default();

        // create hierarchy for external data
        for line in lines {
            println!("tag:");
            for capture in tag_regex.captures_iter(&line) {
                println!("{}", &capture[1]);

            }

            println!("end tag:");
            for capture in end_tag_regex.captures_iter(&line) {
                println!("{}", &capture[1]);

            }

            println!("link:");
            for capture in attribut_regex.captures_iter(&line) {
                println!("{:?} = {:?}", &capture[1], &capture[2]);

            }
            println!("\n");

            println!("content:");
            for capture in content_regex.captures_iter(&line){
                println!("{:?}", &capture[1]);
            }
            println!("\n");
        }

        //

        vec![]
    }

    fn build(nodes: Vec<Node>) -> String {
        format!("{:?}", nodes)
    }
}
