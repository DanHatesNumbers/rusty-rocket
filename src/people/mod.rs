pub mod repository;

use super::schema::people;


#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "people"]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub profession: String,
    pub salary: i32
}

#[derive(Insertable)]
#[table_name = "people"]
pub struct InsertablePerson {
    first_name: String,
    last_name: String,
    age: i32,
    profession: String,
    salary: i32
}

impl InsertablePerson {
    pub fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            first_name: person.first_name,
            last_name: person.last_name,
            age: person.age,
            profession: person.profession,
            salary: person.salary
        }
    }
}
