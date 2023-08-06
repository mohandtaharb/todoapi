use crate::db::*;
use crate::model::{NewTask, Task};
use diesel::prelude::*;
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
pub fn get_task_by_id(t_id: i32) -> Json<Vec<Task>> {
    use crate::schema::task::dsl::*;

    let conn = &mut establish_connection();

    let result: Vec<Task> = task
        .filter(id.eq(t_id))
        .select(Task::as_select())
        .load(conn)
        .expect("Failed to load tasks");

    Json(result)
}

// TODO : Return an error for any extra data POSTED. Eventually return something for a successful insert
#[post("/new", data = "<post>")]
pub fn insert_task(post: Json<NewTask>) {
    use crate::schema::task;
    let conn = &mut establish_connection();

    let new_post = post.into_inner();

    diesel::insert_into(task::table)
        .values(&new_post)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("Failed to save new post");
}
// TODO : Return something
#[delete("/<t_id>")]
pub fn delete_task(t_id: i32) {
    use crate::schema::task::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(task.filter(id.eq(t_id)))
        .execute(conn)
        .expect("Error deleting post");
}
// TODO : Return something
#[put("/t_id")]
pub fn update_task() {
    // TODO
}