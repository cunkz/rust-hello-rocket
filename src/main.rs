#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::serde::{Deserialize, json::Json};
use rocket::form::Form;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TaskV1<'r> {
    description: &'r str,
    complete: bool
}

#[derive(FromForm)]
struct TaskV2<'r> {
    description: &'r str,
    complete: bool,
    r#type: &'r str,
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

#[post("/todo/v1", data = "<task>")]
fn todo_v1(task: Json<TaskV1<'_>>) -> String {
    format!("Description: {}, Complete: {}", task.description, task.complete)
}

#[post("/todo/v2", data = "<task>")]
fn todo_v2(task: Form<TaskV2<'_>>) -> String {
    format!("Description: {}, Complete: {}, Type: {}", task.description, task.complete, task.r#type)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,hello,world,delay,todo_v1,todo_v2])
}
