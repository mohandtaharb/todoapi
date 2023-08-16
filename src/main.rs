#[macro_use]
extern crate rocket;

pub mod db;
pub mod schema;
pub mod tasks;
pub mod user;

use tasks::routes::*;

use dotenvy::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};

use user::auth::login;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(CORS)
        .register(
            "/",
            catchers![not_found, internal_server_error, unauthorized, bad_request],
        )
        .mount(
            "/tasks",
            routes![
                get_pending_tasks,
                get_accomplished_tasks,
                get_all_tasks,
                get_task_by_id,
                insert_task,
                delete_task,
                accomplish,
                unaccomplish,
                change_task_title
            ],
        )
        .mount("/user", routes![login])
}
