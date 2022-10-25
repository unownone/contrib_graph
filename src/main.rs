use contrib_graph::github;
use serde_json;
#[macro_use]
extern crate rocket;

#[get("/<username>")]
async fn get_data(username: String) -> String {
    println!("Running");
    let user = &mut github::GithubUrl::new(username);
    let _response = user.get_repos().await;
    let forked_repos = user.get_forked_repos();
    serde_json::to_string_pretty(&forked_repos).unwrap()
}

#[get("/")]
async fn index() -> String {
    "Hello World".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_data])
}
