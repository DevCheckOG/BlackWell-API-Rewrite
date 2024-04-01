/* BlackWell API Rust Rewrite */

#[macro_use]
extern crate rocket;

use dotenv;
use rocket::{routes, Config, Ignite, Rocket};

mod core;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    core::db::init().await;

    tokio::spawn(core::db::clear_temporal_accounts());
    tokio::spawn(core::db::clear_queue_messages());

    let _ = dotenv::from_path("src/.env");

    let _rocket: Rocket<Ignite> = rocket::build()
        .mount(
            "/",
            routes![
                core::index,
                core::register_acc,
                core::verify_acc,
                core::unregister_acc,
                core::login_acc,
                core::set_profile_acc,
                core::get_profile_acc,
                core::action,
                core::queue
            ],
        )
        .register(
            "/",
            catchers![
                core::not_found,
                core::unprocessable_entity,
                core::too_many_requests
            ],
        )
        .configure(Config {
            address: std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
            port: 8000,
            workers: 4,
            ..Config::default()
        })
        .launch()
        .await?;

    Ok(())
}
