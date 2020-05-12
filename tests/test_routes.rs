extern crate rocket;

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn index_route() {
        let client = Client::new(rocket_api::rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn correct_week_day_route() {
        let client = Client::new(rocket_api::rocket()).expect("valid rocket instance");
        let mut response = client.get("/2").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Week day 2 is Tuesday.".into()));
    }

    #[test]
    fn wrong_week_day_route() {
        let client = Client::new(rocket_api::rocket()).expect("valid rocket instance");
        let mut response = client.get("/9").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Week day 9 does not exist.".into()));
    }
}
