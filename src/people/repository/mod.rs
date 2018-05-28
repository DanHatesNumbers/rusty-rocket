use diesel;
use diesel::prelude::*;
use super::super::schema::people;

use super::super::people::{Person, InsertablePerson};

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Person>> {
    people::table.load::<Person>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Person> {
    people::table.find(id).get_result::<Person>(connection)
}

pub fn insert(person: Person, connection: &PgConnection) -> QueryResult<Person> {
    diesel::insert_into(people::table)
        .values(&InsertablePerson::from_person(person))
        .get_result(connection)
}

pub fn update(id: i32, person: Person, connection: &PgConnection) -> QueryResult<Person> {
    diesel::update(people::table.find(id))
        .set(&person)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(people::table.find(id))
        .execute(connection)
}