#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::fs::{relative, FileServer};
use std::path::Path;

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("../frontend/dist/index.html"))
        .await
        .ok()
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", FileServer::from(relative!("../frontend/dist")))
}
