#![allow(non_snake_case)]
use crate::db::*;

use super::model::*;
use crate::user::auth::AuthenticationToken;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/pending")]
pub fn get_pending_tasks(_token: AuthenticationToken) -> Json<Vec<Task>> {
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
pub fn get_accomplished_tasks(_token: AuthenticationToken) -> Json<Vec<Task>> {
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
pub fn get_all_tasks(_token: AuthenticationToken) -> Json<Vec<Task>> {
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
pub fn get_task_by_id(_token: AuthenticationToken, t_id: i32) -> Result<Json<Task>, Status> {
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
pub fn insert_task(_token: AuthenticationToken, post: Json<NewTask>) -> Result<Json<Task>, Status> {
    use crate::schema::task;
    let conn = &mut establish_connection();

    let new_post = post.into_inner();

    let result = diesel::insert_into(task::table)
        .values(&new_post)
        .returning(Task::as_returning())
        .get_result(conn);

    match result {
        Ok(t) => Ok(Json(t)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<t_id>")]
pub fn delete_task(_token: AuthenticationToken, t_id: i32) -> Status {
    use crate::schema::task::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(task.filter(id.eq(t_id)))
        .execute(conn)
        .expect("Error deleting post");

    Status::NoContent
}

#[get("/<t_id>/accomplish")]
pub fn accomplish(_token: AuthenticationToken, t_id: i32) -> (Status, Option<Json<Task>>) {
    use crate::schema::task::dsl::{accomplished, task};

    let conn = &mut establish_connection();

    let result: QueryResult<Task> = diesel::update(task.find(t_id))
        .set(accomplished.eq(true))
        .returning(Task::as_returning())
        .get_result(conn);

    match result {
        Ok(t) => (Status::Ok, Some(Json(t))),
        Err(diesel::result::Error::NotFound) => (Status::NotFound, None),
        Err(e) => {
            println!("{}", e);
            (Status::InternalServerError, None)
        }
    }
}

#[get("/<t_id>/unaccomplish")]
pub fn unaccomplish(_token: AuthenticationToken, t_id: i32) -> (Status, Option<Json<Task>>) {
    use crate::schema::task::dsl::{accomplished, task};

    let conn = &mut establish_connection();

    let result: QueryResult<Task> = diesel::update(task.find(t_id))
        .set(accomplished.eq(false))
        .returning(Task::as_returning())
        .get_result(conn);

    match result {
        Ok(t) => (Status::Ok, Some(Json(t))),
        Err(diesel::result::Error::NotFound) => (Status::NotFound, None),
        Err(e) => {
            println!("{}", e);
            (Status::InternalServerError, None)
        }
    }
}

#[patch("/<t_id>", data = "<new_title>")]
pub fn change_task_title(
    _token: AuthenticationToken,
    t_id: i32,
    new_title: Json<NewTask<'_>>,
) -> (Status, Option<Json<Task>>) {
    use crate::schema::task::dsl::{name, task};
    let new_title = new_title.into_inner();
    let conn = &mut establish_connection();

    let result: QueryResult<Task> = diesel::update(task.find(t_id))
        .set(name.eq(new_title.name))
        .returning(Task::as_returning())
        .get_result(conn);

    match result {
        Ok(t) => (Status::Ok, Some(Json(t))),
        Err(diesel::result::Error::NotFound) => (Status::NotFound, None),
        Err(e) => {
            println!("{}", e);
            (Status::InternalServerError, None)
        }
    }
}

#[catch(404)]
pub fn not_found() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        msg: "The requested ressource has not been found".to_string(),
    })
}

#[catch(500)]
pub fn internal_server_error() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        msg: "An internal server error has occured".to_string()
    })
}

#[catch(401)]
pub fn unauthorized() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        msg: "The request requires user authentication.".to_string(),
    })
}

#[catch(400)]
pub fn bad_request() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        msg: "The request could not be understood by the server due to malformed syntax.".to_string()
    })
}

#[catch(422)]
pub fn unprocessable_entity() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        msg: "We couldn't process your request due to invalid data. Please check the documentation".to_string()
    })
}