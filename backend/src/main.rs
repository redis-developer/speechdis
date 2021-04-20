#![feature(decl_macro, proc_macro_hygiene)]

use rocket::get;
use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, routes_with_openapi, JsonSchema};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, JsonSchema)]
struct Response {
    reply: String,
}

#[openapi]
#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> Json<Response> {
    if cool {
        Json(Response {
            reply: format!("You're a cool {} year old, {}!", age, name),
        })
    } else {
        Json(Response {
            reply: format!("{}, we need to talk about your coolness.", name),
        })
    }
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi_docs/openapi.json".to_string(),
        ..Default::default()
    }
}

fn main() {
    rocket::ignite()
        .mount("/openapi_docs", routes_with_openapi![hello])
        .mount("/docs", make_swagger_ui(&get_docs()))
        .launch();
}
