
use crate::server::controller;

pub fn run() {
    rocket::ignite()
        .mount("/",
               routes![
                    controller::index,
                    controller::repo,
                    controller::add_repo,
                    controller::sync],
        ).launch();
}