extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;

use rustyrocket_lib::connection_pool::DbConn;
use diesel::result::Error;
use std::env;
use rustyrocket_lib::people;
use rustyrocket_lib::people::Person;
use rocket::http::Status;
use rocket::response::{Failure, status};
use rocket_contrib::Json;

fn error_status(error: Error) -> Failure {
    Failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    })
}

#[get("/")]
fn all(connection: DbConn) -> Result<Json<Vec<Person>>, Failure> {
    people::repository::all(&connection)
        .map(|people| Json(people))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn get(id: i32, connection:DbConn) -> Result<Json<Person>, Failure> {
    people::repository::get(id, &connection)
        .map(|person| Json(person))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data="<person>")]
fn post(person: Json<Person>, connection: DbConn) -> Result<status::Created<Json<Person>>, Failure> {
    people::repository::insert(person.into_inner(), &connection)
        .map(|person| person_created(person))
        .map_err(|error| error_status(error))
}

fn person_created(person: Person) -> status::Created<Json<Person>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!("{host}:{port}/people/{id}", host = host, port = port, id = person.id).to_string(),
        Some(Json(person))
    )
}

#[put("/<id>", format = "application/json", data = "<person>")]
fn put(id: i32, person: Json<Person>, connection: DbConn) -> Result<Json<Person>, Failure> {
    people::repository::update(id, person.into_inner(), &connection)
        .map(|person| Json(person))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
fn delete(id: i32, connection: DbConn) -> Result<status::NoContent, Failure> {
    match people::repository::get(id, &connection) {
        Ok(_) => people::repository::delete(id, &connection)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}