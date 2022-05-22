mod handlers;
mod db;
mod models;
mod error_handler;

#[rocket::launch]
fn rocket() -> _ {
    error_handler::initialize_logger();
    rocket::build()
        .attach(db::stage())
        .attach(handlers::stage())
}