
use crate::server::controller;

pub fn run() {
    rocket::ignite()
        .mount("/services/gtm/sync//",
               routes![
                    controller::repo,
                    controller::add_repo,
                    controller::sync_repo,
                    controller::sync_all],
        ).launch();
}