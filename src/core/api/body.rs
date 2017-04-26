use serde_json::{ Value, Map };

#[derive(Serialize, Deserialize, Debug)]
pub struct JSONApi {
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub licence: String,
    pub authors: Vec<String>,
}

pub fn meta() -> Meta {
    Meta {
        licence: String::from("MIT"),
        authors: vec![
            String::from("Team Chiru, team-chiru@protonmail.com"),
        ],
    }
}

pub fn jsonapi() -> JSONApi {
    JSONApi {
        version: String::from("1.0"),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessBody {
    pub data: Option<Map<String, Value>>,
    pub meta: Meta,
    pub jsonapi: JSONApi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FailureBody {
    pub errors: Option<Map<String, Value>>,
    pub meta: Meta,
    pub jsonapi: JSONApi,
}
