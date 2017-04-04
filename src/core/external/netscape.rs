extern crate regex;

use common::node::Node;

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

impl Converter for Netscape {
    fn parse(bookmark_string: String) -> Vec<Node> {
        let bookmark_string = Netscape::sanitize(bookmark_string);
        let href_regex = r#"(?i)([a-z]+)="([^"]*)""#;
        println!("{}\n", href_regex);

        let content_regex = r#"(?i)<a.*>(.*?)</a>"#;
        println!("{}\n", content_regex);

        let lines: Vec<&str> = bookmark_string.split("\n").collect();
        let re_href = Regex::new(href_regex).unwrap();
        let re_content = Regex::new(content_regex).unwrap();
        for line in lines {
            println!("link:");
            for capture in re_href.captures_iter(&line) {
                println!("{:?} = {:?}", &capture[1], &capture[2]);
            }
            println!("\n");

            println!("content:");
            for capture in re_content.captures_iter(&line){
                println!("{:?}", &capture[1]);
            }
            println!("\n");
            println!("test auth");
        }

        vec![]
    }

    fn build(nodes: Vec<Node>) -> String {
        format!("{:?}", nodes)
    }
}
