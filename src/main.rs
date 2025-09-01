#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rocket_cors::CorsOptions;

#[derive(Serialize)]
struct Greetings {
    buongiorno: String
}

#[get("/buongiorno/<name>")]
fn specific_buongiorno(name: &str) -> Json<Greetings> {
    Json(Greetings{
        buongiorno: name.to_string()
    })
}

#[get("/buongiorno")]
fn generic_buongiorno() -> Json<Greetings> {
    Json(Greetings{
        buongiorno: "principessa".to_string()
    })
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default().to_cors().expect("Failed to create CORS fairing.");
    rocket::build().mount("/", routes![
        specific_buongiorno,
        generic_buongiorno
    ]).attach(cors)
}
