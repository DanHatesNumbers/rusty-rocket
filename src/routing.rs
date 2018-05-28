use rocket;

use handlers;
use rustyrocket_lib::connection_pool;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection_pool::init_pool())
        .mount("/people",
            routes![
                handlers::all,
                handlers::get,
                handlers::post,
                handlers::put,
                handlers::delete
            ])
        .launch();
}