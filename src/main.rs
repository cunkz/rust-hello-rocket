#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]             // <- path
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,hello,world,delay])
}
