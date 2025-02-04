use rocket::Route;
pub mod auth;


pub fn auth_routes() -> Vec<Route> {
    auth::routes()
}