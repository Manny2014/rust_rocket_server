/*
    Rocket Docs:
    - https://rocket.rs/v0.5-rc/guide/configuration/
*/

use rocket::launch;
use car::{Car, CarTransmission, CarLot, CarLine};
use rocket::serde::Deserialize;
use rocket::{State, Config};
use rocket::fairing::AdHoc;

#[macro_use] extern crate rocket;
mod car;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    port: u16,
}

#[get("/")]
pub fn car_lot() -> String {
   let data = CarLot::new("Chicago")
    .add(
        Car::new()
            .make("BMW")
            .model("3")
            .transmission(CarTransmission::Manual)
            .line(CarLine::Lux)
            .build())
    .add(
        Car::new()
        .make("BMW")
        .model("5")
        .transmission(CarTransmission::Automatic)
        .line(CarLine::Lux)
        .build())
    .add(
        Car::new()
        .make("Honda")
        .model("CRV")
        .transmission(CarTransmission::Automatic)
        .line(CarLine::Mid)
        .build())
    .to_string();
    
    data
}

#[get("/")]
fn read_config(rocket_config: &Config, app_config: &State<AppConfig>) -> String {
    format!("{:#?}\n{:#?}", app_config, rocket_config)
}

#[launch]
pub fn rocket() -> _ {

    let rocket = rocket::build()
        .mount("/", routes![read_config])
        .mount("/lot", routes![car_lot])
        .attach(AdHoc::config::<AppConfig>());

    rocket
}



#[cfg(test)]
mod test {
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::config::{Config, LogLevel};

    #[test]
    fn root_path() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}