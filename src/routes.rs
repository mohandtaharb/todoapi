use crate::db::*;
use crate::model::*;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/pending")]
pub fn get_pending_tasks() -> Json<Vec<Task>> {
    use crate::schema::task::dsl::*;

    let conn = &mut establish_connection();

    let result = task
        .filter(accomplished.eq(false))
        .select(Task::as_select())
        .load(conn)
        .expect("Failed to load tasks");

    Json(result)
}

#[get("/accomplished")]
pub fn get_accomplished_tasks() -> Json<Vec<Task>> {
    use crate::schema::task::dsl::*;

    let conn = &mut establish_connection();

    let result = task
        .filter(accomplished.eq(true))
        .select(Task::as_select())
        .load(conn)
        .expect("Failed to load tasks");

    Json(result)
}

#[get("/")]
pub fn get_all_tasks() -> Json<Vec<Task>> {
    use crate::schema::task::dsl::*;

    let conn = &mut establish_connection();

    let result = task
        .select(Task::as_select())
        .load(conn)
        .expect("Failed to load tasks");

    Json(result)
}

// TODO: Make it return a single task. Return a 404 if the task doesn't exist.
#[get("/<t_id>")]
pub fn get_task_by_id(t_id: i32) -> Result<Json<Task>, Status> {
    use crate::schema::task::dsl::*;

    let conn = &mut establish_connection();

    let result = task.find(t_id).first(conn);

    match result {
        Ok(t) => Ok(Json(t)),
        Err(diesel::result::Error::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

// TODO : Return an error for any extra data POSTED. Eventually return something for a successful insert
#[post("/", data = "<post>")]
pub fn insert_task(post: Json<NewTask>) -> Result<Json<Task>, Status> {
    use crate::schema::task;
    let conn = &mut establish_connection();

    let new_post = post.into_inner();

    let result = diesel::insert_into(task::table)
        .values(&new_post)
        .returning(Task::as_returning())
        .get_result(conn);

    match result {
        Ok(t) => Ok(Json(t)),
        Err(_) => Err(Status::InternalServerError)
    }
}
// TODO : Return something
#[delete("/<t_id>")]
pub fn delete_task(t_id: i32) -> Status {
    use crate::schema::task::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(task.filter(id.eq(t_id)))
        .execute(conn)
        .expect("Error deleting post");

    Status::NoContent
}
// TODO : Return something
#[put("/t_id")]
pub fn update_task() {
    // TODO
}

#[catch(404)]
pub fn not_found() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        msg: "Invalid route or ressource not found".to_string(),
    })
}

#[catch(500)]
pub fn internal_server_error() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        msg: "A server error has occured. Please contact the developer explaining exactly what you were doing".to_owned()
    })
}
