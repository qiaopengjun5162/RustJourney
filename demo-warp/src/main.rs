use std::{convert::Infallible, sync::Arc};

use warp::Filter;

use crate::todo_rest::todos_filter;

mod security;
mod todo_rest;

const WEB_FOLDER: &str = "web-folder/";

#[tokio::main]
async fn main() {
    let db_pool = Arc::new(DbPool {});
    // let hello_world = warp::path::end()
    //     .and(warp::get())
    //     .map(|| "Hello, world from root!");

    // APIs
    let hi = warp::path("hi")
        .and(warp::get())
        .map(|| "Hello, world from hi!");
    let apis = hi.or(todos_filter(db_pool.clone()));

    // Static Content
    let content = warp::fs::dir(WEB_FOLDER);
    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));
    let static_site = content.or(root);

    // let routes = hello_world.or(hi).or(content);
    let routes = apis.or(static_site);

    println!("start web-server");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

pub struct DbPool {}

pub fn with_db_pool(
    db_pool: Arc<DbPool>,
) -> impl Filter<Extract = (Arc<DbPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
