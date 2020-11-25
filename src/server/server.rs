
mod controller;
mod service;

pub fn run() {
    rocket::ignite()
        .mount("/",
               routes![
                    controller::index,
                    controller::repo,
                    controller::add_repo],
        ).launch();
}