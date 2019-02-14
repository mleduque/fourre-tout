#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod db;
mod model;
mod schema;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json};

use model::{Model};
use schema::models::dsl::*;

use db::DbConn;

#[get("/")]
fn root() -> &'static str {
    "Nothing here"
}

#[get("/models")]
fn list_models(connection: DbConn) -> Result<Json<Vec<Model>>, Status> {
    match models.load(&*connection) {
        Err(_error) => Err(Status::InternalServerError),
        Ok(result) => Ok(Json(result))
    }
}

fn main() {
    rocket::ignite().manage(db::init_pool()).mount("/", routes![root, list_models]).launch();
}
