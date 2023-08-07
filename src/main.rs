#[macro_use]
extern crate rocket;

pub mod db;
pub mod model;
pub mod routes;
pub mod schema;
use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            get_pending_tasks,
            get_accomplished_tasks,
            get_all_tasks,
            get_task_by_id,
            insert_task,
            delete_task
        ],
    )
}
