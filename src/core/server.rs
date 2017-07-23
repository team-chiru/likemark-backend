extern crate valico;

use iron::prelude::*;
use iron::method::Method;
use iron::status::Status;

use serde_json;
use self::valico::json_dsl;

use super::api::{ header, body };
use super::api::{ RequestRule, ResponseFormater };

use rustless::{
    Application, Api, Nesting, Versioning
};
use rustless::json::ToJson;



fn get_body() -> String {
    let body = body::SuccessBody {
        data: None,
        meta: body::meta(),
        jsonapi: body::jsonapi(),
    };

    serde_json::to_string(&body).unwrap()
}

pub fn serve() {


    let api = Api::build(|api| {
        // Specify API version
        api.version("v1", Versioning::AcceptHeader("chat"));
        api.prefix("api");

        // Create API for chats
        api.mount(Api::build(|chats_api| {

            chats_api.after(|client, _params| {
                    client.set_status(NotFound);
                    Ok(())
            });


            // Add namespace
            chats_api.namespace("chats/:id", |chat_ns| {

                // Valico settings for this namespace
                chat_ns.params(|params| {
                    params.req_typed("id", json_dsl::u64())
                });

                // Create endpoint for POST /chats/:id/users/:user_id
                chat_ns.post("users/:user_id", |endpoint| {

                    // Add description
                    endpoint.desc("Update user");

                    // Valico settings for endpoint params
                    endpoint.params(|params| {
                        params.req_typed("user_id", json_dsl::u64());
                        params.req_typed("id", json_dsl::string())
                    });

                    endpoint.handle(|client, params| {
                        client.json(&params.to_json())
                    })
                });

            });
        }));
    });


    let app = Application::new(api);

    Iron::new(app).http("localhost:3000").unwrap();


    /*Iron::new(|req: &mut Request| {
        let mut resp = match req.method {
            Method::Get =>
                Response::with((
                    header::Matcher::check(req),
                    get_body()
                )),
            Method::Options => Response::with(Status::Ok),
            _ => Response::with(Status::MethodNotAllowed),
        };

        header::Matcher::format(&mut resp);
        Ok(resp)

    }).http("localhost:3000").unwrap();
    */
}
