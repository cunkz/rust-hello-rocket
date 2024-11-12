#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    complete: bool
}

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

#[post("/todo", data = "<task>")]
fn new_todo(task: Json<Task<'_>>) -> String {
    format!("Description: {}, Complete: {}", task.description, task.complete)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,hello,world,delay,new_todo])
}
