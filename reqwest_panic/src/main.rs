#[macro_use] extern crate rocket;

use rocket::{Build, Rocket, tokio};

#[get("/async")]
async fn index_async() -> &'static str {
    let handle = tokio::task::spawn_blocking(|| {
        let client = reqwest::blocking::Client::new();
        "Hello, world! Async"
    });
    let res = handle.await.unwrap();
    res
}


#[get("/")]
fn index() -> &'static str {
    let res = tokio::task::block_in_place(|| {
        let client = reqwest::blocking::Client::new();
        "Hello, world!"
    });
    // using the blocking client here will panic
    // reqwest::blocking::Client::new()
    res
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, index_async])
}
