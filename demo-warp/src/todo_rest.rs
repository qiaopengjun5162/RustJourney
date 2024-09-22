use std::sync::Arc;

use serde_json::{json, Value};
use warp::{reply::Json, Filter};

use crate::{
    security::{do_auth, UserCtx},
    with_db_pool, DbPool,
};

pub fn todos_filter(
    db_pool: Arc<DbPool>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let todos_base = warp::path("todos");
    // List todos
    // warp::path("todos")
    let list = todos_base
        .and(warp::get())
        .and(warp::path::end())
        // 同步 map  异步 and_then
        // .map(|| "will get todos")
        // .and_then(|| async { Ok::<&str, warp::Rejection>("async will get todos") })
        // .and_then(todo_list)
        // .and(check_auth().untuple_one())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and_then(todo_list_async);

    let get = todos_base
        .and(warp::get())
        // .and(check_auth().untuple_one())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::path::param()) // e.g., /todos/123
        .and_then(todo_get);

    let create = todos_base
        .and(warp::post())
        // .and(check_auth().untuple_one())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::body::json())
        .and_then(todo_create);

    list.or(get).or(create)
}

#[allow(dead_code)]
async fn todo_list() -> Result<String, warp::Rejection> {
    Ok("todo list".to_string())
}

async fn todo_list_async(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
) -> Result<Json, warp::Rejection> {
    // TODO - get from DB
    let todos = json!([
        { "id": 1, "title": "Buy milk" },
        { "id": 2, "title": "Buy eggs" }
    ]);
    let todos = warp::reply::json(&todos);
    Ok(todos)
}

async fn todo_get(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
    id: i64,
) -> Result<Json, warp::Rejection> {
    // TODO - get from DB
    // let todo = json!({ "id": id, "title": format!("todo {id}") });
    let todo = json!({ "id": id, "user_id": _user_ctx.user_id, "title": format!("todo {id}") });
    // serde-json to warp-reply
    let todo = warp::reply::json(&todo);
    Ok(todo)
}

async fn todo_create(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
    data: Value,
) -> Result<Json, warp::Rejection> {
    // TODO - write to DB
    let todo = data;
    let todo = warp::reply::json(&todo);
    Ok(todo)
}
